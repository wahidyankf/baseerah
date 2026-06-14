namespace OseBe.Contexts.AiOrchestration

open Giraffe
open OseBe.Contexts.AiOrchestration.Application

/// HTTP API for the ai-orchestration bounded context.
module Api =

    /// GET /api/v1/ai-orchestration/status → 200 with the context readiness.
    let statusHandler: HttpHandler =
        fun next ctx ->
            let readiness = initializeContext ()

            json
                {| state = sprintf "%A" readiness.State
                   capability = readiness.Capability |}
                next
                ctx

    /// Routes for the ai-orchestration context.
    let routes: HttpHandler =
        GET >=> route "/api/v1/ai-orchestration/status" >=> statusHandler
