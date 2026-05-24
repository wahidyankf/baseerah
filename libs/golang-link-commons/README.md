# golang-link-commons

Shared Go link-checking utilities for CLI tools in the Open Sharia Enterprise platform.

## Overview

This library provides link-checking packages shared between CLI tools
(`ayokoding-cli` and `ose-cli`). It validates internal markdown links
within content directories.

## Packages

### `links`

Link-checking utilities for content directories.

- `CheckLinks(contentDir string) (*CheckResult, error)` — walks all `.md` files and validates internal links
- `OutputLinksText(...)` — human-readable text report
- `OutputLinksJSON(...)` — JSON report
- `OutputLinksMarkdown(...)` — Markdown report

Behavior:

- Resolves links to both `target.md` and `target/_index.md`
- Skips external links (`http://`, `https://`, `mailto:`, `//`)
- Skips anchor-only links (`#section`)
- Skips links to static assets (files with extensions like `.xml`, `.pdf`)
- Ignores links inside fenced code blocks

## Usage

Import via the Go workspace replace directive:

```go
import "github.com/wahidyankf/ose-public/libs/golang-link-commons/links"
```

## Development

```bash
# Run tests with coverage enforcement (≥90%)
nx run golang-link-commons:test:quick

# Lint
nx run golang-link-commons:lint

# Tidy dependencies
nx run golang-link-commons:install
```
