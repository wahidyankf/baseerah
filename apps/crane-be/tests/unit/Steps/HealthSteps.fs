module CraneBe.Tests.Unit.Steps.HealthSteps

open TickSpec
open Xunit
open CraneBe.Adapters.In.HttpHandlers
open CraneBe.Adapters.Out.FakeMediaAdapter
open CraneBe.Tests.Unit.Steps.BddState

[<Given>]
let ``the crane-be service is running on its configured port`` () =
    let port = FakeMediaAdapter()
    Client <- Some(buildClient (webApp port))

[<When>]
let ``a client sends GET to /health`` () =
    let client = Client.Value
    let resp = client.GetAsync("/health").Result
    LastStatus <- int resp.StatusCode
    LastBody <- resp.Content.ReadAsStringAsync().Result

[<Then>]
let ``the response status is 200`` () = Assert.Equal(200, LastStatus)

[<Then>]
let ``the response body indicates the service is healthy`` () = Assert.Contains("healthy", LastBody)
