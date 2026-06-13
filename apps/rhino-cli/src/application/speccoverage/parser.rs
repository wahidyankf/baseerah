//! Gherkin feature-file parser.
//!
//! Byte-for-byte port of `apps/rhino-cli/internal/speccoverage/parser.go`.
//! Implements the same rules as the Go original:
//!
//! - `Background:` steps are collected and inserted as a synthetic
//!   `"(Background)"` scenario at position 0.
//! - `Scenario Outline:` steps have their `<placeholder>` tokens expanded
//!   for each row in the associated `Examples:` table and stored in
//!   [`ParsedStep::variants`].
//! - Plain `Scenario:` steps have an empty `variants` vector.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::Error;

/// A single parsed Gherkin step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedStep {
    /// Gherkin keyword without trailing whitespace (e.g. `"Given"`, `"When"`, `"Then"`).
    pub keyword: String,
    /// Step text after the keyword, with `<placeholder>` tokens left verbatim.
    pub text: String,
    /// Expanded step texts produced by substituting each `Examples` row into
    /// the `<placeholder>` tokens.  Empty for plain (non-outline) steps.
    pub variants: Vec<String>,
}

/// A single parsed Gherkin scenario (or the synthetic `Background` scenario).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParsedScenario {
    /// Scenario title as it appears after `Scenario:` or `Scenario Outline:`.
    /// The synthetic background uses the title `"(Background)"`.
    pub title: String,
    /// Ordered list of steps belonging to this scenario.
    pub steps: Vec<ParsedStep>,
}

/// Gherkin step keywords recognised by the parser (each includes a trailing space).
const STEP_KEYWORDS: [&str; 5] = ["Given ", "When ", "Then ", "And ", "But "];

/// Parses a `.feature` file and returns all scenarios, including a synthetic
/// `(Background)` scenario prepended when a `Background:` block is present.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or if a line cannot be read.
pub fn parse_feature_file(path: &Path) -> std::result::Result<Vec<ParsedScenario>, Error> {
    let (scenarios, _, ()) = parse_feature_file_inner(path)?;
    Ok(scenarios)
}

/// Returns all expanded step texts produced by `Scenario Outline` + `Examples`
/// substitution in the given feature file.
///
/// Useful for collecting the full set of concrete step strings when checking
/// whether step definitions cover parametrised scenarios.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or if a line cannot be read.
pub fn expanded_outline_step_texts(path: &Path) -> std::result::Result<Vec<String>, Error> {
    let (_, expanded, ()) = parse_feature_file_inner(path)?;
    Ok(expanded)
}

/// Internal parser implementation shared by [`parse_feature_file`] and
/// [`expanded_outline_step_texts`].
///
/// Returns a triple of `(scenarios, expanded_steps, ())`.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or if a line cannot be read.
fn parse_feature_file_inner(
    path: &Path,
) -> std::result::Result<(Vec<ParsedScenario>, Vec<String>, ()), Error> {
    let file = File::open(path)?;
    let mut scenarios: Vec<ParsedScenario> = Vec::new();
    let mut expanded_steps: Vec<String> = Vec::new();
    let mut bg_steps: Vec<ParsedStep> = Vec::new();
    let mut in_background = false;

    // Index of current scenario in `scenarios` (None when not inside one).
    let mut current_idx: Option<usize> = None;

    // Outline tracking — indices of outline steps within current.steps so we can
    // populate their variants when Examples rows arrive.
    let mut pending_outline_indices: Option<Vec<usize>> = None;

    let mut in_examples = false;
    let mut ex_headers: Option<Vec<String>> = None;

    for raw in BufReader::new(file).lines() {
        let Ok(line_owned) = raw else { continue };
        let line = line_owned.trim();

        if line.starts_with("Background:") {
            in_examples = false;
            ex_headers = None;
            pending_outline_indices = None;
            in_background = true;
            current_idx = None;
            continue;
        }

        if let Some(rest) = line.strip_prefix("Scenario Outline:") {
            in_examples = false;
            ex_headers = None;
            in_background = false;
            scenarios.push(ParsedScenario {
                title: rest.trim().to_string(),
                steps: Vec::new(),
            });
            current_idx = Some(scenarios.len() - 1);
            pending_outline_indices = Some(Vec::new());
            continue;
        }
        if let Some(rest) = line.strip_prefix("Scenario:") {
            in_examples = false;
            ex_headers = None;
            in_background = false;
            scenarios.push(ParsedScenario {
                title: rest.trim().to_string(),
                steps: Vec::new(),
            });
            current_idx = Some(scenarios.len() - 1);
            pending_outline_indices = None;
            continue;
        }

        if line.starts_with("Examples:") {
            in_examples = true;
            ex_headers = None;
            continue;
        }

        if in_examples && line.starts_with('|') {
            let row = parse_row(line);
            if ex_headers.is_none() {
                ex_headers = Some(row);
                continue;
            }
            if let (Some(idxs), Some(idx)) = (pending_outline_indices.as_ref(), current_idx) {
                let headers = ex_headers
                    .as_ref()
                    .expect("ex_headers is Some — is_none() branch above continues");
                for &step_idx in idxs {
                    let text = scenarios[idx].steps[step_idx].text.clone();
                    let exp = expand_step(&text, headers, &row);
                    scenarios[idx].steps[step_idx].variants.push(exp.clone());
                    expanded_steps.push(exp);
                }
            }
            continue;
        }

        for kw in STEP_KEYWORDS {
            if let Some(rest) = line.strip_prefix(kw) {
                let step = ParsedStep {
                    keyword: kw.trim().to_string(),
                    text: rest.trim().to_string(),
                    variants: Vec::new(),
                };
                if in_background {
                    bg_steps.push(step);
                } else if let Some(idx) = current_idx {
                    scenarios[idx].steps.push(step);
                    if let Some(idxs) = pending_outline_indices.as_mut() {
                        idxs.push(scenarios[idx].steps.len() - 1);
                    }
                }
                break;
            }
        }
    }

    if !bg_steps.is_empty() {
        let bg = ParsedScenario {
            title: "(Background)".to_string(),
            steps: bg_steps,
        };
        scenarios.insert(0, bg);
    }

    Ok((scenarios, expanded_steps, ()))
}

