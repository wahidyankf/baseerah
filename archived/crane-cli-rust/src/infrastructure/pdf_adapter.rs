//! PDF adapter trait and implementations for crane-cli.

use crate::domain::PdfMetadata;

/// PDF adapter trait for reading PDF documents.
///
/// All methods return `Result<T, String>` where the error is a human-readable
/// message. Implementations must be `Send + Sync` for use with `Arc`.
pub trait PdfAdapter: Send + Sync {
    /// Returns PDF metadata including page count, title, author, file path, and size.
    ///
    /// # Errors
    ///
    /// Returns `Err(String)` if the PDF cannot be read or metadata extracted.
    fn get_metadata(&self, path: &str) -> Result<PdfMetadata, String>;

    /// Returns sample text from the first `page_count` pages.
    ///
    /// # Errors
    ///
    /// Returns `Err(String)` if the PDF cannot be read or text extracted.
    fn sample_text(&self, path: &str, page_count: usize) -> Result<String, String>;

    /// Returns extracted text from pages `start_page..=end_page` (1-indexed).
    ///
    /// # Errors
    ///
    /// Returns `Err(String)` if the PDF cannot be read or text extracted.
    fn extract_pages(
        &self,
        path: &str,
        start_page: usize,
        end_page: usize,
    ) -> Result<String, String>;
}

/// Production PDF adapter using lopdf.
pub struct LopdfAdapter;

impl LopdfAdapter {
    /// Creates a new `LopdfAdapter`.
    pub fn new() -> Self {
        Self
    }
}

impl Default for LopdfAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfAdapter for LopdfAdapter {
    fn get_metadata(&self, path: &str) -> Result<PdfMetadata, String> {
        let doc = lopdf::Document::load(path).map_err(|e| format!("Failed to read PDF: {e}"))?;

        let page_count = doc.get_pages().len();
        let size_bytes = std::fs::metadata(path)
            .map_err(|e| format!("Failed to read file metadata: {e}"))?
            .len();

        // Extract title and author from info dictionary
        let info_obj = doc.trailer.get(b"Info").ok().and_then(|obj| {
            obj.as_reference()
                .ok()
                .and_then(|id| doc.get_object(id).ok())
        });

        let title = info_obj
            .as_ref()
            .and_then(|obj| obj.as_dict().ok())
            .and_then(|d| d.get(b"Title").ok())
            .and_then(|obj| obj.as_str().ok())
            .map(|b| String::from_utf8_lossy(b).into_owned())
            .filter(|s| !s.is_empty());

        let author = info_obj
            .as_ref()
            .and_then(|obj| obj.as_dict().ok())
            .and_then(|d| d.get(b"Author").ok())
            .and_then(|obj| obj.as_str().ok())
            .map(|b| String::from_utf8_lossy(b).into_owned())
            .filter(|s| !s.is_empty());

        Ok(PdfMetadata {
            pages: page_count,
            title,
            author,
            file: path.to_string(),
            size_bytes,
        })
    }

    fn sample_text(&self, path: &str, page_count: usize) -> Result<String, String> {
        let doc = lopdf::Document::load(path).map_err(|e| format!("Failed to read PDF: {e}"))?;

        let pages = doc.get_pages();
        let page_nums: Vec<u32> = pages.keys().copied().take(page_count).collect();

        if page_nums.is_empty() {
            return Ok(String::new());
        }

        doc.extract_text(&page_nums)
            .map_err(|e| format!("Failed to extract text: {e}"))
    }

    fn extract_pages(
        &self,
        path: &str,
        start_page: usize,
        end_page: usize,
    ) -> Result<String, String> {
        let doc = lopdf::Document::load(path).map_err(|e| format!("Failed to read PDF: {e}"))?;

        let pages = doc.get_pages();
        let mut page_nums: Vec<u32> = pages.keys().copied().collect();
        page_nums.sort_unstable();

        // Filter to requested 1-indexed range
        let filtered: Vec<u32> = page_nums
            .into_iter()
            .enumerate()
            .filter(|(i, _)| {
                let page_1indexed = i + 1;
                page_1indexed >= start_page && page_1indexed <= end_page
            })
            .map(|(_, n)| n)
            .collect();

        if filtered.is_empty() {
            return Ok(String::new());
        }

        doc.extract_text(&filtered)
            .map_err(|e| format!("Failed to extract text: {e}"))
    }
}

/// Fake PDF adapter for testing — returns pre-set data regardless of path.
pub struct FakePdfAdapter {
    /// Pre-set text to return for any text extraction call.
    text: String,
    /// Pre-set page count to return in metadata.
    pages: usize,
    /// Pre-set file size in bytes to return in metadata.
    size_bytes: u64,
}

impl FakePdfAdapter {
    /// Creates a new fake adapter with pre-set data.
    pub fn new(text: &str, pages: usize, size_bytes: u64) -> Self {
        Self {
            text: text.to_string(),
            pages,
            size_bytes,
        }
    }
}

impl PdfAdapter for FakePdfAdapter {
    fn get_metadata(&self, path: &str) -> Result<PdfMetadata, String> {
        Ok(PdfMetadata {
            pages: self.pages,
            title: Some("Fake Document".to_string()),
            author: None,
            file: path.to_string(),
            size_bytes: self.size_bytes,
        })
    }

    fn sample_text(&self, _path: &str, _page_count: usize) -> Result<String, String> {
        Ok(self.text.clone())
    }

    fn extract_pages(
        &self,
        _path: &str,
        _start_page: usize,
        _end_page: usize,
    ) -> Result<String, String> {
        Ok(self.text.clone())
    }
}
