module BeaverNestBe.Domain.Readiness

/// The single liveness payload shape, serialised once at the health handler.
type Liveness = { Status: string }

/// Closed, provider-independent result passed from the application boundary to
/// HTTP. It intentionally carries no file, SQL, exception, or provider detail.
type ReadinessResult =
    | Ready
    | Unavailable

type ReadyResponse =
    { Status: string
      Database: string
      Schema: string }

type UnreadyResponse = { Status: string }

let ok: Liveness = { Status = "ok" }

let readyResponse: ReadyResponse =
    { Status = "ready"
      Database = "ready"
      Schema = "current" }

let unavailableResponse: UnreadyResponse = { Status = "not-ready" }

/// Keeps migration comparison independent of HTTP and database providers.
let schemaState expectedScripts recordedScripts =
    if Set.ofList expectedScripts = Set.ofList recordedScripts then
        "current"
    else
        "pending"
