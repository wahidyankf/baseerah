module OseAppBe.Handlers.HealthHandler

open Giraffe
open OpenAPI.OseAppBe.Contracts.HealthResponse

/// GET /health → 200 with a JSON health payload.
let healthHandler: HttpHandler = fun next ctx -> json { Status = "ok" } next ctx

/// Composition of the health route. Bounded-context routes are added in later phases.
let webApp: HttpHandler =
    choose
        [ GET >=> route "/health" >=> healthHandler
          RequestErrors.NOT_FOUND "Not Found" ]
