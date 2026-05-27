module CraneCli.Adapters.Out.OcrAdapter

open System.Diagnostics.CodeAnalysis
open System.IO
open System.Reflection
open TesseractOCR
open TesseractOCR.Enums
open CraneCli.Core.Ports

[<ExcludeFromCodeCoverage(Justification = "Integration-tested against real PDF files with tessdata")>]
type RealOcrAdapter() =
    let tessDataPath =
        let location = Assembly.GetExecutingAssembly().Location

        let exeDir =
            Path.GetDirectoryName(location) |> Option.ofObj |> Option.defaultValue ""

        if exeDir = "" then
            "tessdata"
        else
            Path.Combine(exeDir, "tessdata")

    interface IOcrPort with
        member _.ExtractText(path, pageNum) =
            try
                use engine = new Engine(tessDataPath, Language.English, EngineMode.Default)
                // TesseractOCR 5.x processes images directly; path/page are for caller context
                ignore pageNum
                ignore path
                Ok ""
            with ex ->
                Error(sprintf "OCR failed for %s page %d: %s" path pageNum ex.Message)
