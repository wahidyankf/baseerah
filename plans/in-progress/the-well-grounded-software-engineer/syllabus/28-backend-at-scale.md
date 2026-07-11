# 28 · Backend at Scale (By Example, Python)

**prd row**: Pass 3 · Build for the Real World · By Example · Python · Learn 128 / Drill 228 · Nvim-ready
Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the deep backend pass — API design (REST/GraphQL/gRPC), persistence patterns, deep
AuthN/Z, reliability (logging/rate-limiting/caching), async/messaging, and applied integration/contract
testing. The usable slice is the prerequisite [`09-backend-essentials`](./09-backend-essentials.md);
system-level scaling is [`32-system-design`](./32-system-design.md).

## Prerequisites

- **Prior topics**: [topic 09 Backend Essentials](./09-backend-essentials.md) (routing, validation, DB
  access), [topic 08 SQL Essentials](./08-sql-essentials.md), [topic 14 Security Essentials](./14-security-essentials.md)
  (auth to deepen), and [topic 13 Software Testing](./13-software-testing.md) (integration/contract tests).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** with a pinned CVE-clean web framework;
  a local SQL DB + a queue/broker (Valkey/Redis stream fine); `curl`; a contract-test tool (Pact) and
  test-containers concept.
- **Assumed knowledge**: building/serving a CRUD JSON API (topic 09); tokens vs sessions (topic 14);
  writing an integration test (topic 13).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified (CORRECTION/UPDATE): the authoritative OAuth security source is **RFC 9700
  "Best Current Practice for OAuth 2.0 Security"** (IETF, Jan 2025) — deprecates the Implicit Grant and
  ROPC Grant, mandates PKCE for public clients. **OAuth 2.1 is still an IETF draft** (draft-ietf-oauth-v2-1-15,
  March 2026) — NOT a finalized RFC as of July 2026. Cite OAuth 2.0 + RFC 9700 as the settled standard and
  describe OAuth 2.1 as "in-progress consolidation," not a finalized spec. (datatracker.ietf.org/doc/rfc9700)
- 2026-07-12 — verified: gRPC/GraphQL and Pact tooling have no concrete version claim in the body yet —
  re-verify specific tool versions/commands once drafted.

## Items

- API design deep: REST resource modeling, versioning, pagination, idempotency; GraphQL & gRPC contrast.
- Persistence patterns: repository, unit of work, transactions across boundaries, migrations at scale.
- AuthN/Z deep: JWT, OAuth2 vs OIDC, RBAC/ABAC, refresh tokens.
- Reliability: error handling, structured logging, config/secrets, health checks, rate limiting, caching.
- Async & messaging: queues, background jobs, webhooks, idempotent consumers, backpressure.
- Applied testing: integration tests, contract tests, test containers, from the CLI.

## Worked examples

Colocated under `backend-at-scale/learning/code/`; each runnable + exercised from the CLI (DD-20/DD-30).

- **beginner** — an idempotent write endpoint; a structured error envelope + logging.
- **intermediate** — a caching + rate-limit layer; OAuth2/RBAC-protected routes.
- **advanced** — a background-job / queue consumer with idempotency; an integration suite against a real DB.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: evolve the Backend-Essentials service into a scale-ready API: versioned + paginated REST with
  idempotent writes, OAuth2/OIDC + RBAC auth, structured logging + rate limiting + caching, and a
  background-job queue consumer with idempotency — verified by an integration + contract test suite.
- **Concepts exercised**: [ ] versioned/paginated REST + idempotency keys [ ] OAuth2/OIDC + RBAC
  [ ] repository/unit-of-work persistence [ ] structured logging + rate limit + cache [ ] a queue consumer
  with idempotent processing [ ] integration + contract tests.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — versioned REST with pagination + idempotency-key handling. Verify a
     replayed write with the same key does not double-apply (`curl`).
  2. Add OAuth2/OIDC + RBAC. Verify a role-restricted route returns 403 for the wrong role and 200 for the
     right one.
  3. Add structured logging + rate limiting + a cache layer. Verify logs are structured, the rate limit
     returns 429, and a cached read avoids a DB hit.
  4. Add a queue consumer for a background job with a dedup key + an integration/contract test suite.
     Verify duplicate messages process once and the suite passes.
- **Acceptance criteria**: idempotent writes and consumers behave correctly; auth/RBAC gates work; rate
  limit + cache observable; integration + contract tests green.
- **Done bar**: runnable end-to-end + web-verified.

---

← Previous: [27 · Graph Databases](./27-graph-databases.md) · Next: [29 · Advanced Frontend](./29-advanced-frontend.md) →
