//! Infrastructure adapters for crane-cli.
pub mod pdf_adapter;
pub use pdf_adapter::{FakePdfAdapter, LopdfAdapter, PdfAdapter};
