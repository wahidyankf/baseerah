namespace OseBe.Contexts.InternalPolicy

open Giraffe
open OseBe.Contexts.InternalPolicy.Application

/// HTTP API for the internal-policy bounded context.
module Api =

    /// GET /api/v1/internal-policy/status → 200 with the context readiness.
    let statusHandler: HttpHandler =
        fun next ctx ->
            let readiness = initializeContext ()

            json
                {| state = sprintf "%A" readiness.State
                   capability = readiness.Capability |}
                next
                ctx

    /// Routes for the internal-policy context.
    let routes: HttpHandler =
        GET >=> route "/api/v1/internal-policy/status" >=> statusHandler
