module CraneCli.Tests.Unit.Steps.CheckAllSteps

open TickSpec
open Xunit
open CraneCli.Tests.Unit.Steps.BddState

let mutable private pdfText: string = ""
let mutable private mdText: string = ""

[<Given>]
let ``a PDF fixture and an MD that matches across all dimensions`` () = ()

[<Given>]
let ``a PDF fixture and an MD missing content`` () = ()

[<When>]
let ``I run "crane check-all" on the pair`` () = ()
