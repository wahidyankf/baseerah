# 41 · API Design (By Example, Python †)

**prd row**: Pass 3 · Build for the Real World · By Example · Python † · Learn 141 / Drill 241 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: designing the contract, not just the endpoint — REST vs GraphQL vs gRPC and when
each fits, OpenAPI as the machine-readable contract, versioning, pagination, idempotency, error
envelopes, and rate limiting. The through-line is that an API is a promise to callers you don't
control, so its shape and its failure modes are the design. Builds on the serving mechanics of
[`40-build-your-own-web-framework`](./40-build-your-own-web-framework.md). `†`: Python, fully
type-annotated (DD-34) — every snippet carries type hints in the mypy-clean spirit.

## Why this exists · the big idea

- **The problem before the solution**: an endpoint that works is not an API — the moment a second
  team, a mobile app, or a paying customer depends on it, every field name, status code, and
  pagination quirk becomes a promise you can't quietly break. Ad-hoc APIs turn every change into a
  coordinated migration and every outage into a guessing game about what the response _should_ be.
- **Keep-this-if-you-forget-everything**: design the contract first and design for the caller you'll
  never meet — a stable, versioned, self-describing contract with predictable errors and idempotent
  writes is what lets clients evolve independently of your server.
- **Big ideas touched**: `coupling-vs-cohesion` (a good contract decouples client from server so each
  changes on its own schedule; a leaky one couples every consumer to your internals),
  `consistency-latency-throughput` (pagination, rate limiting, and the REST/GraphQL/gRPC choice are
  all throughput-and-latency decisions dressed as API style).

## Prerequisites

- **Prior topics**: [topic 11 Backend Essentials](./11-backend-essentials.md) (HTTP verbs, status
  codes, routing, JSON handling) and [topic 39 Backend at Scale](./39-backend-at-scale.md)
  (idempotency, auth, and rate limiting as production concerns).
- **Tools & environment**: a macOS/Linux terminal; **Python** at a recent stable release with type
  hints and `mypy`; a web framework you can serve locally; `curl`/an HTTP client; an OpenAPI toolchain
  (spec validator + a mock/codegen); optionally a gRPC/Protobuf and a GraphQL toolchain for the
  contrast tiers; Neovim/VSCode with the Python LSP (DD-17).
- **Assumed knowledge**: serving a CRUD JSON API (topic 11); what idempotency and rate limiting buy
  you (topic 39); reading and writing typed request/response models (topic 04).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: **OpenAPI** remains the dominant REST contract format and **Protocol Buffers /
  gRPC** and the **GraphQL** specification remain the standard non-REST contrasts — all left correctly
  version-unpinned; the RFC 9110 HTTP semantics (methods, status codes) that REST builds on are
  current.
- 2026-07-12 — verified (GAP for plan owner): specific OpenAPI version (3.0 vs 3.1) and the JSON-Schema
  alignment differ between tooling generations — pin the concrete OpenAPI version and validator when
  drafting the examples. Error-envelope guidance is described generically (a stable, documented error
  shape) rather than mandating RFC 9457/Problem Details, which is a defensible-but-optional choice to
  confirm at drafting.

## Items

- The contract mindset: an API as a promise to callers you don't control; consumer-driven thinking.
- Style selection: REST (resource-oriented, cacheable) vs GraphQL (client-shaped queries) vs gRPC
  (typed, streaming, internal) — the forces that pick each, not a winner.
- OpenAPI as the source of truth: describing the contract, then generating docs/clients/mocks and
  validating requests against it.
- Evolution without breakage: versioning strategies (URL vs header vs additive), deprecation, and
  backward-compatible change rules.
- Robust request/response design: pagination (offset vs cursor), idempotency keys for safe retries,
  and a consistent, documented error envelope.
- Operability of the contract: rate limiting, quotas, and communicating limits (headers, 429) as part
  of the API surface.

## Tensions & trade-offs — when NOT to reach for this

- **GraphQL/gRPC are not upgrades**: GraphQL's client-shaped queries trade a simple cacheable surface
  for query-complexity, N+1, and caching problems you now own; gRPC trades human-readable, browser-
  native calls for typed performance and a Protobuf toolchain. A public, cacheable, human-debuggable
  API is usually still REST — reach for the alternatives only when their specific force applies.
- **Over-versioning is its own tax**: a new version per change forces callers to migrate constantly
  and multiplies the surface you maintain. Most changes should be additive and backward-compatible;
  a new version is for the rare truly-breaking change, not for every field.
- **Idempotency and rate limiting cost complexity**: idempotency keys need storage and dedup logic;
  rate limiting needs counters and a fairness policy. On a low-traffic internal API with trusted
  callers, both can be premature — add each when a real retry-storm or abuse pattern demands it.

