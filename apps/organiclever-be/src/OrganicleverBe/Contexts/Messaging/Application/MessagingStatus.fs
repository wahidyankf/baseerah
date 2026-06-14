namespace OrganicleverBe.Contexts.Messaging

open OrganicleverBe.Contexts.Messaging.Domain

/// Application layer for the messaging bounded context: the messaging status
/// surface that records the JetStream demo outcome.
module Application =

    /// Shared, mutable holder for the messaging status, read by the status
    /// endpoint and written by the startup demo task.
    type SharedMessagingStatus() =
        let mutable outcome: JetStreamDemoOutcome = Pending
        let gate = obj ()

        /// Records the latest JetStream demo outcome.
        member _.Set(value: JetStreamDemoOutcome) = lock gate (fun () -> outcome <- value)

        /// Reads the current JetStream demo outcome.
        member _.Get() = lock gate (fun () -> outcome)

    /// Creates a new shared messaging status initialized to Pending.
    let newShared () = SharedMessagingStatus()
