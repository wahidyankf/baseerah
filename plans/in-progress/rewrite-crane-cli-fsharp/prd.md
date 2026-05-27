# Product Requirements Document

## Product Overview

crane-cli (Content Retrieval And Normalization Engine) is a standalone CLI binary that provides
deterministic, reliable operations for the PDF-to-Markdown conversion pipeline. This rewrite
preserves the exact user-facing interface of the Rust implementation while replacing the
implementation language with F# and restructuring the codebase to strict hexagonal
(ports-and-adapters) architecture with explicit port function type aliases.

## Personas

- **Maintainer (pipeline operator hat)** — runs `crane check-all <path>` as part of the
  pdf-to-md pipeline; cares only that subcommands work identically to the Rust version
- **Maintainer (developer hat)** — reads and writes F# source; benefits from pure Core/Logic
  modules that are testable without any I/O setup
- **swe-fsharp-dev agent** [Repo-grounded] — primary agent executor for implementation phases;
  authors `.fs` files and `.fsproj` project files

## User Stories

1. As a pipeline operator, I want `crane pdf info <file>` to output PDF metadata so that I can
   inspect properties of a PDF before conversion.
2. As a pipeline operator, I want `crane text check <text-file> <pdf-file>` to report text
   completeness findings so that I know whether extracted text covers the PDF content.
3. As a pipeline operator, I want `crane heading check <markdown-file>` to report heading
   structure findings so that I know whether heading depth is correct.
4. As a pipeline operator, I want `crane nesting check <markdown-file>` to report list nesting
   findings so that I can identify malformed nested lists.
5. As a pipeline operator, I want `crane table check <markdown-file>` to report table detection
   findings so that I know whether expected tables are present.
6. As a pipeline operator, I want `crane figure check <markdown-file> <pdf-file>` to report
   figure coverage findings so that I can verify figures were extracted.
7. As a pipeline operator, I want `crane mermaid validate <markdown-file>` to report Mermaid
   diagram validity so that I can fix broken diagram syntax.
8. As a pipeline operator, I want `crane ocr assess <pdf-file>` to report OCR quality so that
   I know whether OCR extraction is reliable.
9. As a pipeline operator, I want `crane report show <report-file>` and `crane report save` to
   manage audit reports so that I can track findings over time.
10. As a pipeline operator, I want `crane skiplist add/remove/show` to manage skip lists so that
    I can exclude known-good or unfixable findings from reports.
11. As a pipeline operator, I want `crane check-all <path>` to run all checks in a single pass
    so that I get a comprehensive audit without multiple invocations.
12. As a developer, I want all domain logic in `Core/Logic/` to be pure functions (no I/O) so
    that I can write xUnit unit tests with no file-system setup.
13. As a developer, I want all I/O boundaries expressed as F# function type aliases in
    `Core/Ports.fs` so that I can inject test doubles without an IoC container.

## Acceptance Criteria

### Phase 1: Plan prerequisite and scaffold

```gherkin
Scenario: remove-inactive plan amended before execution
  Given plans/in-progress/remove-inactive-tech-stack-remnants/delivery.md exists
  When Phase 1 (Dotnet cleanup) items are inspected
  Then Phase 1 must be replaced with a note excluding dotnet/F#/C# from scope
  And the amended plan must be committed before any crane-cli F# code is written
```

```gherkin
Scenario: Rust source archived
  Given apps/crane-cli/ contains Cargo.toml and src/ (Rust)
  When the archival git mv is executed
  Then archived/crane-cli-rust/Cargo.toml exists
  And archived/crane-cli-rust/src/ exists
  And apps/crane-cli/Cargo.toml does not exist
  And apps/crane-cli/src/ does not exist
```

```gherkin
Scenario: F# project scaffold compiles empty
  Given apps/crane-cli/crane-cli.fsproj is created with net10.0 target
  When dotnet build apps/crane-cli/crane-cli.fsproj is run
  Then the build exits 0
  And a crane binary is produced under apps/crane-cli/bin/Debug/net10.0/crane
```

### Phase 2: Core/Domain types

```gherkin
Scenario: Finding domain type is defined
  Given Core/Domain/Finding.fs defines the Finding discriminated union
  When a unit test constructs a Finding.Error with message "test"
  Then the value round-trips through FSharp.SystemTextJson serialization
  And the JSON output contains "level": "error"
```

```gherkin
Scenario: Ports module defines all I/O type aliases
  Given Core/Ports.fs defines ReadPdf, RunOcr, ReadFile, WriteFile, and AppendReport port types
  When an adapter function is created that matches the ReadPdf type alias
  Then it compiles without explicit type annotation (F# type inference suffices)
```

### Phase 3: Core/Logic modules (pure functions)

```gherkin
Scenario: TextChecker returns findings for incomplete text
  Given a text string with 40% coverage of the reference text
  When TextChecker.check is called with the text and reference
  Then the result contains at least one Finding with severity Warning or Error
  And the result contains no IO side effects
```

```gherkin
Scenario: HeadingChecker detects excessive depth
  Given a markdown string with H4 headings
  When HeadingChecker.check is called with maxDepth 3
  Then the result contains a Finding indicating excessive depth
```