## Lineage — why it beat the alternative

- API design consolidated as web services replaced hand-rolled RPC and SOAP: Fielding's REST
  dissertation gave resource-orientation and HTTP-native semantics a theory, the Richardson Maturity
  Model gave teams a ladder to judge how RESTful an API really was, and OpenAPI (from Swagger) turned
  the contract into a machine-readable artifact that generates docs, clients, and mocks. GraphQL and
  gRPC then carved out the cases REST fits worst — client-shaped aggregation and typed internal
  streaming. The durable lesson is contract-first design: the contract, not the code, is the product.
  This hands stable interfaces up to [topic 42 Software Architecture](./42-software-architecture.md)
  and its scaling/failure concerns back to [topic 39 Backend at Scale](./39-backend-at-scale.md).

## Worked examples

Colocated under `api-design/learning/code/`; each runnable and exercised with `curl`/a client, every
Python snippet fully type-annotated and `mypy`-clean (DD-20/DD-30/DD-34).

- **beginner** — design a small REST resource with a documented error envelope and cursor pagination;
  write its OpenAPI spec and validate a live response against it.
- **intermediate** — add idempotency-key handling to a write endpoint and rate limiting with 429 +
  limit headers; prove a replayed write and an over-limit caller behave correctly.
- **advanced** — expose the same resource through GraphQL and a gRPC/Protobuf service and contrast the
  three contracts on caching, over/under-fetching, and evolution.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: design and serve a versioned REST API for one resource, contract-first from an OpenAPI
  spec — with cursor pagination, idempotent writes, a consistent error envelope, and rate limiting —
  then add a GraphQL or gRPC facade over the same data and document when each contract wins.
- **Concepts exercised**: [ ] OpenAPI contract-first [ ] versioning + backward-compatible change
  [ ] cursor pagination [ ] idempotency keys [ ] error envelope [ ] rate limiting (429 + headers).
- **Ordered steps**:
  1. `.../learning/capstone/openapi.yaml` — the contract for a versioned resource with pagination and
     the error envelope. Verify the spec validates and a mock server serves it.
  2. `.../learning/capstone/code/rest.py` — implement the spec; add idempotency-key handling. Verify a
     replayed write with the same key does not double-apply (`curl`) and responses match the spec.
  3. `.../learning/capstone/code/limits.py` — add rate limiting. Verify an over-limit caller gets 429
     with correct limit/remaining headers and a compliant caller succeeds.
  4. `.../learning/capstone/code/facade/` — expose the same data via GraphQL or gRPC and write a short
     contrast note. Verify the facade returns equivalent data and the note names when each style wins.
- **Acceptance criteria**: live responses conform to the OpenAPI contract; idempotent writes and rate
  limiting behave; the second-style facade returns equivalent data; the contrast note is concrete; all
  Python is type-annotated and `mypy`-clean.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **RESTful Web APIs** — Leonard Richardson, Mike Amundsen, Sam Ruby (2013). The standard reference
  for hypermedia-driven, resource-oriented API design.
- **Designing Web APIs** — Brenda Jin, Saurabh Sahni, Amir Shevat (2018). Widely used, product-oriented
  guide to API design decisions (versioning, pagination, developer experience).
- **REST in Practice** — Jim Webber, Savas Parastatidis, Ian Robinson (2010). Connects REST theory to
  hypermedia controls and enterprise integration patterns.
- **gRPC: Up and Running** — Kasun Indrasiri, Danesh Kuruppu (2020). The standard introductory book for
  gRPC service design and Protocol Buffers contracts.

**Papers & articles**

- **Architectural Styles and the Design of Network-based Software Architectures** — Roy T. Fielding
  (2000). The doctoral dissertation that defines REST, the theoretical basis of nearly all modern HTTP
  API design. <https://www.ics.uci.edu/~fielding/pubs/dissertation/top.htm>
- **Richardson Maturity Model** — Martin Fowler, with Leonard Richardson (2010). The canonical
  explanation of the four-level model used to gauge how "RESTful" an API actually is.
  <https://martinfowler.com/articles/richardsonMaturityModel.html>
- **Hypertext Transfer Protocol (HTTP/1.1): Semantics and Content (RFC 7231)** — R. Fielding,
  J. Reschke, eds. (2014). The normative IETF specification defining HTTP methods, status codes, and
  content negotiation that all REST APIs build on. <https://www.rfc-editor.org/rfc/rfc7231>
- **GraphQL Specification** — GraphQL Working Group, Joint Development Foundation (ongoing). The
  official language and execution specification behind GraphQL API design. <https://spec.graphql.org/>

---

← Previous: [40 · Build Your Own Web Framework](./40-build-your-own-web-framework.md) · Next: [42 · Software Architecture](./42-software-architecture.md) →
