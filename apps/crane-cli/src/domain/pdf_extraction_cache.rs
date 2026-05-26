//! PDF extraction cache — wraps a `PdfAdapter` with SHA-256-keyed disk caching.
#![allow(clippy::missing_docs_in_private_items)]
use std::fmt::Write as FmtWrite;

use crate::domain::PdfMetadata;
use crate::infrastructure::PdfAdapter;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;

const CACHE_SUBDIR: &str = "extract";

#[derive(Debug, Serialize, Deserialize)]
struct CachedExtraction {
    #[serde(rename = "pdfSha")]
    pdf_sha: String,
    #[serde(rename = "kind")]
    kind: String,
    #[serde(rename = "extractedAt")]
    extracted_at: String,
    #[serde(rename = "fullText")]
    full_text: String,
}

fn pdf_sha256(path: &str) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("Failed to read PDF for hashing: {e}"))?;
    let hash = Sha256::digest(&bytes);
    let mut hex = String::with_capacity(64);
    for b in &hash {
        write!(hex, "{b:02x}").map_err(|e| format!("Failed to format hash: {e}"))?;
    }
    Ok(hex)
}

fn cache_entry_path(cache_dir: &str, kind: &str, sha: &str) -> std::path::PathBuf {
    let sha16 = &sha[..sha.len().min(16)];
    std::path::Path::new(cache_dir)
        .join(CACHE_SUBDIR)
        .join(format!("{kind}-{sha16}.json"))
}

fn try_read_cache(path: &std::path::Path) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    let entry: CachedExtraction = serde_json::from_str(&content).ok()?;
    Some(entry.full_text)
}

fn write_atomic(path: &std::path::Path, sha: &str, kind: &str, text: &str) -> Result<(), String> {
    if let Some(dir) = path.parent().filter(|d| !d.exists()) {
        std::fs::create_dir_all(dir).map_err(|e| format!("Failed to create cache dir: {e}"))?;
    }

    let entry = CachedExtraction {
        pdf_sha: sha.to_string(),
        kind: kind.to_string(),
        extracted_at: chrono::Utc::now().to_rfc3339(),
        full_text: text.to_string(),
    };
    let json = serde_json::to_string(&entry)
        .map_err(|e| format!("Failed to serialize cache entry: {e}"))?;

    let tmp_path = path.with_extension("json.tmp");
    std::fs::write(&tmp_path, json).map_err(|e| format!("Failed to write cache tmp: {e}"))?;
    std::fs::rename(&tmp_path, path).map_err(|e| format!("Failed to rename cache file: {e}"))?;

    Ok(())
}

/// A PDF adapter wrapping an inner adapter with SHA-256-keyed disk caching.
struct CachingAdapter {
    inner: Arc<dyn PdfAdapter>,
    cache_dir: String,
}

impl PdfAdapter for CachingAdapter {
    fn get_metadata(&self, path: &str) -> Result<PdfMetadata, String> {
        self.inner.get_metadata(path)
    }

    fn sample_text(&self, path: &str, page_count: usize) -> Result<String, String> {
        let Ok(sha) = pdf_sha256(path) else {
            return self.inner.sample_text(path, page_count);
        };
        let kind = format!("sample-{page_count}");
        let cache_path = cache_entry_path(&self.cache_dir, &kind, &sha);

        if let Some(text) = try_read_cache(&cache_path) {
            return Ok(text);
        }

        let text = self.inner.sample_text(path, page_count)?;
        let _ = write_atomic(&cache_path, &sha, &kind, &text);
        Ok(text)
    }

    fn extract_pages(
        &self,
        path: &str,
        start_page: usize,
        end_page: usize,
    ) -> Result<String, String> {
        let Ok(sha) = pdf_sha256(path) else {
            return self.inner.extract_pages(path, start_page, end_page);
        };
        let kind = format!("pages-{start_page}-{end_page}");
        let cache_path = cache_entry_path(&self.cache_dir, &kind, &sha);

        if let Some(text) = try_read_cache(&cache_path) {
            return Ok(text);
        }

        let text = self.inner.extract_pages(path, start_page, end_page)?;
        let _ = write_atomic(&cache_path, &sha, &kind, &text);
        Ok(text)
    }
}

/// Wraps an inner `PdfAdapter` with SHA-256-keyed disk caching.
pub fn wrap(inner: Arc<dyn PdfAdapter>, cache_dir: &str) -> Arc<dyn PdfAdapter> {
    Arc::new(CachingAdapter {
        inner,
        cache_dir: cache_dir.to_string(),
    })
}

/// Returns the default cache directory (`~/.cache/crane` or `$XDG_CACHE_HOME/crane`).
pub fn default_cache_dir() -> String {
    let xdg = std::env::var("XDG_CACHE_HOME").unwrap_or_default();
    if !xdg.is_empty() {
        return format!("{xdg}/crane");
    }
    let home = dirs_next();
    format!("{home}/.cache/crane")
}

fn dirs_next() -> String {
    std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string())
}
