//! Frontend error taxonomy.
//!
//! Mirrors `merk/src/error.rs`'s shape so the same code/message
//! conventions hold on both sides of the wire. The variants split into
//! two groups:
//!
//! - **`BadRequest` / `Unauthorized` / `Forbidden` / `NotFound` /
//!   `Conflict`** — domain errors surfaced to the user. Constructed
//!   either from server responses (mapped via the GraphQL error
//!   message) or from local validation. Logged at `warn`.
//! - **`Upstream` / `Internal`** — transport/parse failures the user
//!   can't act on. Logged at `error` with a structured `origin` field
//!   so log filters work the same way as on the backend.
//!
//! There is no `IntoResponse` / `status_code` impl — that's a
//! server-only concern. The frontend renders [`Error::message`] to the
//! user and uses [`Error::client_code`] for matching/branching in view
//! code.

use std::borrow::Cow;
use thiserror::Error;
use tracing::{error, warn};

#[derive(Debug, Error)]
pub enum Error {
    /// Server rejected the request body or local validation failed.
    #[error("{message}")]
    BadRequest {
        code: Cow<'static, str>,
        message: String,
    },

    /// No / expired credentials. Triggers a redirect to login.
    #[error("{message}")]
    Unauthorized {
        code: Cow<'static, str>,
        message: String,
    },

    /// Authenticated but not permitted for this resource.
    #[error("{message}")]
    Forbidden {
        code: Cow<'static, str>,
        message: String,
    },

    /// Resource doesn't exist.
    #[error("{message}")]
    NotFound {
        code: Cow<'static, str>,
        message: String,
    },

    /// Resource state conflict (e.g. duplicate email).
    #[error("{message}")]
    Conflict {
        code: Cow<'static, str>,
        message: String,
    },

    /// Network/transport failure: server unreachable, malformed
    /// response, websocket dropped, etc. Distinct from `Internal` so
    /// the UI can show "check your connection" copy.
    #[error("{origin}: {message}")]
    Upstream {
        origin: Cow<'static, str>,
        message: String,
        #[source]
        source: Option<anyhow::Error>,
    },

    /// Client-side bug: parsing error, missing config, panic. Should
    /// be rare; logs include the source chain for debugging.
    #[error("{origin}: {message}")]
    Internal {
        origin: Cow<'static, str>,
        message: String,
        #[source]
        source: Option<anyhow::Error>,
    },
}

