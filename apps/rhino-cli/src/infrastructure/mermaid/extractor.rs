//! File-system Mermaid extractor adapter — implements [`MermaidExtractorPort`].

use crate::application::mermaid::port::MermaidExtractorPort;
use crate::domain::mermaid::{self, MermaidBlock};

/// File-system adapter that extracts Mermaid blocks from content strings.
pub struct FsMermaidExtractor;

impl MermaidExtractorPort for FsMermaidExtractor {
    fn extract_blocks(&self, file_path: &str, content: &str) -> Vec<MermaidBlock> {
        mermaid::extract_blocks(file_path, content)
    }
}
