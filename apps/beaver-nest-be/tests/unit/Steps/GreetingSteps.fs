module BeaverNestBe.Tests.Unit.Steps.GreetingSteps

open System.Net.Http
open TickSpec
open Xunit
open BeaverNestBe.WebApp
open BeaverNestBe.Tests.Unit.Steps.BddState

// Step definitions for the greeting scenario, binding the Gherkin steps to
// the in-process Giraffe routing surface for the spec coverage validator.

let mutable private client: HttpClient option = None
let mutable private lastStatus = 0
let mutable private lastBody = ""

[<When>]
let ``I send a GET request to "/api/v1/hello"`` () =
    let resp = client.Value.GetAsync("/api/v1/hello").Result
    lastStatus <- int resp.StatusCode
    lastBody <- resp.Content.ReadAsStringAsync().Result

[<Then>]
let ``the response body field "message" equals "Hello from BeaverNest"`` () =
    Assert.Contains("Hello from BeaverNest", lastBody)

// Rule-16 finding SG-004: an undeclared query string is ignored. The `\?` here
// is a literal backslash-escape, not a typo — rhino-cli's spec-coverage
// checker compiles this backtick name directly as a `^…$` regex pattern
// (see apps/rhino-cli/src/application/speccoverage/extractors.rs), so an
// unescaped "?" would be read as a regex quantifier and never match the
// Gherkin step's literal "?".
[<When>]
let ``I send a GET request to "/api/v1/hello\?extra=param"`` () =
    let resp = client.Value.GetAsync("/api/v1/hello?extra=param").Result
    lastStatus <- int resp.StatusCode
    lastBody <- resp.Content.ReadAsStringAsync().Result

// Rule-16 finding SG-001: a wrong HTTP method on a known path is refused with
// the same catch-all 404 envelope as an unknown path (Giraffe matches path+verb
// together, so there is no intermediate "path matched, verb didn't" state).
[<When>]
let ``I send a POST request to "/api/v1/hello"`` () =
    let resp = client.Value.PostAsync("/api/v1/hello", new StringContent("")).Result
    lastStatus <- int resp.StatusCode
    lastBody <- resp.Content.ReadAsStringAsync().Result
