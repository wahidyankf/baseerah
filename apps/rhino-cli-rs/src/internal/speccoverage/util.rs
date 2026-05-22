// Shared string utilities for the speccoverage suite.
// Ports normalizeWS, firstNonEmpty, unescapeString from checker.go.

pub fn normalize_ws(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn first_non_empty<'a>(a: &'a str, b: &'a str) -> &'a str {
    if !a.is_empty() {
        a
    } else {
        b
    }
}

/// JS/TS-style escape sequence handling: `\'`, `\"`, `\\`, `\/`, `\n`, `\t`, `\r`.
pub fn unescape_string(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'\\' && i + 1 < bytes.len() {
            let c = match bytes[i + 1] {
                b'\'' => '\'',
                b'"' => '"',
                b'\\' => '\\',
                b'/' => '/',
                b'n' => '\n',
                b't' => '\t',
                b'r' => '\r',
                other => other as char,
            };
            out.push(c);
            i += 2;
        } else {
            out.push(bytes[i] as char);
            i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_ws_collapses_runs_and_trims() {
        assert_eq!(normalize_ws("  a  b\t c\n"), "a b c");
    }

    #[test]
    fn first_non_empty_returns_a_when_present() {
        assert_eq!(first_non_empty("a", "b"), "a");
        assert_eq!(first_non_empty("", "b"), "b");
        assert_eq!(first_non_empty("", ""), "");
    }

    #[test]
    fn unescape_string_handles_common_sequences() {
        assert_eq!(unescape_string(r#"a\"b"#), "a\"b");
        assert_eq!(unescape_string(r"a\\b"), "a\\b");
        assert_eq!(unescape_string(r"a\nb"), "a\nb");
        assert_eq!(unescape_string(r"a\tb"), "a\tb");
        assert_eq!(unescape_string(r"a\/b"), "a/b");
    }
}
