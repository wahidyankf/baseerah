namespace OseBe.Contexts.RegulatorySource

open Giraffe
open OseBe.Contexts.RegulatorySource.Application

/// HTTP API for the regulatory-source bounded context.
module Api =

    /// GET /api/v1/regulatory-source/status → 200 with the context readiness.
    let statusHandler: HttpHandler =
        fun next ctx ->
            let readiness = initializeContext ()

            json
                {| state = sprintf "%A" readiness.State
                   capability = readiness.Capability |}
                next
                ctx

    /// Routes for the regulatory-source context.
    let routes: HttpHandler =
        GET >=> route "/api/v1/regulatory-source/status" >=> statusHandler
