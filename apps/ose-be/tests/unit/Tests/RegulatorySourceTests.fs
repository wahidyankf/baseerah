module OseBe.Tests.Unit.Tests.RegulatorySourceTests

open Xunit
open OseBe.Contexts.RegulatorySource.Application
open OseBe.Contexts.RegulatorySource.Domain

[<Fact>]
let ``regulatory-source reports ready to accept regulatory documents`` () =
    let readiness = initializeContext ()
    Assert.Equal(ContextReadiness.Ready, readiness.State)
    Assert.Contains("regulatory", readiness.Capability)
