//! GraphQL client.
//!
//! Cynic produces strongly typed `Operation` values that already serialize
//! into the wire body. We POST them, deserialize a `cynic::GraphQlResponse`,
//! and transparently retry once on auth failure after exchanging the
//! refresh token.
//!
//! `GqlError` is a thin wrapper around the message field so the rest of
//! the crate doesn't have to depend on cynic's error types.

use cynic::{GraphQlResponse, Operation};
use serde::{Deserialize, Serialize};

use crate::api::{
    clear_all_tokens, get_auth_token, get_refresh_token, set_auth_token, set_refresh_token,
};
use crate::components::toast;
use crate::config;
use crate::error::from_graphql_message;

/// Trimmed error type so callers don't see cynic internals.
#[derive(Debug, Clone)]
pub struct GqlError {
    pub message: String,
}

impl From<&cynic::GraphQlError> for GqlError {
    fn from(e: &cynic::GraphQlError) -> Self {
        GqlError {
            message: e.message.clone(),
        }
    }
}

/// Send a cynic operation and return its `data` payload. Returns `None`
/// on transport error, GraphQL error, or `data: null` from the server.
/// On `Unauthorized` the executor swaps the refresh token once before
/// giving up; if the refresh itself fails, all tokens are cleared.
pub(super) async fn run<ResponseData, Vars>(
    operation: Operation<ResponseData, Vars>,
) -> Option<ResponseData>
where
    ResponseData: serde::de::DeserializeOwned + 'static,
    Vars: serde::Serialize,
{
    match send::<ResponseData, Vars>(&operation).await {
        Ok(data) => Some(data),
        Err(SendError::Auth) => {
            if try_refresh().await {
                send::<ResponseData, Vars>(&operation).await.ok()
            } else {
                clear_all_tokens();
                None
            }
        }
        Err(SendError::Other) => None,
    }
}

enum SendError {
    Auth,
    Other,
}

async fn send<ResponseData, Vars>(
    operation: &Operation<ResponseData, Vars>,
) -> Result<ResponseData, SendError>
where
    ResponseData: serde::de::DeserializeOwned + 'static,
    Vars: serde::Serialize,
{
    let mut req = reqwest::Client::new().post(config::graphql_url());
    if let Some(token) = get_auth_token() {
        req = req.bearer_auth(token);
    }

    let body = serde_json::to_vec(operation).map_err(|_| SendError::Other)?;
    let resp = req
        .header("content-type", "application/json")
        .body(body)
        .send()
        .await
        .map_err(|_| SendError::Other)?;

    let bytes = resp.bytes().await.map_err(|_| SendError::Other)?;
    let parsed: GraphQlResponse<ResponseData> =
        serde_json::from_slice(&bytes).map_err(|_| SendError::Other)?;

    if let Some(errs) = &parsed.errors {
        for e in errs {
            tracing::warn!(message = %e.message, "GraphQL error");
        }
        if errs.iter().any(is_auth_error) {
            return Err(SendError::Auth);
        }
        // Surface the first non-auth error to the user. The retry path
        // for auth errors above is silent; everything else is an
        // actionable failure the user should see.
        if let Some(first) = errs.iter().find(|e| !is_auth_error(e)) {
            toast::push_error(&from_graphql_message(&first.message));
        }
        if parsed.data.is_none() {
            return Err(SendError::Other);
        }
    }

    parsed.data.ok_or(SendError::Other)
}

fn is_auth_error(e: &cynic::GraphQlError) -> bool {
    let m = e.message.to_lowercase();
    m == "unauthorized"
        || m.contains("invalid token")
        || m.contains("token expired")
        || m.contains("authentication required")
}

/// Try to swap the stored refresh token for a fresh pair. Returns
/// `false` if no refresh token is stored or the server rejects it.
/// Hand-rolled rather than recursing back through `run` so a stale
/// access token can't cause a refresh storm.
async fn try_refresh() -> bool {
    let Some(refresh) = get_refresh_token() else {
        return false;
    };

    #[derive(Serialize)]
    struct Vars {
        refresh_token: String,
    }
    #[derive(Deserialize)]
    struct Pair {
        token: String,
        #[serde(rename = "refreshToken")]
        refresh_token: Option<String>,
    }
    #[derive(Deserialize)]
    struct Data {
        #[serde(rename = "refreshToken")]
        refresh_token: Option<Pair>,
    }
    #[derive(Serialize)]
    struct Body<'a> {
        query: &'a str,
        variables: Vars,
    }

    let body = Body {
        query: "mutation R($refreshToken: String!) {
            refreshToken(refreshToken: $refreshToken) { token refreshToken }
        }",
        variables: Vars {
            refresh_token: refresh,
        },
    };

    let resp = match reqwest::Client::new()
        .post(config::graphql_url())
        .json(&body)
        .send()
        .await
    {
        Ok(r) => r,
        Err(_) => return false,
    };

    #[derive(Deserialize)]
    struct Wrap {
        data: Option<Data>,
    }
    let parsed: Wrap = match resp.json().await {
        Ok(j) => j,
        Err(_) => return false,
    };
    let Some(Pair {
        token,
        refresh_token,
    }) = parsed.data.and_then(|d| d.refresh_token)
    else {
        return false;
    };

    set_auth_token(Some(token));
    if let Some(new_refresh) = refresh_token {
        set_refresh_token(Some(new_refresh));
    }
    true
}
