module CraneBe.Program

open System.Threading
open Microsoft.AspNetCore.Builder
open Microsoft.Extensions.DependencyInjection
open Giraffe
open CraneBe.Adapters.Out.RealMediaAdapter
open CraneBe.Adapters.In.HttpHandlers
open CraneBe.Adapters.In.NatsSubscriber

[<EntryPoint>]
let main argv =
    match CraneBe.Config.load () with
    | Error msg ->
        eprintfn "Configuration error: %s" msg
        1
    | Ok config ->
        use cts = new CancellationTokenSource()
        let builder = WebApplication.CreateBuilder(argv)
        builder.Services.AddGiraffe() |> ignore
        let app = builder.Build()
        let port = RealMediaAdapter()

        let _natsTask =
            startSubscribersAsync config.OrganicLeverNatsUrl config.OseAppNatsUrl port cts.Token

        app.UseGiraffe(webApp port)
        app.Run($"http://0.0.0.0:{config.Port}")
        0
