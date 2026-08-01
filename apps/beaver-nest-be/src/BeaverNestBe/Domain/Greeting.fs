module BaseerahBe.Domain.Greeting

/// The single source of the greeting text — referenced nowhere else in `src/`.
let private text = "Hello from Baseerah"

type Greeting = { Message: string }

let greeting: Greeting = { Message = text }
