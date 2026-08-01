module BaseerahBe.Api.GreetingHandlers

open Giraffe
open BaseerahBe.Domain.Greeting

let greetingHandler: HttpHandler = fun next ctx -> json greeting next ctx
