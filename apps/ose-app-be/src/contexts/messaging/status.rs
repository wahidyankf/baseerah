//! Messaging status — tracks the `JetStream` demo outcome.

use std::sync::Arc;
use tokio::sync::Mutex;

/// Outcome of the `JetStream` durable demo.
#[derive(Debug, Clone, Default)]
pub struct MessagingStatus {
    /// Whether the demo message was delivered and acknowledged (None = pending).
    pub jetstream_demo: Option<String>,
}

/// Shared messaging status.
pub type SharedMessagingStatus = Arc<Mutex<MessagingStatus>>;

/// Create a new shared status handle.
pub fn new_shared() -> SharedMessagingStatus {
    Arc::new(Mutex::new(MessagingStatus::default()))
}
