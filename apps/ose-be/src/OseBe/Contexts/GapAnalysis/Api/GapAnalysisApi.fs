namespace OseBe.Contexts.GapAnalysis

open Giraffe
open OseBe.Contexts.GapAnalysis.Application

/// HTTP API for the gap-analysis bounded context.
module Api =

    /// GET /api/v1/gap-analysis/status → 200 with the context readiness.
    let statusHandler: HttpHandler =
        fun next ctx ->
            let readiness = initializeContext ()

            json
                {| state = sprintf "%A" readiness.State
                   capability = readiness.Capability |}
                next
                ctx

    /// Routes for the gap-analysis context.
    let routes: HttpHandler =
        GET >=> route "/api/v1/gap-analysis/status" >=> statusHandler
