module BaseerahBe.WebApp

open Giraffe
open BaseerahBe.Domain.ErrorBody
open BaseerahBe.Api.HealthHandlers
open BaseerahBe.Api.GreetingHandlers

/// Single error-formatting function every non-2xx response goes through.
let private errorBody (message: string) : ErrorBody = { Error = message }

let private notFoundHandler: HttpHandler =
    setStatusCode 404 >=> json (errorBody "not found")

/// Applied to every response, 2xx and 4xx alike — passive security hygiene per
/// Rule-16 finding AET-001 (the `Server` header itself is suppressed separately
/// in Program.fs, at the Kestrel level, since a Giraffe HttpHandler runs inside
/// the pipeline after Kestrel has already decided that header).
let private securityHeaders: HttpHandler =
    setHttpHeader "X-Content-Type-Options" "nosniff"

/// Composed HTTP handler. Routes are added incrementally as each Gherkin
/// scenario turns green; the 404 fallback is the last-resort route.
let webApp: HttpHandler =
    securityHeaders
    >=> choose
            [ GET >=> route "/api/v1/health" >=> healthHandler
              GET >=> route "/api/v1/hello" >=> greetingHandler
              notFoundHandler ]
