# Phase 2 — `libs/fsharp-crane-core` Keep/Delete Audit

17 `.fs` files under `libs/fsharp-crane-core/src/`, every one under the `CraneCore.*` module
namespace.

| File                               | Module                               | Purpose                                                       |
| ---------------------------------- | ------------------------------------ | ------------------------------------------------------------- |
| `Placeholder.fs`                   | `CraneCore.Placeholder`              | Empty scaffold placeholder, no logic.                         |
| `Convert.fs`                       | `CraneCore.Convert`                  | PDF→Markdown conversion orchestration (text vs. OCR branch).  |
| `Core/Ports.fs`                    | `CraneCore.Ports`                    | `IPdfPort`/`IOcrPort` hexagonal-architecture port interfaces. |
| `Core/Domain/Finding.fs`           | `CraneCore.Domain.Finding`           | Validation-finding domain type for PDF-to-MD fidelity checks. |
| `Core/Domain/PdfMetadata.fs`       | `CraneCore.Domain.PdfMetadata`       | PDF metadata domain type (page count, etc.).                  |
| `Core/Domain/Report.fs`            | `CraneCore.Domain.Report`            | Fidelity-check report domain type.                            |
| `Core/Logic/FigureChecker.fs`      | `CraneCore.Logic.FigureChecker`      | Verifies figure/image coverage in converted Markdown.         |
| `Core/Logic/HeadingChecker.fs`     | `CraneCore.Logic.HeadingChecker`     | Verifies heading-hierarchy fidelity vs. source PDF.           |
| `Core/Logic/MermaidValidator.fs`   | `CraneCore.Logic.MermaidValidator`   | Validates Mermaid diagrams converted from PDF figures.        |
| `Core/Logic/NestingChecker.fs`     | `CraneCore.Logic.NestingChecker`     | Verifies list/section nesting fidelity.                       |
| `Core/Logic/OcrAssessor.fs`        | `CraneCore.Logic.OcrAssessor`        | Assesses OCR text-quality confidence.                         |
| `Core/Logic/PdfExtractionCache.fs` | `CraneCore.Logic.PdfExtractionCache` | Caches extracted PDF page text between chunked runs.          |
| `Core/Logic/ReportManager.fs`      | `CraneCore.Logic.ReportManager`      | Assembles/writes the fidelity-check report.                   |
| `Core/Logic/SkiplistManager.fs`    | `CraneCore.Logic.SkiplistManager`    | Manages a skip-list of already-processed PDF chunks.          |
| `Core/Logic/TableChecker.fs`       | `CraneCore.Logic.TableChecker`       | Verifies table-data fidelity vs. source PDF.                  |
| `Core/Logic/TextChecker.fs`        | `CraneCore.Logic.TextChecker`        | Verifies verbatim text fidelity vs. source PDF.               |
| `Adapters/Out/OcrAdapter.fs`       | `CraneCore.Adapters.Out.OcrAdapter`  | `tesseract`-backed `IOcrPort` implementation.                 |
| `Adapters/Out/PdfAdapter.fs`       | `CraneCore.Adapters.Out.PdfAdapter`  | `pdftotext`-backed `IPdfPort` implementation.                 |

Every module lives under `CraneCore.*` and every responsibility (PDF sampling/extraction, OCR
assessment, fidelity checkers for figures/headings/mermaid/nesting/tables/text, report assembly) is
specific to the `crane-cli` PDF-to-Markdown conversion pipeline (mirrored by the
`pdf-to-md-{maker,checker,fixer}` agent family). None of the 17 files contains a generic,
domain-independent F# utility (string helpers, generic Result combinators, etc.) that any other app
or lib in this repo could plausibly import.

## VERDICT: DELETE

## External-consumer check

`rg -n 'fsharp-crane-core|CraneCore' --glob '!libs/fsharp-crane-core/**' --glob '!plans/**'` — every
hit that represents actual **code consumption** (`.fsproj` `ProjectReference`, `open CraneCore.*`,
Nx `implicitDependencies`) is confined to `apps/crane-cli/` (`crane-cli.fsproj`, `project.json`,
`Program.fs`, `CliAdapter.fs`, and its `tests/unit/**` step/test files) — confirming DELETE is safe
from a build/dependency standpoint.

The remaining hits are prose/documentation mentions, not code consumption, and are not covered by
this step's original acceptance text (`apps/crane-cli/`, `specs/apps/crane/`, `repo-config.yml`) —
that text undercounted legitimate doc homes. They fall into three buckets, each already handled
elsewhere in this plan rather than left dangling:

- `specs/libs/fsharp-crane-core/**` — deleted by this same delivery step
  (`git rm -r libs/fsharp-crane-core specs/libs/fsharp-crane-core`).
- `apps/ose-www/content/updates/2026-06-15-...md` — deleted by this phase's app-directory removal
  (`ose-www` is one of the 22 retired apps).
- `AGENTS.md`, `libs/README.md`, `docs/reference/monorepo-structure.md`,
  `repo-governance/conventions/structure/licensing.md`, `specs/README.md`,
  `apps/rhino-cli/src/commands/specs_validate_counts.rs` (comment only), and
  `generated-socials/linkedin/**` (historical archive, intentionally left as-is) — prose references
  swept up by Phase 3's repo-wide `rg` sweeps for retired-app/lib names (delivery.md Phase 3 has
  multiple such sweeps, including one against `crane-cli` specifically).
