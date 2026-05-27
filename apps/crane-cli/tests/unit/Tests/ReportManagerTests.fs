module CraneCli.Tests.Unit.Tests.ReportManagerTests

open Xunit
open CraneCli.Core.Domain.PdfMetadata
open CraneCli.Core.Domain.Report

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
