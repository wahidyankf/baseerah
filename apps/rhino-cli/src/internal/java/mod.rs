//! Java source-tree validation: null-safety annotation enforcement.
//!
//! The `lang java null-safety-annotations validate` command scans a source root and verifies
//! every package (any directory containing at least one `.java` file) has a
//! `package-info.java` carrying the required null-safety annotation.

pub mod reporter;
pub mod scanner;
pub mod types;
pub mod validator;
