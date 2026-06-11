module CraneCore.Convert

open System
open CraneCore.Ports

/// Walk-skeleton PDF-to-Markdown conversion.
/// Detects whether the PDF is text-based (>10 words from SampleText) or image-based,
/// then returns text content accordingly.
let convertPdfToMarkdown (pdfPort: IPdfPort) (ocrPort: IOcrPort) (path: string) : Result<string, string> =
    match pdfPort.SampleText(path, 3) with
    | Error msg -> Error(sprintf "Failed to sample PDF: %s" msg)
    | Ok sample ->
        let wordCount =
            sample.Split([| ' '; '\n'; '\t' |], StringSplitOptions.RemoveEmptyEntries).Length

        if wordCount > 10 then
            pdfPort.ExtractPages(path, 1, 999)
        else
            ocrPort.ExtractText(path, 1)
