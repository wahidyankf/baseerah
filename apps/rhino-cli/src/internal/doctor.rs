//! Backward-compatibility re-exports for the `doctor` feature.
//!
//! The implementation now lives in `crate::application::doctor`.
//! This module keeps existing callers working during the hexagonal migration.
pub use crate::application::doctor::*;
