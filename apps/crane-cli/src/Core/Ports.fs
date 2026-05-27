module CraneCli.Core.Ports

open CraneCli.Core.Domain.PdfMetadata
open CraneCli.Core.Domain.Finding

/// Port for reading PDF metadata and text content
type IPdfPort =
    abstract member GetMetadata: path: string -> Result<PdfMetadata, string>
    abstract member SampleText: path: string * pageCount: int -> Result<string, string>
    abstract member ExtractPages: path: string * startPage: int * endPage: int -> Result<string, string>

/// Port for running OCR on image-based PDFs
type IOcrPort =
    abstract member ExtractText: path: string * pageNum: int -> Result<string, string>

/// Type aliases for functional port injection
type ReadFile = string -> Result<string, exn>
type WriteFile = string -> string -> Result<unit, exn>
type AppendReport = string -> Finding list -> Result<unit, exn>
