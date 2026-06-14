module OseBe.Tests.Unit.Steps.BddState

open Microsoft.AspNetCore.TestHost
open Microsoft.AspNetCore.Hosting
open Microsoft.Extensions.DependencyInjection
open Giraffe

/// Build an in-process test server from a Giraffe HttpHandler.
let buildClient (handler: HttpHandler) : System.Net.Http.HttpClient =
    let builder =
        WebHostBuilder()
            .ConfigureServices(fun (s: IServiceCollection) -> s.AddGiraffe() |> ignore)
            .Configure(fun app -> app.UseGiraffe(handler))

    let server = new TestServer(builder)
    server.CreateClient()

/// Mutable scenario state shared across TickSpec step bindings.
let mutable Client: System.Net.Http.HttpClient option = None

/// Last HTTP status code observed by a When step.
let mutable LastStatus: int = 0

/// Last HTTP response body observed by a When step.
let mutable LastBody: string = ""

/// Last context-readiness capability string observed by a When step.
let mutable LastCapability: string = ""
