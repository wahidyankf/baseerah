namespace OrganicleverBe.Contexts.Health

open Giraffe
open OrganicleverBe.Contexts.Health.Application

/// HTTP API for the health bounded context.
module Api =

    /// GET /api/v1/health → 200 with the liveness payload.
    let getHealthHandler: HttpHandler =
        fun next ctx ->
            let status = getHealth ()
            json {| status = status.Status |} next ctx

    /// Routes for the health context.
    let routes: HttpHandler = GET >=> route "/api/v1/health" >=> getHealthHandler
