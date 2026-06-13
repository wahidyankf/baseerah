module OseAppBe.Tests.Unit.Tests.HealthHandlerTests

open System.Net
open Xunit
open OseAppBe.Handlers.HealthHandler
open OseAppBe.Tests.Unit.Steps.BddState

[<Fact>]
let ``health handler returns 200`` () =
    let client = buildClient webApp
    let resp = client.GetAsync("/health").Result
    Assert.Equal(HttpStatusCode.OK, resp.StatusCode)

[<Fact>]
let ``health handler returns JSON status ok`` () =
    let client = buildClient webApp
    let resp = client.GetAsync("/health").Result
    let body = resp.Content.ReadAsStringAsync().Result
    Assert.Contains("\"status\"", body)
    Assert.Contains("ok", body)
    Assert.Equal("application/json", resp.Content.Headers.ContentType.MediaType)
