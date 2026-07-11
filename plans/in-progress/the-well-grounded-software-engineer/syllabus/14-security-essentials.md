# 14 · Security Essentials (By Example, Python)

**prd row**: Pass 1 · First Working Software · By Example · Python · Learn 114 / Drill 214 · Nvim-ready
Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the **usable slice** — the everyday security a developer applies to the software they just
learned to build: the OWASP-style top risks, safe input handling, auth done right, secret hygiene, and
dependency safety. Threat modeling, cryptographic depth, and org-scale controls go to
[`38-it-security`](./38-it-security.md) and [`40-defensive-security`](./40-defensive-security.md) (DD-11).
This topic closes Pass 1 and anchors two inter-topic capstones.

## Prerequisites

- **Prior topics**: [topic 04 Just Enough Python](./04-just-enough-python.md),
  [topic 08 SQL Essentials](./08-sql-essentials.md) (SQL-injection examples), and
  [topic 09 Backend Essentials](./09-backend-essentials.md) — the HTTP service you built there is the
  application these attacks/defenses target.
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x**; the Backend-Essentials app + its DB;
  **`curl`** to send malicious/edge requests; a pinned CVE-clean password-hash library (**argon2**/bcrypt);
  a dependency-audit CLI (**`pip-audit`**).
- **Assumed knowledge**: reading/writing Python; issuing HTTP requests with `curl`; basic SQL and a
  parameterized query (topic 08); how a request reaches a handler (topic 09).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified (CORRECTION): **OWASP Top 10:2025** is now current (published Jan 2026, supersedes
  2021). Order: A01 Broken Access Control, A02 Security Misconfiguration, A03 **Software Supply Chain
  Failures (new)**, A04 Cryptographic Failures, A05 Injection, A06 Insecure Design, A07 Authentication
  Failures, A08 Software/Data Integrity Failures, A09 Security Logging & Alerting Failures, A10
  **Mishandling of Exceptional Conditions (new, replaces SSRF)**. Author against 2025 wording/order.
  (owasp.org Top10/2025)
- 2026-07-12 — verified: Argon2id min-tier params `m=19456 (19 MiB), t=2, p=1`; bcrypt work factor min 10
  (as high as perf allows), hard 72-byte input limit. `pip-audit` **2.10.0** (reads requirements/
  pyproject/venv against PyPA Advisory DB + OSV, needs Python ≥3.10). Parameterized-query guidance
  unchanged. (cheatsheetseries.owasp.org / pypi.org)

## Items

- **Threat basics**: the OWASP-style top risks at a working-developer level; thinking like an attacker.
- **Injection**: SQL injection shown live against a naive query, then fixed with parameterized queries;
  command injection.
- **Input validation & output encoding**: XSS intro; trust boundaries; allow-list validation.
- **Authentication done right**: password hashing (argon2/bcrypt, never plaintext/MD5), session vs token,
  secure cookie flags.
- **Authorization**: least privilege; the difference from authentication.
- **Secret hygiene**: env vars over hardcoding, never commit secrets, rotation intro (ties to repo policy).
- **Transport & headers**: HTTPS/TLS in practice; key security headers.
- **Dependency safety**: `pip-audit`; pinning; the CVE-clean supply-chain stance (DD-23) applied.

## Worked examples

Colocated under `security-essentials/learning/code/`; each attack + fix runnable against the
Backend-Essentials app with `curl` (DD-20/DD-30).

- **beginner** — SQL injection against a naive endpoint, then the parameterized fix; show both requests.
- **intermediate** — password storage: crack a plaintext/MD5 store, then argon2/bcrypt with correct params.
- **advanced** — an allow-list validation + output-encoding pass on a form; `pip-audit` a dependency set
  and remediate a flagged CVE.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: take the Backend-Essentials API and harden it end to end — fix an injectable query, add
  argon2/bcrypt password auth with secure session/token handling, allow-list input validation, secret
  hygiene via env vars, security headers, and a clean `pip-audit` — with a before/after attack transcript.
- **Concepts exercised**: [ ] parameterized queries (injection fix) [ ] argon2/bcrypt password hashing
  [ ] session/token + secure cookie flags [ ] allow-list validation + output encoding [ ] secrets in env,
  not code [ ] security headers [ ] `pip-audit` clean.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — copy the Backend-Essentials app; script an injection attack with
     `curl` that succeeds. Verify the exploit works, then parameterize the query. Verify the exploit fails.
  2. Add argon2/bcrypt-backed registration/login; store only hashes. Verify a login works and the DB holds
     no plaintext.
  3. Add allow-list validation + output encoding + security headers. Verify malformed/hostile inputs are
     rejected and headers are present (`curl -I`).
  4. Move secrets to env vars; run `pip-audit`. Verify no secret is in the tree and `pip-audit` is clean.
