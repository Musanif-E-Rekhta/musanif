//! Multipart-style file upload client.
//!
//! merk's `POST /api/v1/admin/uploads` accepts the raw body with metadata
//! in headers (`X-Filename`, `Content-Type`). Returns an `uploaded_asset`
//! row that the admin UI passes to `startIngestionJob`.

use crate::api::http;
use crate::api::token::get_auth_token;
use crate::config::api_base_url;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UploadedAssetResponse {
    pub id: String,
    pub filename: String,
    pub mime: String,
    pub size_bytes: i64,
    pub bucket: String,
    pub object: String,
    pub status: String,
}

/// Upload `bytes` as the file body to `POST /api/v1/admin/uploads`.
/// Bearer auth required; admin/editor role enforced server-side.
pub async fn upload_admin_asset(
    filename: &str,
    mime: &str,
    bytes: Vec<u8>,
) -> Result<UploadedAssetResponse, http::HttpError> {
    let url = format!("{}/admin/uploads", api_base_url());
    let mut req = reqwest::Client::new()
        .post(&url)
        .header("X-Filename", filename)
        .header(reqwest::header::CONTENT_TYPE, mime)
        .body(bytes);
    if let Some(token) = get_auth_token() {
        req = req.bearer_auth(token);
    }
    let res = req.send().await.map_err(http::HttpError::transport)?;
    if !res.status().is_success() {
        return Err(http::HttpError::status(res.status().as_u16()));
    }
    res.json().await.map_err(http::HttpError::decode)
}
