//! Backward-compatibility re-exports for the `testcoverage` feature.
//!
//! The implementation now lives in `crate::application::testcoverage`.
//! This module keeps existing callers working during the hexagonal migration.
pub use crate::application::testcoverage::*;
