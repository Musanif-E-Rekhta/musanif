//! GraphQL subscription client over the `graphql-transport-ws` protocol.
//!
//! Wired on both targets:
//!   - **wasm32 (web/admin):** `gloo-net::websocket::futures::WebSocket`.
//!   - **native (desktop/mobile):** `async-tungstenite::tokio::connect_async`.
//!
//! Wire format (https://github.com/enisdenjo/graphql-ws/blob/master/PROTOCOL.md):
//!   client → `connection_init` (payload may carry auth headers)
//!   server → `connection_ack`
//!   client → `subscribe` (id, payload: { query, variables })
//!   server → `next` ×N    (id, payload: { data, errors? })
//!   server → `complete`   (id) — terminal
//!   client → `complete`   (id) — to disconnect early
//!
//! `subscribe_job_events` returns once the server completes the stream
//! or the WebSocket closes. The caller is expected to spawn it via
//! Dioxus's `spawn` / `use_future` and let it run for the lifetime of
//! the view.

use crate::graphql::admin::{JobEventSub, JobEventsSubscription, JobVars};

/// Public callback shape: receives every successful `next` payload as a
/// typed `JobEventSub`. Errors are logged at debug and skipped.
pub type JobEventCallback = Box<dyn FnMut(JobEventSub) + 'static>;

#[cfg(target_arch = "wasm32")]
pub use wasm_impl::subscribe_job_events;

#[cfg(not(target_arch = "wasm32"))]
pub use native_impl::subscribe_job_events;

#[cfg(not(target_arch = "wasm32"))]
mod native_impl {
    use async_tungstenite::tokio::connect_async;
    use async_tungstenite::tungstenite::client::IntoClientRequest;
    use async_tungstenite::tungstenite::http::HeaderValue;
    use async_tungstenite::tungstenite::protocol::Message;
    use cynic::SubscriptionBuilder;
    use futures_util::{SinkExt, StreamExt};
    use serde::Deserialize;
    use serde_json::{json, Value};

    use super::{JobEventCallback, JobEventSub, JobEventsSubscription, JobVars};
    use crate::api::get_auth_token;
    use crate::config;

    const SUB_ID: &str = "1";

    fn ws_url() -> String {
        let http = config::graphql_url();
        let base = if let Some(rest) = http.strip_prefix("https://") {
            format!("wss://{rest}")
        } else if let Some(rest) = http.strip_prefix("http://") {
            format!("ws://{rest}")
        } else {
            http.to_string()
        };
        format!("{base}/ws")
    }

    pub async fn subscribe_job_events(job: String, mut on_event: JobEventCallback) {
        let url = ws_url();

        // Build a request that advertises the `graphql-transport-ws`
        // subprotocol. tungstenite's `IntoClientRequest` for &str does
        // not set this header, so we wrap it.
        let mut request = match url.as_str().into_client_request() {
            Ok(r) => r,
            Err(e) => {
                tracing::debug!(error = %e, "WS request build failed");
                return;
            }
        };
        request.headers_mut().insert(
            "Sec-WebSocket-Protocol",
            HeaderValue::from_static("graphql-transport-ws"),
        );

        let (mut ws, _resp) = match connect_async(request).await {
            Ok(conn) => conn,
            Err(e) => {
                tracing::debug!(error = %e, "graphql-transport-ws connect failed");
                return;
            }
        };

        // 1. connection_init — bearer in payload mirrors the wasm path.
        let mut init_payload = json!({});
        if let Some(token) = get_auth_token() {
            init_payload = json!({ "Authorization": format!("Bearer {token}") });
        }
        let init = json!({ "type": "connection_init", "payload": init_payload });
        if ws.send(Message::Text(init.to_string().into())).await.is_err() {
            return;
        }

        if !await_ack(&mut ws).await {
            return;
        }

        // 2. subscribe
        let op = JobEventsSubscription::build(JobVars { job });
        let subscribe = json!({
            "type": "subscribe",
            "id": SUB_ID,
            "payload": serde_json::to_value(&op).unwrap_or(Value::Null),
        });
        if ws
            .send(Message::Text(subscribe.to_string().into()))
            .await
            .is_err()
        {
            return;
        }

        // 3. read loop
        while let Some(frame) = ws.next().await {
            let text = match frame {
                Ok(Message::Text(t)) => t.to_string(),
                Ok(Message::Ping(p)) => {
                    let _ = ws.send(Message::Pong(p)).await;
                    continue;
                }
                Ok(Message::Close(_)) => break,
                Ok(_) => continue,
                Err(e) => {
                    tracing::debug!(error = %e, "WS read failed");
                    break;
                }
            };
            let parsed: ServerMessage = match serde_json::from_str(&text) {
                Ok(m) => m,
                Err(_) => continue,
            };
            match parsed.ty.as_str() {
                "next" => {
                    let event = parsed
                        .payload
                        .as_ref()
                        .and_then(|p| p.get("data"))
                        .and_then(|d| d.get("jobEvents"))
                        .cloned()
                        .and_then(|v| serde_json::from_value::<JobEventSub>(v).ok());
                    if let Some(ev) = event {
                        on_event(ev);
                    }
                }
                "error" => {
                    tracing::warn!(payload = ?parsed.payload, "subscription error");
                }
                "complete" => break,
                "ping" => {
                    let pong = json!({ "type": "pong" }).to_string();
                    let _ = ws.send(Message::Text(pong.into())).await;
                }
                _ => {}
            }
        }
    }

