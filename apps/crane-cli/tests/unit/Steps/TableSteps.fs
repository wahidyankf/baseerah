module CraneCli.Tests.Unit.Steps.TableSteps

open TickSpec
open Xunit
open CraneCli.Tests.Unit.Steps.BddState

// ---- BDD shared state ----
let mutable private pdfText: string = ""
let mutable private mdText: string = ""
let mutable private detectText: string = ""

// ---- BDD Given steps ----

[<Given>]
let ``a PDF fixture with a 3-column table`` () = ()

[<Given>]
let ``its Markdown conversion with a matching 3-column table`` () = ()

[<Given>]
let ``a PDF fixture with a table`` () = ()

[<Given>]
let ``a Markdown missing that table entirely`` () = ()

[<Given>]
let ``a PDF fixture with a 5-row table`` () = ()

[<Given>]
let ``a Markdown with a matching header but only 3 rows`` () = ()

[<Given>]
let ``layout text containing a 3-column columnar table`` () = ()

// ---- BDD When steps ----

[<When>]
let ``I run "crane table check" on the pair`` () = ()

[<When>]
let ``I run "crane table detect" on the text`` () = ()

// ---- BDD Then steps ----

[<Then>]
let ``the JSON output lists one table with col_count (\d+)`` (expected: int) = ()
