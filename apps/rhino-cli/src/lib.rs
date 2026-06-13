//! `rhino-cli` library crate — Repository Hygiene & `INtegration` Orchestrator.
//!
//! Exposes the [`cli`] entry point, the [`commands`] dispatch layer,
//! and the [`internal`] implementation modules.
#![forbid(unsafe_code)]

/// Application-layer use cases and port definitions.
pub mod application;
/// CLI entry point and subcommand routing.
pub mod cli;
/// Inbound command adapters (one per CLI subcommand).
pub mod commands;
/// Pure-domain types, parsers, and validation rules — no I/O.
pub mod domain;
/// I/O adapters: reporters, file-system extractors, and infrastructure helpers.
pub mod infrastructure;
/// Internal implementation modules shared across commands.
pub mod internal;

/// Test-only helpers for serializing process-global state (e.g. the cwd).
#[cfg(test)]
pub(crate) mod test_support;
