module OseBe.WebApp

open Giraffe
open OseBe.Contexts.Messaging.Application

/// Composes the HTTP routes for every bounded context into a single handler,
/// bound to the shared messaging status surface.
let buildWebApp (status: SharedMessagingStatus) : HttpHandler =
    choose
        [ OseBe.Contexts.Health.Api.routes
          OseBe.Contexts.RegulatorySource.Api.routes
          OseBe.Contexts.InternalPolicy.Api.routes
          OseBe.Contexts.GapAnalysis.Api.routes
          OseBe.Contexts.AiOrchestration.Api.routes
          OseBe.Contexts.Messaging.Api.routes status
          RequestErrors.NOT_FOUND "Not Found" ]

/// Default composed handler with a fresh (pending) messaging status, used by
/// in-process unit tests of the routing surface.
let webApp: HttpHandler = buildWebApp (newShared ())
