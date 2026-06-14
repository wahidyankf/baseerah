namespace OseBe.Contexts.InternalPolicy

open OseBe.Domain.Readiness

/// Domain types for the internal-policy bounded context.
///
/// Ingests and stores company-internal documents (SOPs, manuals, procedures)
/// with version and scope metadata. Detailed domain types are added in the
/// internal-policy feature plan.
module Domain =

    /// Re-export of the shared readiness state DU for this context.
    type ContextReadiness = OseBe.Domain.Readiness.ContextReadiness

    /// Re-export of the shared readiness record for this context.
    type Readiness = OseBe.Domain.Readiness.Readiness

    /// One-line capability advertised by this context.
    [<Literal>]
    let Capability = "accept internal policy documents"
