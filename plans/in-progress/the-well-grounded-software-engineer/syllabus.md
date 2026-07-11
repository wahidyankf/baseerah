# Syllabus — The Well-Grounded Software Engineer

Per-topic detail for every canonical topic in [prd.md](./prd.md) — **the single source of truth** for
topic set, level, format, primary language, and weights. This file adds the missing dimension: for each
topic, the concrete **Items** (subtopics the learning subtree and drilling page must cover) and the
named **Worked examples** each topic authors.

**How to read this**: topic order, slugs, format, and primary language come from the prd table; this
file never restates weights (they live in prd.md). For **By Example** topics the worked examples map to
the `by-example/{beginner,intermediate,advanced}` progression; for **Annotated-concept** topics they map
to per-theme worked-example pages (with WCAG-accessible Mermaid diagrams where code does not fit). Every
`apps-ayokoding-www-*-maker` step in [delivery.md](./delivery.md) reads the matching `§<slug>` section
here and must cover **every listed item and worked example** to the mastery bar (DD-8).

**Coverage is a floor, not a cap** (DD-8): the items/examples below are the minimum surface a topic must
reach to leave a reader well-grounded; a maker may add more where mastery demands it, never fewer.

---

## L1 · Interview Core

### §`data-structures-and-algorithms` — Data Structures & Algorithms (By Example, Python)

**Items**

- Complexity analysis: Big-O / Θ / Ω, amortized analysis, space vs time trade-offs.
- Linear structures: dynamic array, linked list (singly/doubly), stack, queue, deque, ring buffer.
- Hashing: hash tables, collision strategies, hash-set/hash-map, load factor.
- Trees: binary tree, BST, balanced trees (AVL/red-black overview), heap / priority queue, trie.
- Graphs: representations (adjacency list/matrix), BFS, DFS, topological sort, Dijkstra, union-find.
- Sorting & searching: binary search, merge/quick/heap sort, counting/radix, stability.
- Algorithmic paradigms: recursion, divide & conquer, greedy, dynamic programming, backtracking,
  two-pointers, sliding window.

**Worked examples**

- Beginner: reverse a linked list; validate balanced parentheses with a stack; two-sum with a hash map.
- Intermediate: BFS/DFS over a graph; binary-search variants (first/last occurrence); merge-sort with
  invariants; min-heap top-k.
- Advanced: DP (edit distance / longest-common-subsequence / knapsack); Dijkstra shortest path;
  union-find for connected components; backtracking (N-queens / word search).

### §`computer-science-foundations` — Computer Science Foundations (Annotated-concept, Python\*)

**Items**

- Number systems & data representation: binary/hex, two's complement, IEEE-754 floats, endianness,
  Unicode/UTF-8.
- Boolean logic & digital abstraction: gates, truth tables, combinational vs sequential.
- Computer organization: CPU/registers/ALU, memory hierarchy (cache/RAM/disk), the stack & heap.
- Automata & formal languages: finite automata, regular vs context-free, the Chomsky hierarchy (survey).
- Computability & complexity classes: Turing machines (concept), halting problem, P vs NP (intuition).
- Information & encoding: entropy intuition, lossless vs lossy, checksums/hashing basics.

**Worked examples**

- Represent & convert integers/floats; demonstrate float rounding error and its mitigation.
- Hand-trace a small finite automaton recognizing a language; map a regex to its FA.
- Walk a function call through the stack frame; show cache-friendly vs cache-hostile array traversal.
- Classify sample problems as tractable vs intractable with reasoning (P/NP intuition).

### §`computer-networking` — Computer Networking (Annotated-concept, Python\*)

**Items**

- Network models: the OSI 7-layer and TCP/IP 4-layer models, encapsulation, per-layer responsibilities.
- Link & Internet layers: MAC/ARP, IPv4 vs IPv6 addressing, subnetting/CIDR, NAT, routing basics.
- Transport layer: TCP (three-way handshake, reliability, flow & congestion control) vs UDP; ports & sockets.
- Application layer: DNS resolution, HTTP/1.1 vs HTTP/2 vs HTTP/3 (QUIC), the TLS handshake, WebSockets.
- Diagnostics & performance: ping/traceroute/dig/netstat/tcpdump; latency vs bandwidth vs throughput.
- Delivery & edge: firewalls, proxies, load balancers, CDNs, and the well-known ports.

