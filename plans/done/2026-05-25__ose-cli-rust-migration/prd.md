# Product Requirements Document — ose-cli Rust Migration

## Product Overview

Deliver a Rust reimplementation of the `ose-cli` command-line tool that provides identical user-facing behavior to the Go original. The tool validates internal markdown links in `apps/ose-web/content/` and reports broken links in text, JSON, or Markdown format. The migration also delivers a new shared Rust library (`libs/rust-commons/`) that encapsulates the link-checking logic.

For the business rationale behind this migration, see [brd.md](./brd.md).

## Personas

- **Maintainer as site editor** — runs `nx run ose-cli:test:quick` as part of the pre-push quality gate before publishing ose-web content changes; relies on the tool to catch broken internal links before they reach production.
- **CI pipeline** — executes `nx run ose-cli:test:quick` and `nx run ose-cli:spec-coverage` on every push to `main`; expects deterministic, non-flaky results.
- **Future Rust CLI authors** — will import `libs/rust-commons/` for link-checking logic rather than re-implementing it.

## User Stories

### US-1: Link Check (default text output)

As a site editor, I want to run `ose-cli links check` and receive a human-readable report of broken internal links in `apps/ose-web/content/`, so that I can fix them before pushing.

### US-2: Link Check (JSON output)

As a CI pipeline, I want to run `ose-cli links check -o json` and receive a machine-readable JSON report of link check results, so that the output can be parsed by downstream tools or logged.

### US-3: Link Check (Markdown output)

As a site editor, I want to run `ose-cli links check -o markdown` and receive a Markdown-formatted report, so that I can paste it into a GitHub issue or documentation page.

### US-4: Custom content directory

As a site editor, I want to pass `--content <path>` to `ose-cli links check`, so that I can check a non-default content directory without editing any configuration.

### US-5: Quiet mode

As a CI pipeline, I want to pass `--quiet` to `ose-cli links check` to suppress informational output, so that only errors appear in CI logs.

### US-6: Verbose mode

As a site editor, I want to pass `--verbose` to `ose-cli links check` to see a completion timestamp, so that I can judge how long the check took.

### US-7: Exit code signals broken links

As a CI pipeline, I want `ose-cli links check` to exit with a non-zero status code when broken links are found, so that the CI job fails automatically and does not require parsing output.

### US-8: Shared library usable by sibling CLI

As a future Rust CLI author (specifically, the `ayokoding-cli` Rust port), I want to import `rust-commons::links::check_links()` from `libs/rust-commons/`, so that I do not duplicate the link-walking logic.

## Acceptance Criteria

### AC-1: Help flag works

```gherkin
Scenario: Help flag exits successfully
  Given the ose-cli binary is built
  When the user runs ose-cli --help
  Then the process exits with code 0
  And stdout contains "ose-cli"
```

### AC-2: Unknown subcommand exits with failure

```gherkin
Scenario: Unknown subcommand exits with failure
  Given the ose-cli binary is built
  When the user runs ose-cli not-a-real-command
  Then the process exits with a non-zero code
```

### AC-3: Invalid output format exits with failure

```gherkin
Scenario: Invalid output format is rejected
  Given the ose-cli binary is built
  When the user runs ose-cli --output bad-format links check
  Then the process exits with a non-zero code
```

### AC-4: Links check passes on clean content directory

```gherkin
Scenario: Links check passes when all links resolve
  Given a temporary directory containing markdown files with valid internal links
  When the user runs ose-cli links check --content <temp-dir>
  Then the process exits with code 0
  And stdout contains "Link Check Complete"
  And stdout contains "Broken:   0"
```

### AC-5: Links check fails and reports broken links

```gherkin
Scenario: Links check reports broken links and exits non-zero
  Given a temporary directory containing a markdown file with a broken internal link
  When the user runs ose-cli links check --content <temp-dir>
  Then the process exits with a non-zero code
  And stdout contains the broken link's source file and target
```

### AC-6: JSON output is parseable

```gherkin
Scenario: JSON output is valid JSON with expected keys
  Given a temporary content directory
  When the user runs ose-cli links check --content <temp-dir> -o json
  Then stdout is valid JSON
  And the JSON object contains keys: status, duration_ms, checked, broken, broken_links
```

### AC-7: Markdown output contains expected headings

```gherkin
Scenario: Markdown output contains report headings
  Given a temporary content directory
  When the user runs ose-cli links check --content <temp-dir> -o markdown
  Then stdout contains "# Link Check Report"
  And stdout contains "## Summary"
```

### AC-8: Quiet mode suppresses output on success

```gherkin
Scenario: Quiet mode suppresses informational output
  Given a temporary content directory with valid links
  When the user runs ose-cli links check --content <temp-dir> --quiet
  Then the process exits with code 0
  And stdout is empty (no text output)
```

### AC-9: Nonexistent content directory exits with failure

```gherkin
Scenario: Nonexistent content directory is rejected
  Given a path that does not exist on disk
  When the user runs ose-cli links check --content <nonexistent-path>
  Then the process exits with a non-zero code
```

### AC-10: Code blocks are skipped (no false positives)

```gherkin
Scenario: Links inside fenced code blocks are not checked
  Given a markdown file containing a fenced code block with link-like syntax [text](target)
  When the user runs ose-cli links check --content <temp-dir>
  Then no link inside the code block is reported as broken
```

### AC-11: rust-commons check_links returns CheckResult

```gherkin
Scenario: rust-commons::links::check_links() returns a result
  Given a temporary content directory
  When check_links() is called with the directory path
  Then it returns Ok(CheckResult) with checked_count >= 0
  And broken_links is empty when all links resolve
```

### AC-12: Default content directory is apps/ose-web/content

```gherkin
Scenario: Default content directory is used when --content is omitted
  Given the user is in the repo root
  When the user runs ose-cli links check (no --content flag)
  Then the tool checks apps/ose-web/content
```

## Product Scope

### In-Scope Features

- `ose-cli links check` command with `--content`, `--output`/`-o`, `--verbose`/`-v`, `--quiet`/`-q`, `--no-color` flags
- Text, JSON, and Markdown output formats (identical structure to Go version)
- Exit code 0 on success, non-zero on broken links found or error
- Code-block skipping in link extraction (no false positives from fenced blocks)
- `libs/rust-commons/` crate with `links::check_links()`, `links::output_links_text()`, `links::output_links_json()`, `links::output_links_markdown()` public functions
- 90% line coverage for `libs/rust-commons/`

### Out-of-Scope Features

- External link validation (HTTP/HTTPS links) — intentionally excluded as in Go version
- Anchor (`#fragment`) link validation — excluded as in Go version
- Parallel link-checking — deferred; serial walk is correct and sufficient
- New subcommands beyond `links check`
- Color output implementation beyond the `--no-color` flag accepting the argument

## Product Risks

| Risk                                                   | Likelihood | Mitigation                                                                                                                                   |
| ------------------------------------------------------ | ---------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| Hugo path resolution logic differs from Go port        | Medium     | Port the exact algorithm (`targetExists` checks `.md` form and `/_index.md` form); add integration test against real `apps/ose-web/content/` |
| Fenced code block detection regression                 | Low        | Unit test with multi-line fenced blocks, nested backticks, and tilde fences                                                                  |
| JSON output shape mismatch breaks downstream consumers | Low        | Pin the JSON field names and types in acceptance criteria; verify against Go output shape                                                    |
