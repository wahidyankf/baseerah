# 09 · Backend Essentials (By Example, Python)

**prd row**: Pass 1 · First Working Software · By Example · Python (PostgreSQL) · Learn 109 / Drill 209 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the **usable slice** — a real HTTP JSON service wired to a database, run and tested from
the CLI. Scale, deep auth, caching, and messaging are deferred to
[`28-backend-at-scale`](./28-backend-at-scale.md) (DD-11). HTTP fundamentals are introduced here (they
precede [topic 10 Networking](./10-networking-essentials.md) in the spiral).

## Prerequisites

- **Prior topics**: [topic 04 Just Enough Python](./04-just-enough-python.md) and
  [topic 08 SQL Essentials](./08-sql-essentials.md) (the service persists to a relational DB).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** in a `venv`; a pinned CVE-clean web
  framework (FastAPI/Flask) + `uvicorn`; **`curl`** to exercise endpoints; SQLite (from topic 08) or a
  local PostgreSQL for the persistence example.
- **Assumed knowledge**: reading/writing Python functions and modules; basic SQL queries and a
  parameterized query from Python (topic 08). No prior web-framework experience required.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28). Re-confirm version pins at authoring.

- 2026-07-12 — verified: **FastAPI 0.139.0**, **uvicorn 0.51.0**, **Flask 3.1.3** — all current/CVE-clean.
  HTTP method/status/statelessness semantics are standards-stable (RFC 9110, unchanged since 2022).
  (pypi.org)

## Items

- **HTTP fundamentals**: methods, status codes, headers, statelessness.
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

---

← Previous: [08 · SQL Essentials](./08-sql-essentials.md) · Next: [10 · Networking Essentials](./10-networking-essentials.md) →