**Worked examples**

- Trace a URL request end to end — DNS lookup → TCP handshake → TLS → HTTP exchange — as an annotated
  sequence diagram.
- Subnet a network by hand: split a CIDR block, compute host counts, gateway and broadcast addresses.
- A Python `socket` client/server pair contrasting TCP and UDP behavior, annotated line by line.
- Read real `dig` and `traceroute` output and explain each record/hop.

### §`object-oriented-programming` — Object-Oriented Programming (By Example, Python)

**Items**

- Core mechanics: classes, objects, fields, methods, constructors, `self`, identity vs equality.
- Four pillars: encapsulation, inheritance, polymorphism, abstraction.
- Composition over inheritance; interfaces / abstract base classes / duck typing.
- SOLID principles; law of Demeter; cohesion & coupling.
- Essential design patterns: strategy, factory, observer, adapter, decorator, singleton (+ its costs).
- Object lifecycle & state; immutability; value vs reference semantics.

**Worked examples**

- Beginner: model a `BankAccount` with encapsulated balance + invariants; equality/`__eq__`/`__hash__`.
- Intermediate: strategy pattern for pluggable pricing; factory for object creation; observer for events.
- Advanced: refactor an inheritance hierarchy to composition; apply SOLID to a small domain model;
  decorator pattern for cross-cutting behavior.

### §`programming-paradigms` — Programming Paradigms (By Example, Python\*\* survey)

**Items**

- Imperative & procedural; structured programming.
- Object-oriented (cross-reference `object-oriented-programming`).
- Functional (cross-reference `functional-programming`); declarative vs imperative.
- Logic programming (Prolog-style intuition); constraint programming (survey).
- Event-driven & reactive; dataflow.
- Choosing a paradigm: matching paradigm to problem; multi-paradigm languages.

**Worked examples**

- Solve one small problem (e.g. word-frequency count) four ways: imperative, OO, functional, declarative
  — same output, contrasted trade-offs.
- Express a rule set imperatively vs declaratively; show a reactive/event-driven counter.
- Decision table mapping problem shapes to fitting paradigms.

### §`functional-programming` — Functional Programming (By Example, Python)

**Items**

- Pure functions & referential transparency; side effects & where to push them (functional core /
  imperative shell).
- Immutability & persistent data; first-class & higher-order functions; closures.
- `map`/`filter`/`reduce`; comprehensions; function composition; currying & partial application.
- Recursion & tail-call intuition; laziness / generators.
- Algebraic thinking: `Option`/`Maybe`, `Result`/`Either` error handling; functors/monads (gentle,
  practical framing).
- Managing state functionally; reducing shared mutable state.

**Worked examples**

- Beginner: refactor a loop to `map`/`filter`/`reduce`; pure vs impure versions of one function.
- Intermediate: compose small functions into a pipeline; `Option`/`Result` for safe parsing; closures
  for configurable behavior.
- Advanced: functional-core/imperative-shell restructuring of a stateful task; a lazy generator pipeline;
  a small `Maybe`/`Either` chain replacing exception control flow.

### §`concurrency-and-parallelism` — Concurrency & Parallelism (By Example, Python)

**Items**

- Concurrency vs parallelism; processes vs threads vs async; the GIL and its implications in Python,
  and free-threaded CPython (PEP 703/779, the `3.14t` build; officially supported since 3.14).
- Synchronization: locks/mutexes, semaphores, condition variables, deadlock/livelock/starvation.
- Race conditions & data races; atomicity; memory-visibility intuition.
- Message passing & queues; producer/consumer; thread/process pools.
- `async`/`await` & event loops; cooperative vs preemptive scheduling.
- Parallel decomposition: map-reduce style, work stealing (concept), amdahl's-law intuition.

**Worked examples**

