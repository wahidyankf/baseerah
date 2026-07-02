//! Per-language step-definition extractors.
//!
//! Ports `rust_steps.go`, `java_steps.go`, `dart_steps.go`,
//! `clojure_steps.go`, `elixir_steps.go`, `python_steps.go`,
//! `dotnet_steps.go`, and the Go/TS extractors that live inside
//! `checker.go`.
//!
//! Each public function reads the file at `path` and inserts extracted step
//! entries into `sm` via the [`super::matcher`] helpers. Per-language nuance
//! (verbatim strings, regex form, F# backtick names, Python `parsers.parse`,
//! etc.) is preserved verbatim from the Go original.

use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

use anyhow::Error;
use regex::Regex;

use super::matcher::{
    StepMatcher, add_python_step_to_matcher_with_origin, add_step_to_matcher_with_origin,
};
use super::util::{first_non_empty, normalize_ws, unescape_string};

// ============================================================
// Regex registry — compiled lazily, mirrors Go package-level vars.
// ============================================================

/// Matches a Rust `#[given("…")]` / `#[when("…")]` / `#[then("…")]` literal
/// step attribute.
fn rs_step_literal_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"#\[(?:given|when|then)\s*\(\s*"((?:[^"\\]|\\.)*)"\s*\)\s*\]"#)
            .expect("valid regex")
    })
}

/// Matches a Rust `#[given(expr = "…")]` Cucumber-expression step attribute.
fn rs_step_expr_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"#\[(?:given|when|then)\s*\(\s*expr\s*=\s*"((?:[^"\\]|\\.)*)"\s*\)\s*\]"#)
            .expect("valid regex")
    })
}

/// Matches a Rust `#[given(regex = r#"…"#)]` regex step attribute.
fn rs_step_regex_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r##"#\[(?:given|when|then)\s*\(\s*regex\s*=\s*r#"(.*?)"#\s*\)\s*\]"##)
            .expect("valid regex")
    })
}

/// Matches a JVM (Java/Kotlin) `@Given("…")` / `@When("…")` / `@Then("…")`
/// / `@And("…")` / `@But("…")` annotation.
fn jvm_step_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"@(?:Given|When|Then|And|But)\s*\(\s*"((?:[^"\\]|\\.)*)"\s*\)"#)
            .expect("valid regex")
    })
}

/// Matches a Dart `s.given("…", …)` / `s.when("…", …)` / etc. step call.
fn dart_step_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r#"(?s)\b(?:s|scenario)\.(?:given|when|then|and|but)\s*\(\s*(?:"((?:[^"\\]|\\.)*)"|'((?:[^'\\]|\\.)*)')\s*,"#,
        )
        .expect("valid regex")
    })
}

/// Matches a Clojure `(Given "…")` / `(When "…")` / etc. step form.
fn clj_step_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"\((?:Given|When|Then|And|But)\s+"((?:[^"\\]|\\.)*)""#).expect("valid regex")
    })
}

/// Matches an Elixir `defgiven ~r/…/` / `defwhen ~r/…/` / etc. step macro.
fn ex_step_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"def(?:given|when|then|and_|but_)\s+~r/\^?(.*?)\$?/").expect("valid regex")
    })
}

/// Matches a Python pytest-bdd `@given("…")` / `@when("…")` / etc. decorator,
/// including the `parsers.parse(…)` and `parsers.cfparse(…)` wrappers.
fn py_step_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r#"(?s)@(?:given|when|then|step)\s*\(\s*(?:parsers\.(?:parse|cfparse)\s*\(\s*)?(?:"((?:[^"\\]|\\.)*)"|'((?:[^'\\]|\\.)*)')\s*\)?\s*(?:,\s*[^)]*)?\)"#,
        )
        .expect("valid regex")
    })
}

/// Matches a Python `@scenario("feature.feature", "Title")` decorator.
fn py_scenario_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"@scenario\s*\(\s*"[^"]*"\s*,\s*"((?:[^"\\]|\\.)*)"\s*\)"#)
            .expect("valid regex")
    })
}

/// Matches a C# verbatim-string `[Given(@"…")]` step attribute.
fn cs_verbatim_step_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"(?s)\[(?:Given|When|Then|And|But)\s*\(\s*@"((?:[^"]|"")*)"\s*\)\s*\]"#)
            .expect("valid regex")
    })
}

