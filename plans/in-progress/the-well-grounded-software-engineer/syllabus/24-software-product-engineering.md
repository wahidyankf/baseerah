# 24 · Software Product Engineering ▲ (Annotated-concept, — ‡)

**prd row**: Pass 2 · Solidify the Core · Annotated-concept · — ‡ · Learn 124 / Drill 224 · Nvim-ready
Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the **Product & Delivery** track (`▲`) — product thinking for engineers: discovery,
prioritization, value delivery, and metrics, so engineers build the right thing, not just build the thing
right. Leadership/governance topic (`‡`): **no code** — prose, worked design/decision exercises, and
diagrams. Pairs with [`25-project-management`](./25-project-management.md), which handles delivery
execution.

## Prerequisites

- **Prior topics**: no code prerequisites. Assumes the reader has **built working software** across Pass 1
  (e.g. [topic 09 Backend](./09-backend-essentials.md), [topic 12 Frontend](./12-frontend-essentials.md),
  [topic 13 Testing](./13-software-testing.md)) so product trade-offs land against real building experience.
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

---

← Previous: [23 · Advanced SQL & Query Performance](./23-advanced-sql-and-query-performance.md) · Next: [25 · Project Management](./25-project-management.md) →
