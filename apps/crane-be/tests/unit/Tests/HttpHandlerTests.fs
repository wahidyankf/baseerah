module CraneBe.Tests.Unit.Tests.HttpHandlerTests

open System.Net.Http
open Xunit
open CraneBe.Core.Ports
open CraneBe.Adapters.In.HttpHandlers
open CraneBe.Tests.Unit.Steps.BddState

/// A media adapter that always returns an error for testing the 500 path.
type FailingMediaAdapter() =
    interface IMediaPort with
        member _.Convert(_bytes) = Error "adapter failure"

[<Fact>]
let ``pdf-to-md handler returns 500 when convert fails`` () =
    let port = FailingMediaAdapter()
    let client = buildClient (webApp port)
    // Use valid PDF magic bytes so we pass the PDF validation
    let pdfBytes = [| 0x25uy; 0x50uy; 0x44uy; 0x46uy; 0x2Duy; 0x31uy; 0x2Euy; 0x34uy |]
    use content = new ByteArrayContent(pdfBytes)
    let resp = client.PostAsync("/media/pdf-to-md", content).Result
    Assert.Equal(500, int resp.StatusCode)
