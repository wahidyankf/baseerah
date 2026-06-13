use crate::domain::mermaid::MermaidBlock;

/// Port for extracting Mermaid blocks from a content string.
pub trait MermaidExtractorPort {
    /// Extracts all Mermaid fenced code blocks from `content` at `file_path`.
    fn extract_blocks(&self, file_path: &str, content: &str) -> Vec<MermaidBlock>;
}
