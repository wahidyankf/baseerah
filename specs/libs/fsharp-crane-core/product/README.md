# Product — fsharp-crane-core

C4 Level 1 product framing for `fsharp-crane-core`. See
[Specs Directory Structure Convention](../../../../repo-governance/conventions/structure/specs-directory-structure.md)
for the canonical layout.

## Overview

`fsharp-crane-core` (F# module namespace `CraneCore`) is the shared domain/logic core behind
`crane-cli`'s PDF-to-Markdown conversion fidelity checks. It decides whether a PDF should be
converted via direct text extraction or OCR, and hosts the checkers (heading, table, figure,
Mermaid, nesting, text) that verify a Markdown conversion is a verbatim, complete representation
of its source PDF.

See [overview.md](./overview.md) for the full product overview.
