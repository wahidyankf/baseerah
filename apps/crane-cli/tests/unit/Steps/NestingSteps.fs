module CraneCli.Tests.Unit.Steps.NestingSteps

open TickSpec
open CraneCli.Tests.Unit.Steps.BddState

// ---- BDD shared state ----
let mutable private pdfText: string = ""
let mutable private mdText: string = ""

// ---- BDD Given steps ----

[<Given>]
let ``a PDF fixture with a single-level bullet list`` () = ()

[<Given>]
let ``its Markdown conversion with matching single-level nesting`` () = ()

[<Given>]
let ``a PDF fixture where nested items appear under a parent`` () = ()

[<Given>]
let ``a Markdown with those items at the wrong nesting level`` () = ()

[<Given>]
let ``a PDF fixture with two-level nesting`` () = ()

[<Given>]
let ``a Markdown with the second level at depth three instead of two`` () = ()

// ---- BDD When steps ----

[<When>]
let ``I run "crane nesting check" on the pair`` () = ()