- Beginner: threads vs `asyncio` for I/O-bound work; a shared-counter race and its lock fix.
- Intermediate: producer/consumer with a bounded queue; a thread/process pool over a CPU/I/O workload.
- Advanced: reproduce & resolve a deadlock; `asyncio` gather over many I/O tasks; process-based parallelism
  to sidestep the GIL for CPU-bound work.

---

## L2 · Build & Ship

### §`software-engineering-practices` — Software Engineering Practices (Annotated-concept, Python\*)

**Items**

- Version control: Git model (commits/branches/merges/rebases), trunk-based development, conventional
  commits, PR review.
- Testing discipline: the test pyramid, TDD (red/green/refactor), unit/integration/e2e, coverage & its
  limits, test doubles.
- Code quality: linting/formatting, code review, refactoring, technical debt, readability.
- CI/CD: pipelines, quality gates, artifact/build, release strategies (blue-green, canary).
- Collaboration & process: Agile/Scrum/Kanban intuition, estimation pitfalls, documentation, ADRs.
- Debugging & observability basics; incident hygiene.

**Worked examples**

- Walk a feature through TDD red → green → refactor with a real Python test.
- A commit-history worked example: messy history → clean conventional-commit history.
- A minimal CI pipeline (lint → test → build) annotated stage-by-stage with a Mermaid flow.
- An ADR worked example (decision, context, consequences) for a small technical choice.

### §`data-storage` — Data Storage (Databases) (By Example, SQL + Python †)

**Items**

- Relational model: tables, keys (primary/foreign), normalization (1NF–3NF), constraints.
- SQL: `SELECT`/`JOIN`/`GROUP BY` + aggregate functions (`COUNT`/`SUM`/`AVG`/`MIN`/`MAX`), subqueries,
  CTEs, window functions, DDL vs DML.
- Transactions & ACID; isolation levels; locking & MVCC intuition.
- Indexing: B-tree/hash indexes, query planning, `EXPLAIN`, when indexes hurt.
- NoSQL families: key-value, document, wide-column, graph — CAP-theorem trade-offs, when to pick each.
- Access from code: drivers, connection pooling, parameterized queries (SQL-injection avoidance),
  ORMs vs raw SQL, migrations.
- Data modeling: OLTP vs OLAP intuition, denormalization trade-offs, schema evolution.

**Worked examples**

- Beginner: design a normalized schema for a small domain; basic `SELECT`/`INSERT`/`UPDATE`; a two-table
  `JOIN`; parameterized query from Python.
- Intermediate: aggregations + `GROUP BY` + `HAVING`; a window-function report; a transaction with
  commit/rollback; add an index and read `EXPLAIN`.
- Advanced: model the same data relationally vs as a document store; a migration that evolves a schema
  safely; connection pooling + N+1-query diagnosis and fix.

### §`backend-development` — Backend Development (By Example, Python)

**Items**

- HTTP fundamentals: methods, status codes, headers, content negotiation, statelessness.
- API design: REST resource modeling, versioning, pagination, idempotency; GraphQL and gRPC
  (service-to-service) contrast (survey).
- Request lifecycle: routing, middleware, controllers/handlers, serialization/validation.
- Persistence: repository pattern, transactions, migrations (cross-reference `data-storage`).
- AuthN/AuthZ: sessions vs tokens (JWT), OAuth2 (authorization) vs OpenID Connect (authentication),
  password hashing, RBAC.
- Reliability: error handling, logging, config/secrets, health checks, rate limiting, caching.
- Testing backends: unit vs integration, contract tests, test containers intuition.

**Worked examples**

- Beginner: a minimal HTTP JSON endpoint; request validation + structured error response.
- Intermediate: a CRUD resource with a repository + DB; token auth middleware; pagination + filtering.
- Advanced: idempotent write endpoint; a caching + rate-limit layer; an integration test suite hitting a
  real DB.

### §`frontend-development` — Frontend Development (By Example, TypeScript †)

**Items**

