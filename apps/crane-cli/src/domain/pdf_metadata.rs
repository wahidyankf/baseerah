//! PDF metadata model for crane-cli.

use serde::{Deserialize, Serialize};

/// Metadata extracted from a PDF file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PdfMetadata {
    /// Number of pages in the PDF.
    #[serde(rename = "pages")]
    pub pages: usize,

    /// Title from the PDF info dictionary, if present.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Author from the PDF info dictionary, if present.
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// File path of the PDF.
    #[serde(rename = "file")]
    pub file: String,

    /// File size in bytes.
    #[serde(rename = "size_bytes")]
    pub size_bytes: u64,
}
