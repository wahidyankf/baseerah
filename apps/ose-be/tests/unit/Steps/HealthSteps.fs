module OseBe.Tests.Unit.Steps.HealthSteps

open TickSpec
open Xunit
open OseBe.WebApp
open OseBe.Tests.Unit.Steps.BddState

[<Given>]
let ``the ose-be service is running`` () = Client <- Some(buildClient webApp)

[<When>]
let ``I send GET /api/v1/health`` () =
    let client = Client.Value
    let resp = client.GetAsync("/api/v1/health").Result
    LastStatus <- int resp.StatusCode
    LastBody <- resp.Content.ReadAsStringAsync().Result

[<Then>]
let ``the response status is 200`` () = Assert.Equal(200, LastStatus)

[<Then>]
let ``the response body has a "status" field equal to "healthy"`` () =
    Assert.Contains("\"status\"", LastBody)
    Assert.Contains("healthy", LastBody)