/// Matches a C# regular-string `[Given("…")]` step attribute.
fn cs_regular_step_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"(?s)\[(?:Given|When|Then|And|But)\s*\(\s*"((?:[^"\\]|\\.)*)"\s*\)\s*\]"#)
            .expect("valid regex")
    })
}

/// Matches an F# `[<Given>]` / `[<When>]` / etc. step attribute marker line.
fn fs_step_attr_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\[<(?:Given|When|Then|And|But)>]").expect("valid regex"))
}

/// Matches an F# inline step: `let [<Given>] ``step text`` () =`.
fn fs_step_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"let\s+(?:\[<(?:Given|When|Then|And|But)>\]\s*)?``((?:[^`]|`[^`])*)``")
            .expect("valid regex")
    })
}

/// Matches an F# `let ``backtick name`` …` binding used for multi-line step style.
fn fs_let_backtick_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"let\s+``((?:[^`]|`[^`])*)``").expect("valid regex"))
}

// ============================================================
// Per-language extractors
// ============================================================

/// Extracts step definitions from a Rust source file.
///
/// Recognises three forms in priority order:
///
/// 1. `regex = r#"…"#` — raw regex (most specific).
/// 2. `expr = "…"` — Cucumber expression.
/// 3. `"literal"` — plain string (also accepts Cucumber expressions).
///
/// # Errors
///
/// Returns an error if the file cannot be read.
///
/// # Panics
///
/// Panics if a regex capture group that is always present is absent (indicates
/// a bug in the compiled regex).
pub fn extract_rust_step_texts(
    path: &Path,
    sm: &mut StepMatcher,
) -> std::result::Result<(), Error> {
    let content = fs::read_to_string(path)?;
    let path_s = path.to_string_lossy();
    for line in content.lines() {
        // Regex form (most specific) first.
        for caps in rs_step_regex_re().captures_iter(line) {
            let pattern = caps
                .get(1)
                .expect("capture group 1 always present")
                .as_str();
            if let Ok(re) = Regex::new(pattern) {
                sm.add_pattern_with_origin(re, pattern, &path_s);
            }
        }
        // Expr form — Cucumber expressions.
        for caps in rs_step_expr_re().captures_iter(line) {
            add_step_to_matcher_with_origin(
                sm,
                caps.get(1)
                    .expect("capture group 1 always present")
                    .as_str(),
                &path_s,
            );
        }
        // Literal form — also may have Cucumber expressions.
        for caps in rs_step_literal_re().captures_iter(line) {
            add_step_to_matcher_with_origin(
                sm,
                caps.get(1)
                    .expect("capture group 1 always present")
                    .as_str(),
                &path_s,
            );
        }
    }
    Ok(())
}

/// Extracts step definitions from a JVM (Java or Kotlin) source file.
///
/// Recognises `@Given("…")`, `@When("…")`, `@Then("…")`, `@And("…")`,
/// and `@But("…")` annotations.
///
/// # Errors
///
/// Returns an error if the file cannot be read.
///
/// # Panics
///
/// Panics if a regex capture group that is always present is absent (indicates
/// a bug in the compiled regex).
pub fn extract_jvm_step_texts(path: &Path, sm: &mut StepMatcher) -> std::result::Result<(), Error> {
    let content = fs::read_to_string(path)?;
    let path_s = path.to_string_lossy();
    for line in content.lines() {
        for caps in jvm_step_re().captures_iter(line) {
            let raw = caps
                .get(1)
                .expect("capture group 1 always present")
                .as_str();
            // Java/Kotlin string literals require a doubled backslash (`\\`)
            // to embed a single `\` at runtime — e.g. a `^`-anchored regex
            // source `\\?` (escaped literal `?`) or a Cucumber-expression
            // source `\\/`/`\\{`/`\\}` (escaped literal `/`/`{`/`}`) both
            // compile to one runtime backslash. The regex capture pulls raw
            // source bytes, so it must be unescaped to the true runtime
            // string value before `add_step_to_matcher_with_origin` decides
            // whether that value is a regex, a Cucumber expression, or an
            // exact literal — otherwise doubled backslashes are misread
            // (e.g. `\\?` as "zero-or-one backslash", never matching `?`).
            add_step_to_matcher_with_origin(sm, &unescape_string(raw), &path_s);
        }
    }
    Ok(())
}

