namespace OseBe.Contexts.GapAnalysis

open OseBe.Domain.Readiness
open OseBe.Contexts.GapAnalysis.Domain

/// Application use cases for the gap-analysis bounded context.
module Application =

    /// Initializes the context boundary and reports its readiness.
    let initializeContext () : Readiness =
        { State = Ready
          Capability = Domain.Capability }
