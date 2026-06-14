namespace OrganicleverBe.Contexts.Messaging

open Giraffe
open OrganicleverBe.Contexts.Messaging.Domain
open OrganicleverBe.Contexts.Messaging.Application

/// HTTP API for the messaging bounded context: the messaging status surface.
module Api =

    /// GET /api/v1/system/status/messaging → reports the JetStream demo outcome.
    let statusHandler (status: SharedMessagingStatus) : HttpHandler =
        fun next ctx ->
            let outcome = status.Get() |> outcomeToString
            json {| jetstream_demo = outcome |} next ctx

    /// Routes for the messaging context, bound to the shared status surface.
    let routes (status: SharedMessagingStatus) : HttpHandler =
        GET >=> route "/api/v1/system/status/messaging" >=> statusHandler status
