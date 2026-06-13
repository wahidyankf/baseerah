//! Backward-compatibility re-exports for the `envbackup` feature.
//!
//! The implementation now lives in `crate::application::env::backup`.
//! This module keeps existing callers working during the hexagonal migration.
pub use crate::application::env::backup::*;