    async fn await_ack(
        ws: &mut async_tungstenite::WebSocketStream<
            async_tungstenite::tokio::ConnectStream,
        >,
    ) -> bool {
        while let Some(frame) = ws.next().await {
            let text = match frame {
                Ok(Message::Text(t)) => t.to_string(),
                Ok(Message::Ping(p)) => {
                    let _ = ws.send(Message::Pong(p)).await;
                    continue;
                }
                Ok(_) => continue,
                Err(_) => return false,
            };
            if let Ok(msg) = serde_json::from_str::<ServerMessage>(&text) {
                match msg.ty.as_str() {
                    "connection_ack" => return true,
                    "connection_error" => {
                        tracing::warn!(payload = ?msg.payload, "WS connection_error");
                        return false;
                    }
                    "ping" => {
                        let pong = json!({ "type": "pong" }).to_string();
                        let _ = ws.send(Message::Text(pong.into())).await;
                    }
                    _ => {}
                }
            }
        }
        false
    }

    #[derive(Deserialize)]
    struct ServerMessage {
        #[serde(rename = "type")]
        ty: String,
        #[serde(default)]
        payload: Option<Value>,
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm_impl {
    use cynic::SubscriptionBuilder;
    use futures_util::{SinkExt, StreamExt};
    use gloo_net::websocket::futures::WebSocket;
    use gloo_net::websocket::Message;
    use serde::Deserialize;
    use serde_json::{json, Value};

    use super::{JobEventCallback, JobEventSub, JobEventsSubscription, JobVars};
    use crate::api::get_auth_token;
    use crate::config;

    const SUB_ID: &str = "1";

    /// Build the `ws://…/api/graphql/ws` URL from the configured
    /// `graphql_url`. `https → wss`, `http → ws`, then append `/ws`.
    fn ws_url() -> String {
        let http = config::graphql_url();
        let base = if let Some(rest) = http.strip_prefix("https://") {
            format!("wss://{rest}")
        } else if let Some(rest) = http.strip_prefix("http://") {
            format!("ws://{rest}")
        } else {
            http.to_string()
        };
        format!("{base}/ws")
    }

    pub async fn subscribe_job_events(job: String, mut on_event: JobEventCallback) {
        let url = ws_url();
        let mut ws = match WebSocket::open_with_protocol(&url, "graphql-transport-ws") {
            Ok(ws) => ws,
            Err(e) => {
                tracing::debug!(error = ?e, "graphql-transport-ws open failed");
                return;
            }
        };

        // 1. connection_init — bearer token rides in the payload as some
        // gateways reject ad-hoc headers on the WS upgrade.
        let mut init_payload = json!({});
        if let Some(token) = get_auth_token() {
            init_payload = json!({ "Authorization": format!("Bearer {token}") });
        }
        let init = json!({ "type": "connection_init", "payload": init_payload });
        if ws.send(Message::Text(init.to_string())).await.is_err() {
            return;
        }

        // 2. wait for connection_ack
        if !await_ack(&mut ws).await {
            return;
        }

        // 3. subscribe
        let op = JobEventsSubscription::build(JobVars { job });
        let subscribe = json!({
            "type": "subscribe",
            "id": SUB_ID,
            "payload": serde_json::to_value(&op).unwrap_or(Value::Null),
        });
        if ws.send(Message::Text(subscribe.to_string())).await.is_err() {
            return;
        }

        // 4. read loop until `complete` or socket close.
        while let Some(frame) = ws.next().await {
            let text = match frame {
                Ok(Message::Text(t)) => t,
                Ok(Message::Bytes(_)) => continue,
                Err(e) => {
                    tracing::debug!(error = ?e, "WS read failed");
                    break;
                }
            };
            let parsed: ServerMessage = match serde_json::from_str(&text) {
                Ok(m) => m,
                Err(_) => continue,
            };
            match parsed.ty.as_str() {
                "next" => {
                    // payload.data.jobEvents → JobEventSub
                    let event = parsed
                        .payload
                        .as_ref()
                        .and_then(|p| p.get("data"))
                        .and_then(|d| d.get("jobEvents"))
                        .cloned()
                        .and_then(|v| serde_json::from_value::<JobEventSub>(v).ok());
                    if let Some(ev) = event {
                        on_event(ev);
                    }
                }
                "error" => {
                    tracing::warn!(payload = ?parsed.payload, "subscription error");
                }
                "complete" => break,
                _ => {}
            }
        }
    }

    async fn await_ack(ws: &mut WebSocket) -> bool {
        while let Some(frame) = ws.next().await {
            let text = match frame {
                Ok(Message::Text(t)) => t,
                Ok(Message::Bytes(_)) => continue,
                Err(_) => return false,
            };
            if let Ok(msg) = serde_json::from_str::<ServerMessage>(&text) {
                match msg.ty.as_str() {
                    "connection_ack" => return true,
                    "connection_error" => {
                        tracing::warn!(payload = ?msg.payload, "WS connection_error");
                        return false;
                    }
                    "ping" => {
                        let pong = json!({ "type": "pong" }).to_string();
                        let _ = ws.send(Message::Text(pong)).await;
                    }
                    _ => {}
                }
            }
        }
        false
    }

    #[derive(Deserialize)]
    struct ServerMessage {
        #[serde(rename = "type")]
        ty: String,
        #[serde(default)]
        payload: Option<Value>,
    }
}
