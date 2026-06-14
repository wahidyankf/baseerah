namespace OrganicleverBe.Contexts.Messaging

/// Domain types for the messaging bounded context.
///
/// Models the outcome of the JetStream durable demo run at startup. JetStream is
/// the NATS persistent message-streaming subsystem; a durable consumer remembers
/// its delivery position across restarts.
module Domain =

    /// Outcome of the JetStream durable demo run.
    type JetStreamDemoOutcome =
        /// The demo has not yet run.
        | Pending
        /// The demo message was delivered and acknowledged.
        | DeliveredAndAcked
        /// The demo failed with a diagnostic message.
        | Failed of reason: string

    /// Renders the demo outcome as the wire-format status string.
    let outcomeToString (outcome: JetStreamDemoOutcome) : string =
        match outcome with
        | Pending -> "pending"
        | DeliveredAndAcked -> "delivered_and_acked"
        | Failed reason -> sprintf "failed: %s" reason
