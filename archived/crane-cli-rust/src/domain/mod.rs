//! Domain types and port definitions for crane-cli.
pub mod figure_checker;
pub mod finding;
pub mod heading_checker;
pub mod mermaid_validator;
pub mod nesting_checker;
pub mod ocr_assessor;
pub mod pdf_extraction_cache;
pub mod pdf_metadata;
pub mod report;
pub mod report_manager;
pub mod skiplist_manager;
pub mod table_checker;
pub mod text_checker;
pub use finding::{Criticality, Finding};
pub use pdf_metadata::PdfMetadata;
pub use report::SkipListEntry;