- The platform: HTML semantics, CSS layout (fl/box/grid), the DOM, the event loop in the browser.
- Component model: components, props, state, unidirectional data flow, rendering & reconciliation.
- State management: local vs shared state, derived state, data fetching & caching.
- Forms & validation; controlled vs uncontrolled inputs.
- Accessibility (WCAG AA), semantic markup, keyboard nav, ARIA basics.
- Performance: Core Web Vitals (LCP/INP/CLS), bundle size, code-splitting, memoization, rendering cost,
  SSR/hydration.
- TypeScript for UI: typing props/state, discriminated unions for UI states, generics.

**Worked examples**

- Beginner: a typed counter/component with props & state; a semantic, accessible form.
- Intermediate: data-fetching component with loading/error/empty states; a controlled form with
  validation; list rendering with keys.
- Advanced: a small state-managed feature (shared state + derived selectors); a memoization/perf fix; an
  accessibility remediation of a broken widget.

### §`android-app-development` — Android App Development (By Example, Kotlin †)

**Items**

- App fundamentals: activities, fragments, the lifecycle, the manifest, intents.
- UI: Jetpack Compose (declarative UI), state hoisting, recomposition; views (survey).
- Architecture: ViewModel, unidirectional data flow, repository, dependency injection intuition.
- Data & persistence: Room, DataStore, networking (Retrofit/coroutines).
- Concurrency: Kotlin coroutines & flows on Android.
- Platform concerns: permissions, background work, navigation, resource/config changes.

**Worked examples**

- Beginner: a Compose screen with state; lifecycle-aware logging.
- Intermediate: a ViewModel-backed list with a repository + Room; a network call with coroutines +
  loading/error states.
- Advanced: navigation across screens with saved state; a flow-driven reactive UI; handling a config
  change without losing state.

### §`ios-app-development` — iOS App Development (By Example, Swift †)

**Items**

- App fundamentals: app/scene lifecycle, the view hierarchy, the responder chain.
- UI: SwiftUI (declarative views, state/binding/observable); UIKit (survey/contrast).
- Architecture: MVVM, the Observation framework (`@Observable` macro, iOS 17+) with `@State`/`@Binding`
  (legacy `ObservableObject`/`@Published` as contrast), dependency injection intuition.
- Data & persistence: `Codable`, `URLSession` networking, Core Data / SwiftData intuition.
- Concurrency: Swift `async`/`await`, actors, structured concurrency.
- Platform concerns: navigation, permissions, background tasks, app lifecycle events.

**Worked examples**

- Beginner: a SwiftUI view driven by `@State`; a binding-based form.
- Intermediate: an MVVM screen with an `ObservableObject` view model + networking with `async`/`await` and
  loading/error states.
- Advanced: navigation stack with passed state; an actor-isolated data cache; persistence round-trip.

---

## L3 · Design at Scale

### §`software-architecture` — Software Architecture (Annotated-concept, Python\*)

**Items**

- Architectural styles: layered, hexagonal/ports-and-adapters, functional core/imperative shell,
  event-driven, microservices vs monolith.
- Quality attributes: modifiability, scalability, availability, performance, security — trade-off thinking.
- Boundaries & modularity: bounded contexts and DDD tactical patterns (aggregate, entity, value object),
  coupling/cohesion, dependency direction, DIP at scale.
- Cross-cutting concerns: config, logging, error handling, transactions across boundaries.
- Documentation: C4 model, ADRs, diagrams-as-communication.
- Evolutionary architecture: fitness functions, strangler-fig migration.

**Worked examples**

- A monolith → modular boundaries refactor, shown as before/after C4-style Mermaid diagrams.
- Ports-and-adapters worked example: domain core isolated from an interchangeable adapter.
- An ADR trade-off analysis for one significant decision (e.g. sync vs async integration).

### §`system-design` — System Design (Annotated-concept, Python\*)

**Items**

- Requirements: functional vs non-functional, capacity estimation, back-of-envelope math.
- Scaling: vertical vs horizontal, statelessness, load balancing, sharding, replication.
- Data: SQL vs NoSQL at scale, caching strategies, consistency models, CAP/PACELC.
- Communication: sync vs async, message queues, pub/sub, idempotency, backpressure.
- Reliability: redundancy, failover, health checks, rate limiting, circuit breakers.
- Observability & operations: metrics/logs/traces, SLIs/SLOs (cross-reference `site-reliability-engineering`).
- Canonical designs: URL shortener, news feed, chat, rate limiter, object store (as case studies).