/// Extracts step definitions from a Dart source file.
///
/// Recognises `s.given("…", …)` / `s.when("…", …)` / etc. call patterns
/// with both double-quoted and single-quoted string literals.
///
/// # Errors
///
/// Returns an error if the file cannot be read.
pub fn extract_dart_step_texts(
    path: &Path,
    sm: &mut StepMatcher,
) -> std::result::Result<(), Error> {
    let content = fs::read_to_string(path)?;
    let path_s = path.to_string_lossy();
    for caps in dart_step_re().captures_iter(&content) {
        let dq = caps.get(1).map_or("", |m| m.as_str());
        let sq = caps.get(2).map_or("", |m| m.as_str());
        let text = unescape_string(first_non_empty(dq, sq));
        add_step_to_matcher_with_origin(sm, &text, &path_s);
    }
    Ok(())
}

/// Extracts step definitions from a Clojure source file.
///
/// Recognises `(Given "…")`, `(When "…")`, `(Then "…")`, `(And "…")`,
/// and `(But "…")` step forms.
///
/// # Errors
///
/// Returns an error if the file cannot be read.
///
/// # Panics
///
/// Panics if a regex capture group that is always present is absent (indicates
/// a bug in the compiled regex).
pub fn extract_clojure_step_texts(
    path: &Path,
    sm: &mut StepMatcher,
) -> std::result::Result<(), Error> {
    let content = fs::read_to_string(path)?;
    let path_s = path.to_string_lossy();
    for line in content.lines() {
        for caps in clj_step_re().captures_iter(line) {
            add_step_to_matcher_with_origin(
                sm,
                caps.get(1)
                    .expect("capture group 1 always present")
                    .as_str(),
                &path_s,
            );
        }
    }
    Ok(())
}

/// Extracts step definitions from an Elixir source file.
///
/// Recognises `defgiven ~r/…/`, `defwhen ~r/…/`, `defthen ~r/…/`, etc.
/// macros and compiles the sigil body as a regex pattern.
///
/// # Errors
///
/// Returns an error if the file cannot be read.
///
/// # Panics
///
/// Panics if a regex capture group that is always present is absent (indicates
/// a bug in the compiled regex).
pub fn extract_elixir_step_texts(
    path: &Path,
    sm: &mut StepMatcher,
) -> std::result::Result<(), Error> {
    let content = fs::read_to_string(path)?;
    let path_s = path.to_string_lossy();
    for line in content.lines() {
        for caps in ex_step_re().captures_iter(line) {
            let pattern = caps
                .get(1)
                .expect("capture group 1 always present")
                .as_str();
            if let Ok(re) = Regex::new(pattern) {
                sm.add_pattern_with_origin(re, pattern, &path_s);
            }
        }
    }
    Ok(())
}

/// Extracts step definitions from a Python (pytest-bdd) source file.
///
/// Recognises `@given(…)`, `@when(…)`, `@then(…)`, and `@step(…)` decorators,
/// including the `parsers.parse(…)` and `parsers.cfparse(…)` wrappers.
///
/// Double braces `{{` / `}}` are collapsed to single braces before dispatching
/// to the Python matcher helper.
///
/// # Errors
///
/// Returns an error if the file cannot be read.
pub fn extract_python_step_texts(
    path: &Path,
    sm: &mut StepMatcher,
) -> std::result::Result<(), Error> {
    let content = fs::read_to_string(path)?;
    let path_s = path.to_string_lossy();
    for caps in py_step_re().captures_iter(&content) {
        let dq = caps.get(1).map_or("", |m| m.as_str());
        let sq = caps.get(2).map_or("", |m| m.as_str());
        let mut text = first_non_empty(dq, sq).to_string();
        // Python uses {{...}} for literal braces in parsers.parse format strings.
        text = text.replace("{{", "{").replace("}}", "}");
        add_python_step_to_matcher_with_origin(sm, &text, &path_s);
    }
    Ok(())
}

