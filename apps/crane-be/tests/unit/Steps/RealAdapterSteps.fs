module CraneBe.Tests.Unit.Steps.RealAdapterSteps

/// Stub step definitions for @integration @e2e and @e2e scenarios.
/// These stubs satisfy spec-coverage in Phase 2.
/// Real bindings will be wired in Phase 3 (real adapter) and Phase 4 (e2e runner).
open TickSpec
open Xunit
open CraneBe.Tests.Unit.Steps.BddState

[<Given>]
let ``crane-be is configured with the real PdfPig/Tesseract adapter`` () =
    // Phase 3: wire the real adapter here
    raise (System.NotImplementedException("Real adapter not yet implemented — Phase 3"))

[<When>]
let ``a client sends POST /media/pdf-to-md with a real sample PDF`` () =
    // Phase 3/4: wire real PDF fixture here
    raise (System.NotImplementedException("Real PDF test not yet implemented — Phase 3/4"))

[<Then>]
let ``the response body contains markdown extracted from the PDF`` () =
    // Phase 3: verify real adapter response
    raise (System.NotImplementedException("Real adapter response check not yet implemented — Phase 3"))

[<Then>]
let ``the response Content-Type is text/markdown`` () =
    // Phase 4: verify content-type header in e2e runner
    raise (System.NotImplementedException("Content-Type check not yet implemented — Phase 4"))
