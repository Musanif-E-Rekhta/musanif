//! Binary asset fetcher — PDF page previews, server-rendered images.
//!
//! Returns raw bytes; callers convert to a data URL for `<img>` rendering.

use crate::api::http;
use crate::api::token::get_auth_token;
use crate::config::api_base_url;

/// `GET /api/v1/admin/ingestion-jobs/{job_id}/pages/{n}.png`
pub async fn fetch_page_preview(job_id: &str, page_n: i64) -> Result<Vec<u8>, http::HttpError> {
    let url = format!(
        "{}/admin/ingestion-jobs/{}/pages/{}.png",
        api_base_url(),
        job_id,
        page_n
    );
    let mut req = reqwest::Client::new().get(&url);
    if let Some(token) = get_auth_token() {
        req = req.bearer_auth(token);
    }
    let res = req.send().await.map_err(http::HttpError::transport)?;
    if !res.status().is_success() {
        return Err(http::HttpError::status(res.status().as_u16()));
    }
    let bytes = res.bytes().await.map_err(http::HttpError::transport)?;
    Ok(bytes.to_vec())
}
