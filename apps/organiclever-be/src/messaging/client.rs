//! NATS client initialization.

use anyhow::Result;
use async_nats::Client;

/// Connect to NATS at `url`. Fails fast if unreachable.
///
/// # Errors
///
/// Returns `Err` if the NATS connection cannot be established.
pub async fn connect(url: &str) -> Result<Client> {
    let client = async_nats::connect(url)
        .await
        .map_err(|e| anyhow::anyhow!("Cannot connect to NATS at {url}: {e}"))?;
    Ok(client)
}

/// Issue a request to `subject` with `payload` and return the reply payload.
///
/// # Errors
///
/// Returns `Err` if the request times out or the reply cannot be parsed.
pub async fn request(client: &Client, subject: &str, payload: Vec<u8>) -> Result<Vec<u8>> {
    let subject = subject.to_owned();
    let response = client
        .request(subject, payload.into())
        .await
        .map_err(|e| anyhow::anyhow!("NATS request failed: {e}"))?;
    Ok(response.payload.to_vec())
}