**Worked examples**

- Design a URL shortener end-to-end (API, data model, scaling, caching) with a Mermaid architecture map.
- Design a rate limiter (token bucket vs sliding window) with trade-off analysis.
- Design a scalable read-heavy feed with caching + fan-out trade-offs.

---

## L4 · Broaden Delivery

### §`windows-app-development` — Windows App Development (By Example, C# †)

**Items**

- .NET fundamentals: the runtime, project/build model, NuGet.
- Desktop UI: WinUI/WPF (XAML, data binding, MVVM); WinForms (survey).
- Async on the UI thread: `async`/`await`, dispatcher, cancellation.
- Data & persistence: file I/O, settings, local DB/SQLite.
- Packaging & deployment: MSIX intuition, app lifecycle.

**Worked examples**

- Beginner: a XAML window with data binding; a simple command handler.
- Intermediate: an MVVM view with an observable model + async data load.
- Advanced: cancellation + progress reporting on a long task; persistence + settings round-trip.

### §`linux-app-development` — Linux App Development (By Example, Python)

**Items**

- The Linux process/runtime model as seen from an app: env, args, exit codes, signals.
- Filesystem & I/O: paths, permissions, file descriptors, streams, temp files.
- Building CLIs: argument parsing, config, logging, exit-code discipline.
- IPC & processes: subprocess, pipes, sockets (survey), environment/config.
- Packaging & distribution: virtualenvs, dependencies, systemd unit intuition, containers (survey).
- Daemons & scheduling: long-running services, cron, graceful shutdown on signals.

**Worked examples**

- Beginner: a well-behaved CLI (args, `--help`, exit codes, stderr vs stdout).
- Intermediate: a script that shells out via `subprocess` with error handling; signal-handled graceful
  shutdown.
- Advanced: a small long-running daemon with logging + a systemd-style lifecycle; a pipe/IPC example.

### §`cloud-containers-and-iac` — Cloud, Containers & IaC (Annotated-concept, YAML/HCL †)

**Items**

- Cloud service models: IaaS/PaaS/SaaS, regions & availability zones, the shared-responsibility model,
  core primitives (compute/storage/network/identity).
- Containers: images vs containers, Dockerfile layers, registries, the OCI standard; container vs VM.
- Orchestration: Kubernetes objects (Pod/Deployment/Service/Ingress/ConfigMap/Secret), scheduling,
  autoscaling, health probes.
- Infrastructure as Code: declarative vs imperative, Terraform (HCL) providers/state/plan–apply,
  idempotency, drift detection.
- CI/CD & delivery: pipelines, artifacts, environments, GitOps, blue-green/canary at the infra layer.
- Cost, security & operations: least-privilege IAM, secrets, tagging, observability, cost awareness.

**Worked examples**

- A multi-stage Dockerfile building a small app image, contrasting image size before/after, annotated
  layer by layer.
- A Kubernetes Deployment + Service manifest walked field by field (YAML) with a request-flow diagram.
- A Terraform module (HCL) through its plan → apply → destroy lifecycle, showing state and idempotency.
- A minimal CI/CD pipeline definition annotated stage by stage.

### §`data-engineering` — Data Engineering (Annotated-concept, Python)

**Items**

- Pipelines: batch vs streaming, ETL vs ELT, orchestration (DAGs), idempotent/retriable tasks.
- Storage layers: data lake vs warehouse vs lakehouse; the medallion architecture (bronze/silver/gold);
  file formats (CSV/JSON/Parquet); partitioning.
- Modeling for analytics: star schema, fact/dimension, slowly-changing dimensions.
- Data quality: validation, schema enforcement, deduplication, lineage.
- Processing: map-reduce intuition, windowing in streams, exactly-once vs at-least-once.
- Governance: PII handling, retention, cataloging.

**Worked examples**

