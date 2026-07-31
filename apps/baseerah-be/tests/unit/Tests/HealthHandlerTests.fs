module BaseerahBe.Tests.Unit.Tests.HealthHandlerTests

open System.Net
open Xunit
open BaseerahBe.WebApp
open BaseerahBe.Tests.Unit.Steps.BddState

// @covers specs/apps/baseerah/behavior/baseerah-be/gherkin/health/service-health.feature:The service reports liveness
[<Fact>]
let ``health route returns 200 with status ok`` () =
    let client = buildClient webApp
    let resp = client.GetAsync("/api/v1/health").Result
    Assert.Equal(HttpStatusCode.OK, resp.StatusCode)
    let body = resp.Content.ReadAsStringAsync().Result
    Assert.Contains("\"status\":\"ok\"", body)
