# Components — fsharp-crane-core

C4 Level 3 components for `fsharp-crane-core`.

| Module                             | Export                                  | Purpose                                                                         |
| ---------------------------------- | --------------------------------------- | ------------------------------------------------------------------------------- |
| `Core/Domain/Finding.fs`           | `Finding`                               | Verbatim-fidelity finding record (category, criticality, confidence, locations) |
| `Core/Domain/PdfMetadata.fs`       | `PdfMetadata`                           | PDF page count, title, author, size                                             |
| `Core/Domain/Report.fs`            | `SkipListEntry`                         | Accepted-exception skiplist entry                                               |
| `Core/Ports.fs`                    | `IPdfPort`, `IOcrPort`                  | Port contracts for PDF/OCR I/O                                                  |
| `Core/Ports.fs`                    | `ReadFile`, `WriteFile`, `AppendReport` | Functional port type aliases                                                    |
| `Core/Logic/HeadingChecker.fs`     | heading-hierarchy checks                | Detects skipped/incorrect heading levels                                        |
| `Core/Logic/TableChecker.fs`       | table-integrity checks                  | Verifies table structure is preserved                                           |
| `Core/Logic/FigureChecker.fs`      | figure-coverage checks                  | Matches "Figure N" references against Mermaid blocks                            |
| `Core/Logic/MermaidValidator.fs`   | Mermaid syntax validation               | Validates generated Mermaid diagram blocks                                      |
| `Core/Logic/NestingChecker.fs`     | nesting-depth checks                    | Verifies list/heading nesting fidelity                                          |
| `Core/Logic/TextChecker.fs`        | verbatim text checks                    | Compares extracted text against the Markdown output                             |
| `Core/Logic/OcrAssessor.fs`        | OCR quality assessment                  | Scores OCR output quality                                                       |
| `Core/Logic/ReportManager.fs`      | report assembly                         | Aggregates `Finding` lists into a report                                        |
| `Core/Logic/SkiplistManager.fs`    | skiplist lookups                        | Filters findings against accepted exceptions                                    |
| `Core/Logic/PdfExtractionCache.fs` | extraction caching                      | Caches PDF text-extraction results                                              |
| `Convert.fs`                       | `convertPdfToMarkdown`                  | Walk-skeleton PDF→Markdown conversion (text vs. OCR route)                      |
| `Adapters/Out/PdfAdapter.fs`       | `IPdfPort` implementation               | PdfPig-backed PDF reader                                                        |
| `Adapters/Out/OcrAdapter.fs`       | `IOcrPort` implementation               | TesseractOCR-backed OCR reader                                                  |

See [../behavior/gherkin/convert/](../behavior/gherkin/convert/) for the behavioral spec.
See [component-fsharp-crane-core.md](./component-fsharp-crane-core.md) for the C4 component
diagram placeholder.
