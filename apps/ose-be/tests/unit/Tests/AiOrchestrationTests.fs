module OseBe.Tests.Unit.Tests.AiOrchestrationTests

open Xunit
open OseBe.Contexts.AiOrchestration.Application
open OseBe.Contexts.AiOrchestration.Domain

[<Fact>]
let ``ai-orchestration reports ready to wrap LLM calls via OpenRouter`` () =
    let readiness = initializeContext ()
    Assert.Equal(ContextReadiness.Ready, readiness.State)
    Assert.Contains("OpenRouter", readiness.Capability)
