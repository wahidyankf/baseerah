module BaseerahBe.Tests.Unit.Tests.NotFoundHandlerTests

open System.Net
open System.Text.Json
open Xunit
open BaseerahBe.WebApp
open BaseerahBe.Tests.Unit.Steps.BddState

// @covers specs/apps/baseerah/behavior/baseerah-be/gherkin/hello/greeting.feature:An unknown route is refused
[<Fact>]
let ``unknown route returns 404 with a non-empty JSON error`` () =
    let client = buildClient webApp
    let resp = client.GetAsync("/api/v1/does-not-exist").Result
    Assert.Equal(HttpStatusCode.NotFound, resp.StatusCode)
    let body = resp.Content.ReadAsStringAsync().Result
    let doc = JsonDocument.Parse(body)
    let error = doc.RootElement.GetProperty("error").GetString()
    Assert.False(System.String.IsNullOrEmpty(error))
