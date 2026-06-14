namespace OseBe.Contexts.AiOrchestration

open OseBe.Domain.Readiness

/// Domain types for the ai-orchestration bounded context.
///
/// Wraps LLM calls (OpenRouter), prompt management, and token-budget accounting.
/// Detailed domain types are added in the ai-orchestration feature plan; for now
/// the context advertises its readiness via the shared readiness vocabulary.
module Domain =

    /// Re-export of the shared readiness state DU for this context.
    type ContextReadiness = OseBe.Domain.Readiness.ContextReadiness

    /// Re-export of the shared readiness record for this context.
    type Readiness = OseBe.Domain.Readiness.Readiness

    /// One-line capability advertised by this context.
    [<Literal>]
    let Capability = "wrap LLM calls via OpenRouter"
