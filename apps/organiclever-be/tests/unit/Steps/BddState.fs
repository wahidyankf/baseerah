module OrganicleverBe.Tests.Unit.Steps.BddState

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
