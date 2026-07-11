# 59 · Site Reliability Engineering (Annotated-concept, Python \*)

**prd row**: Pass 4 · Concurrency & Systems · Annotated-concept · Python \* · Learn 159 /
Drill 259 · Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `*` concept-centric (Python where code appears) — operating software reliably at scale:
SLIs/SLOs/error budgets, the four golden signals, observability (metrics/logs/traces), alerting on symptoms
not causes, incident response & blameless postmortems, toil reduction, and capacity/load. The Pass-4 closer
and reliability capstone anchor. Builds on the ops/observability threads
([`13-software-testing`](./13-software-testing.md),
[`34-containers-and-orchestration`](./34-containers-and-orchestration.md)).

## Prerequisites

- **Prior topics**: [topic 28 Backend at Scale](./28-backend-at-scale.md) (the services being operated),
  [topic 34 Containers & Orchestration](./34-containers-and-orchestration.md) (where they run), and
  [topic 32 System Design](./32-system-design.md) (load, capacity, failure modes).
- **Tools & environment**: **Python 3.x** for the runnable mechanisms; a local metrics stack
  (Prometheus-style scrape + a dashboard) runnable via containers (DD-20); Neovim/VSCode (DD-17).
- **Assumed knowledge**: running a backend service (topic 28); containers/orchestration basics (topic 34);
  reasoning about load + failure (topic 32).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified (exact match to Google SRE canon): SLIs/SLOs/error budgets (budget = 100% − SLO),
  the four golden signals (latency, traffic, errors, saturation), symptom-based alerting, blameless
  postmortems + incident command, and toil reduction / capacity planning are unchanged. (sre.google/sre-book)
- 2026-07-12 — verified: **Prometheus = Apache-2.0**, CNCF-graduated, de-facto metrics standard;
  **OpenTelemetry** is the current CNCF-graduated instrumentation standard for metrics/tracing. No drift.
  (github.com/prometheus/prometheus/blob/main/LICENSE)

## Items

- Reliability as a feature: SLIs, SLOs, error budgets, the budget-vs-velocity trade-off.
- The four golden signals: latency, traffic, errors, saturation.
- Observability: metrics vs logs vs traces; instrumenting a service; dashboards.
- Alerting: symptom-based (not cause-based) alerts; alert fatigue; on-call intuition.
- Incident response: severity, incident command, blameless postmortems, action items.
- Toil & automation: identifying toil, eliminating it; capacity planning & load (concept).

## Worked examples

Colocated under `site-reliability-engineering/learning/code/`; Python + a local metrics stack (DD-20/DD-30).

- **beginner** — instrument a small service with the four golden signals; expose a metrics endpoint.
- **intermediate** — define an SLI + SLO + error budget in code; a symptom-based alert rule.
- **advanced** — a dashboard over the golden signals; a worked blameless-postmortem artifact for a seeded
  incident.

## Capstone spec — intra-topic (subject → runnable mechanisms + reliability artifact)

- **Goal**: make a small service **observable and operable** — instrument the four golden signals, define an
  SLI/SLO with an error budget in code, wire a symptom-based alert and a dashboard, then run a seeded
  incident and produce a blameless postmortem with action items — the full SRE loop from measurement to
  learning.
- **Concepts exercised**: [ ] four-golden-signals instrumentation + a metrics endpoint [ ] an SLI + SLO +
  error budget defined in code [ ] a symptom-based (not cause-based) alert rule [ ] a golden-signals
  dashboard [ ] a seeded incident + a blameless postmortem with action items.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — instrument a small service with latency/traffic/errors/saturation +
     a metrics endpoint. Verify the endpoint exposes all four signals under load.
  2. Define an SLI + SLO + error budget in code and a symptom-based alert rule. Verify the alert fires when
     the SLO is violated (drive it with injected errors/latency) and stays quiet otherwise.
  3. Add a dashboard; run a seeded incident and write `postmortem.md` (blameless, with timeline + action
     items). Verify the dashboard reflects the incident and the postmortem is symptom-focused and blameless.
- **Acceptance criteria**: all four golden signals are instrumented; the SLO + error budget are defined in
  code; the alert is symptom-based and fires correctly; the dashboard reflects reality; the postmortem is
  blameless with concrete action items.
