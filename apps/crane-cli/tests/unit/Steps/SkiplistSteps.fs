module CraneCli.Tests.Unit.Steps.SkiplistSteps

open TickSpec
open Xunit
open CraneCli.Tests.Unit.Steps.BddState

// ---- BDD shared state ----
let mutable private currentMdBasename: string = "nist-sp-800-53"
let mutable private currentTempPath: string = ""

// ---- BDD Given steps ----

[<Given>]
let ``no existing skip list for "([^"]*)"`` (mdBasename: string) = ()

[<Given>]
let ``a skip list for "([^"]*)" already containing the entry for text-completeness "([^"]*)"``
    (mdBasename: string)
    (description: string)
    =
    ()

[<Given>]
let ``a skip list containing "([^|]*)\| ([^|]*)\| ([^"]*)"``
    (category: string)
    (mdBasename: string)
    (description: string)
    =
    ()

// ---- BDD When steps ----

[<When>]
let ``I run "crane skiplist add nist-sp-800-53 text-completeness '([^']*)'"`` (description: string) = ()

[<When>]
let ``I run "crane skiplist add" with the same arguments`` () = ()

[<When>]
let ``I run "crane skiplist check nist-sp-800-53 mermaid-syntax '([^']*)'"`` (description: string) = ()

[<When>]
let ``I run "crane skiplist check nist-sp-800-53 text-completeness '([^']*)'"`` (description: string) = ()

// ---- BDD Then steps ----

[<Then>]
let ``the skip list file is created`` () = ()

[<Then>]
let ``it contains one entry with category "([^"]*)"`` (category: string) = ()

[<Then>]
let ``the skip list file contains exactly one matching entry`` () = ()

[<Then>]
let ``the JSON output contains match true`` () = ()

[<Then>]
let ``the JSON output contains match false`` () = ()
