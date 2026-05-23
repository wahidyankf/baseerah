// Byte-for-byte port of `apps/rhino-cli/internal/speccoverage/cucumber_expr.go`.
// Cucumber-expression → regex conversion, plus Python pytest-bdd parsers.parse format.

use std::sync::OnceLock;

use regex::Regex;

fn cucumber_param_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\{[^}]+\}").expect("valid regex"))
}

fn python_parsers_param_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\{(\w+)(?::([dgw]))?\}").expect("valid regex"))
}

/// Processes Cucumber expression escape sequences in literal text.
/// `\X` → `X` for `\(`, `\)`, `\{`, `\}`, `\/`, `\\`.
pub fn unescape_cucumber_expr(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut i = 0usize;
    while i < chars.len() {
        if chars[i] == '\\' && i + 1 < chars.len() {
            out.push(chars[i + 1]);
            i += 2;
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

pub fn cucumber_param_to_regex(param_name: &str) -> &'static str {
    match param_name {
        "string" => "\"[^\"]*\"",
        "int" | "byte" | "short" | "long" => r"-?\d+",
        "float" | "double" | "bigdecimal" => r"-?\d+\.?\d*",
        "word" => r"\S+",
        _ => ".+",
    }
}

pub fn cucumber_expr_to_regex(text: &str) -> String {
    let re = cucumber_param_re();
    let mut sb = String::new();
    let mut remaining = text;
    loop {
        match re.find(remaining) {
            None => {
                sb.push_str(&regex::escape(&unescape_cucumber_expr(remaining)));
                break;
            }
            Some(m) => {
                sb.push_str(&regex::escape(&unescape_cucumber_expr(
                    &remaining[..m.start()],
                )));
                let param = &remaining[m.start()..m.end()];
                let inner = &param[1..param.len() - 1];
                sb.push_str(cucumber_param_to_regex(inner));
                remaining = &remaining[m.end()..];
            }
        }
    }
    sb
}

pub fn has_cucumber_expressions(text: &str) -> bool {
    cucumber_param_re().is_match(text)
}

pub fn convert_python_parsers_expr(text: &str) -> String {
    let re = python_parsers_param_re();
    let mut sb = String::new();
    let mut remaining = text;
    loop {
        match re.find(remaining) {
            None => {
                sb.push_str(&regex::escape(remaining));
                break;
            }
            Some(m) => {
                sb.push_str(&regex::escape(&remaining[..m.start()]));
                let caps = re.captures(&remaining[m.start()..m.end()]).unwrap();
                let format_spec = caps.get(2).map(|x| x.as_str()).unwrap_or("");
                let chunk = match format_spec {
                    "d" => r"-?\d+",
                    "g" => r"-?\d+\.?\d*",
                    "w" => r"\S+",
                    _ => ".+",
                };
                sb.push_str(chunk);
                remaining = &remaining[m.end()..];
            }
        }
    }
    sb
}

pub fn is_python_parsers_expr(text: &str) -> bool {
    python_parsers_param_re().is_match(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unescape_basic_escapes() {
        assert_eq!(unescape_cucumber_expr(r"\(foo\)"), "(foo)");
        assert_eq!(unescape_cucumber_expr(r"a\\b"), "a\\b");
        assert_eq!(unescape_cucumber_expr("no escapes"), "no escapes");
    }

    #[test]
    fn cucumber_param_string_maps_to_quoted() {
        assert_eq!(cucumber_param_to_regex("string"), "\"[^\"]*\"");
    }

    #[test]
    fn cucumber_param_int_maps_to_signed_digits() {
        assert_eq!(cucumber_param_to_regex("int"), r"-?\d+");
        assert_eq!(cucumber_param_to_regex("long"), r"-?\d+");
    }

    #[test]
    fn cucumber_param_float_maps_to_signed_decimal() {
        assert_eq!(cucumber_param_to_regex("float"), r"-?\d+\.?\d*");
    }

    #[test]
    fn cucumber_param_word_maps_to_non_whitespace() {
        assert_eq!(cucumber_param_to_regex("word"), r"\S+");
    }

    #[test]
    fn cucumber_param_unknown_maps_to_any() {
        assert_eq!(cucumber_param_to_regex("custom"), ".+");
    }

    #[test]
    fn cucumber_expr_to_regex_string_param() {
        let r = cucumber_expr_to_regex("user enters {string}");
        assert_eq!(r, "user enters \"[^\"]*\"");
    }

    #[test]
    fn cucumber_expr_to_regex_escapes_literals() {
        let r = cucumber_expr_to_regex("a (1.0) b");
        // ( and . are regex specials → must be escaped.
        assert!(r.contains("\\("));
        assert!(r.contains("\\."));
    }

    #[test]
    fn cucumber_expr_to_regex_handles_escape_then_param() {
        // \(foo\) is literal "(foo)", then {int}
        let r = cucumber_expr_to_regex(r"\(foo\) {int}");
        assert!(r.contains(r"\(foo\)") || r.contains("\\(foo\\)"));
        assert!(r.contains(r"-?\d+"));
    }

    #[test]
    fn has_cucumber_expressions_detects_braces() {
        assert!(has_cucumber_expressions("user enters {string}"));
        assert!(!has_cucumber_expressions("user enters foo"));
    }

    #[test]
    fn python_parsers_d_maps_to_digit() {
        let r = convert_python_parsers_expr("count is {n:d}");
        assert!(r.contains(r"-?\d+"));
    }

    #[test]
    fn python_parsers_g_maps_to_float() {
        let r = convert_python_parsers_expr("ratio {r:g}");
        assert!(r.contains(r"-?\d+\.?\d*"));
    }

    #[test]
    fn python_parsers_w_maps_to_word() {
        let r = convert_python_parsers_expr("word {w:w}");
        assert!(r.contains(r"\S+"));
    }

    #[test]
    fn python_parsers_plain_name_maps_to_any() {
        let r = convert_python_parsers_expr("plain {x}");
        assert!(r.contains(".+"));
    }

    #[test]
    fn is_python_parsers_detects_format() {
        assert!(is_python_parsers_expr("{name}"));
        assert!(is_python_parsers_expr("{name:d}"));
        assert!(!is_python_parsers_expr("plain text"));
    }
}
