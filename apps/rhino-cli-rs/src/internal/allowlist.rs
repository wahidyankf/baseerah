// Mirrors `apps/rhino-cli/internal/allowlist/allowlist.go`.

pub fn apps_with_ddd() -> &'static [&'static str] {
    &["organiclever", "wahidyankf", "ose-platform", "ayokoding"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn membership() {
        let v = apps_with_ddd();
        assert_eq!(v.len(), 4);
        assert!(v.contains(&"organiclever"));
        assert!(v.contains(&"ayokoding"));
    }
}
