namespace OseBe.Contexts.InternalPolicy

open OseBe.Domain.Readiness
open OseBe.Contexts.InternalPolicy.Domain

/// Application use cases for the internal-policy bounded context.
module Application =

    /// Initializes the context boundary and reports its readiness.
    let initializeContext () : Readiness =
        { State = Ready
          Capability = Domain.Capability }
