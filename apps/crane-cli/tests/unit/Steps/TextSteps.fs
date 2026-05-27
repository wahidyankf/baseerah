module CraneCli.Tests.Unit.Steps.TextSteps

open TickSpec
open Xunit
open CraneCli.Tests.Unit.Steps.BddState

// ---- BDD shared state ----
let mutable private pdfText: string = ""
let mutable private mdText: string = ""

// ---- BDD Given steps ----

[<Given>]
let ``a PDF fixture and its complete Markdown pair`` () = ()

[<Given>]
let ``a PDF fixture and a Markdown missing one section`` () = ()

[<Given>]
let ``a PDF with multiple consecutive spaces and its normalized Markdown`` () = ()

[<Given>]
let ``a PDF with "Organisation" and a Markdown with "Organization"`` () = ()

// ---- BDD When steps ----

[<When>]
let ``I run "crane text check" on the pair`` () = ()

// ---- BDD Then steps ----

[<Then>]
let ``the JSON output is an empty array`` () = ()

[<Then>]
let ``the JSON output contains a finding`` () = ()

[<Then>]
let ``the finding criticality is "([^"]*)"`` (expected: string) = ()

[<Then>]
let ``the finding category is "([^"]*)"`` (expected: string) = ()

[<Then>]
let ``no CRITICAL or HIGH finding is raised for that word`` () = ()
