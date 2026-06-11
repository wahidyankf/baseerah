//! `JetStream` durable publish/consume/ack demo.

use anyhow::Result;
use async_nats::jetstream;
use futures::StreamExt as _;

/// `JetStream` stream name for the demo.
const STREAM_NAME: &str = "OSE_APP_MESSAGING_DEMO";
/// `JetStream` subject for demo messages.
const SUBJECT: &str = "ose-app.messaging.demo";
/// Durable consumer name for the demo.
const CONSUMER_NAME: &str = "ose-app-messaging-demo";

/// Run the `JetStream` durable demo: create stream + durable consumer, publish one
/// message, consume it with ack. Returns `"delivered_and_acked"` on success.
///
/// # Errors
///
/// Returns `Err` if any `JetStream` operation fails.
pub async fn run(client: &async_nats::Client) -> Result<String> {
    let js = jetstream::new(client.clone());

    // Create or get stream
    js.get_or_create_stream(jetstream::stream::Config {
        name: STREAM_NAME.to_string(),
        subjects: vec![SUBJECT.to_string()],
        ..Default::default()
    })
    .await
    .map_err(|e| anyhow::anyhow!("JetStream stream error: {e}"))?;

    // Create or get durable consumer
    let stream = js
        .get_stream(STREAM_NAME)
        .await
        .map_err(|e| anyhow::anyhow!("JetStream get_stream error: {e}"))?;
    let consumer = stream
        .get_or_create_consumer(
            CONSUMER_NAME,
            jetstream::consumer::pull::Config {
                durable_name: Some(CONSUMER_NAME.to_string()),
                ..Default::default()
            },
        )
        .await
        .map_err(|e| anyhow::anyhow!("JetStream consumer error: {e}"))?;

    // Publish demo message
    js.publish(SUBJECT, b"demo message".as_slice().into())
        .await
        .map_err(|e| anyhow::anyhow!("JetStream publish error: {e}"))?
        .await
        .map_err(|e| anyhow::anyhow!("JetStream publish ack error: {e}"))?;

    // Consume with ack (fetch up to 1)
    let mut messages = consumer
        .fetch()
        .max_messages(1)
        .messages()
        .await
        .map_err(|e| anyhow::anyhow!("JetStream fetch error: {e}"))?;

    if let Some(msg) = messages.next().await {
        let msg = msg.map_err(|e| anyhow::anyhow!("JetStream message error: {e}"))?;
        msg.ack()
            .await
            .map_err(|e| anyhow::anyhow!("JetStream ack error: {e}"))?;
    }

    Ok("delivered_and_acked".to_string())
}
