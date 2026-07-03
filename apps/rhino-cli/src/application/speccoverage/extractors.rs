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

/// Matches a Rust `#[given(…)]` / `#[when(…)]` / `#[then(…)]` literal step
/// attribute (string argument, no `expr =`/`regex =` prefix).
///
/// Deliberately omits a quote-wrapped example inline: illustrating the exact
/// attribute-with-quoted-argument shape here would itself satisfy the
/// pattern below when this file is scanned as Rust source (as every `.rs`
/// file under `apps/rhino-cli` is by [`extract_rust_step_texts`]), producing
/// a spurious self-referential orphan step-definition finding.
fn rs_step_literal_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"#\[(?:given|when|then)\s*\(\s*"((?:[^"\\]|\\.)*)"\s*\)\s*\]"#)
            .expect("valid regex")
    })
}

/// Matches a Rust `#[given(…)]` / `#[when(…)]` / `#[then(…)]` literal step
/// attribute whose argument is a hash-delimited raw string (`r#"…"#`) rather
/// than an escaped-quote string. Authors reach for this form specifically
/// when the step text itself embeds literal double quotes (e.g. a step
/// asserting behavior for markdown containing `"Claude Code"`) to avoid
/// `\"`-escaping every embedded quote — real precedent:
/// `apps/rhino-cli/tests/repo_governance.rs`. Omitting this form silently
/// dropped every step defined this way, producing false step-coverage gaps.
/// (See [`rs_step_literal_re`] for why this doc comment omits a quote-wrapped
/// example.)
fn rs_step_literal_raw_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r##"#\[(?:given|when|then)\s*\(\s*r#"(.*?)"#\s*\)\s*\]"##).expect("valid regex")
    })
}

/// Matches a Rust `#[given(expr = …)]` Cucumber-expression step attribute.
/// (See [`rs_step_literal_re`] for why the example omits quote marks.)
fn rs_step_expr_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"#\[(?:given|when|then)\s*\(\s*expr\s*=\s*"((?:[^"\\]|\\.)*)"\s*\)\s*\]"#)
            .expect("valid regex")
    })
}

/// Matches a Rust `#[given(regex = …)]` regex step attribute whose argument
/// uses the hash-delimited raw-string form (`r#"…"#`). (See
/// [`rs_step_literal_re`] for why the example omits quote marks.)
fn rs_step_regex_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r##"#\[(?:given|when|then)\s*\(\s*regex\s*=\s*r#"(.*?)"#\s*\)\s*\]"##)
            .expect("valid regex")
    })
}

