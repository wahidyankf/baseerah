//! Application use cases and port definitions.

/// Agent validation, sync, conversion, and binding use cases.
pub mod agents;
/// Allowlist of full-stack applications with DDD bounded-context registries.
pub mod allowlist;
/// Bounded-context registry loader and validator.
pub mod bcregistry;
/// Documentation validation use cases.
pub mod docs;
/// Doctor (toolchain-check) use case.
pub mod doctor;
/// Environment-file use cases (backup, validate).
pub mod env;
/// Git pre-commit use cases and port definitions.
pub mod git;
/// Glossary validator use case.
pub mod glossary;
/// Mermaid validation use cases and extractor port.
pub mod mermaid;
/// Agent and workflow naming convention validators.
pub mod naming;
/// Repository governance audit use cases.
pub mod repo_governance;
/// Severity level enum and resolution helpers.
pub mod severity;
/// Spec-coverage validation use cases.
pub mod speccoverage;
/// Spec-tree validators.
pub mod specs;
/// Test-coverage analysis use cases.
pub mod testcoverage;
