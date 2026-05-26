module OseAppBe.Contexts.Health.Application.UseCases

open OseAppBe.Contexts.Health.Domain.Types

let getHealth () : HealthStatus = { Status = "healthy" }
