module CraneCli.Program

open CraneCore.Adapters.Out.PdfAdapter
open CraneCli.Adapters.In.CliAdapter

[<EntryPoint>]
let main argv =
    let pdfAdapter = RealPdfAdapter() :> CraneCore.Ports.IPdfPort
    run pdfAdapter argv
