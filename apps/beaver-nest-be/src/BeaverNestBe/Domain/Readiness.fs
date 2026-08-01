module BaseerahBe.Domain.Readiness

/// The single liveness payload shape, serialised once at the health handler.
type Readiness = { Status: string }

let ok: Readiness = { Status = "ok" }