- **Done bar**: runnable end-to-end + reliability artifact + web-verified.

## Capstone spec — inter-topic: capstone-concurrency-and-systems (Pass-4 boundary)

> **Weight**: 695 (Pass-4 boundary inter-topic capstone). Anchored here as the Pass-4 closer; integrates the
> pass's concurrency + systems-depth topics. Kind: **subject → full runnable**.

- **Goal**: build a **concurrent, systems-aware, observable service** that ties Pass 4 together — a
  work-processing service using a real concurrency model (CSP-Go **or** actor-Elixir), backed by a
  systems-level component, containerized, and instrumented with SRE golden signals + an SLO — demonstrating
  that concurrency, systems depth, and reliability compose into one operable system.
- **Concepts exercised**: [ ] a concurrency model in anger (Go CSP: goroutines/channels/`context`
  [topic 42] **or** Elixir actors: GenServer/supervision [topic 44]) [ ] a systems-level component (a C
  primitive / memory-aware data path [topics 52/53/55] **or** a justified equivalent) [ ] containerized +
  orchestrated deployment [topic 34, Pass 3] [ ] SRE instrumentation: four golden signals + an SLI/SLO +
  error budget [topic 59] [ ] a symptom-based alert + dashboard.
- **Ordered steps**:
  1. `.../capstone/capstone-concurrency-and-systems/code/` — a concurrent work-processing service in Go
     (CSP) **or** Elixir (actors), with a bounded worker pool / supervised workers and graceful shutdown.
     Verify it processes a concurrent workload with no race (Go `-race`) / clean supervision (Elixir) and
     shuts down gracefully.
  2. Add a systems-level component (or a justified equivalent) and containerize the service. Verify the
     container builds and runs the full workload.
  3. Instrument the four golden signals + an SLI/SLO + error budget; add a symptom-based alert + dashboard.
     Verify the signals expose under load, the SLO alert fires on violation, and the dashboard reflects it.
- **Acceptance criteria**: the concurrency model is used correctly (race-free / properly supervised); the
  service is containerized; golden signals + SLO + alert + dashboard all work; graceful shutdown holds.
- **Done bar**: runnable end-to-end + observable + web-verified.

## Capstone spec — inter-topic: capstone-concurrency-showdown (cross-cutting)

> **Weight**: 696 (cross-cutting inter-topic capstone). Kind: **subject → full runnable + comparison
> artifact**. A deliberate CSP-vs-actor head-to-head.

- **Goal**: solve the **same** concurrent problem twice — once with **CSP-style Go**
  (goroutines/channels/`select`/`context`) and once with the **actor-model Elixir/OTP**
  (GenServer/supervision/"let it crash") — then write a grounded comparison of the two paradigms on the same
  workload: how each handles coordination, backpressure, failure/supervision, and observability.
- **Concepts exercised**: [ ] the same problem in Go CSP [topic 42] and Elixir actors [topic 44] [ ] channel
  coordination + `select` + `context` cancellation (Go) [ ] GenServer + supervision trees + "let it crash"
  (Elixir) [ ] backpressure + failure handling contrasted [ ] a decision write-up: when each model fits.
- **Ordered steps**:
  1. `.../capstone/capstone-concurrency-showdown/go/` — solve the chosen concurrent problem (e.g. a
     fan-out/fan-in pipeline with cancellation + backpressure) in Go. Verify it runs `-race`-clean and
     handles cancellation + a failing worker.
  2. `.../elixir/` — solve the identical problem with GenServer + a supervision tree. Verify it runs and a
     crashing worker is supervised/restarted without taking down the system.
  3. `comparison.md` — contrast the two on coordination, backpressure, failure/supervision, testability, and
     observability, with a concrete "when to reach for which" recommendation grounded in the two
     implementations. Verify each claim points at real behaviour in the two codebases.
- **Acceptance criteria**: both implementations solve the same problem correctly (Go race-free; Elixir
  supervised); the comparison is concrete and evidence-backed, not generic; the recommendation is justified.
- **Done bar**: both runnable end-to-end + comparison artifact + web-verified.

---

← Previous: [58 · Compilers, Parsers & Transpilers](./58-compilers-parsers-and-transpilers.md) · Next: [60 · IT Governance & GRC](./60-it-governance-grc.md) →
