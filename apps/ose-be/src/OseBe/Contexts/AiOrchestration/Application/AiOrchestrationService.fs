namespace OseBe.Contexts.AiOrchestration

open OseBe.Domain.Readiness
open OseBe.Contexts.AiOrchestration.Domain

/// Application use cases for the ai-orchestration bounded context.
module Application =

    /// Initializes the context boundary and reports its readiness.
    let initializeContext () : Readiness =
        { State = Ready
          Capability = Domain.Capability }
