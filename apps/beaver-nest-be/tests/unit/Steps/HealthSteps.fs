module BeaverNestBe.Tests.Unit.Steps.HealthSteps

open System.Net.Http
open TickSpec
open Xunit
open BeaverNestBe.WebApp
open BeaverNestBe.Tests.Unit.Steps.BddState

// Step definitions binding the health/greeting Gherkin backgrounds and shared
// assertions to the in-process Giraffe routing surface, so the spec coverage
// validator resolves every step to executable code.

let mutable private client: HttpClient option = None
let mutable private lastStatus = 0
let mutable private lastBody = ""

[<Given>]
let ``the beaver-nest-be service is running on port 19320`` () = client <- Some(buildClient webApp)

[<Given>]
let ``the service has finished starting`` () = client <- Some(buildClient webApp)

[<When>]
let ``I send a GET request to "/api/v1/health"`` () =
    let resp = client.Value.GetAsync("/api/v1/health").Result
    lastStatus <- int resp.StatusCode
    lastBody <- resp.Content.ReadAsStringAsync().Result

[<Then>]
let ``the response status is 200`` () = Assert.Equal(200, lastStatus)

[<Then>]
let ``the response body field "status" equals "ok"`` () =
    Assert.Contains("\"status\":\"ok\"", lastBody.Replace(" ", ""))
