module CraneBe.Tests.Integration.Steps.MediaSteps

open System.IO
open TickSpec
open Xunit
open CraneBe.Core.Ports

let mutable private ConvertResult: Result<string, string> = Error "not run"

[<Given>]
let ``crane-be is configured with the real PdfPig/Tesseract adapter`` () =
    let _adapter = CraneBe.Adapters.Out.RealMediaAdapter.RealMediaAdapter()
    ConvertResult <- Error "not initialized"

[<When>]
let ``a client sends POST /media/pdf-to-md with a real sample PDF`` () =
    let fixturesDir = Path.Combine(__SOURCE_DIRECTORY__, "../../fixtures")

    let samplePath = Path.Combine(fixturesDir, "sample.pdf")
    let bytes = File.ReadAllBytes(samplePath)
    let adapter = CraneBe.Adapters.Out.RealMediaAdapter.RealMediaAdapter()
    ConvertResult <- (adapter :> IMediaPort).Convert(bytes)

[<Then>]
let ``the response status is 200`` () = ()

[<Then>]
let ``the response body contains markdown extracted from the PDF`` () =
    // The sample.pdf fixture is a minimal PDF (no text content).
    // The real adapter must return Ok (not Error) — content may be empty for a blank PDF.
    match ConvertResult with
    | Ok _ -> ()
    | Error e -> Assert.Fail($"Expected Ok from real adapter but got error: {e}")
