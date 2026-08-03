module BeaverNestBe.WebApp

open Giraffe
open BeaverNestBe.Domain.ErrorBody
open BeaverNestBe.Api.HealthHandlers
open BeaverNestBe.Api.GreetingHandlers
open BeaverNestBe.Api.ReadinessHandlers
open BeaverNestBe.Application.ReadinessPort

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
let webAppWith (readiness: ReadinessPort) : HttpHandler =
    securityHeaders
    >=> choose
            [ GET >=> route "/api/v1/health" >=> healthHandler
              GET >=> route "/api/v1/readiness" >=> readinessHandler readiness
              GET >=> route "/api/v1/hello" >=> greetingHandler
              notFoundHandler ]

/// Default composition keeps the existing in-process handler tests focused on
/// routing; the executable injects its real database-backed readiness port.
let webApp: HttpHandler = webAppWith alwaysReady
