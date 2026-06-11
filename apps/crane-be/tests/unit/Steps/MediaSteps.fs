module CraneBe.Tests.Unit.Steps.MediaSteps

open System.Net.Http
open TickSpec
open Xunit
open CraneBe.Adapters.In.HttpHandlers
open CraneBe.Adapters.Out.FakeMediaAdapter
open CraneBe.Tests.Unit.Steps.BddState

[<Given>]
let ``crane-be is configured with the fake media adapter`` () =
    let port = FakeMediaAdapter()
    Client <- Some(buildClient (webApp port))

[<When>]
let ``a client sends POST /media/pdf-to-md with sample PDF bytes`` () =
    let client = Client.Value
    // Minimal PDF magic bytes: %PDF-1.4
    let pdfBytes = [| 0x25uy; 0x50uy; 0x44uy; 0x46uy; 0x2Duy; 0x31uy; 0x2Euy; 0x34uy |]
    use content = new ByteArrayContent(pdfBytes)
    let resp = client.PostAsync("/media/pdf-to-md", content).Result
    LastStatus <- int resp.StatusCode
    LastBody <- resp.Content.ReadAsStringAsync().Result

[<Then>]
let ``the response body contains the canned markdown output`` () =
    Assert.Contains("Fake Markdown", LastBody)

[<When>]
let ``a client sends POST /media/pdf-to-md with an empty body`` () =
    let client = Client.Value
    use content = new ByteArrayContent([||])
    let resp = client.PostAsync("/media/pdf-to-md", content).Result
    LastStatus <- int resp.StatusCode
    LastBody <- resp.Content.ReadAsStringAsync().Result

[<Then>]
let ``the response status is 400`` () = Assert.Equal(400, LastStatus)

[<Then>]
let ``the response body indicates the PDF payload was missing`` () =
    Assert.Contains("PDF payload missing", LastBody)

[<When>]
let ``a client sends POST /media/pdf-to-md with bytes that are not a PDF`` () =
    let client = Client.Value
    // Non-PDF bytes — plain text
    let nonPdfBytes = System.Text.Encoding.UTF8.GetBytes("this is not a pdf")
    use content = new ByteArrayContent(nonPdfBytes)
    let resp = client.PostAsync("/media/pdf-to-md", content).Result
    LastStatus <- int resp.StatusCode
    LastBody <- resp.Content.ReadAsStringAsync().Result

[<Then>]
let ``the response status is 422`` () = Assert.Equal(422, LastStatus)

[<Then>]
let ``the response body indicates the payload could not be parsed as a PDF`` () =
    Assert.Contains("Payload could not be parsed as a PDF", LastBody)
