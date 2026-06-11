module CraneBe.Program

open Microsoft.AspNetCore.Builder
open Microsoft.Extensions.DependencyInjection
open Giraffe
open CraneBe.Adapters.Out.FakeMediaAdapter
open CraneBe.Adapters.In.HttpHandlers

[<EntryPoint>]
let main argv =
    match CraneBe.Config.load () with
    | Error msg ->
        eprintfn "Configuration error: %s" msg
        1
    | Ok config ->
        let builder = WebApplication.CreateBuilder(argv)
        builder.Services.AddGiraffe() |> ignore
        let app = builder.Build()
        let port = FakeMediaAdapter()
        app.UseGiraffe(webApp port)
        app.Run($"http://0.0.0.0:{config.Port}")
        0
