//! Allowlist of full-stack applications that have DDD bounded-context registries.
//!
//! Mirrors `apps/rhino-cli/internal/allowlist/allowlist.go`.
//!
//! Inclusion criterion: every full-stack app that ships a populated
//! `ddd/bounded-contexts.yaml` registry belongs here, regardless of whether
//! all declared BCs have Gherkin coverage yet.
//!   - organiclever: bounded-contexts.yaml + feature files present
//!   - ose:          bounded-contexts.yaml present (5 BCs declared); unified from ose-app
//!     and ose-platform during standardize-app-spec-trees plan (2026-06-11)

/// Returns the list of application identifiers that maintain a DDD
/// bounded-context registry (`ddd/bounded-contexts.yaml`).
///
/// This list drives DDD validation: only apps present here are checked for
/// context coverage, glossary completeness, and Gherkin alignment.
pub fn apps_with_ddd() -> &'static [&'static str] {
    &["organiclever", "ose"]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the allowlist contains the expected number of entries
    /// and includes the required application identifiers.
    #[test]
    fn membership() {
        let v = apps_with_ddd();
        assert_eq!(v.len(), 2);
        assert!(v.contains(&"organiclever"));
        assert!(v.contains(&"ose"));
    }
}
