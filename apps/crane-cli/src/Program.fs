module CraneCli.Program

open CraneCli.Adapters.Out.PdfAdapter
open CraneCli.Adapters.In.CliAdapter

[<EntryPoint>]
let main argv =
    let pdfAdapter = RealPdfAdapter() :> CraneCli.Core.Ports.IPdfPort
    run pdfAdapter argv
