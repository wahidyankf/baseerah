# Product Requirements Document — crane-cli Rust Migration

## Product Overview

`crane` (Content Retrieval And Normalization Engine) is a CLI invoked by the pdf-to-md-\* agent
family. It validates PDF-to-Markdown conversion fidelity across text completeness, heading depth,
table integrity, figure coverage, Mermaid syntax, OCR quality, and more. The Rust port preserves
every subcommand, every flag, and every JSON output field.

## Personas

- **pdf-to-md-maker agent** — invokes `crane pdf info`, `crane pdf type`, `crane pdf extract`,
  `crane check-all`.
- **pdf-to-md-checker agent** — invokes `crane check-all`, `crane text check`,
  `crane mermaid validate`, `crane report init/finalize`, `crane skiplist add/check/list`.
- **pdf-to-md-fixer agent** — invokes `crane skiplist add`, `crane report finalize`.

## User Stories

1. As the pdf-to-md-maker agent, I can run `crane pdf info <path>` and receive JSON with `pages`,
   `size_bytes`, optional `title`, optional `author`, and `file` fields.
2. As the pdf-to-md-maker agent, I can run `crane pdf type <path>` and receive JSON `{"type":"text"}`
   (exit 0) or `{"type":"image"}` (exit 1).
3. As the pdf-to-md-maker agent, I can run `crane pdf extract <path> -s 1 -n 5 -o out.txt` to
   write extracted text to a file.
4. As the pdf-to-md-checker agent, I can run `crane check-all <pdf> <md>` and receive a JSON array
   of Finding objects (exit 0 if empty, exit 1 if findings exist).
5. As the pdf-to-md-checker agent, I can run `crane ocr extract <pdf>` to extract text from
   image-only PDF pages via tesseract OCR (real implementation, not a stub).
6. As the pdf-to-md-checker agent, I can run `crane ocr quality <md>` to assess OCR error rates
   in `<!-- OCR: ... -->` sections.
7. As the pdf-to-md-fixer agent, I can manage the skip list via `crane skiplist add/check/list`
   with deterministic SHA-256 keying.
8. As any agent, I can run `crane --version` or `crane -V` to receive the version string.

## Acceptance Criteria (Gherkin)

```gherkin
Feature: PDF subcommands parity

  Scenario: crane pdf info returns valid JSON with pages field
    Given a text-based PDF fixture exists at tests/integration/fixtures/sample-text.pdf
    When I run "crane pdf info tests/integration/fixtures/sample-text.pdf"
    Then the exit code is 0
    And the JSON output contains a field "pages" with a positive integer

  Scenario: crane pdf type detects text PDF
    Given a text-based PDF fixture exists
    When I run "crane pdf type" on the fixture
    Then the JSON output contains type "text"
    And the exit code is 0

  Scenario: crane check-all on matching pair returns empty array
    Given a PDF fixture and an MD that matches across all dimensions
    When I run "crane check-all <pdf> <md>"
    Then the JSON output is "[]"
    And the exit code is 0

  Scenario: crane ocr extract produces text via tesseract
    Given an image-only PDF fixture exists
    When I run "crane ocr extract" on the fixture
    Then the exit code is 0
    And the output contains non-empty text

  Scenario: crane skiplist add persists entry with stable key
    Given no existing skip list for "nist-sp-800-53"
    When I run "crane skiplist add nist-sp-800-53 text-completeness 'Page header on p.3'"
    Then the skip list file contains exactly one entry
    And the key is a 16-character lowercase hex string

  Scenario: crane --version returns a version string
    When I run "crane --version"
    Then the exit code is 0
    And the output matches "\d+\.\d+\.\d+"
```

## Product Scope

### In Scope

| Subcommand        | Operations                                                       |
| ----------------- | ---------------------------------------------------------------- |
| `crane pdf`       | `info`, `type`, `extract` (with `-s`, `-n`, `-o`)                |
| `crane text`      | `check` (PDF vs MD), `search` (MD segment lookup)                |
| `crane heading`   | `infer`, `check`                                                 |
| `crane nesting`   | `infer`, `check`                                                 |
| `crane table`     | `detect`, `check`                                                |
| `crane figure`    | `detect`, `check`                                                |
| `crane mermaid`   | `validate`                                                       |
| `crane ocr`       | `extract` (tesseract), `quality` (markdown analysis)             |
| `crane report`    | `init`, `finalize`                                               |
| `crane skiplist`  | `add`, `check`, `list`                                           |
| `crane check-all` | aggregates text/heading/nesting/table/figure/mermaid in one pass |

### Out of Scope

- Changing the JSON output schema.
- Adding new subcommands.
- Changing exit code semantics.
- Porting the F# TickSpec integration test runner (use cucumber-rs instead).

## Product Risks

| Risk                                                                    | Mitigation                                              |
| ----------------------------------------------------------------------- | ------------------------------------------------------- | -------- | ---------------------------------- |
| lopdf text extraction produces different word ordering for complex PDFs | Integration test with real fixture PDF gates acceptance |
| tesseract system library unavailable in dev environment                 | `npm run doctor` step verifies; install docs updated    |
| Skip list SHA-256 key changes between F# and Rust implementations       | Port the exact same algorithm: SHA-256(UTF-8("basename  | category | description")), first 16 hex chars |
