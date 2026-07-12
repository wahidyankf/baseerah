# 11 · Backend Essentials (By Example, Python)

**prd row**: Pass 1 · Core Foundations · By Example · Python (PostgreSQL) · Learn 111 / Drill 211 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the **usable slice** — a real HTTP JSON service wired to a database, run and tested from
the CLI. Scale, deep auth, caching, and messaging are deferred to
[`39-backend-at-scale`](./39-backend-at-scale.md) (DD-11). HTTP fundamentals are introduced here (they
precede [topic 12 Networking](./12-networking-essentials.md) in the spiral).

## Why this exists · the big idea

- **The problem before the solution**: many clients need to share and change the same durable state over
  a network — that demands a server mediating access, not a local script.
- **Keep-this-if-you-forget-everything**: a backend is a stateless pipeline — receive, validate, persist,
  respond — with all the real state pushed down into the database.
- **Big ideas touched**: `taming-state` (HTTP is deliberately stateless so the hard state lives in one
  place, the DB); `layering-and-leaks` (request → handler → repository → store is a layering you keep clean).

## Prerequisites

- **Prior topics**: [topic 4 Just Enough Python](./04-just-enough-python.md) and
  [topic 10 SQL Essentials](./10-sql-essentials.md) (the service persists to a relational DB).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** in a `venv`; a pinned CVE-clean web
  framework (FastAPI/Flask) + `uvicorn`; **`curl`** to exercise endpoints; SQLite (from topic 10) or a
  local PostgreSQL for the persistence example.
- **Assumed knowledge**: reading/writing Python functions and modules; basic SQL queries and a
  parameterized query from Python (topic 10). No prior web-framework experience required.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28). Re-confirm version pins at authoring.

- 2026-07-12 — verified: **FastAPI 0.139.0**, **uvicorn 0.51.0**, **Flask 3.1.3** — all current/CVE-clean.
  HTTP method/status/statelessness semantics are standards-stable (RFC 9110, unchanged since 2022).
  (pypi.org)

## Items

- **HTTP fundamentals**: methods, status codes, headers, statelessness.
- **The vanilla tier first**: serve one route with the language's raw stdlib HTTP server (Python
  `http.server`/`wsgiref`) — read the request line and write a status line + headers by hand — so the
  framework's routing is understood before it's leaned on.
- **A minimal web framework** (FastAPI/Flask) run from the CLI (`uvicorn`): routing, handlers, JSON
  in/out.
- **Request validation** + structured error responses; serialization.
- **Persistence**: wire to the SQL-Essentials database; a repository-style access function; migration
  basics.
- **AuthN basics**: sessions vs tokens intro (deep authz → `it-security` / `backend-at-scale`).
- **Local dev loop**: serve via CLI, test with `curl`.

## Worked examples

Colocated under `backend-essentials/learning/code/`; each served via `uvicorn` and exercised with `curl`
(DD-20/DD-30).

- **beginner** — a minimal HTTP JSON endpoint served from the CLI, exercised with `curl`.
- **intermediate** — a CRUD resource backed by the SQL-Essentials DB; validation + error response.
- **advanced** — a token-check middleware; pagination + filtering on a list endpoint.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a small HTTP JSON API (a task or notes service) with full CRUD backed by the SQL DB,
  request validation, structured errors, a token-check middleware, and pagination — runnable via
  `uvicorn` and fully exercisable with `curl`.
- **Concepts exercised**: [ ] routing + handlers [ ] JSON in/out + validation [ ] structured error
  envelope [ ] repository-style DB access (parameterized) [ ] token-check middleware [ ] pagination +
  filtering.
- **Ordered steps**:
  1. `.../learning/capstone/code/app/` — the framework app + a `GET /health`. Verify
     `uvicorn app.main:app` serves and `curl localhost:8000/health` returns 200 JSON.
  2. CRUD endpoints backed by the DB with parameterized queries + validation. Verify `curl` create → read
     → update → delete round-trips and invalid bodies return a structured 4xx.
  3. A token-check middleware protecting writes. Verify a missing/invalid token returns 401.
  4. Pagination + filtering on the list endpoint. Verify `?limit=&offset=&filter=` behaves.
- **Acceptance criteria**: every endpoint returns correct status codes; writes require a valid token;
  invalid input yields structured errors; a `pytest` suite (or a documented `curl` script) passes.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **RESTful Web APIs** — Richardson, Amundsen, Ruby (2013). Definitive successor to "RESTful Web Services": resource design, hypermedia, API-description formats.
- **Building Microservices** — Sam Newman (2nd ed., 2021). Standard reference for decomposing backends into independently deployable, well-bounded services.
- **Release It!** — Michael Nygard (2nd ed., 2018). Canonical catalog of production-readiness patterns (circuit breaker, bulkhead, timeout).

**Papers & articles**

- **Architectural Styles and the Design of Network-based Software Architectures** — Roy T. Fielding (2000, dissertation). Introduces REST. <https://roy.gbiv.com/pubs/dissertation/top.htm>
- **RFC 9110: HTTP Semantics** — Fielding, Nottingham, Reschke, eds. (2022). Current IETF standard for HTTP methods, status codes, headers. <https://www.rfc-editor.org/rfc/rfc9110>

---

← Previous: [10 · SQL Essentials](./10-sql-essentials.md) · Next: [12 · Networking Essentials](./12-networking-essentials.md) →
