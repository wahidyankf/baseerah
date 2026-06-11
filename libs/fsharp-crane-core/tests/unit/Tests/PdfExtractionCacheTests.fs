module CraneCore.Tests.Unit.Tests.PdfExtractionCacheTests

open System
open System.IO
open Xunit
open CraneCore.Ports
open CraneCore.Logic.PdfExtractionCache
open CraneCore.Domain.PdfMetadata

type FakePdfPort(text: string, pages: int, sizeBytes: int64) =
    interface IPdfPort with
        member _.GetMetadata(path) =
            Ok
                { Pages = pages
                  Title = Some "Fake"
                  Author = None
                  File = path
                  SizeBytes = sizeBytes }

        member _.SampleText(_path, _pageCount) = Ok text
        member _.ExtractPages(_path, _startPage, _endPage) = Ok text

type FailingPdfPort() =
    interface IPdfPort with
        member _.GetMetadata(_path) = Error "metadata not available"
        member _.SampleText(_path, _pageCount) = Error "sample failed"
        member _.ExtractPages(_path, _startPage, _endPage) = Error "extract failed"

let private withTempPdf (f: string -> string -> unit) =
    let tmpDir =
        Path.Combine(Path.GetTempPath(), sprintf "crane-cache-test-%s" (Guid.NewGuid().ToString("N").[..7]))

    Directory.CreateDirectory(tmpDir) |> ignore
    let pdfPath = Path.Combine(tmpDir, "test.pdf")
    File.WriteAllText(pdfPath, "fake pdf content for testing purposes")

    try
        f pdfPath tmpDir
    finally
        if Directory.Exists(tmpDir) then
            Directory.Delete(tmpDir, true)

[<Fact>]
let ``wrap returns IPdfPort that proxies metadata`` () =
    let inner = FakePdfPort("some text", 5, 1024L) :> IPdfPort
    let cacheDir = Path.GetTempPath()
    let cached = wrap inner cacheDir
    let result = cached.GetMetadata("fake.pdf")
    Assert.True(result.IsOk)

[<Fact>]
let ``defaultCacheDir returns a non-empty string`` () =
    let dir = defaultCacheDir ()
    Assert.NotEmpty(dir)

[<Fact>]
let ``defaultCacheDir uses XDG_CACHE_HOME when set`` () =
    let prev = Environment.GetEnvironmentVariable("XDG_CACHE_HOME")

    try
        let tmpXdg = Path.GetTempPath()
        Environment.SetEnvironmentVariable("XDG_CACHE_HOME", tmpXdg)
        let dir = defaultCacheDir ()
        Assert.True(dir.StartsWith(tmpXdg))
    finally
        Environment.SetEnvironmentVariable("XDG_CACHE_HOME", prev)

[<Fact>]
let ``wrap cached adapter returns same text on second call for nonexistent pdf`` () =
    let inner = FakePdfPort("hello world text", 1, 512L) :> IPdfPort
    let cacheDir = Path.GetTempPath()
    let cached = wrap inner cacheDir
    let result1 = cached.SampleText("fake.pdf", 5)
    let result2 = cached.SampleText("fake.pdf", 5)
    Assert.Equal(result1, result2)

[<Fact>]
let ``wrap with real file caches SampleText on first call`` () =
    withTempPdf (fun pdfPath cacheDir ->
        let inner = FakePdfPort("cached text content", 3, 1024L) :> IPdfPort
        let cached = wrap inner cacheDir
        let result = cached.SampleText(pdfPath, 3)

        match result with
        | Ok text -> Assert.Equal("cached text content", text)
        | Error msg -> Assert.Fail(sprintf "SampleText failed: %s" msg))

[<Fact>]
let ``wrap with real file returns cached SampleText on second call`` () =
    withTempPdf (fun pdfPath cacheDir ->
        let inner = FakePdfPort("cached sample result", 3, 1024L) :> IPdfPort
        let cached = wrap inner cacheDir
        let result1 = cached.SampleText(pdfPath, 3)
        let result2 = cached.SampleText(pdfPath, 3)

        match result1, result2 with
        | Ok t1, Ok t2 -> Assert.Equal(t1, t2)
        | _ -> Assert.Fail("Both calls should succeed"))

[<Fact>]
let ``wrap with real file caches ExtractPages on first call`` () =
    withTempPdf (fun pdfPath cacheDir ->
        let inner = FakePdfPort("extracted pages text", 10, 2048L) :> IPdfPort
        let cached = wrap inner cacheDir
        let result = cached.ExtractPages(pdfPath, 1, 5)

        match result with
        | Ok text -> Assert.Equal("extracted pages text", text)
        | Error msg -> Assert.Fail(sprintf "ExtractPages failed: %s" msg))

[<Fact>]
let ``wrap with real file returns cached ExtractPages on second call`` () =
    withTempPdf (fun pdfPath cacheDir ->
        let inner = FakePdfPort("pages cache result", 10, 2048L) :> IPdfPort
        let cached = wrap inner cacheDir
        let result1 = cached.ExtractPages(pdfPath, 1, 5)
        let result2 = cached.ExtractPages(pdfPath, 1, 5)

        match result1, result2 with
        | Ok t1, Ok t2 -> Assert.Equal(t1, t2)
        | _ -> Assert.Fail("Both ExtractPages calls should succeed"))

[<Fact>]
let ``wrap propagates inner SampleText error when pdf not readable`` () =
    let inner = FailingPdfPort() :> IPdfPort
    let cacheDir = Path.GetTempPath()
    let cached = wrap inner cacheDir

    match cached.SampleText("nonexistent.pdf", 3) with
    | Error _ -> ()
    | Ok _ -> Assert.Fail("Expected error for failing adapter")

[<Fact>]
let ``wrap propagates inner ExtractPages error when pdf not readable`` () =
    let inner = FailingPdfPort() :> IPdfPort
    let cacheDir = Path.GetTempPath()
    let cached = wrap inner cacheDir

    match cached.ExtractPages("nonexistent.pdf", 1, 5) with
    | Error _ -> ()
    | Ok _ -> Assert.Fail("Expected error for failing adapter")
