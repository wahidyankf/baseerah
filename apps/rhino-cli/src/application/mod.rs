//! Application use cases and port definitions.

/// Doctor (toolchain-check) use case.
pub mod doctor;
/// Environment-file use cases (backup, validate).
pub mod env;
/// Git pre-commit use cases and port definitions.
pub mod git;
/// Mermaid validation use cases and extractor port.
pub mod mermaid;
/// Test-coverage analysis use cases.
pub mod testcoverage;
