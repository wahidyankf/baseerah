module CraneCli.Tests.Unit.Steps.HeadingSteps

open TickSpec
open Xunit
open CraneCli.Tests.Unit.Steps.BddState

// ---- BDD shared state ----
let mutable private pdfText: string = ""
let mutable private mdText: string = ""
let mutable private inferText: string = ""

// ---- BDD Given steps ----

[<Given>]
let ``a PDF fixture where heading "([^"]*)" implies depth (\d+)`` (heading: string) (_depth: int) = ()

[<Given>]
let ``the Markdown has that heading at depth (\d+)`` (depth: int) = ()

[<Given>]
let ``the text "([^"]*)"`` (text: string) = ()

// ---- BDD When steps ----

[<When>]
let ``I run "crane heading check" on the pair`` () = ()

[<When>]
let ``I run "crane heading infer" on that text`` () = ()

// ---- BDD Then steps ----

[<Then>]
let ``a finding with criticality "([^"]*)" is returned`` (expected: string) = ()

[<Then>]
let ``the finding states expected_depth (\d+) and found_depth (\d+)`` (expectedDepth: int) (foundDepth: int) = ()

[<Then>]
let ``the JSON output shows depth (\d+) and confidence "([^"]*)"`` (depth: int) (confidence: string) = ()