- A batch ETL worked example (extract → transform → load) with idempotency, shown as a DAG diagram.
- A star-schema model for a small analytics domain (fact + dimensions).
- A data-quality gate (schema validation + dedup) worked over a sample dataset.

### §`creating-ai-powered-apps` — Creating AI-Powered Apps (By Example, Python)

**Items**

- LLM app fundamentals: prompts, tokens, context windows, temperature, structured output.
- Context engineering (beyond prompt engineering): memory, token-budget allocation, context compression,
  tool-result orchestration.
- Calling models: SDK/API usage, streaming, error/retry, cost & latency awareness.
- Retrieval-augmented generation: chunking, embeddings, vector search, grounding & citations.
- Tool use / function calling; the Model Context Protocol (MCP) as the standardized tool-integration
  layer; agent loops (survey); guardrails.
- Evaluation: golden sets, LLM-as-judge intuition, regression testing prompts.
- Safety & reliability: prompt-injection awareness, PII, hallucination mitigation, fallbacks.

**Worked examples**

- Beginner: a prompt → structured-output call with validation and retry.
- Intermediate: a minimal RAG pipeline (embed → retrieve → ground → answer with citations).
- Advanced: a tool-calling loop with guardrails; a small eval harness scoring outputs against a golden set.

### §`it-security` — IT Security (Annotated-concept, Python\*)

**Items**

- Foundations: CIA triad, threat modeling (STRIDE), attack surface, defense in depth, least privilege.
- AppSec: the current OWASP Top 10:2025 (broken access control, security misconfiguration, software
  supply-chain failures, cryptographic failures, injection, insecure design, authentication failures,
  integrity failures, logging/alerting failures, mishandling exceptional conditions), input validation,
  output encoding.
- Cryptography (applied): hashing vs encryption, symmetric/asymmetric, TLS, password storage, secrets
  management.
- AuthN/AuthZ: sessions, tokens/JWT, OAuth2/OIDC, RBAC/ABAC, MFA.
- Network & infra security: firewalls, segmentation, TLS everywhere, supply-chain risk.
- Operations: logging/monitoring, incident response, vulnerability management, secure SDLC.

**Worked examples**

