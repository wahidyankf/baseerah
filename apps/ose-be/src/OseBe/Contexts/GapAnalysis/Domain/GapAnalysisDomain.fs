namespace OseBe.Contexts.GapAnalysis

open OseBe.Domain.Readiness

/// Domain types for the gap-analysis bounded context.
///
/// Compares a regulatory-source corpus against an internal-policy corpus and
/// emits GapItem records, orchestrating ai-orchestration for LLM-assisted
/// comparison. Detailed domain types are added in the gap-analysis feature plan.
module Domain =

    /// Re-export of the shared readiness state DU for this context.
    type ContextReadiness = OseBe.Domain.Readiness.ContextReadiness

    /// Re-export of the shared readiness record for this context.
    type Readiness = OseBe.Domain.Readiness.Readiness

    /// One-line capability advertised by this context.
    [<Literal>]
    let Capability = "compare regulatory and policy documents"