- **Acceptance criteria**: every attack in the transcript fails after hardening; passwords are hashed;
  no secret is committed; `pip-audit` exits clean; the app still serves all endpoints.
- **Done bar**: runnable end-to-end (attack transcript flips red→green) + web-verified.

<!-- Inter-topic capstone spec block: this file anchors two milestone bundles -->

## Capstone spec — inter-topic: capstone-first-working-software (Pass-1 boundary)

- **Weight**: `capstone-first-working-software/_index.md` = **245** (section root, after Pass 1). Kind:
  **pass-boundary**, integrating Pass 1 topics 04–14 (build → store → test → secure).
- **Goal**: ship one small but **complete, secure, tested working application** that a reader builds by
  integrating everything in Pass 1: a Python HTTP JSON service (topic 09) over a normalized SQL database
  (topic 08), driven by clean Python (04) with a Bash run/setup script (05), sound data structures (06)
  and an OO domain model (07), a full test suite across the pyramid (13), and the security hardening from
  this topic (14). Networking (10) and the TS/frontend slice (11/12) appear as the client/consumer side.
- **Concepts integrated**: [ ] HTTP JSON API + validation (09) [ ] normalized DB + parameterized DAL (08)
  [ ] domain model / OOP (07) [ ] apt data structures & algorithms (06) [ ] Bash setup/run script (05)
  [ ] pytest + Hypothesis + integration tests (13) [ ] security hardening: hashed auth, injection-safe,
  secrets in env, `pip-audit` clean (14) [ ] a `curl`/HTTP client walkthrough (10).
- **Ordered steps**:
  1. `capstone-first-working-software/code/` — scaffold the service (09) + schema/migrations (08) + a
     `setup.sh` run script (05). Verify `./setup.sh` boots the app and `curl /health` returns 200.
  2. Implement the domain model (07) + core CRUD with parameterized DAL (06/08). Verify `curl` round-trips
     every resource and invalid input yields structured errors.
  3. Add auth (argon2/bcrypt) + input validation + security headers + env secrets (14). Verify the Pass-1
     attack transcript fails and `pip-audit` is clean.
  4. Build the test suite (13): unit (pytest/Vitest where applicable), a Hypothesis property test, and an
     integration test. Verify all green and coverage is generated.
- **Acceptance criteria**: a reader on a clean machine runs `./setup.sh`, exercises every endpoint with
  `curl`, passes the full test suite, and confirms the app is injection-safe with hashed auth and no
  committed secrets — end to end, no hidden setup.
- **Done bar**: runnable end-to-end (clean-machine reproduction) + web-verified.

## Capstone spec — inter-topic: capstone-full-stack-app (cross-cutting)

- **Weight**: `capstone-full-stack-app/_index.md` = **246** (section root, immediately after
  first-working-software). Kind: **cross-cutting**, integrating Frontend (12) + Backend (09) + SQL (08).
- **Goal**: connect a typed **frontend** (topic 12) to the **backend** (09) over **HTTP** (10), persisted
  in **SQL** (08), so the reader sees a full vertical slice: an accessible UI that reads and writes real
  data through a real API into a real database — the "it actually works, top to bottom" moment.
- **Concepts integrated**: [ ] typed UI with loading/error/empty states (12/11) [ ] `fetch` to the API
  over HTTP (10) [ ] backend endpoints + validation (09) [ ] SQL persistence (08) [ ] end-to-end request
  path narrated (10) [ ] a Testing-Library UI test + an API integration test (13).
- **Ordered steps**:
  1. `capstone-full-stack-app/code/backend/` — reuse the hardened service (09/14) with a CORS-safe read
     endpoint. Verify `curl` returns JSON from the DB.
  2. `capstone-full-stack-app/code/frontend/` — a typed UI (12) that fetches and renders the list with
     loading/error/empty states. Verify the UI shows live data and each state is reachable.
  3. Wire a create/update form (12) posting to the API (09). Verify a UI action persists to the DB and the
     list reflects it after refetch.
  4. Add a Testing-Library test for the UI and an integration test for the endpoint (13). Verify both green.
- **Acceptance criteria**: a reader runs the backend + frontend, performs a create/read/update from the UI,
  confirms the change landed in the SQL DB, and both the UI test and API test pass — the whole stack works
  together.
- **Done bar**: runnable end-to-end (full vertical slice) + web-verified.

---

← Previous: [13 · Software Testing](./13-software-testing.md) · Next: [15 · Computer Science Foundations](./15-computer-science-foundations.md) →
