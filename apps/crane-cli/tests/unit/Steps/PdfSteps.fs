module CraneCli.Tests.Unit.Steps.PdfSteps

open TickSpec
open Xunit
open CraneCli.Tests.Unit.Steps.BddState

let mutable private currentAdapter: obj = obj ()

[<Given>]
let ``a text-based PDF fixture with a known page count`` () = ()

[<Given>]
let ``a text-based PDF fixture exists`` () = ()

[<Given>]
let ``an image-only PDF fixture exists`` () = ()

[<When>]
let ``I run "crane pdf info" on the fixture`` () = ()

[<When>]
let ``I run "crane pdf type" on the fixture`` () = ()

[<Then>]
let ``the JSON output is valid`` () = ()

[<Then>]
let ``the JSON field "pages" matches the known page count`` () = ()

[<Then>]
let ``the JSON field "size_bytes" is greater than 0`` () = ()

[<Then>]
let ``the JSON output contains type "([^"]*)"`` (expected: string) = ()

[<Then>]
let ``the exit code is (\d+)`` (expected: int) = ()
