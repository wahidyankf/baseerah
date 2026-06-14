namespace OseBe.Contexts.RegulatorySource

open OseBe.Domain.Readiness
open OseBe.Contexts.RegulatorySource.Domain

/// Application use cases for the regulatory-source bounded context.
module Application =

    /// Initializes the context boundary and reports its readiness.
    let initializeContext () : Readiness =
        { State = Ready
          Capability = Domain.Capability }
