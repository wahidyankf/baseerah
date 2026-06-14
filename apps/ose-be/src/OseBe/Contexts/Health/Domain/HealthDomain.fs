namespace OseBe.Contexts.Health

/// Domain types for the health bounded context.
module Domain =

    /// Represents the liveness health status of the application.
    type HealthStatus = { Status: string }
