module CraneCore.Domain.Finding

open System.Text.Json.Serialization

type Finding =
    { [<JsonPropertyName("category")>]
      Category: string
      [<JsonPropertyName("criticality")>]
      Criticality: string
      [<JsonPropertyName("confidence")>]
      Confidence: string
      [<JsonPropertyName("location_pdf")>]
      LocationPdf: string option
      [<JsonPropertyName("location_md")>]
      LocationMd: string option
      [<JsonPropertyName("description")>]
      Description: string
      [<JsonPropertyName("pdf_text")>]
      PdfText: string option
      [<JsonPropertyName("fix_suggestion")>]
      FixSuggestion: string option
      [<JsonPropertyName("auto_fixable")>]
      AutoFixable: bool }
