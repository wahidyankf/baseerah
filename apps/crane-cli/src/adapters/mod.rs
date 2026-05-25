//! Adapters for crane-cli — external service integrations.
pub mod pdf_adapter;
pub use pdf_adapter::{FakePdfAdapter, LopdfAdapter, PdfAdapter};
