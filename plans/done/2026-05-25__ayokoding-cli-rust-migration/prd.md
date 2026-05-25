# Product Requirements Document — ayokoding-cli Rust Migration

## Product Overview

`ayokoding-cli` is a single-purpose CLI tool that validates internal links inside
`apps/ayokoding-web/content`. Content authors and CI pipelines run `ayokoding-cli links check` to
detect broken Hugo-path links (`/en/...`, `/id/...`) before they reach production. This plan rewrites
the binary in Rust with identical external behavior — same flags, same subcommands, same exit codes,
same output formats.

## Personas

- **Content author** — runs `ayokoding-cli links check` locally before committing content changes to
  confirm no internal links are broken.
- **CI pipeline** — runs `nx run ayokoding-cli:test:quick` on every push; must exit 0 for the push
  to proceed.
- **swe-rust-dev agent** — implements the Rust source files following the delivery checklist.

## User Stories

### US-01: Run link check with default content directory

As a content author,
I want to run `ayokoding-cli links check` with no flags,
So that links in the default `apps/ayokoding-web/content` directory are validated without requiring
me to specify the path.

### US-02: Run link check against a custom content directory

As a content author,
I want to run `ayokoding-cli links check --content <path>`,
So that I can validate links in a non-default content directory during local development.

### US-03: Get structured JSON output

As a CI pipeline script,
I want to run `ayokoding-cli links check --output json`,
So that I can parse the link-check results programmatically.

### US-04: Detect broken internal links

As a content author,
I want the command to exit with a non-zero status when broken links are found,
So that a pre-push hook or CI check can block the commit until broken links are fixed.

### US-05: Skip external links automatically

As a content author,
I want external HTTP/HTTPS links to be silently skipped,
So that the check does not fail on URLs that require network access.

### US-06: Quiet mode for scripting

As a CI pipeline script,
I want to suppress all output except errors via `--quiet`,
So that CI logs are not cluttered with informational messages.

## Acceptance Criteria

### AC-01: Default content directory

```gherkin
Scenario: A content directory with all valid Hugo-path links passes validation
  Given ayokoding-web content where all internal links resolve correctly
  When the developer runs links check
  Then the command exits successfully
```

### AC-02: Broken link detection

```gherkin
Scenario: A broken internal link is detected and reported
  Given ayokoding-web content with a link pointing to a non-existent page
  When the developer runs links check
  Then the command exits with a failure code
```

### AC-03: External URL skipping

```gherkin
Scenario: External URLs are not validated
  Given ayokoding-web content with only external HTTPS links
  When the developer runs links check
  Then the command exits successfully
```

### AC-04: JSON output

```gherkin
Scenario: JSON output produces structured results
  Given ayokoding-web content where all internal links resolve correctly
  When the developer runs links check with JSON output
  Then the command exits successfully
  And the output is valid JSON
```

### AC-05: CLI surface parity (smoke test)

```gherkin
Scenario: The Rust binary exposes the same root flags
  Given the ayokoding-cli binary is built
  When the developer runs ayokoding-cli --help
  Then the output contains "--verbose", "--quiet", "--output", "--no-color"
  And the output contains the "links" subcommand
```

### AC-06: links check subcommand help

```gherkin
Scenario: The links check subcommand exposes the --content flag
  Given the ayokoding-cli binary is built
  When the developer runs ayokoding-cli links check --help
  Then the output contains "--content"
  And the default value shown is "apps/ayokoding-web/content"
```

### AC-07: Go source archived

```gherkin
Scenario: Go source is preserved in the archive
  Given the migration is complete
  When the developer inspects the archived directory
  Then "archived/ayokoding-cli/" exists
  And it contains "main.go", "go.mod", and the "cmd/" directory
```

### AC-08: Go shared libraries removed

```gherkin
Scenario: Go shared libraries are deleted after migration
  Given both ose-cli and ayokoding-cli have been migrated to Rust
  And no other app or lib imports golang-link-commons or golang-commons
  When the developer runs the cleanup step
  Then "libs/golang-link-commons/" no longer exists
  And "libs/golang-commons/" no longer exists
```

### AC-09: Coverage gate

```gherkin
Scenario: Coverage gate passes at 90% line coverage
  Given the Rust implementation is complete
  When the developer runs cargo llvm-cov --fail-under-lines 90
  Then the command exits successfully
  And the line coverage report shows at least 90% coverage
```

## Product Scope

**In scope:**

- `ayokoding-cli links check` subcommand with `--content` flag
- Root flags: `--verbose` / `-v`, `--quiet` / `-q`, `--output` / `-o` (text/json/markdown),
  `--no-color`
- Same exit-code semantics as the Go version (0 = no broken links, 1 = broken links found or error)
- Nx targets in `apps/ayokoding-cli/project.json`: build, install, fmt, fmt:check, lint, deny:check,
  check:msrv, run, typecheck, test:unit, test:quick, test:integration, spec-coverage
- Archival of Go source to `archived/ayokoding-cli/`
- Deletion of `libs/golang-link-commons/` and `libs/golang-commons/` (gated on no other consumers)

**Out of scope:**

- New subcommands (e.g., `links fix`)
- External link validation
- Colorized output (preserved as a flag but color rendering is a nice-to-have, not tested)
- Windows builds (same scope as `rhino-cli`)

## Product-Level Risks

| Risk                                                   | Impact                                                    | Mitigation                                                                                             |
| ------------------------------------------------------ | --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| The Rust port silently changes link-check logic        | Users get false-positive passes or missed errors          | All four existing Gherkin scenarios (AC-01 through AC-04) are ported as unit tests and must pass       |
| The `--content` flag default is wrong in the Rust port | CI always passes even with broken links (wrong directory) | AC-06 explicitly tests the default value; smoke test in `tests/cli_smoke.rs` verifies the flag default |
| Cleanup deletes a lib still needed                     | Build breakage for another app                            | Phase 3 `grep -r` gate is mandatory before `rm -rf`; any match blocks deletion                         |
