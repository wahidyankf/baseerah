module BaseerahBe.Tests.Unit.Tests.GreetingHandlerTests

open System.Net
open Xunit
open BaseerahBe.WebApp
open BaseerahBe.Tests.Unit.Steps.BddState

// @covers specs/apps/baseerah/behavior/baseerah-be/gherkin/hello/greeting.feature:The service returns a greeting
[<Fact>]
let ``hello route returns 200 with the greeting`` () =
    let client = buildClient webApp
    let resp = client.GetAsync("/api/v1/hello").Result
    Assert.Equal(HttpStatusCode.OK, resp.StatusCode)
    let body = resp.Content.ReadAsStringAsync().Result
    Assert.Contains("\"message\":\"Hello from Baseerah\"", body)

// @covers specs/apps/baseerah/behavior/baseerah-be/gherkin/hello/greeting.feature:An undeclared query string is ignored
[<Fact>]
let ``hello route ignores an undeclared query string`` () =
    let client = buildClient webApp
    let resp = client.GetAsync("/api/v1/hello?extra=param").Result
    Assert.Equal(HttpStatusCode.OK, resp.StatusCode)
    let body = resp.Content.ReadAsStringAsync().Result
    Assert.Contains("\"message\":\"Hello from Baseerah\"", body)
