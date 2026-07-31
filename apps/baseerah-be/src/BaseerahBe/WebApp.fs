module BaseerahBe.WebApp

open Giraffe
open BaseerahBe.Domain.ErrorBody
open BaseerahBe.Api.HealthHandlers
open BaseerahBe.Api.GreetingHandlers

/// Single error-formatting function every non-2xx response goes through.
let private errorBody (message: string) : ErrorBody = { Error = message }

let private notFoundHandler: HttpHandler =
    setStatusCode 404 >=> json (errorBody "not found")

/// Composed HTTP handler. Routes are added incrementally as each Gherkin
/// scenario turns green; the 404 fallback is the last-resort route.
let webApp: HttpHandler =
    choose
        [ GET >=> route "/api/v1/health" >=> healthHandler
          GET >=> route "/api/v1/hello" >=> greetingHandler
          notFoundHandler ]
