//! Environment-file use cases.
//!
//! Moved from `crate::internal::envbackup` and `crate::internal::envvalidate`.
//! Public API unchanged; those modules re-export everything from here.

/// Env-file backup and restore use case.
pub mod backup;
/// Env-injection manifest consistency use case.
pub mod injection;
/// Env-contract drift detection use case.
pub mod validate;
