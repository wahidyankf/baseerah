# 40 · Build Your Own Web Framework (By Example, Python †)

**prd row**: Pass 3 · Build for the Real World · By Example · Python † · Learn 140 / Drill 240 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the build-your-own tier for the backend band — a minimal web framework that
demystifies the ones you use (Flask/FastAPI/Django): the WSGI/ASGI contract, a router, a middleware
chain, request/response objects, and lightweight dependency injection. Interleaved after
[`39-backend-at-scale`](./39-backend-at-scale.md), it makes the "magic" of `@app.route` and
middleware concrete. `†`: Python, fully type-annotated (DD-34) — every snippet carries type hints in
the mypy-clean spirit.

## Why this exists · the big idea

- **The problem before the solution**: every framework feels like magic until you've built one — you
  can't reason about a mysterious 500, a middleware ordering bug, or a slow request when the router,
  the request lifecycle, and the server boundary are all opaque. Rebuilding the core turns "the
  framework did something" into "I know exactly what happens between the socket and my handler".
- **Keep-this-if-you-forget-everything**: a web framework is a thin function that turns an incoming
  request (an environ/scope dict) into a response, via a router that picks a handler and a middleware
  chain that wraps it. Everything else is convenience over that one transformation.
- **Big ideas touched**: `abstraction-and-its-cost` (a framework hides the server protocol, routing,
  and lifecycle behind decorators — building it exposes what that convenience costs and where it
  constrains you), `layering-and-leaks` (WSGI/ASGI is the seam between server and app — you'll see
  exactly where the socket, the protocol, and your handler meet, and where each layer bleeds through).

## Prerequisites

- **Prior topics**: [topic 11 Backend Essentials](./11-backend-essentials.md) (routing, request
  handling, status codes as a _user_ of a framework) and [topic 39 Backend at Scale](./39-backend-at-scale.md)
  (middleware, auth, and reliability patterns you'll now implement the substrate for).
- **Tools & environment**: a macOS/Linux terminal; **Python** at a recent stable release with type
  hints and `mypy`; a WSGI server (e.g. a reference `gunicorn`/`waitress`) and/or an ASGI server
  (e.g. `uvicorn`); `curl`; Neovim/VSCode with the Python LSP (DD-17). No web framework — that's the
  point.
- **Assumed knowledge**: serving and calling a CRUD JSON endpoint through an existing framework
  (topic 11); what middleware and routing do from the outside (topics 11/39); functions as
  first-class values and decorators (topic 04).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: **WSGI (PEP 3333)** and **ASGI** remain the stable server↔app contracts for
  Python — WSGI for synchronous apps, ASGI for async/streaming/WebSockets. Both are left correctly
  version-unpinned; the `environ`/`start_response` (WSGI) and `scope`/`receive`/`send` (ASGI) shapes
  are unchanged.
- 2026-07-12 — verified (GAP for plan owner): the topic teaches both WSGI and ASGI paths but the
  build-your-own capstone should pick one primary target at drafting time (ASGI is the forward-looking
  default for async handlers); leaving both fully in-scope risks an over-large example. Concrete server
  package + version to be pinned when drafted.

## Items

- The server↔app boundary: WSGI (`environ` + `start_response`) and ASGI (`scope`/`receive`/`send`) —
  what the server hands you and what you must hand back.
- A router: mapping method + path (with path parameters) to a handler; the decorator that registers
  a route and why it's just a function returning a function.
- Typed request/response objects: parsing the raw environ/scope into an ergonomic `Request`, building
  a `Response` (status, headers, body) that serializes back to the protocol.
- The middleware chain: wrapping the handler in an onion of before/after concerns (logging, auth,
  errors) and why ordering is a correctness property, not a style choice.
- Lightweight dependency injection: a tiny provider/registry so handlers declare what they need
  instead of reaching for globals.
- Error handling and the framework's contract: turning an exception into a proper response instead
  of a leaked stack trace.

## Worked examples

Colocated under `build-your-own-web-framework/learning/code/`; each runnable behind a real WSGI/ASGI
server and exercised with `curl`, every Python snippet fully type-annotated and `mypy`-clean
(DD-20/DD-30/DD-34).

- **beginner** — a WSGI/ASGI app that parses the environ/scope into a typed `Request` and returns a
  typed `Response`; serve it and hit it with `curl`.
- **intermediate** — add a router with path parameters and a `@route` decorator, plus a two-layer
  middleware chain (request logging + error handling); observe ordering.
- **advanced** — add lightweight dependency injection and a JSON body/response codec, then port one
  handler from a real framework onto your framework to prove the contract matches.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a minimal but real typed web framework — WSGI/ASGI entrypoint, router with path
  params, an ordered middleware chain, typed request/response objects, and lightweight DI — that
  serves a small JSON API behind a standard server and passes an integration test suite driven by
  `curl`/a client.
- **Concepts exercised**: [ ] WSGI/ASGI entrypoint [ ] router + path params [ ] typed request/response
  [ ] ordered middleware chain [ ] lightweight DI [ ] error-to-response handling.
- **Ordered steps**:
  1. `.../learning/capstone/code/app.py` — a WSGI/ASGI callable that builds a typed `Request` and
     returns a typed `Response`. Verify a standard server serves it and `curl` gets a 200 with the
     expected body; `mypy` clean.
  2. `.../learning/capstone/code/router.py` — a router + `@route` decorator with a path parameter.
     Verify a parameterized route resolves and an unknown path returns 404.
  3. `.../learning/capstone/code/middleware.py` — a logging + error-handling middleware chain. Verify
     ordering (logging wraps errors) and that a raised exception becomes a 500 response, not a leaked
     trace.
  4. `.../learning/capstone/code/di.py` — a provider registry injecting a dependency into a handler,
     plus an integration test suite. Verify handlers receive their declared dependency and the suite
     passes.
- **Acceptance criteria**: the framework serves a JSON API behind a real server; routing, middleware
  ordering, DI, and error-to-response all behave; the integration suite is green; all Python is
  type-annotated and `mypy`-clean.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Papers & articles**

- **Let's Build A Web Server (Parts 1–3)** — Ruslan Spivak (2015). Widely cited free tutorial series
  building an HTTP server and a WSGI-compatible framework from raw sockets upward.
  <https://ruslanspivak.com/lsbaws-part1/>
- **PEP 3333 – Python Web Server Gateway Interface (WSGI) v1.0.1** — Phillip J. Eby / Python Software
  Foundation (2010). The standard request/response contract between Python web servers and the
  frameworks built on top of them. <https://peps.python.org/pep-3333/>
- **ASGI (Asynchronous Server Gateway Interface) Specification** — ASGI Team (continually maintained).
  The async successor to WSGI defining the routing, middleware, and lifecycle contract for modern
  Python frameworks. <https://asgi.readthedocs.io/en/latest/specs/main.html>
- **Rack Specification (SPEC.rdoc)** — Rack Core Team (continually maintained). The Ruby middleware and
  request-lifecycle contract underlying Sinatra, Rails, and minimal Ruby frameworks — a useful
  cross-language mirror of WSGI/ASGI. <https://github.com/rack/rack/blob/main/SPEC.rdoc>

---

← Previous: [39 · Backend at Scale](./39-backend-at-scale.md) · Next: [41 · API Design](./41-api-design.md) →
