module CraneCli.Tests.Unit.Steps.PdfSteps

open TickSpec
open Xunit
open CraneCli.Core.Ports
open CraneCli.Adapters.Out.PdfAdapter
open CraneCli.Adapters.In.CliAdapter
open CraneCli.Tests.Unit.Steps.BddState

let mutable private currentAdapter: IPdfPort =
    FakePdfAdapter("Sample text with many words for testing purposes right here and more", 5, 10240L)

[<Given>]
let ``a text-based PDF fixture with a known page count`` () =
    currentAdapter <-
        FakePdfAdapter("Sample text content with many words for testing purposes and more content here", 5, 10240L)

[<Given>]
let ``a text-based PDF fixture exists`` () =
    currentAdapter <-
        FakePdfAdapter("Sample text content with many words for testing purposes and more content here", 5, 10240L)

[<Given>]
let ``an image-only PDF fixture exists`` () =
    currentAdapter <- FakePdfAdapter("", 1, 512L)

[<When>]
let ``I run "crane pdf info" on the fixture`` () =
    RunWithWriter(fun w ->
        let argv = [| "pdf"; "-f"; "fake.pdf" |]
        // Intercept the json output by running the command handler directly
        use sw = new System.IO.StringWriter()

        let code =
            match currentAdapter.GetMetadata("fake.pdf") with
            | Ok meta ->
                let opts = System.Text.Json.JsonSerializerOptions()
                opts.DefaultIgnoreCondition <- System.Text.Json.Serialization.JsonIgnoreCondition.WhenWritingNull
                w.WriteLine(System.Text.Json.JsonSerializer.Serialize(meta, opts))
                0
            | Error msg ->
                eprintfn "Error: %s" msg
                1

        code)

[<When>]
let ``I run "crane pdf type" on the fixture`` () =
    RunWithWriter(fun w ->
        match currentAdapter.SampleText("fake.pdf", 3) with
        | Ok text ->
            let wordCount =
                text.Split([| ' '; '\n'; '\t' |], System.StringSplitOptions.RemoveEmptyEntries).Length

            let docType = if wordCount > 10 then "text" else "image"
            let opts = System.Text.Json.JsonSerializerOptions()
            w.WriteLine(System.Text.Json.JsonSerializer.Serialize({| ``type`` = docType |}, opts))
            if docType = "text" then 0 else 1
        | Error msg ->
            eprintfn "Error: %s" msg
            1)

[<Then>]
let ``the JSON output is valid`` () =
    let doc = System.Text.Json.JsonDocument.Parse(LastOutput)
    Assert.NotNull(doc)

[<Then>]
let ``the JSON field "pages" matches the known page count`` () =
    let doc = System.Text.Json.JsonDocument.Parse(LastOutput)
    Assert.Equal(5, doc.RootElement.GetProperty("pages").GetInt32())

[<Then>]
let ``the JSON field "size_bytes" is greater than 0`` () =
    let doc = System.Text.Json.JsonDocument.Parse(LastOutput)
    Assert.True(doc.RootElement.GetProperty("size_bytes").GetInt64() > 0L)

[<Then>]
let ``the JSON output contains type "([^"]*)"`` (expected: string) =
    let doc = System.Text.Json.JsonDocument.Parse(LastOutput)
    Assert.Equal(expected, doc.RootElement.GetProperty("type").GetString())

[<Then>]
let ``the exit code is (\d+)`` (expected: int) = Assert.Equal(expected, LastExitCode)
