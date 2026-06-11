//! crane-be HTTP and NATS clients for PDF-to-Markdown conversion.

use anyhow::Result;
use async_nats::Client;

/// Convert PDF bytes to markdown via crane-be over NATS request/reply.
///
/// # Errors
///
/// Returns `Err` if the NATS request fails or the response is not valid UTF-8.
pub async fn convert_via_nats(client: &Client, pdf_bytes: Vec<u8>) -> Result<String> {
    let response = super::client::request(client, "crane.convert", pdf_bytes).await?;
    String::from_utf8(response).map_err(|e| anyhow::anyhow!("crane NATS reply is not UTF-8: {e}"))
}

/// Convert PDF bytes to markdown via crane-be over HTTP.
///
/// # Errors
///
/// Returns `Err` if the HTTP request fails or returns a non-200 status.
pub async fn convert_via_http(crane_url: &str, pdf_bytes: Vec<u8>) -> Result<String> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{crane_url}/media/pdf-to-md"))
        .header("Content-Type", "application/octet-stream")
        .body(pdf_bytes)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("crane HTTP request failed: {e}"))?;
    if !resp.status().is_success() {
        return Err(anyhow::anyhow!("crane HTTP returned {}", resp.status()));
    }
    resp.text()
        .await
        .map_err(|e| anyhow::anyhow!("crane HTTP response read error: {e}"))
}