/// Splits a Gherkin table row into its cell values, trimming whitespace.
///
/// Leading and trailing pipe characters are removed before splitting.
fn parse_row(line: &str) -> Vec<String> {
    let s = line.trim().trim_matches('|');
    s.split('|').map(|p| p.trim().to_string()).collect()
}

/// Substitutes `<header>` tokens in `text` with the corresponding values from
/// `row`, paired by index into `headers`.
///
/// If `row` is shorter than `headers`, the excess headers are left unexpanded.
fn expand_step(text: &str, headers: &[String], row: &[String]) -> String {
    let mut out = text.to_string();
    for (i, h) in headers.iter().enumerate() {
        if i >= row.len() {
            break;
        }
        out = out.replace(&format!("<{h}>"), &row[i]);
    }
    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn write_feature(content: &str) -> (TempDir, std::path::PathBuf) {
        let tmp = TempDir::new().unwrap();
        let p = tmp.path().join("x.feature");
        fs::write(&p, content).unwrap();
        (tmp, p)
    }

    #[test]
    fn parses_simple_scenario_with_three_steps() {
        let (_tmp, p) = write_feature(
            "Feature: foo\n\nScenario: bar\n  Given a precondition\n  When an action\n  Then an outcome\n",
        );
        let scenarios = parse_feature_file(&p).unwrap();
        assert_eq!(scenarios.len(), 1);
        assert_eq!(scenarios[0].title, "bar");
        assert_eq!(scenarios[0].steps.len(), 3);
        assert_eq!(scenarios[0].steps[0].keyword, "Given");
        assert_eq!(scenarios[0].steps[0].text, "a precondition");
        assert_eq!(scenarios[0].steps[2].keyword, "Then");
    }

    #[test]
    fn background_steps_yield_synthetic_first_scenario() {
        let (_tmp, p) = write_feature(
            "Feature: foo\n\nBackground:\n  Given baseline\n\nScenario: bar\n  Then result\n",
        );
        let scenarios = parse_feature_file(&p).unwrap();
        assert_eq!(scenarios.len(), 2);
        assert_eq!(scenarios[0].title, "(Background)");
        assert_eq!(scenarios[0].steps[0].text, "baseline");
        assert_eq!(scenarios[1].title, "bar");
    }

    #[test]
    fn outline_steps_get_variants_per_examples_row() {
        let (_tmp, p) = write_feature(
            "Feature: foo\n\nScenario Outline: bar\n  Given <state>\n  Then <result>\n\nExamples:\n  | state | result |\n  | A     | X      |\n  | B     | Y      |\n",
        );
        let scenarios = parse_feature_file(&p).unwrap();
        assert_eq!(scenarios.len(), 1);
        assert_eq!(scenarios[0].steps[0].text, "<state>");
        assert_eq!(scenarios[0].steps[0].variants, vec!["A", "B"]);
        assert_eq!(scenarios[0].steps[1].variants, vec!["X", "Y"]);
    }

    #[test]
    fn expanded_outline_step_texts_returns_all_variants() {
        let (_tmp, p) = write_feature(
            "Scenario Outline: x\n  Given <s>\n\nExamples:\n  | s |\n  | A |\n  | B |\n",
        );
        let exp = expanded_outline_step_texts(&p).unwrap();
        assert_eq!(exp, vec!["A", "B"]);
    }

    #[test]
    fn missing_file_returns_error() {
        let err = parse_feature_file(Path::new("/nonexistent/foo.feature")).unwrap_err();
        assert!(!err.to_string().is_empty());
    }
}
