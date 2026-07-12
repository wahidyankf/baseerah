# 90 · Site Reliability Engineering (Annotated-concept, Python \*)

**prd row**: Pass 5 · Internals & Lead at Altitude · Annotated-concept · Python \* · Learn 190 /
Drill 290 · Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `*` concept-centric (Python where code appears) — operating software reliably at scale:
SLIs/SLOs/error budgets, the four golden signals, observability (metrics/logs/traces), alerting on symptoms
not causes, incident response & blameless postmortems, toil reduction, and capacity/load. The **journey
closer** — this final topic anchors the whole-journey `capstone-lead-at-altitude` (the Pass-4 concurrency
capstones now anchor at [`85-compilers-parsers-and-transpilers`](./85-compilers-parsers-and-transpilers.md),
the Pass-4 closer). Builds on the ops/observability threads
([`15-software-testing`](./15-software-testing.md),
[`50-containers-and-orchestration`](./50-containers-and-orchestration.md)).

## Why this exists · the big idea

- **The problem before the solution**: every service fails eventually, and chasing 100% uptime is both
  impossible and ruinously expensive. Without a principled target, teams either over-invest in reliability
  nobody needs or get blindsided by the outage that actually matters.
- **Keep-this-if-you-forget-everything**: reliability is a feature you _budget_, not a binary you promise —
  measure user-facing symptoms (SLIs), set an SLO, spend the error budget on velocity, alert on symptoms,
  and learn blamelessly.
- **Big ideas touched**: `consistency-latency-throughput` — the golden signals and error budget are where
  the distributed-systems trilemma becomes an operational dial; `determinism-vs-emergence` — a system at
  scale behaves in ways no one designed, so you observe and respond to emergent behaviour rather than
  predict it; `correctness-vs-pragmatism` — 100% is the wrong target, and the error budget makes "reliable
  enough" a disciplined, negotiated compromise.

## Prerequisites

- **Prior topics**: [topic 39 Backend at Scale](./39-backend-at-scale.md) (the services being operated),
  [topic 50 Containers & Orchestration](./50-containers-and-orchestration.md) (where they run), and
  [topic 44 System Design](./44-system-design.md) (load, capacity, failure modes).
- **Tools & environment**: **Python 3.x** for the runnable mechanisms; a local metrics stack
  (Prometheus-style scrape + a dashboard) runnable via containers (DD-20); Neovim/VSCode (DD-17).
- **Assumed knowledge**: running a backend service (topic 39); containers/orchestration basics (topic 50);
  reasoning about load + failure (topic 44).

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
- OpenTelemetry (OTel): vendor-neutral instrumentation — traces, metrics, and logs through one
  SDK/collector — as the standard observability pipeline.
- Alerting: symptom-based (not cause-based) alerts; alert fatigue; on-call intuition.
- Incident response: severity, incident command, blameless postmortems, action items.
- Toil & automation: identifying toil, eliminating it; capacity planning & load (concept).

## Tensions & trade-offs — when NOT to reach for this

- **Reliability vs velocity**: every extra nine of uptime costs exponentially more and slows shipping. The
  error budget exists precisely so the trade is explicit and owned, not re-argued case by case.
- **Symptom vs cause alerting**: alert on causes and you drown in noise for failures users never felt;
  alert only on symptoms and a slow-burning root cause can hide. Symptom/SLO-burn alerts that page, plus
  diagnostic signals that don't, split the difference.
- **Coverage vs alert fatigue**: more alerts feel safer, but page-storms desensitize on-call and the real
  incident gets missed. Fewer, symptom-based, budget-burn alerts beat exhaustive cause alerts.
- **Automating toil vs its cost**: automation frees humans, but every automation is itself a system to
  maintain that can fail worse than the manual step. Automate the repetitive and reversible first.

## Lineage — why it beat the alternative

- SRE emerged at Google (Ben Treynor, ~2003) as the answer to a structural conflict: developers want to
  ship, a separate ops team wants to freeze, and the split produces either fragile speed or safe
  stagnation. SRE dissolved the conflict by making reliability a measurable, budgeted engineering concern
  owned jointly — SLOs quantify "reliable enough," the error budget turns the dev-vs-ops fight into a
  shared number, and blameless postmortems (borrowed from aviation and medicine safety culture) replaced
  blame with systemic learning. It beat both the throw-it-over-the-wall model and the "just add more nines"
  instinct because it made the trade-off explicit rather than political. As the journey's closer it gathers
  the program's operational threads — [`15-software-testing`](./15-software-testing.md),
  [`50-containers-and-orchestration`](./50-containers-and-orchestration.md),
  [`39-backend-at-scale`](./39-backend-at-scale.md), [`44-system-design`](./44-system-design.md) — into the
  altitude question every senior engineer eventually owns: not "does it work?" but "how reliable does it
  need to be, and what will we trade for that?" — which is why it anchors `capstone-lead-at-altitude`.

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

