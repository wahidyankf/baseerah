//! Documentation validation sub-modules.
//!
//! Port of `apps/rhino-cli/internal/docs/`.
//!
//! Each sub-module validates a distinct aspect of repository markdown files:
//! - [`frontmatter`] — YAML frontmatter schema compliance.
//! - [`heading_hierarchy`] — H1 uniqueness and heading-level ordering.
//! - [`links`] — Relative link resolution.
//! - [`naming`] — Lowercase kebab-case filename convention.

/// YAML frontmatter validation for software-engineering and governance docs.
pub mod frontmatter;
/// Markdown heading hierarchy validation (single H1, no skipped levels).
pub mod heading_hierarchy;
/// Relative markdown link validation across the repository.
pub mod links;
/// Markdown filename convention validation (lowercase kebab-case).
pub mod naming;
