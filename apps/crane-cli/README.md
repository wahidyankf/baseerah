# crane-cli

Content Retrieval And Normalization Engine — an F# CLI that provides reliable, deterministic
operations for the PDF-to-Markdown conversion pipeline. Built with a hexagonal ports-and-adapters
architecture using Giraffe-style functional patterns.

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
dotnet run --project apps/crane-cli/crane-cli.fsproj -- --help
```

Or after publishing:

```bash
apps/crane-cli/dist/crane --help
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
dotnet publish apps/crane-cli/crane-cli.fsproj -c Release -o apps/crane-cli/dist

# Unit tests (116 tests, ≥95% line coverage)
dotnet test apps/crane-cli/tests/unit/crane-cli-unit-tests.fsproj

# Integration tests (37 Gherkin scenarios via TickSpec)
dotnet test apps/crane-cli/tests/integration/crane-cli-integration-tests.fsproj

# Format check
fantomas --check apps/crane-cli/src

# Format
fantomas apps/crane-cli/src
```

## Nx Targets

```bash
npx nx run crane-cli:build            # Release build (publish)
npx nx run crane-cli:typecheck        # dotnet build (type check)
npx nx run crane-cli:lint             # fantomas check + fsharplint
npx nx run crane-cli:fmt              # fantomas format
npx nx run crane-cli:test:quick       # Unit tests + ≥95% line coverage
npx nx run crane-cli:test:integration # TickSpec integration tests
npx nx run crane-cli:spec-coverage    # Gherkin spec coverage validation
```

## Architecture

Hexagonal ports-and-adapters layout:

```
src/
  Core/
    Domain/       # Finding, PdfMetadata, Report record types
    Ports.fs      # IPdfPort, IOcrPort interface definitions
    Logic/        # Pure functions: TextChecker, HeadingChecker, …
  Adapters/
    In/
      CliAdapter.fs  # Argu CLI argument parsing + command dispatch
    Out/
      PdfAdapter.fs  # PdfPig real adapter + FakePdfAdapter for tests
      OcrAdapter.fs  # TesseractOCR real adapter
  Program.fs        # Composition root — wires adapters into CLI
tests/
  unit/             # xUnit + Coverlet (≥95% line coverage)
  integration/      # TickSpec BDD (37 Gherkin scenarios)
```
