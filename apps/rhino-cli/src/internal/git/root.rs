//! Backward-compatibility re-export of the git root locator.
//!
//! The implementation now lives in `crate::infrastructure::git::root`.
pub use crate::infrastructure::git::root::find_root;
