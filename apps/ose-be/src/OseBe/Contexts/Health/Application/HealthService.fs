namespace OseBe.Contexts.Health

open OseBe.Contexts.Health.Domain

/// Application use cases for the health bounded context.
module Application =

    /// Returns the current health status of the application.
    ///
    /// Pure function — no I/O, no web dependency.
    let getHealth () : HealthStatus = { Status = "healthy" }