- A SQL-injection and an XSS worked example: vulnerable code → exploit → fixed code.
- Password storage done right (salted hashing) vs wrong, contrasted.
- A software-supply-chain-failure walkthrough (OWASP 2025 #3): a compromised/typo-squatted dependency →
  impact → mitigations (lockfiles, pinned versions, SCA scanning, provenance/SBOM).
- A threat model (STRIDE) for a small service, presented as a diagram + mitigations table.

---

## L5 · Systems & Language Depth

### §`linux-os` — Linux OS (By Example, C + shell †)

**Items**

- Kernel vs user space; system calls; the process model (fork/exec/wait), PIDs, signals.
- Memory: virtual memory, paging, the process address space, mmap intuition.
- Filesystems: inodes, file descriptors, VFS, permissions, mounts.
- Scheduling: processes vs threads, context switches, priorities (concept).
- IPC: pipes, signals, shared memory, sockets.
- The shell & tooling: `ps`/`top`/`strace`/`/proc`, observing a running system.

**Worked examples**

- Beginner: `fork`/`exec`/`wait` in C; inspect a process via `/proc` and shell tools.
- Intermediate: signal handling in C; a pipe between two processes.
- Advanced: shared-memory IPC; `strace` a program and read its syscalls; an mmap'd file example.

### §`windows-os` — Windows OS (By Example, C + PowerShell †)

**Items**

- Windows architecture: user vs kernel mode, the Win32 API, subsystems, the registry.
- Processes & threads: creation (`CreateProcess`), handles, the object model, scheduling.
- Memory management: virtual memory, working sets, heaps.
- Synchronization: Win32 mutexes/events/critical sections.
- Filesystem & I/O: NTFS concepts, handles, async/overlapped I/O intuition.
- Tooling: PowerShell for inspection, Task Manager/Process Explorer concepts.

**Worked examples**

- Beginner: create a process with the Win32 API in C; enumerate processes via PowerShell.
- Intermediate: a Win32 mutex/critical-section synchronization example.
- Advanced: handle-based file I/O; inspect a running process's memory/handles with PowerShell + tooling.

### §`system-programming` — System Programming (By Example, C †)

**Items**

- Memory model: stack vs heap, pointers, `malloc`/`free`, alignment, ownership discipline.
- Undefined behavior & safety: buffer overflows, use-after-free, integer overflow.
- Manual resource management: file descriptors, scope-based cleanup by hand (goto-cleanup pattern,
  `__attribute__((cleanup))`), error/`errno` handling. (C has no RAII — cleanup is explicit.)
- Low-level data: bit manipulation, structs/unions, endianness, serialization.
- Building & linking: compilation units, headers, static vs dynamic linking, the ABI (concept).
- Interfacing with the OS: syscalls, signals, basic sockets.

**Worked examples**

- Beginner: a dynamic array in C with correct `malloc`/`realloc`/`free`; bit-manipulation utilities.
- Intermediate: a linked structure with disciplined ownership + cleanup; `errno`-based error handling.
- Advanced: a small allocator or memory pool; a minimal socket client/server; a serialization routine with
  endianness handling.

### §`lisp` — Lisp (By Example, Scheme †)

**Items**

- S-expressions & homoiconicity; code as data; the reader/evaluator model.
- Core forms: `define`, `let`, `lambda`, `cond`/`if`, recursion.
- Lists & higher-order functions: `map`/`fold`/`filter`, cons cells.
- Recursion as iteration: proper tail calls / tail-call optimization (Scheme mandates them).
- Macros: quasiquote/unquote, hygienic macros (`syntax-rules`), code that writes code.
- State & scope: lexical vs dynamic scope, closures.
- REPL-driven development.

**Worked examples**

- Beginner: recursive list processing (length/reverse/map from scratch); `let`/`lambda` basics.
- Intermediate: higher-order functions & closures; a small interpreter for arithmetic S-expressions.
- Advanced: a `syntax-rules` macro that introduces new syntax (e.g. `unless`, `while`, or a small DSL);
  metaprogramming with quasiquote; a tail-recursive rewrite that runs in constant stack.

### §`type-systems` — Type Systems / Hindley–Milner (By Example, OCaml/Haskell †)

**Items**

- What a type system buys you: soundness, expressiveness, the safety/flexibility trade-off.
- Algebraic data types: sums, products, records, pattern matching, exhaustiveness.
- Parametric polymorphism / generics; type inference.
- Hindley–Milner: unification, principal types, `let`-polymorphism, the inference algorithm (intuition).
- Type classes / functors-applicatives-monads (practical, gentle).
- Advanced (survey): higher-kinded types, GADTs, dependent types (intuition only).

**Worked examples**

- Beginner: define ADTs (`Option`, a small `Shape` sum type) and exhaustive pattern matches.
- Intermediate: a polymorphic function whose type is inferred; hand-trace unification on a small expression.
- Advanced: a type-class/monadic pipeline (e.g. `Option`/`Result` chaining); a worked HM inference on a
  `let`-polymorphic expression.

### §`compilers-parsers-and-transpilers` — Compilers, Parsers & Transpilers (By Example, Python)

**Items**

- Pipeline overview: lexing → parsing → AST → semantic analysis → codegen/interpretation.
- Lexing: tokens, regular languages, hand-written vs generated lexers.
- Parsing: grammars (BNF), recursive descent, precedence, ASTs, error recovery.
- Semantic analysis: symbol tables, scoping, type checking (cross-reference `type-systems`).
- Backends: tree-walking interpreter, bytecode + VM, transpilation to another language.
- Optimizations (survey): constant folding, dead-code elimination.

**Worked examples**

- Beginner: a tokenizer for a small arithmetic language.
- Intermediate: a recursive-descent parser building an AST with operator precedence; a tree-walking
  evaluator.
- Advanced: a transpiler emitting target-language source from the AST; a bytecode compiler + tiny VM.

---

## L6 · Advanced Ops

### §`site-reliability-engineering` — Site Reliability Engineering (Annotated-concept, Python\*)

**Items**

- SRE principles: SLIs/SLOs/SLAs, error budgets, toil reduction, blameless postmortems.
- Observability: metrics, logs, traces; the four golden signals; dashboards & alerting.
- Reliability patterns: redundancy, graceful degradation, circuit breakers, retries with backoff,
  rate limiting.
- Capacity & performance: load testing, autoscaling, saturation, back-of-envelope capacity.
- Incident management: on-call, severity levels, runbooks, incident lifecycle.
- Change safety: canary/blue-green, rollbacks, progressive delivery.

**Worked examples**

- Define SLIs/SLOs + an error budget for a sample service, with the math.
- A retry-with-backoff + circuit-breaker worked example (code + failure-behavior diagram).
- A postmortem worked example (timeline, root cause, action items) for a simulated incident.

---

## L7 · Leadership & Product

### §`it-governance-grc` — IT Governance (IT GRC) (Annotated-concept, — ‡)

**Items**

- Governance vs management; frameworks (COBIT/ITIL intuition), decision rights, accountability.
- Risk management: identification, assessment (likelihood × impact), treatment, risk register.
- Compliance: common regimes (GDPR/SOC 2/ISO 27001 intuition), controls, evidence, audits.
- Policy & standards: policy hierarchy, exceptions, enforcement.
- Vendor & third-party risk; data governance & privacy.
- Metrics: KRIs/KPIs, maturity models.

**Worked examples** (design/decision exercises — minimal code)

- Build a small risk register (assess/treat) for a sample system.
- Map a compliance requirement to concrete technical controls + evidence.
- A governance decision-rights (RACI-style) worked example for a change process.

### §`project-management` — Project Management (Annotated-concept, — ‡)

**Items**

- Delivery methodologies: waterfall vs agile (Scrum/Kanban), hybrid; when each fits.
- Scope, schedule, cost — the triple constraint; trade-offs.
- Planning: work breakdown, estimation (story points/velocity, planning-poker pitfalls), dependencies,
  critical path.
- Execution: backlog, sprints, standups, risk/issue tracking, stakeholder communication.
- Metrics: burndown/burnup, cycle time, lead time, throughput.
- Risk & change management; retrospectives & continuous improvement.

**Worked examples** (design/decision exercises)

- Break a sample feature into a WBS + dependency graph; identify the critical path.
- Run an estimation worked example and show why velocity beats hours.
- Interpret a burndown chart to diagnose a slipping sprint + corrective action.

### §`software-product-engineering` — Software Product Engineering (Annotated-concept, — ‡)

**Items**

- Product thinking for engineers: user problems vs solutions, outcomes vs output.
- Discovery: user research, problem validation, opportunity assessment, JTBD.
- Prioritization: RICE/MoSCoW/impact-effort, roadmap trade-offs.
- Delivery of value: MVP, iterative delivery, experimentation (A/B), feature flags.
- Metrics: north-star metric, AARRR/funnel, activation/retention.
- Engineer ↔ product ↔ design collaboration; writing good specs.

**Worked examples** (design/decision exercises)

- Turn a vague feature request into a validated problem statement + MVP scope.
- Prioritize a sample backlog with RICE and defend the ordering.
- Design an A/B experiment (hypothesis, metric, guardrail) for a product change.

### §`engineering-management` — Engineering Management (Annotated-concept, — ‡)

**Items**

- The IC → manager transition; what changes; the manager's job.
- People: 1:1s, feedback, growth/career ladders, performance management, hiring & onboarding.
- Team health: psychological safety, team topologies, delegation, motivation.
- Execution: goal setting (OKRs), planning, prioritization, managing up & across.
- Technical leadership: architecture stewardship, tech-debt strategy, quality culture.
- Scaling: org design, process without bureaucracy, remote/distributed teams.

**Worked examples** (design/decision exercises)

- A 1:1 + feedback worked scenario (situation → behavior → impact framing).
- Design OKRs for a sample team + how they cascade from company goals.
- A delegation/prioritization decision worked through a realistic constraint (limited capacity, competing asks).
