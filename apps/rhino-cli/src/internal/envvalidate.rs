//! Backward-compatibility re-exports for the `envvalidate` feature.
//!
//! The implementation now lives in `crate::application::env::validate`.
//! This module keeps existing callers working during the hexagonal migration.
pub use crate::application::env::validate::*;
