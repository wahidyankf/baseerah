module CraneCli.Tests.Unit.Steps.FigureSteps

open TickSpec
open CraneCli.Tests.Unit.Steps.BddState

// ---- BDD shared state ----
let mutable private pdfText: string = ""
let mutable private mdText: string = ""

// ---- BDD Given steps ----

[<Given>]
let ``a PDF fixture referencing "Figure (\d+)"`` (num: string) = ()

[<Given>]
let ``its Markdown with a Mermaid code block near that reference`` () = ()

[<Given>]
let ``its Markdown with a "\[FIGURE (\d+): \.\.\.\]" placeholder`` (num: string) = ()

[<Given>]
let ``a Markdown with no Mermaid block or placeholder for Figure (\d+)`` (_num: string) = ()

// ---- BDD When steps ----

[<When>]
let ``I run "crane figure check" on the pair`` () = ()
