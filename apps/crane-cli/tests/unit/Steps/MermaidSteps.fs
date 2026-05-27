module CraneCli.Tests.Unit.Steps.MermaidSteps

open TickSpec
open Xunit
open CraneCli.Tests.Unit.Steps.BddState

// ---- BDD shared state ----
let mutable private mdFixture: string = ""

// ---- BDD Given steps ----

[<Given>]
let ``a Markdown fixture with a syntactically valid "([^"]*)" block`` (blockType: string) = ()

[<Given>]
let ``a Markdown fixture with a Mermaid block starting with "([^"]*)"`` (keyword: string) = ()

[<Given>]
let ``a Markdown fixture with a Mermaid block containing unbalanced "\["`` () = ()

[<Given>]
let ``a Markdown fixture with one block per known diagram type`` () = ()

// ---- BDD When steps ----

[<When>]
let ``I run "crane mermaid validate" on the fixture`` () = ()

// ---- BDD Then steps ----

[<Then>]
let ``the finding description mentions "([^"]*)"`` (keyword: string) = ()

[<Then>]
let ``a finding with criticality "([^"]*)" and category "([^"]*)" is returned`` (crit: string) (cat: string) = ()
