module OseBe.Tests.Unit.Steps.ContextReadinessSteps

open TickSpec
open Xunit
open OseBe.Domain.Readiness
open OseBe.Tests.Unit.Steps.BddState

[<When>]
let ``the ai-orchestration bounded context is initialized`` () =
    let r = OseBe.Contexts.AiOrchestration.Application.initializeContext ()
    Assert.Equal(Ready, r.State)
    LastCapability <- r.Capability

[<When>]
let ``the gap-analysis bounded context is initialized`` () =
    let r = OseBe.Contexts.GapAnalysis.Application.initializeContext ()
    Assert.Equal(Ready, r.State)
    LastCapability <- r.Capability

[<When>]
let ``the internal-policy bounded context is initialized`` () =
    let r = OseBe.Contexts.InternalPolicy.Application.initializeContext ()
    Assert.Equal(Ready, r.State)
    LastCapability <- r.Capability

[<When>]
let ``the regulatory-source bounded context is initialized`` () =
    let r = OseBe.Contexts.RegulatorySource.Application.initializeContext ()
    Assert.Equal(Ready, r.State)
    LastCapability <- r.Capability

[<Then>]
let ``the context is ready to wrap LLM calls via OpenRouter`` () =
    Assert.Contains("OpenRouter", LastCapability)

[<Then>]
let ``the context is ready to compare regulatory and policy documents`` () =
    Assert.Contains("compare", LastCapability)

[<Then>]
let ``the context is ready to accept internal policy documents`` () =
    Assert.Contains("internal policy", LastCapability)

[<Then>]
let ``the context is ready to accept regulatory documents`` () =
    Assert.Contains("regulatory", LastCapability)
