module CraneCli.Tests.Unit.Steps.OcrSteps

open TickSpec
open Xunit
open CraneCli.Tests.Unit.Steps.BddState

// ---- BDD shared state ----
let mutable private mdFixture: string = ""

// ---- BDD Given steps ----

[<Given>]
let ``a Markdown fixture with an OCR-tagged section at 15% estimated error rate`` () = ()

[<Given>]
let ``a Markdown fixture with an OCR-tagged section at 1% estimated error rate`` () = ()

[<Given>]
let ``a Markdown fixture with no OCR page tags`` () = ()

// ---- BDD When steps ----

[<When>]
let ``I run "crane ocr quality" on the fixture`` () = ()

// ---- BDD Then steps ----

[<Then>]
let ``the finding includes the OCR page number`` () = ()
