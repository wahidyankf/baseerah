# fsharp-crane-core — Product Overview

`fsharp-crane-core` provides the `CraneCore` domain: PDF/Markdown domain types (`Finding`,
`PdfMetadata`, `SkipListEntry`), the ports contract (`IPdfPort`, `IOcrPort`, `ReadFile`,
`WriteFile`, `AppendReport`) that decouples logic from PDF/OCR I/O, and the walk-skeleton
`convertPdfToMarkdown` function that samples a PDF's text (`SampleText`), routes to
`ExtractPages` when the sample looks text-based (> 10 words) or to OCR (`ExtractText`) otherwise.
It also hosts the fidelity checkers (heading hierarchy, table integrity, figure coverage, Mermaid
validity, nesting depth, OCR quality assessment) consumed by `crane-cli` and the
`pdf-to-md-{maker,checker,fixer}` agents.

See [README.md](./README.md) for C4 L1 product framing.
