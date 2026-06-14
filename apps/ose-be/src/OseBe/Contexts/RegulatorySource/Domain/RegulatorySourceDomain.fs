namespace OseBe.Contexts.RegulatorySource

open OseBe.Domain.Readiness

/// Domain types for the regulatory-source bounded context.
///
/// Ingests and stores regulator-published rule documents with provenance
/// metadata. Detailed domain types are added in the regulatory-source feature
/// plan.
module Domain =

    /// Re-export of the shared readiness state DU for this context.
    type ContextReadiness = OseBe.Domain.Readiness.ContextReadiness

    /// Re-export of the shared readiness record for this context.
    type Readiness = OseBe.Domain.Readiness.Readiness

    /// One-line capability advertised by this context.
    [<Literal>]
    let Capability = "accept regulatory documents"
