module CraneCli.Tests.Unit.Tests.HeadingCheckerTests

open Xunit
open CraneCli.Core.Logic.HeadingChecker

[<Fact>]
let ``inferDepthFromNumbering returns None for non-numbered line`` () =
    let result = inferDepthFromNumbering "Introduction"
    Assert.Equal(None, result)

[<Fact>]
let ``inferDepthFromNumbering returns depth 2 for top-level numbered section`` () =
    let result = inferDepthFromNumbering "1. Introduction"
    Assert.Equal(Some(2, "HIGH"), result)

[<Fact>]
let ``inferDepthFromNumbering returns depth 3 for second-level`` () =
    let result = inferDepthFromNumbering "1.1 Overview"
    Assert.Equal(Some(3, "HIGH"), result)

[<Fact>]
let ``extractMdHeadings returns empty list for text without headings`` () =
    let result = extractMdHeadings "some plain text\nno headings here"
    Assert.Empty(result)

[<Fact>]
let ``extractMdHeadings extracts H2 heading`` () =
    let result = extractMdHeadings "## Section Title"
    Assert.Equal(1, result.Length)
    Assert.Equal(2, result.[0].Depth)

[<Fact>]
let ``checkHeadings returns empty for matching headings`` () =
    let pdfText = "1. Introduction"
    let mdText = "## Introduction"
    let result = checkHeadings pdfText mdText
    Assert.Empty(result)
