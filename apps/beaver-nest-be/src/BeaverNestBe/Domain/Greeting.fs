module BeaverNestBe.Domain.Greeting

/// The single source of the greeting text — referenced nowhere else in `src/`.
let private text = "Hello from BeaverNest"

type Greeting = { Message: string }

let greeting: Greeting = { Message = text }
