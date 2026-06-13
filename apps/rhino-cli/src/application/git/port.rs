//! Port for reading staged file lists from the git index.

use std::path::Path;

use anyhow::Error;

/// Port for reading the files currently staged in the git index.
pub trait StagedFileProvider: Send {
    /// Returns the list of staged file paths relative to `git_root`.
    ///
    /// # Errors
    /// Returns an error if the git command fails or output cannot be decoded.
    fn get_staged(&self, git_root: &Path) -> Result<Vec<String>, Error>;
}
