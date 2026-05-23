// Mirrors `apps/rhino-cli/internal/allowlist/allowlist.go`.

// Inclusion criterion: every full-stack app that ships a populated
// `ddd/bounded-contexts.yaml` registry belongs here, regardless of whether
// all declared BCs have Gherkin coverage yet.
//   - organiclever: bounded-contexts.yaml + feature files present
//   - wahidyankf:   bounded-contexts.yaml + feature files present
//   - ose-platform: bounded-contexts.yaml + feature files present
//   - ayokoding:    bounded-contexts.yaml + feature files present
//   - ose-app:      bounded-contexts.yaml present (4 BCs declared); features pending
pub fn apps_with_ddd() -> &'static [&'static str] {
    &[
        "organiclever",
        "wahidyankf",
        "ose-platform",
        "ayokoding",
        "ose-app",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn membership() {
        let v = apps_with_ddd();
        assert_eq!(v.len(), 5);
        assert!(v.contains(&"organiclever"));
        assert!(v.contains(&"ayokoding"));
        assert!(v.contains(&"ose-app"));
    }
}