/// Matches a Rust `#[given(regex = …)]` regex step attribute whose argument
/// uses the bare raw-string form (no `#` delimiter). Just as valid Rust as
/// the hash-delimited form handled by [`rs_step_regex_re`] whenever the
/// pattern itself contains no literal `"`, and in fact the form most
/// cucumber-rs step defs in this repo use — omitting this second form
/// silently dropped every step defined this way, producing false
/// step-coverage gaps.
fn rs_step_regex_bare_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"#\[(?:given|when|then)\s*\(\s*regex\s*=\s*r"(.*?)"\s*\)\s*\]"#)
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
/// Recognises five forms in priority order:
///
/// 1. `regex = r#"…"#` — raw regex, hash-delimited raw string (most specific).
/// 2. `regex = r"…"` — raw regex, bare raw string.
/// 3. `expr = "…"` — Cucumber expression.
/// 4. `"literal"` — plain string (also accepts Cucumber expressions).
/// 5. `r#"literal"#` — plain string whose text embeds quotes, hash-delimited
///    raw string.
///
/// Scans the whole file content rather than line-by-line: rustfmt routinely
/// wraps a `#[given(…)]`/`#[when(…)]`/`#[then(…)]` attribute whose
/// combined length exceeds `max_width` onto its own line(s) (attribute on one
/// line, string argument and closing bracket on the next), and a per-line
/// scan would never see the opening paren, string, and closing bracket in a
/// single match — silently dropping the step definition and producing a
/// false step-coverage gap even though cucumber-rs (which binds attributes
/// from the token stream, not source lines) matches the step correctly at
/// runtime. Same class of bug already fixed for the JVM/Kotlin and Dart
/// extractors (see [`extract_jvm_step_texts`] and [`extract_dart_step_texts`]).
/// None of the five regexes below need a dotall flag to support this: they
/// have no `.` metacharacter, and `\s`/negated character classes already
/// match `\n`.
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

    // Regex form (most specific) first — hash-delimited raw string, then bare
    // raw string (see [`rs_step_regex_bare_re`] for why both are needed).
    for caps in rs_step_regex_re().captures_iter(&content) {
        let pattern = caps
            .get(1)
            .expect("capture group 1 always present")
            .as_str();
        if let Ok(re) = Regex::new(pattern) {
            sm.add_pattern_with_origin(re, pattern, &path_s);
        }
    }
    for caps in rs_step_regex_bare_re().captures_iter(&content) {
        let pattern = caps
            .get(1)
            .expect("capture group 1 always present")
            .as_str();
        if let Ok(re) = Regex::new(pattern) {
            sm.add_pattern_with_origin(re, pattern, &path_s);
        }
    }
    // Expr form — Cucumber expressions.
    for caps in rs_step_expr_re().captures_iter(&content) {
        add_step_to_matcher_with_origin(
            sm,
            caps.get(1)
                .expect("capture group 1 always present")
                .as_str(),
            &path_s,
        );
    }
    // Literal form — also may have Cucumber expressions.
    for caps in rs_step_literal_re().captures_iter(&content) {
        add_step_to_matcher_with_origin(
            sm,
            caps.get(1)
                .expect("capture group 1 always present")
                .as_str(),
            &path_s,
        );
    }
    // Literal form, hash-delimited raw-string argument (text embeds quotes).
    for caps in rs_step_literal_raw_re().captures_iter(&content) {
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
    // Scan the whole file content, not line-by-line: formatters routinely
    // wrap long step strings onto their own line (`@When(\n  "..."\n)`), and
    // a per-line scan would never see the opening paren, string, and closing
    // paren in a single match. `jvm_step_re()` needs no dotall flag for this
    // — it has no `.` metacharacter, and its negated character classes
    // (`[^"\\]`) already match `\n` — so matching against `&content` (like
    // `extract_dart_step_texts` already does) is sufficient.
    for caps in jvm_step_re().captures_iter(&content) {
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
    fn rust_literal_step_uses_raw_string_form_when_text_embeds_quotes() {
        // A plain literal step (no `regex =`/`expr =` prefix) whose text contains
        // embedded double quotes is written using the hash-delimited raw-string
        // attribute form rather than an escaped-quote string — real precedent:
        // `apps/rhino-cli/tests/repo_governance.rs`'s step asserting behavior for
        // a governance markdown file containing the literal text "Claude Code"
        // inside a code fence. The previous extractor recognised only the
        // escaped-quote form, silently dropping every step defined this way.
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "x.rs",
            "#[given(r#\"a governance markdown file containing \"Claude Code\" inside a code fence\"#)]\nfn step() {}\n",
        );
        let mut sm = StepMatcher::new();
        extract_rust_step_texts(&p, &mut sm).unwrap();
        assert!(
            sm.matches("a governance markdown file containing \"Claude Code\" inside a code fence")
        );
    }

    #[test]
    fn rust_literal_step_attribute_split_across_lines_is_still_extracted() {
        // rustfmt wraps a `#[given(…)]`/`#[when(…)]`/`#[then(…)]` attribute
        // whose combined length exceeds `max_width` onto its own line(s):
        //
        //   #[given(
        //       "a repository with a convention file that is missing the heading"
        //   )]
        //
        // A line-by-line scan (the previous implementation) never sees the opening
        // paren, string, and closing bracket in a single line, silently dropping
        // the step definition and producing a false step-coverage gap even though
        // cucumber-rs itself (which operates on the token stream, not source
        // lines) binds the step correctly at runtime. Same class of bug already
        // fixed for the JVM/Kotlin extractor (see
        // `jvm_step_regex_matches_annotation_split_across_lines` above).
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "x.rs",
            "#[given(\n    \"a repository with a convention file that is missing the heading\"\n)]\nfn step() {}\n",
        );
        let mut sm = StepMatcher::new();
        extract_rust_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("a repository with a convention file that is missing the heading"));
    }

    #[test]
    fn rust_regex_step_attribute_split_across_lines_is_still_extracted() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "x.rs",
            "#[then(\n    regex = r#\"^result is (\\d+)$\"#\n)]\nfn step() {}\n",
        );
        let mut sm = StepMatcher::new();
        extract_rust_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("result is 7"));
    }

    #[test]
    fn rust_expr_step_attribute_split_across_lines_is_still_extracted() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "x.rs",
            "#[when(\n    expr = \"count is {int}\"\n)]\nfn step() {}\n",
        );
        let mut sm = StepMatcher::new();
        extract_rust_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("count is 42"));
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
    fn rust_regex_step_uses_bare_raw_string_form() {
        // `regex = r"…"` (no `#` delimiter) is just as valid Rust as
        // `regex = r#"…"#` and is in fact the form most cucumber-rs step defs
        // in this very repo use (e.g. `apps/rhino-cli/tests/test_coverage.rs`,
        // `tests/agents.rs`, `tests/convention.rs`) whenever the pattern itself
        // contains no literal `"` needing the hash-delimited escape. The
        // previous extractor recognised only the hash-delimited form, silently
        // dropping every step defined this way and producing a false
        // step-coverage gap.
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "x.rs",
            "#[given(regex = r\"^a Go coverage file recording (\\d+)% line coverage$\")]\nfn step(n: u32) {}\n",
        );
        let mut sm = StepMatcher::new();
        extract_rust_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("a Go coverage file recording 90% line coverage"));
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
    fn jvm_step_regex_matches_annotation_split_across_lines() {
        // Kotlin/Java formatters wrap long step strings onto their own line:
        // `@When(\n  "..."\n)`. The extractor previously scanned line-by-line
        // (`for line in content.lines()`), so `jvm_step_re()` — which needs
        // the opening paren, string, and closing paren on one match — never
        // saw the annotation at all, silently dropping the step definition.
        // None of the character classes in the pattern need a dotall flag
        // (there is no `.` metacharacter — `[^"\\]` already matches `\n`),
        // so scanning the whole file content (like the Dart extractor
        // already does) is sufficient to fix this.
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "AuthSteps.kt",
            "@When(\n  \"^the client sends POST /api/v1/auth/login with body \\\\{ \\\"username\\\": \\\"([^\\\"]+)\\\", \\\"password\\\": \\\"([^\\\"]+)\\\" \\\\}$\"\n)\nfun step() {}\n",
        );
        let mut sm = StepMatcher::new();
        extract_jvm_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches(
            r#"the client sends POST /api/v1/auth/login with body { "username": "alice", "password": "Str0ng#Pass1" }"#
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
