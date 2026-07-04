//! Multi-language BDD spec coverage validation.
//!
//! Port of `apps/rhino-cli/internal/speccoverage/`.
//! Walks `.feature` trees and source trees, matches step definitions to
//! Gherkin scenarios, and reports coverage gaps and orphan step implementations.

pub mod checker;
pub mod cucumber_expr;
pub mod extractors;
pub mod matcher;
pub mod parser;
pub mod reporter;
pub mod runtime_check;
pub mod types;
pub mod util;