/// Extracts `@scenario("feature.feature", "Title")` scenario titles from a
/// Python pytest-bdd test file.
///
/// # Errors
///
/// Returns an error if the file cannot be read.
///
/// # Panics
///
/// Panics if a regex capture group that is always present is absent (indicates
/// a bug in the compiled regex).
pub fn extract_python_scenario_titles(
    test_file_path: &Path,
) -> std::result::Result<HashSet<String>, Error> {
    let content = fs::read_to_string(test_file_path)?;
    let mut titles = HashSet::new();
    for line in content.lines() {
        for caps in py_scenario_re().captures_iter(line) {
            titles.insert(normalize_ws(
                caps.get(1)
                    .expect("capture group 1 always present")
                    .as_str(),
            ));
        }
    }
    Ok(titles)
}

/// Extracts step definitions from a C# `SpecFlow` source file.
///
/// Processes verbatim strings (`@"…"`) before regular strings (`"…"`) so that
/// the more specific form takes priority.  Verbatim `""` escape sequences are
/// collapsed to a single `"` character.
///
/// # Errors
///
/// Returns an error if the file cannot be read.
///
/// # Panics
///
/// Panics if a regex capture group that is always present is absent (indicates
/// a bug in the compiled regex).
pub fn extract_csharp_step_texts(
    path: &Path,
    sm: &mut StepMatcher,
) -> std::result::Result<(), Error> {
    let content = fs::read_to_string(path)?;
    let path_s = path.to_string_lossy();
    // Verbatim strings first (more specific).
    for caps in cs_verbatim_step_re().captures_iter(&content) {
        let text = caps
            .get(1)
            .expect("capture group 1 always present")
            .as_str()
            .replace("\"\"", "\"");
        add_step_to_matcher_with_origin(sm, &text, &path_s);
    }
    // Regular strings.
    for caps in cs_regular_step_re().captures_iter(&content) {
        add_step_to_matcher_with_origin(
            sm,
            caps.get(1)
                .expect("capture group 1 always present")
                .as_str(),
            &path_s,
        );
    }
    Ok(())
}

/// Extracts step definitions from an F# `TickSpec` source file.
///
/// Handles two layout styles:
///
/// 1. **Inline** — attribute and backtick name on the same line:
///    `let [<Given>] ``step text`` () =`.
/// 2. **Multi-line** — attribute on one line, `let ``step text`` () =` on the
///    next.
///
/// The extracted name is normalised and wrapped in `^…$` anchors before
/// compilation as a regex pattern (F# backtick names act as patterns in
/// `TickSpec`).
///
/// # Errors
///
/// Returns an error if the file cannot be read.
///
/// # Panics
///
/// Panics if a regex capture group that is always present is absent (indicates
/// a bug in the compiled regex).
pub fn extract_fsharp_step_texts(
    path: &Path,
    sm: &mut StepMatcher,
) -> std::result::Result<(), Error> {
    let content = fs::read_to_string(path)?;
    let path_s = path.to_string_lossy();
    let mut prev_line_has_step_attr = false;
    for line in content.lines() {
        let this_line_has_step_attr = fs_step_attr_re().is_match(line);

        // Case 1: inline style — attribute + backtick name on same line.
        if this_line_has_step_attr {
            for caps in fs_step_re().captures_iter(line) {
                add_fsharp_step_pattern(
                    caps.get(1)
                        .expect("capture group 1 always present")
                        .as_str(),
                    &path_s,
                    sm,
                );
            }
        }

        // Case 2: multiline style — attribute on previous line, this line is `let ``…``  () =`.
        if prev_line_has_step_attr && !this_line_has_step_attr {
            for caps in fs_let_backtick_re().captures_iter(line) {
                add_fsharp_step_pattern(
                    caps.get(1)
                        .expect("capture group 1 always present")
                        .as_str(),
                    &path_s,
                    sm,
                );
            }
        }
        prev_line_has_step_attr = this_line_has_step_attr;
    }
    Ok(())
}

