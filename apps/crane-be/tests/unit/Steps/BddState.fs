module CraneBe.Tests.Unit.Steps.BddState

open Microsoft.AspNetCore.TestHost
open Microsoft.AspNetCore.Hosting
open Microsoft.Extensions.DependencyInjection
open Giraffe

/// Last HTTP response status code captured in the When step.
let mutable LastStatus: int = 0

/// Last HTTP response body captured in the When step.
let mutable LastBody: string = ""

/// The in-process HTTP client for the current scenario.
let mutable Client: System.Net.Http.HttpClient option = None

/// Build an in-process test server using the given Giraffe HttpHandler.
let buildClient (handler: HttpHandler) : System.Net.Http.HttpClient =
    let builder =
        WebHostBuilder()
            .ConfigureServices(fun (s: IServiceCollection) -> s.AddGiraffe() |> ignore)
            .Configure(fun app -> app.UseGiraffe(handler))

    let server = new TestServer(builder)
    server.CreateClient()
