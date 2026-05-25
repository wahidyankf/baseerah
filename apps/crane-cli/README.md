# crane-cli

Content Retrieval And Normalization Engine — a Rust CLI that provides reliable, deterministic
operations for the PDF-to-Markdown conversion pipeline.

## System Dependencies

Required before building or running OCR:

| Platform      | Command                                                                              |
| ------------- | ------------------------------------------------------------------------------------ |
| macOS         | `brew install tesseract poppler`                                                     |
| Ubuntu/Debian | `sudo apt-get install tesseract-ocr libleptonica-dev libtesseract-dev poppler-utils` |

Set `TESSDATA_PREFIX` to your tesseract data directory (e.g., `/opt/homebrew/share/tessdata`
on macOS Homebrew).

## Usage

```bash
cargo run --manifest-path apps/crane-cli/Cargo.toml -- --help
```

Or after `cargo build --release`:

```bash
apps/crane-cli/target/release/crane --help
```

## Subcommands

- `crane pdf` — PDF operations (info, type, extract)
- `crane text` — Text completeness checking
- `crane heading` — Heading depth inference and checking
- `crane nesting` — List nesting analysis
- `crane table` — Table detection and checking
- `crane figure` — Figure coverage checking
- `crane mermaid` — Mermaid diagram validation
- `crane ocr` — OCR quality assessment and extraction (requires tesseract + poppler)
- `crane report` — Audit report management
- `crane skiplist` — Skip list management
- `crane check-all` — Aggregate all checks in one pass

## Development

```bash
# Build
cargo build --release --manifest-path apps/crane-cli/Cargo.toml

# Unit tests (152 tests, ≥95% coverage)
cargo test --manifest-path apps/crane-cli/Cargo.toml --test unit

# Integration tests (37 Gherkin scenarios)
cargo test --manifest-path apps/crane-cli/Cargo.toml --test integration

# Lint and format check
cargo fmt --manifest-path apps/crane-cli/Cargo.toml -- --check
cargo clippy --manifest-path apps/crane-cli/Cargo.toml --all-targets -- -D warnings
```

## Nx Targets

```bash
npx nx run crane-cli:build           # Release build
npx nx run crane-cli:lint            # fmt check + clippy
npx nx run crane-cli:typecheck       # cargo check
npx nx run crane-cli:test:quick      # Unit tests + ≥95% coverage
npx nx run crane-cli:test:integration # Cucumber-rs integration tests
npx nx run crane-cli:spec-coverage   # Gherkin spec coverage validation
```
