module CraneCore.Tests.Unit.Tests.ConvertTests

open Xunit
open CraneCore.Domain.PdfMetadata
open CraneCore.Ports

type MockTextPdfPort(sampleText: string) =
    interface IPdfPort with
        member _.GetMetadata(_path) =
            Ok
                { Pages = 1
                  Title = None
                  Author = None
                  File = "mock.pdf"
                  SizeBytes = 0L }

        member _.SampleText(_path, _pageCount) = Ok sampleText
        member _.ExtractPages(_path, _startPage, _endPage) = Ok sampleText

type MockOcrPort() =
    interface IOcrPort with
        member _.ExtractText(_path, _pageNum) = Ok "ocr result"

[<Fact>]
let ``convertPdfToMarkdown exists`` () =
    let _ = CraneCore.Convert.convertPdfToMarkdown
    Assert.True(true)

[<Fact>]
let ``convertPdfToMarkdown returns text for text-based PDF`` () =
    let manyWords = "word one two three four five six seven eight nine ten eleven"
    let pdfPort = MockTextPdfPort(manyWords) :> IPdfPort
    let ocrPort = MockOcrPort() :> IOcrPort
    let result = CraneCore.Convert.convertPdfToMarkdown pdfPort ocrPort "fake.pdf"

    match result with
    | Ok text -> Assert.Equal(manyWords, text)
    | Error msg -> Assert.Fail(sprintf "Expected Ok but got Error: %s" msg)

[<Fact>]
let ``convertPdfToMarkdown uses OCR for image-based PDF`` () =
    let fewWords = "just few"
    let pdfPort = MockTextPdfPort(fewWords) :> IPdfPort
    let ocrPort = MockOcrPort() :> IOcrPort
    let result = CraneCore.Convert.convertPdfToMarkdown pdfPort ocrPort "fake.pdf"

    match result with
    | Ok text -> Assert.Equal("ocr result", text)
    | Error msg -> Assert.Fail(sprintf "Expected Ok but got Error: %s" msg)
