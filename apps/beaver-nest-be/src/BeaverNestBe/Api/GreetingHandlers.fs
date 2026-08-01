module BeaverNestBe.Api.GreetingHandlers

open Giraffe
open BeaverNestBe.Domain.Greeting

let greetingHandler: HttpHandler = fun next ctx -> json greeting next ctx
