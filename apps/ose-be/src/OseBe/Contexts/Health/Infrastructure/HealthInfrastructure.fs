namespace OseBe.Contexts.Health

/// Infrastructure layer for the health bounded context.
///
/// The health context is stateless and has no infrastructure adapters (no
/// database, no external service). This module exists to anchor the layer in
/// the hexagonal slice.
module Infrastructure =

    /// Marker indicating the health context has no infrastructure dependencies.
    let hasInfrastructureDependencies = false
