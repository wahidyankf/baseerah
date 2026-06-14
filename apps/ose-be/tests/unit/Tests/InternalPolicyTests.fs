module OseBe.Tests.Unit.Tests.InternalPolicyTests

open Xunit
open OseBe.Contexts.InternalPolicy.Application
open OseBe.Contexts.InternalPolicy.Domain

[<Fact>]
let ``internal-policy reports ready to accept internal policy documents`` () =
    let readiness = initializeContext ()
    Assert.Equal(ContextReadiness.Ready, readiness.State)
    Assert.Contains("internal policy", readiness.Capability)
