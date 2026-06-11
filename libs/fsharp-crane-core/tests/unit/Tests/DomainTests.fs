module CraneCore.Tests.Unit.Tests.DomainTests

open Xunit
open CraneCore.Domain.Finding
open CraneCore.Domain.PdfMetadata
open CraneCore.Domain.Report

[<Fact>]
let ``Finding type has category field`` () =
    let f =
        { Category = "text-completeness"
          Criticality = "HIGH"
          Confidence = "HIGH"
          LocationPdf = None
          LocationMd = None
          Description = "test"
          PdfText = None
          FixSuggestion = None
          AutoFixable = false }

    Assert.Equal("text-completeness", f.Category)

[<Fact>]
let ``PdfMetadata type has pages field`` () =
    let meta =
        { Pages = 10
          Title = Some "Test"
          Author = None
          File = "test.pdf"
          SizeBytes = 1024L }

    Assert.Equal(10, meta.Pages)

[<Fact>]
let ``SkipListEntry type has md_basename field`` () =
    let entry =
        { MdBasename = "test.md"
          Category = "text-completeness"
          Description = "some text"
          Key = "abc123"
          Accepted = "2026-01-01"
          Reason = "false positive" }

    Assert.Equal("test.md", entry.MdBasename)
