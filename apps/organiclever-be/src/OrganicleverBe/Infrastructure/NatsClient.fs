module OrganicleverBe.Infrastructure.NatsClient

open System
open System.Threading.Tasks
open NATS.Client.Core

/// Reads ORGANICLEVER_BE_NATS_URL or falls back to the default local NATS URL.
let natsUrl () : string =
    match Environment.GetEnvironmentVariable("ORGANICLEVER_BE_NATS_URL") with
    | null
    | "" -> "nats://localhost:4222"
    | value -> value

/// Opens a best-effort NATS.Net connection on boot. Messaging is exercised at the
/// e2e level (JetStream demo, later phases); a failed connect here logs and is
/// non-fatal so the HTTP host still serves /health.
let connectAsync (url: string) : Task<NatsConnection option> =
    task {
        let conn = new NatsConnection(NatsOpts(Url = url))

        try
            do! conn.ConnectAsync()
            printfn "NATS connected: %s" url
            return Some conn
        with ex ->
            eprintfn "NATS connect failed (%s): %s" url ex.Message
            do! conn.DisposeAsync()
            return None
    }