<!-- Inter-topic capstone spec block: this file (the journey's final topic) anchors the whole-journey capstone -->

## Capstone spec — inter-topic: capstone-lead-at-altitude (whole-journey)

> **Weight**: `capstone-lead-at-altitude/_index.md` = **1005** (section root, after the journey's final
> topic 90). Kind: **whole-journey synthesis → leadership/decision artifact + a shipped system**. The
> capstone of the entire program — it looks back across all 90 topics.

- **Goal**: act as the **technical lead of the whole journey** — take one of the earlier runnable systems (the
  `capstone-concurrency-and-systems` service from [`85-compilers-parsers-and-transpilers`](./85-compilers-parsers-and-transpilers.md)
  or the `capstone-real-world-delivery` app) and **operate it at altitude**: define its SLOs and reliability
  posture (topic 90), author a technical strategy + prioritization record that a team could execute (topics
  32/33), and produce a whole-journey **retrospective** that names, per pass, what the relearn-and-drill habit
  changed — closing the program by turning the individual learning loop into an organizational one.
- **Concepts integrated**: [ ] SLIs/SLOs/error budgets + a golden-signals dashboard on a real service (90)
  [ ] a one-page technical strategy tying team → product outcomes (32/33) [ ] a prioritization/trade-off
  decision record for the service's roadmap (33) [ ] a growth-plan + leading-through-influence frame (33)
  [ ] a whole-journey retrospective mapping each pass (P0–P5) to a concrete capability gained.
- **Ordered steps**:
  1. `capstone-lead-at-altitude/code/` — take an earlier capstone service, define its SLI/SLO/error budget,
     and stand up a golden-signals dashboard + a symptom-based alert (90). Verify the SLO alert fires on an
     injected violation and the dashboard reflects load.
  2. `strategy.md` + `prioritization.md` — a one-page technical strategy linking the service's reliability
     work to product outcomes, and a prioritization record for its roadmap under an error-budget constraint
     (32/33). Verify every bet traces to an outcome and each priority states its trade-off.
  3. `retrospective.md` — a whole-journey retrospective: for each pass (P0 Editor Foundations → P5 Internals &
     Lead at Altitude) name one capability the relearn-and-drill habit produced, and the organizational
     practice that would sustain it. Verify every pass is covered and each entry is concrete, not generic.
- **Acceptance criteria**: the chosen service is genuinely operable (SLO + alert + dashboard work); the
  strategy and prioritization artifacts are executable and trade-off-explicit; the retrospective covers all
  six passes with concrete, evidence-backed capabilities; the set reads as a lead's altitude view of the
  whole journey.
- **Done bar**: the service is runnable + observable, the leadership artifacts are internally coherent, the
  whole-journey retrospective is complete + web-verified.

## Read more

**Books**

- **Site Reliability Engineering: How Google Runs Production Systems** — Betsy Beyer, Chris Jones, Jennifer Petoff, Niall Richard Murphy (eds.) (2016). The book that defined SRE as a discipline; free online from Google. <https://sre.google/sre-book/table-of-contents/>
- **The Site Reliability Workbook** — Betsy Beyer, Niall Richard Murphy, David K. Rensin, Kent Kawahara, Stephen Thorne (eds.) (2018). The hands-on companion applying SRE principles (SLOs, error budgets, incident response) in practice; free online. <https://sre.google/workbook/table-of-contents/>
- **Seeking SRE** — David N. Blank-Edelman (ed.) (2018). Widely cited collection of essays showing how SRE principles are adapted across diverse organizations.
- **Implementing Service Level Objectives** — Alex Hidalgo (2020). The definitive practical guide to SLIs, SLOs, and error budgets, the core measurement toolkit of SRE.

---

← Previous: [89 · Platform Engineering & Developer Experience](./89-platform-engineering-and-devex.md) · Next: [Syllabus overview](./overview.md) →
