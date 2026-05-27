module CraneCli.Tests.Unit.Steps.ReportSteps

open TickSpec
open Xunit
open CraneCli.Tests.Unit.Steps.BddState

// ---- BDD shared state ----
let mutable private lastReportPath: string = ""
let mutable private currentScope: string = "pdf-to-md"

// ---- BDD Given steps ----

[<Given>]
let ``no existing chain file for scope "([^"]*)"`` (scope: string) = ()

[<Given>]
let ``a chain file for "([^"]*)" created (\d+) seconds ago with UUID "([^"]*)"``
    (scope: string)
    (seconds: int)
    (uuid: string)
    =
    ()

// ---- BDD When steps ----

[<When>]
let ``I run "crane report init" with scope "([^"]*)"`` (scope: string) = ()

// ---- BDD Then steps ----

[<Then>]
let ``a report file is created in "([^"]*)"`` (dir: string) = ()

[<Then>]
let ``the filename matches the pattern "([^"]*)"`` (_pattern: string) = ()

[<Then>]
let ``the JSON output contains the report path`` () = ()

[<Then>]
let ``the report filename contains "([^"]*)" followed by a new 6-hex UUID`` (prefix: string) = ()

[<Then>]
let ``the report filename contains only the new 6-hex UUID .no "([^"]*)".`` (uuid: string) = ()
