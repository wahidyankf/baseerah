module OseAppBe.Contexts.Health.Api.Http.Handlers

open Giraffe
open OseAppBe.Contexts.Health.Application.UseCases

let handle: HttpHandler = fun next ctx -> json (getHealth ()) next ctx
