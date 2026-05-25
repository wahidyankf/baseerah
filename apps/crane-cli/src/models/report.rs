//! Report models for crane-cli skip list management.

use serde::{Deserialize, Serialize};

/// A single entry in the known-false-positives skip list.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkipListEntry {
    /// Basename of the Markdown file this entry applies to.
    #[serde(rename = "md_basename")]
    pub md_basename: String,

    /// Category of the finding (e.g. "text-completeness").
    #[serde(rename = "category")]
    pub category: String,

    /// Description of the finding that was marked as false positive.
    #[serde(rename = "description")]
    pub description: String,

    /// Stable SHA-256-derived key (first 16 hex chars).
    #[serde(rename = "key")]
    pub key: String,

    /// Timestamp when this entry was accepted.
    #[serde(rename = "accepted")]
    pub accepted: String,

    /// Reason this entry was accepted as a false positive.
    #[serde(rename = "reason")]
    pub reason: String,
}
