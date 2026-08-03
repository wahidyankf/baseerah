module BeaverNestBe.Tests.Unit.Tests.ReadinessHandlerTests

open System.Net
open System.Net.Http
open Xunit
open BeaverNestBe.Application.ReadinessPort
open BeaverNestBe.WebApp
open BeaverNestBe.Tests.Unit.Steps.BddState

let private readinessResponse port =
    let client = buildClient (webAppWith port)
    client.GetAsync("/api/v1/readiness").Result

let private assertNoStoreWithoutValidator (response: HttpResponseMessage) =
    Assert.Equal("no-store", response.Headers.GetValues("Cache-Control") |> Seq.head)
    Assert.False(response.Headers.Contains("ETag"))
    Assert.False(response.Content.Headers.Contains("Last-Modified"))

// @covers specs/apps/beaver-nest/behavior/beaver-nest-be/gherkin/health/readiness-ready.feature:Ready workspace reports database and schema state
[<Fact>]
let ``ready port produces a safe current readiness response`` () =
    let response = readinessResponse alwaysReady
    Assert.Equal(HttpStatusCode.OK, response.StatusCode)
    assertNoStoreWithoutValidator response
    let body = response.Content.ReadAsStringAsync().Result
    Assert.Contains("\"status\":\"ready\"", body)
    Assert.Contains("\"database\":\"ready\"", body)
    Assert.Contains("\"schema\":\"current\"", body)

// @covers specs/apps/beaver-nest/behavior/beaver-nest-be/gherkin/health/readiness-unready.feature:Unready workspace returns a safe response
[<Fact>]
let ``unavailable port maps every failure to a safe 503 response`` () =
    let response = readinessResponse (fromProbe (fun () -> false))
    Assert.Equal(HttpStatusCode.ServiceUnavailable, response.StatusCode)
    assertNoStoreWithoutValidator response
    let body = response.Content.ReadAsStringAsync().Result
    Assert.Equal("{\"status\":\"not-ready\"}", body)
