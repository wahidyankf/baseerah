module BaseerahBe.Program

open Microsoft.AspNetCore.Builder
open Microsoft.AspNetCore.Hosting
open Microsoft.Extensions.DependencyInjection
open Microsoft.Extensions.Hosting
open Giraffe
open BaseerahBe.WebApp

let private configureApp (app: IApplicationBuilder) = app.UseGiraffe webApp

let private configureServices (services: IServiceCollection) = services.AddGiraffe() |> ignore

/// Reads BASEERAH_BE_PORT, defaulting to 19320 when unset or blank.
let port () : int =
    match System.Environment.GetEnvironmentVariable("BASEERAH_BE_PORT") with
    | null
    | "" -> 19320
    | value -> int value

[<EntryPoint>]
let main args =
    Host
        .CreateDefaultBuilder(args)
        .ConfigureWebHostDefaults(fun webHostBuilder ->
            webHostBuilder
                .UseUrls($"http://0.0.0.0:{port ()}")
                .Configure(configureApp)
                .ConfigureServices(configureServices)
            |> ignore)
        .Build()
        .Run()

    0
