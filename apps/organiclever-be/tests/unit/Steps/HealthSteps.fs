module OrganicleverBe.Tests.Unit.Steps.HealthSteps

open System.Net.Http
open TickSpec
open Xunit
open OrganicleverBe.WebApp
open OrganicleverBe.Tests.Unit.Steps.BddState

// Step definitions for the be-health context (health-check.feature). These bind
// the Gherkin steps to the in-process Giraffe routing surface so the spec
// coverage validator resolves every step to executable code.

let mutable private client: HttpClient option = None
let mutable private lastStatus = 0
let mutable private lastBody = ""

[<Given>]
let ``the API is running`` () = client <- Some(buildClient webApp)

[<When>]
let ``an operations engineer sends GET /health`` () =
    let resp = client.Value.GetAsync("/health").Result
    lastStatus <- int resp.StatusCode
    lastBody <- resp.Content.ReadAsStringAsync().Result

[<When>]
let ``an unauthenticated engineer sends GET /health`` () =
    let resp = client.Value.GetAsync("/health").Result
    lastStatus <- int resp.StatusCode
    lastBody <- resp.Content.ReadAsStringAsync().Result

[<Then>]
let ``the response status code should be 200`` () = Assert.Equal(200, lastStatus)

[<Then>]
let ``the health status should be "ok"`` () =
    Assert.Contains("\"status\"", lastBody)
    Assert.Contains("ok", lastBody)

[<Then>]
let ``the response should not include detailed component health information`` () =
    Assert.DoesNotContain("database", lastBody)
    Assert.DoesNotContain("components", lastBody)
