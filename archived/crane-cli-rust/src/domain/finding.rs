//! Finding data model for crane-cli audit results.

use serde::{Deserialize, Serialize};

/// Criticality level for a finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Criticality {
    /// Critical severity — must fix immediately.
    Critical,
    /// High severity — should fix soon.
    High,
    /// Medium severity — should fix eventually.
    Medium,
    /// Low severity — informational.
    Low,
}

impl std::fmt::Display for Criticality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Criticality::Critical => write!(f, "CRITICAL"),
            Criticality::High => write!(f, "HIGH"),
            Criticality::Medium => write!(f, "MEDIUM"),
            Criticality::Low => write!(f, "LOW"),
        }
    }
}

/// A single audit finding from a crane check.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Finding {
    /// Category of the finding (e.g. "text-completeness").
    #[serde(rename = "category")]
    pub category: String,

    /// Criticality string (e.g. "CRITICAL", "HIGH").
    #[serde(rename = "criticality")]
    pub criticality: String,

    /// Confidence level string (e.g. "HIGH").
    #[serde(rename = "confidence")]
    pub confidence: String,

    /// Location in the PDF (line or page reference), if applicable.
    #[serde(rename = "location_pdf", skip_serializing_if = "Option::is_none")]
    pub location_pdf: Option<String>,

    /// Location in the Markdown file, if applicable.
    #[serde(rename = "location_md", skip_serializing_if = "Option::is_none")]
    pub location_md: Option<String>,

    /// Human-readable description of the finding.
    #[serde(rename = "description")]
    pub description: String,

    /// The PDF text excerpt related to this finding, if applicable.
    #[serde(rename = "pdf_text", skip_serializing_if = "Option::is_none")]
    pub pdf_text: Option<String>,

    /// Suggested fix for the finding, if applicable.
    #[serde(rename = "fix_suggestion", skip_serializing_if = "Option::is_none")]
    pub fix_suggestion: Option<String>,

    /// Whether this finding can be automatically fixed.
    #[serde(rename = "auto_fixable")]
    pub auto_fixable: bool,
}