impl Error {
    pub fn bad_request(code: impl Into<Cow<'static, str>>, message: impl Into<String>) -> Self {
        Self::BadRequest {
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::Unauthorized {
            code: Cow::Borrowed("unauthorized"),
            message: message.into(),
        }
    }

    pub fn invalid_token() -> Self {
        Self::Unauthorized {
            code: Cow::Borrowed("invalid_token"),
            message: "Invalid or expired token".into(),
        }
    }

    pub fn forbidden(code: impl Into<Cow<'static, str>>, message: impl Into<String>) -> Self {
        Self::Forbidden {
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound {
            code: Cow::Borrowed("not_found"),
            message: message.into(),
        }
    }

    pub fn conflict(code: impl Into<Cow<'static, str>>, message: impl Into<String>) -> Self {
        Self::Conflict {
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn upstream(origin: impl Into<Cow<'static, str>>, message: impl Into<String>) -> Self {
        Self::Upstream {
            origin: origin.into(),
            message: message.into(),
            source: None,
        }
    }

    pub fn internal(origin: impl Into<Cow<'static, str>>, message: impl Into<String>) -> Self {
        Self::Internal {
            origin: origin.into(),
            message: message.into(),
            source: None,
        }
    }

    /// Stable code suitable for matching in view code.
    pub fn client_code(&self) -> &str {
        match self {
            Error::BadRequest { code, .. }
            | Error::Unauthorized { code, .. }
            | Error::Forbidden { code, .. }
            | Error::NotFound { code, .. }
            | Error::Conflict { code, .. } => code,
            Error::Upstream { .. } => "upstream",
            Error::Internal { .. } => "internal",
        }
    }

    /// User-facing message.
    pub fn message(&self) -> &str {
        match self {
            Error::BadRequest { message, .. }
            | Error::Unauthorized { message, .. }
            | Error::Forbidden { message, .. }
            | Error::NotFound { message, .. }
            | Error::Conflict { message, .. }
            | Error::Upstream { message, .. }
            | Error::Internal { message, .. } => message,
        }
    }

    /// Emit a structured log line at the right severity. Mirrors the
    /// server's `IntoResponse` logging step so the logs read the same
    /// across the wire.
    pub fn log(&self) {
        match self {
            Error::Internal { source, .. } | Error::Upstream { source, .. } => match source {
                Some(src) => error!(error = ?self, source_chain = %format!("{src:#}")),
                None => error!(?self),
            },
            _ => warn!(?self),
        }
    }
}

/// Best-effort conversion from a GraphQL error message into a domain
/// `Error` variant. Mirrors `merk`'s server-side mapping: messages the
/// backend tags as `unauthorized` / `not_found` / `forbidden` /
/// `conflict` / `bad_request` are recognised by substring; anything
/// else falls through to `Upstream("graphql")`.
pub fn from_graphql_message(message: &str) -> Error {
    let lower = message.to_lowercase();
    if lower == "unauthorized"
        || lower.contains("invalid token")
        || lower.contains("token expired")
        || lower.contains("authentication required")
    {
        Error::invalid_token()
    } else if lower.contains("forbidden") || lower.contains("admin") {
        Error::forbidden("forbidden", message.to_string())
    } else if lower.contains("not found") {
        Error::not_found(message.to_string())
    } else if lower.contains("already exists") || lower.contains("conflict") {
        Error::conflict("conflict", message.to_string())
    } else if lower.contains("invalid") || lower.contains("validation") {
        Error::bad_request("validation_error", message.to_string())
    } else {
        Error::upstream("graphql", message.to_string())
    }
}

crate::from_as_error! {
    serde_json::Error    => internal("serde_json") + src,
    std::num::ParseIntError => bad_request("parse_int"),
    std::num::ParseFloatError => bad_request("parse_float"),
}

// Reqwest is in scope on every target; gated From impls for transport-
// specific errors live next to their callers via the macro below.

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        // `is_connect` is native-only on reqwest; on wasm only
        // `is_timeout` / `is_decode` are available, the rest of the
        // taxonomy collapses into a generic "http" upstream.
        let origin: Cow<'static, str> = if e.is_timeout() {
            Cow::Borrowed("timeout")
        } else if e.is_decode() {
            Cow::Borrowed("decode")
        } else {
            #[cfg(not(target_arch = "wasm32"))]
            {
                if e.is_connect() {
                    Cow::Borrowed("connect")
                } else {
                    Cow::Borrowed("http")
                }
            }
            #[cfg(target_arch = "wasm32")]
            {
                Cow::Borrowed("http")
            }
        };
        Error::Upstream {
            origin,
            message: e.to_string(),
            source: Some(anyhow::Error::new(e)),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl From<gloo_net::websocket::WebSocketError> for Error {
    fn from(e: gloo_net::websocket::WebSocketError) -> Self {
        Error::upstream("websocket", e.to_string())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<async_tungstenite::tungstenite::Error> for Error {
    fn from(e: async_tungstenite::tungstenite::Error) -> Self {
        Error::upstream("websocket", e.to_string())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<keyring::Error> for Error {
    fn from(e: keyring::Error) -> Self {
        match e {
            keyring::Error::NoEntry => Error::not_found("keyring entry not found"),
            other => Error::internal("keyring", other.to_string()),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_and_code_round_trip() {
        let e = Error::not_found("missing book");
        assert_eq!(e.client_code(), "not_found");
        assert_eq!(e.message(), "missing book");
        assert_eq!(format!("{e}"), "missing book");
    }

    #[test]
    fn from_serde_json_lands_in_internal() {
        let bad: serde_json::Error = serde_json::from_str::<u32>("nope").unwrap_err();
        let mapped: Error = bad.into();
        match mapped {
            Error::Internal { origin, .. } => assert_eq!(origin, "serde_json"),
            other => panic!("unexpected variant: {other:?}"),
        }
    }

    #[test]
    fn graphql_message_classifier_picks_right_variants() {
        assert!(matches!(
            from_graphql_message("Unauthorized"),
            Error::Unauthorized { .. }
        ));
        assert!(matches!(
            from_graphql_message("Book not found"),
            Error::NotFound { .. }
        ));
        assert!(matches!(
            from_graphql_message("email already exists"),
            Error::Conflict { .. }
        ));
        assert!(matches!(
            from_graphql_message("server exploded"),
            Error::Upstream { .. }
        ));
    }
}
