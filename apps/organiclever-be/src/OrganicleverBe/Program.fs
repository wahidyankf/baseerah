module OrganicleverBe.Program

open Microsoft.AspNetCore.Builder
open Microsoft.AspNetCore.Hosting
open Microsoft.EntityFrameworkCore
open Microsoft.Extensions.DependencyInjection
open Microsoft.Extensions.Hosting
open Giraffe
open OrganicleverBe.Infrastructure.AppDbContext
open OrganicleverBe.Infrastructure.Database
open OrganicleverBe.Infrastructure.NatsClient
open OrganicleverBe.Handlers.HealthHandler

// Routes: /health now; journal CRUD routes are added in Phase 4.
let private configureApp (app: IApplicationBuilder) = app.UseGiraffe webApp

let private configureServices (connStr: string) (services: IServiceCollection) =
    services.AddGiraffe() |> ignore

    services.AddDbContext<AppDbContext>(fun opts -> opts.UseNpgsql(connStr).UseSnakeCaseNamingConvention() |> ignore)
    |> ignore

let private buildHost (args: string[]) (connStr: string) =
    Host
        .CreateDefaultBuilder(args)
        .ConfigureWebHostDefaults(fun webHostBuilder ->
            webHostBuilder.Configure(configureApp).ConfigureServices(configureServices connStr)
            |> ignore)
        .Build()

[<EntryPoint>]
let main args =
    // 1. Config (fail-fast on missing DATABASE_URL).
    let connStr = requireDatabaseUrl ()

    // 2. Schema migration on boot (DbUp embedded scripts).
    runMigrations connStr

    // 3. Messaging: best-effort NATS connect (non-fatal; exercised at e2e).
    let natsConnection =
        connectAsync (natsUrl ()) |> Async.AwaitTask |> Async.RunSynchronously

    // 4. HTTP host (Giraffe routes + EF DbContext).
    let host = buildHost args connStr

    host.Run()

    natsConnection |> Option.iter (fun c -> c.DisposeAsync().AsTask().Wait())
    0