/// Normalises an F# backtick-quoted step name and registers it as a `^…$`
/// anchored regex pattern in `sm`.
fn add_fsharp_step_pattern(name: &str, path: &str, sm: &mut StepMatcher) {
    let text = normalize_ws(name);
    let pattern = format!("^{text}$");
    if let Ok(re) = Regex::new(&pattern) {
        sm.add_pattern_with_origin(re, &pattern, path);
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn write(dir: &Path, name: &str, content: &str) -> std::path::PathBuf {
        let p = dir.join(name);
        fs::write(&p, content).unwrap();
        p
    }

    #[test]
    fn rust_literal_step_is_added_as_exact() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "x.rs",
            "#[given(\"user logs in\")]\nfn step() {}\n",
        );
        let mut sm = StepMatcher::new();
        extract_rust_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("user logs in"));
    }

    #[test]
    fn rust_expr_step_is_compiled_as_cucumber_pattern() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "x.rs",
            "#[when(expr = \"count is {int}\")]\nfn step() {}\n",
        );
        let mut sm = StepMatcher::new();
        extract_rust_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("count is 42"));
    }

    #[test]
    fn rust_regex_step_uses_raw_pattern() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "x.rs",
            "#[then(regex = r#\"^result is (\\d+)$\"#)]\nfn step() {}\n",
        );
        let mut sm = StepMatcher::new();
        extract_rust_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("result is 7"));
    }

    #[test]
    fn jvm_step_extraction() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "Steps.java",
            "@Given(\"user logs in\")\nvoid step() {}\n@When(\"submits {string}\")\nvoid step2() {}\n",
        );
        let mut sm = StepMatcher::new();
        extract_jvm_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("user logs in"));
        assert!(sm.matches(r#"submits "alice""#));
    }

    #[test]
    fn jvm_step_regex_unescapes_doubled_backslash_before_literal_question_mark() {
        // Java/Kotlin source must double a backslash to embed a single `\` in
        // the compiled regex, so `@When("^...pl\\?from=...$")` in the .java
        // file is the runtime regex `^...pl\?from=...$` (an escaped literal
        // `?`). Feeding the raw, un-unescaped source bytes straight to
        // `Regex::new` turned `\\?` into "zero-or-one literal backslash",
        // never matching the literal `?` in the Gherkin step text.
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "ReportingSteps.java",
            "@When(\"^alice sends GET /api/v1/reports/pl\\\\?from=2025-01-01&to=2025-01-31&currency=USD$\")\nvoid step() {}\n",
        );
        let mut sm = StepMatcher::new();
        extract_jvm_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches(
            "alice sends GET /api/v1/reports/pl?from=2025-01-01&to=2025-01-31&currency=USD"
        ));
    }

    #[test]
    fn dart_step_extraction() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "steps.dart",
            r#"s.given("user logs in", (Scenario s) async {});"#,
        );
        let mut sm = StepMatcher::new();
        extract_dart_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("user logs in"));
    }

    #[test]
    fn clojure_step_extraction() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "steps.clj",
            "(Given \"user logs in\" []\n  ...)\n",
        );
        let mut sm = StepMatcher::new();
        extract_clojure_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("user logs in"));
    }

    #[test]
    fn elixir_step_extraction() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "steps.ex",
            "defgiven ~r/^user logs in$/ do\nend\n",
        );
        let mut sm = StepMatcher::new();
        extract_elixir_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("user logs in"));
    }

    #[test]
    fn python_step_extraction_with_parsers_parse() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "steps.py",
            "@given(parsers.parse(\"count is {n:d}\"))\ndef step(n):\n    pass\n",
        );
        let mut sm = StepMatcher::new();
        extract_python_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("count is 42"));
    }

    #[test]
    fn python_scenario_titles_extracted() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "test.py",
            "@scenario(\"foo.feature\", \"User logs in\")\ndef test_login():\n    pass\n",
        );
        let titles = extract_python_scenario_titles(&p).unwrap();
        assert!(titles.contains("User logs in"));
    }

    #[test]
    fn csharp_verbatim_string_step() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "Steps.cs",
            "[Given(@\"user says \"\"hello\"\"\")]\nvoid Step() {}\n",
        );
        let mut sm = StepMatcher::new();
        extract_csharp_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches(r#"user says "hello""#));
    }

    #[test]
    fn fsharp_inline_step() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "Steps.fs",
            "let [<Given>] ``user logs in`` () = ()\n",
        );
        let mut sm = StepMatcher::new();
        extract_fsharp_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("user logs in"));
    }

    #[test]
    fn fsharp_multiline_step() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "Steps.fs",
            "[<Given>]\nlet ``user logs in`` () = ()\n",
        );
        let mut sm = StepMatcher::new();
        extract_fsharp_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("user logs in"));
    }
}
