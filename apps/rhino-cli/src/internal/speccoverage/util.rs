//! Shared string utilities for the spec-coverage suite.
//!
//! Ports `normalizeWS`, `firstNonEmpty`, and `unescapeString` from
//! `checker.go`. All functions are pure and allocation-minimal.

/// Collapses all runs of ASCII whitespace to a single space and trims leading
/// and trailing whitespace.
///
/// Uses [`str::split_whitespace`], which handles tabs, newlines, and multiple
/// consecutive spaces uniformly.
pub fn normalize_ws(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Returns `a` if it is non-empty, otherwise returns `b`.
///
/// Mirrors Go's `firstNonEmpty(a, b string) string` helper used when
/// extracting step text from regex capture groups that may be empty.
pub fn first_non_empty<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.is_empty() { b } else { a }
}

/// Interprets JS/TS-style escape sequences and returns the decoded string.
///
/// Recognized sequences: `\'`, `\"`, `\\`, `\/`, `\n`, `\t`, `\r`.
/// Any other `\X` pair passes `X` through unchanged.
///
/// The implementation iterates by Unicode scalar values (chars), not bytes,
/// making it safe for arbitrary UTF-8 input.
pub fn unescape_string(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut i = 0usize;
    while i < chars.len() {
        if chars[i] == '\\' && i + 1 < chars.len() {
            let c = match chars[i + 1] {
                '\'' => '\'',
                '"' => '"',
                '\\' => '\\',
                '/' => '/',
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                other => other,
            };
            out.push(c);
            i += 2;
        } else {
            out.push(chars[i]);
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
