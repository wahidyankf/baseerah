module CraneBe.Tests.Integration.Steps.BddState

/// Shared mutable state for BDD integration scenario steps.
let mutable LastStatus: int = 0
let mutable LastBody: string = ""
