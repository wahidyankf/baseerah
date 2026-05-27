module CraneCli.Tests.Unit.Tests.TextCheckerTests

open Xunit
open CraneCli.Core.Domain.Finding

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
