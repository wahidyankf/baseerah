module BaseerahBe.Api.HealthHandlers

open Giraffe
open BaseerahBe.Domain.Readiness

let healthHandler: HttpHandler = fun next ctx -> json ok next ctx