```gherkin
Scenario: MermaidValidator detects unclosed code fence
  Given a markdown string with an unclosed mermaid code fence
  When MermaidValidator.validate is called
  Then the result contains a Finding indicating invalid Mermaid syntax
```

```gherkin
Scenario: SkiplistManager persists and retrieves skip entries
  Given SkiplistManager.load is called with a ReadFile port returning valid JSON
  When SkiplistManager.add is called with a new entry
  And SkiplistManager.save is called with a WriteFile port
  Then the WriteFile port receives the updated JSON including the new entry
```

### Phase 4: Adapters/Out

```gherkin
Scenario: PdfAdapter reads a valid PDF file
  Given a real PDF file exists on the filesystem
  When PdfAdapter.readPdf is called with the file path (satisfying ReadPdf port)
  Then the result is Ok with non-empty PdfContent
  And no exception is thrown
```

```gherkin
Scenario: PdfAdapter returns Error for a missing file
  Given a file path that does not exist on the filesystem
  When PdfAdapter.readPdf is called with that path
  Then the result is Error with a descriptive PdfError message
```

```gherkin
Scenario: OcrAdapter returns text for a PDF page
  Given a real PDF file with selectable text
  When OcrAdapter.runOcr is called with the file path and page index
  Then the result is Ok with non-empty extracted text
```

### Phase 5: Adapters/In and composition root

```gherkin
Scenario: crane --help displays all subcommands
  Given the crane binary is built
  When crane --help is invoked
  Then the output contains: pdf, text, heading, nesting, table, figure, mermaid, ocr, report, skiplist, check-all
```

```gherkin
Scenario: crane pdf info outputs metadata JSON
  Given a valid PDF file at /tmp/test.pdf
  When crane pdf info /tmp/test.pdf is invoked
  Then exit code is 0
  And stdout contains valid JSON with a "pageCount" field
```

```gherkin
Scenario: crane check-all returns non-zero exit code for findings
  Given a markdown file with known heading depth violations
  When crane check-all is invoked on the directory containing that file
  Then exit code is 1
  And stderr or stdout contains at least one Finding message
```

### Phase 6: Integration tests (TickSpec)

```gherkin
Scenario: All feature files in specs/apps/crane bind to step definitions
  Given specs/apps/crane/behavior/cli/gherkin/**/*.feature exist
  When dotnet test crane-cli-unit-tests.fsproj is run
  Then all scenarios execute without "No step definitions" errors
  And the test run exits 0
```

```gherkin
Scenario: Integration test suite passes for PDF and OCR scenarios
  Given the crane binary is built and tesseract is installed
  When dotnet test crane-cli-integration-tests.fsproj is run
  Then all scenarios in pdf-commands.feature pass
  And all scenarios in ocr-quality.feature pass
  And the test run exits 0
```

### Phase 7: Nx targets and CI

```gherkin
Scenario: All Nx targets execute without error
  Given apps/crane-cli/project.json defines build, typecheck, lint, test:quick, test:integration, spec-coverage
  When each target is run via npx nx run crane-cli:<target>
  Then each exits 0
```

```gherkin
Scenario: crane-cli-integration CI workflow passes
  Given .github/workflows/crane-cli-integration.yml uses setup-dotnet and dotnet test
  When a push to main triggers the workflow
  Then the integration job exits 0
```

## Product Scope

**In-scope features:**

- All 11 subcommands: pdf, text, heading, nesting, table, figure, mermaid, ocr, report,
  skiplist, check-all (same interface as Rust version)
- Strict hexagonal directory layout: `src/Core/`, `src/Adapters/In/`, `src/Adapters/Out/`,
  `src/Core/Ports.fs`
- xUnit 2.9.2 unit tests for all pure Core/Logic modules
- TickSpec 2.0.5 BDD step definitions consuming existing Gherkin in `specs/apps/crane/`
- Nx targets: build, typecheck, lint, test:quick, test:integration, spec-coverage, fmt,
  fmt:check, dev, run
- CI: updated `crane-cli-integration.yml` using `dotnet test`
- Bundled `tessdata/eng.traineddata` (copy from archived F# source)

**Out-of-scope features:**

- New subcommands or flags not present in the Rust version
- GUI or interactive mode
- Windows native binary (CI targets Ubuntu; macOS dev use via `dotnet run`)
- NuGet package publication
- Performance benchmarking relative to the Rust binary

## Product Risks

| Risk                                                             | Likelihood | Impact | Note                                                                                                                                                                                                |
| ---------------------------------------------------------------- | ---------- | ------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| TickSpec step binding regression (unmatched steps fail silently) | Low        | High   | xunit runner config sets `maxParallelThreads: 1`; TickSpec throws on unbound steps                                                                                                                  |
| PdfPig API change breaks existing behavior                       | Low        | Medium | Pinned to 0.1.14 (pre-1.0, no SemVer guarantee) [Web-cited, 2026-05-27, https://www.nuget.org/packages/PdfPig/0.1.14 — "0.1.14 released 2026-03-22, current stable"]; Gherkin tests act as contract |
| tesseract native lib not found on macOS dev environment          | Medium     | Low    | README documents `brew install tesseract`; CI uses apt-get as in current Rust workflow                                                                                                              |
