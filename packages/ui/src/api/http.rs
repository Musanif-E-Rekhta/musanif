//! Shared HTTP plumbing for the REST surface (uploads + binary).

use crate::config;

#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("transport: {0}")]
    Transport(String),
    #[error("status {0}")]
    Status(u16),
    #[error("decode: {0}")]
    Decode(String),
}

impl HttpError {
    pub fn transport(e: impl ToString) -> Self {
        Self::Transport(e.to_string())
    }
    pub fn status(code: u16) -> Self {
        Self::Status(code)
    }
    pub fn decode(e: impl ToString) -> Self {
        Self::Decode(e.to_string())
    }
}

pub fn url(path: &str) -> String {
    format!("{}{}", config::api_base_url(), path)
}
