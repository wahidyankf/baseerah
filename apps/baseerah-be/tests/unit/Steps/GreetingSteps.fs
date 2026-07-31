module BaseerahBe.Tests.Unit.Steps.GreetingSteps

open System.Net.Http
open TickSpec
open Xunit
open BaseerahBe.WebApp
open BaseerahBe.Tests.Unit.Steps.BddState

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
let ``the response body field "message" equals "Hello from Baseerah"`` () =
    Assert.Contains("Hello from Baseerah", lastBody)
