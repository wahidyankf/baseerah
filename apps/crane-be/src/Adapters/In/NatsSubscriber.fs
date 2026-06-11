module CraneBe.Adapters.In.NatsSubscriber

open System
open System.Text
open System.Threading
open System.Threading.Tasks
open NATS.Client.Core
open CraneBe.Core.Ports

let private handleMsg (port: IMediaPort) (msg: NatsMsg<byte[]>) =
    task {
        // NATS.Net uses nullable byte[] on the Data property — treat null as empty
        let bytes: byte[] = msg.Data |> Option.ofObj |> Option.defaultValue [||]

        let response =
            if bytes.Length = 0 then
                """{"error":"empty payload"}"""
            else
                match port.Convert(bytes) with
                | Ok markdown -> markdown
                | Error e ->
                    let escaped = e.Replace("\\", "\\\\").Replace("\"", "\\\"")

                    sprintf """{"error":"%s"}""" escaped

        let responseBytes = Encoding.UTF8.GetBytes(response)

        if not (String.IsNullOrEmpty(msg.ReplyTo)) then
            do! msg.ReplyAsync<byte[]>(responseBytes)
    }

let private runSubscriberAsync (natsUrl: string) (port: IMediaPort) (cancel: CancellationToken) : Task<unit> =
    task {
        let opts = NatsOpts(Url = natsUrl)
        let conn = new NatsConnection(opts)

        try
            do! conn.ConnectAsync()

            let subscription =
                conn.SubscribeAsync<byte[]>("crane.convert", "crane.workers", cancellationToken = cancel)

            let enumerator = subscription.GetAsyncEnumerator(cancel)
            let mutable running = true

            while running do
                try
                    let! hasNext = enumerator.MoveNextAsync()

                    if hasNext then
                        do! handleMsg port enumerator.Current
                    else
                        running <- false
                with
                | :? OperationCanceledException -> running <- false
                | ex -> eprintfn "NatsSubscriber error: %s" ex.Message
        finally
            conn.DisposeAsync().AsTask() |> ignore
    }

let startSubscribersAsync
    (orgNatsUrl: string)
    (oseNatsUrl: string)
    (port: IMediaPort)
    (cancel: CancellationToken)
    : Task =
    let t1: Task<unit> = runSubscriberAsync orgNatsUrl port cancel
    let t2: Task<unit> = runSubscriberAsync oseNatsUrl port cancel
    Task.WhenAll(t1, t2)
