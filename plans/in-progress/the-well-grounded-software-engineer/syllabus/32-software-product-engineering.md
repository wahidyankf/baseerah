# 32 · Software Product Engineering ▲ (Annotated-concept, — ‡)

**prd row**: Pass 2 · Depth, Design & Craft · Annotated-concept · — ‡ · Learn 132 / Drill 232 · Nvim-ready
Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the **Product & Delivery** track (`▲`) — product thinking for engineers: discovery,
prioritization, value delivery, and metrics, so engineers build the right thing, not just build the thing
right. Leadership/governance topic (`‡`): **no code** — prose, worked design/decision exercises, and
diagrams. Pairs with [`09-project-management`](./09-project-management.md), which handles delivery
execution.

## Why this exists · the big idea

- **The problem before the solution**: engineers optimize _building the thing right_ and can ship a
  flawless product nobody needs — the most expensive waste in software is a well-built wrong thing.
- **Keep-this-if-you-forget-everything**: start from the user's problem and the outcome, not the feature —
  output is motion, outcome is the point, and the two are easy to confuse.
- **Big ideas touched**: `correctness-vs-pragmatism` (MVP and experiments are deliberately incomplete-but-validated
  bets), `mechanism-vs-policy` (product decides _what_ to build; engineering is the mechanism that builds it).

## Prerequisites

- **Prior topics**: no code prerequisites. Assumes the reader has **built working software** across Pass 1
  (e.g. [topic 11 Backend](./11-backend-essentials.md), [topic 14 Frontend](./14-frontend-essentials.md),
  [topic 15 Testing](./15-software-testing.md)) so product trade-offs land against real building experience.
- **Tools & environment**: a macOS/Linux terminal and a Markdown editor (Neovim per DD-17) for the written
  artifacts; no runtime/toolchain — deliverables are decision documents, not programs.
- **Assumed knowledge**: what it takes to ship a small feature end to end; reading a simple metric/funnel.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: JTBD (Christensen), RICE (Intercom), MoSCoW (DSDM), AARRR/"Pirate Metrics"
  (McClure), and north-star-metric framing are all long-established durable frameworks with no
  canon-changing revision — low risk for a stable-framework topic. (general product-management literature)

## Items

- Product thinking for engineers: user problems vs solutions, outcomes vs output.
- Discovery: user research, problem validation, opportunity assessment, JTBD.
- Prioritization: RICE/MoSCoW/impact–effort, roadmap trade-offs.
- Delivery of value: MVP, iterative delivery, experimentation (A/B), feature flags.
- Metrics: north-star metric, AARRR/funnel, activation/retention.
- Engineer ↔ product ↔ design collaboration; writing good specs.

## Tensions & trade-offs — when NOT to reach for this

- **Discovery vs delivery**: too much research and you analysis-paralyze; too little and you build
  confidently in the wrong direction. The bet is always under uncertainty — validate the _riskiest_
  assumption most cheaply, then commit, rather than researching everything or nothing.
- **MVP vs credibility**: a "minimum" product too thin damages trust and mis-measures demand (users reject
  the execution, not the idea); too fat and you've spent the learning budget before the first signal.
  "Minimum" describes the _hypothesis_ under test, not the smallest possible code.
- **Metrics vs judgment**: a north-star focuses a team, but any single metric is gameable — engagement is
  not value, and a locally optimized funnel can degrade the whole. Quantitative signal informs product
  judgment; it does not replace it.

## Lineage — why it beat the alternative

- Product engineering rose as a reaction to two failures: waterfall's build-the-full-spec-then-discover-it's-wrong
  (1970s–90s), and feature-factory Agile that shipped output velocity while ignoring outcomes. Lean Startup
  (Ries, 2011) reframed the unit of progress as _validated learning_; Jobs-to-be-Done (Christensen) reframed
  features as hired for a job; continuous discovery (Torres) wove research into delivery instead of front-loading
  it. The durable idea beneath the framework churn is singular: _reduce the cost of being wrong_. That is why
  this topic pairs with [`09-project-management`](./09-project-management.md) (deliver the validated thing) and
  matures into the strategic altitude of [`33-engineering-management`](./33-engineering-management.md) /
  [`33-engineering-management`](./33-engineering-management.md).

## Worked examples

Design/decision exercises under `software-product-engineering/learning/` (prose + diagrams; no `code/`
runtime — DD-27 leadership kind).

- **problem-framing** — turn a vague feature request into a validated problem statement + MVP scope.
- **prioritization** — prioritize a sample backlog with RICE and defend the ordering.
- **experiment-design** — design an A/B experiment (hypothesis, metric, guardrail) for a product change.

## Capstone spec — intra-topic (leadership ‡ → design/decision artifact)

- **Goal**: produce a compact **product brief** for a small feature: a validated problem statement (JTBD),
  an MVP scope with explicit non-goals, a RICE-prioritized backlog, a north-star + supporting metrics, and
  an A/B experiment design — a decision artifact an engineer could hand to a team and act on.
- **Concepts exercised**: [ ] problem vs solution framing (JTBD) [ ] MVP scope + non-goals [ ] RICE
  prioritization with defense [ ] north-star + funnel metrics [ ] an A/B experiment (hypothesis/metric/
  guardrail).
- **Ordered steps**:
  1. `software-product-engineering/learning/capstone/brief.md` — problem statement + JTBD + evidence.
     Verify it states the user problem, not a pre-chosen solution.
  2. Add MVP scope + explicit non-goals + a RICE-ranked backlog. Verify each RICE score is justified and the
     ordering is defended.
  3. Add the metrics section (north-star + AARRR funnel) and an A/B experiment design. Verify the
     experiment names a hypothesis, a primary metric, and a guardrail metric.
- **Acceptance criteria**: the brief is internally consistent (scope serves the stated problem; metrics
  measure the outcome; the experiment tests the hypothesis) and defensible without hand-waving.
- **Done bar**: produces the stated artifact (product brief) + web-verified.

## Read more

**Books**

- **Inspired: How to Create Tech Products Customers Love** — Marty Cagan (2008; 2nd ed. 2017). The standard reference on product management and product engineering practice at technology companies.
- **Continuous Discovery Habits** — Teresa Torres (2021). Canonical modern guide to product discovery techniques for cross-functional product teams.
- **The Lean Startup** — Eric Ries (2011). Foundational text on build-measure-learn and validated learning for product development.
- **The Mom Test** — Rob Fitzpatrick (2013). Standard practical reference on running customer discovery conversations that surface truth instead of false validation.
- **Shape Up: Stop Running in Circles and Ship Work that Matters** — Ryan Singer (2019). Free, widely adopted framework for shaping and scoping product work in fixed cycles. <https://basecamp.com/shapeup>

---

← Previous: [31 · Agentic Coding](./31-agentic-coding.md) · Next: [33 · Engineering Management](./33-engineering-management.md) →
