# System Context — fsharp-crane-core

C4 Level 1 system context for `fsharp-crane-core`.

## Actors and consumers

- **`crane-cli`** — the F# CLI (`apps/crane-cli`) that references `fsharp-crane-core.fsproj` and
  drives the domain via its `PdfAdapter`/`OcrAdapter` port implementations.
- **`pdf-to-md-{maker,checker,fixer}` agents** — invoke `crane-cli` to convert PDFs to Markdown and
  verify verbatim fidelity (heading hierarchy, tables, figures, Mermaid diagrams).

`fsharp-crane-core` has no runtime dependency on any backend service; PDF text extraction and OCR
are performed through the `IPdfPort`/`IOcrPort` abstractions, implemented by the adapters in
`src/Adapters/Out/` (backed by PdfPig and TesseractOCR).

See [context.md](./context.md) for the C4 context diagram placeholder.
