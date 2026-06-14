namespace OseBe.Contexts.Health

open Giraffe
open OpenAPI.OseBe.Contracts.HealthResponse
open OseBe.Contexts.Health.Application

/// HTTP API for the health bounded context.
module Api =

    /// GET /api/v1/health → 200 with the liveness payload.
    let getHealthHandler: HttpHandler =
        fun next ctx ->
            let status = getHealth ()
            json { Status = status.Status } next ctx

    /// Routes for the health context.
    let routes: HttpHandler = GET >=> route "/api/v1/health" >=> getHealthHandler
