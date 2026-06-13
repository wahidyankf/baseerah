//! Shared test-only helpers for serializing process-global state.
//!
//! The process current-working-directory (cwd) is global mutable state. Several
//! unit tests spawn `git` child processes whose behaviour depends on the cwd
//! (or on `git`'s repository discovery walking up from a temp dir). When
//! `cargo test` runs these tests concurrently — the default — they can race:
//! one test's `TempDir` cleanup or any future `set_current_dir` call can
//! invalidate the cwd another test depends on, producing intermittent failures.
//!
//! [`CwdLock`] gives every cwd-sensitive test exclusive access to the cwd for
//! its duration and restores the original cwd on drop (even on panic).

use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};

/// Process-wide lock guarding every test that reads or mutates the cwd.
///
/// A single shared mutex means cwd-sensitive tests never run concurrently with
/// one another, eliminating the data race on the process-global cwd.
static CWD_GUARD: Mutex<()> = Mutex::new(());

/// RAII guard that serializes cwd-sensitive tests and restores the original cwd.
///
/// Construct one at the top of any test that reads the process cwd (directly or
/// via a spawned `git` process) or mutates it via `std::env::set_current_dir`.
/// Hold the returned value for the whole test body:
///
/// ```ignore
/// let _cwd = CwdLock::acquire();
/// // ... cwd-sensitive work ...
/// ```
///
/// On drop the original cwd is restored, so a test that changes the cwd cannot
/// leak that change into the next test that acquires the lock.
pub(crate) struct CwdLock {
    /// Held for the lifetime of the guard to serialize cwd-sensitive tests.
    _guard: MutexGuard<'static, ()>,
    /// The cwd captured at construction, restored on drop when available.
    original: Option<PathBuf>,
}

impl CwdLock {
    /// Acquires the shared cwd lock and records the current cwd for restoration.
    ///
    /// Lock poisoning (a panic in another cwd-sensitive test) is recovered from
    /// gracefully: the inner guard is reused so serialization still holds.
    pub(crate) fn acquire() -> Self {
        let guard = CWD_GUARD
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let original = std::env::current_dir().ok();
        Self {
            _guard: guard,
            original,
        }
    }
}

impl Drop for CwdLock {
    fn drop(&mut self) {
        if let Some(dir) = self.original.take() {
            // Best-effort restore; ignore failure so drop never panics.
            let _ = std::env::set_current_dir(dir);
        }
    }
}
