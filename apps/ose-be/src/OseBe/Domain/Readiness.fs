namespace OseBe.Domain

/// Shared domain vocabulary for bounded-context readiness.
///
/// The OSE backend exposes several bounded contexts whose detailed feature work
/// is deferred to per-context feature plans. Until then each context advertises
/// a readiness state and a one-line capability description, so the context
/// boundary is established and independently testable. This is the only domain
/// type shared across contexts (no other cross-context coupling is permitted).
module Readiness =

    /// Lifecycle state of a bounded context boundary.
    type ContextReadiness =
        /// The context boundary is established and ready to accept work.
        | Ready
        /// The context boundary is declared but not yet initialized.
        | Pending

    /// A bounded context's advertised readiness: its lifecycle state plus a
    /// human-readable capability summary.
    type Readiness =
        { State: ContextReadiness
          Capability: string }
