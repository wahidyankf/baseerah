# 25 · Project Management ▲ (Annotated-concept, — ‡)

**prd row**: Pass 2 · Solidify the Core · Annotated-concept · — ‡ · Learn 125 / Drill 225 · Nvim-ready
Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the **Product & Delivery** track (`▲`) — delivery execution: methodologies, the triple
constraint, planning/estimation, execution mechanics, metrics, and risk/change management. Leadership
topic (`‡`): **no code** — prose, worked design/decision exercises, and diagrams. Closes Pass 2 and
anchors the `capstone-solid-core` inter-topic capstone that re-engineers the Pass-1 app with everything
Pass 2 taught. People leadership deepens in [`61-engineering-management`](./61-engineering-management.md).

## Prerequisites

- **Prior topics**: no code prerequisites. Pairs with
  [topic 24 Software Product Engineering](./24-software-product-engineering.md) (what to build → how to
  deliver it); assumes Pass-1/Pass-2 building experience so estimation and scope trade-offs are concrete.
- **Tools & environment**: a macOS/Linux terminal and a Markdown editor (Neovim per DD-17) for the plans
  and charts; no runtime — deliverables are planning artifacts and decision documents.
- **Assumed knowledge**: what a feature's worth of work feels like; reading a simple chart; the idea of a
  dependency between tasks.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: the current official **Scrum Guide is the November 2020 revision** (Schwaber &
  Sutherland) — no newer official revision supersedes it in 2026; cite the 2020 guide specifically
  (13 pages, softened prescriptive language). Critical-path method, burndown/cycle-time/lead-time metrics,
  and story-point/velocity estimation remain standard unchanged PM vocabulary. (scrumguides.org)

## Items

- Delivery methodologies: waterfall vs agile (Scrum/Kanban), hybrid; when each fits.
- Scope, schedule, cost — the triple constraint; trade-offs.
- Planning: work breakdown, estimation (story points/velocity, planning-poker pitfalls), dependencies,
  critical path.
- Execution: backlog, sprints, standups, risk/issue tracking, stakeholder communication.
- Metrics: burndown/burnup, cycle time, lead time, throughput.
- Risk & change management; retrospectives & continuous improvement.

## Worked examples

Design/decision exercises under `project-management/learning/` (prose + diagrams; no `code/` runtime —
DD-27 leadership kind).

- **wbs-critical-path** — break a sample feature into a WBS + dependency graph; identify the critical path.
- **estimation** — run an estimation worked example and show why velocity beats hours.
- **burndown-diagnosis** — interpret a burndown chart to diagnose a slipping sprint + corrective action.

## Capstone spec — intra-topic (leadership ‡ → design/decision artifact)

- **Goal**: produce a compact **delivery plan** for a small project: a work-breakdown structure with a
  dependency graph and critical path, a velocity-based estimate, a sprint/backlog plan, a risk register,
  and a metrics plan (burndown + cycle time) — a decision artifact a team could execute against.
- **Concepts exercised**: [ ] WBS + dependency graph + critical path [ ] velocity/story-point estimation
  [ ] a sprint/backlog plan [ ] a risk register with mitigations [ ] a metrics plan (burndown/cycle time).
- **Ordered steps**:
  1. `project-management/learning/capstone/plan.md` — the WBS + a Mermaid dependency graph; mark the
     critical path. Verify the critical path is the longest dependency chain in the graph.
  2. Add a velocity-based estimate + a sprint/backlog breakdown. Verify the estimate uses points/velocity
     (not raw hours) and the sprint plan respects dependencies.
  3. Add a risk register (likelihood/impact/mitigation) + a metrics plan. Verify each top risk has a
     concrete mitigation and each metric names what decision it informs.
- **Acceptance criteria**: the plan is internally consistent (critical path drives the schedule; estimates
  and sprints align; risks have mitigations) and executable without hand-waving.
- **Done bar**: produces the stated artifact (delivery plan) + web-verified.

<!-- Inter-topic capstone spec block: this file anchors the Pass-2 boundary capstone -->

## Capstone spec — inter-topic: capstone-solid-core (Pass-2 boundary)

- **Weight**: `capstone-solid-core/_index.md` = **355** (section root, after Pass 2). Kind:
  **pass-boundary**, integrating Pass 2 topics 15–25 (design + paradigms + concurrency + algorithms +
  advanced SQL + practices + product/delivery discipline).
- **Goal**: take the **`capstone-first-working-software`** app from Pass 1 and **re-engineer it to a
  professional core**: apply SOLID + patterns (16), choose paradigms deliberately with a functional core
  (17/18), make a hot path concurrent and correct (19), improve an algorithm/complexity (20), tune the
  data layer with `EXPLAIN`-driven indexing (23), wrap it in an engineering workflow — clean git history,
  CI gate, ADRs (22) — and frame the work with product/delivery discipline (24/25). CS-foundations
  reasoning (15) justifies the performance choices.
- **Concepts integrated**: [ ] SOLID + patterns refactor (16) [ ] deliberate paradigm choice + functional
  core (17/18) [ ] safe concurrency on a hot path (19) [ ] an algorithm/complexity improvement (20/15)
  [ ] `EXPLAIN`-driven SQL tuning (23) [ ] CI gate + clean history + ADR (22) [ ] a product brief + delivery
  plan framing the work (24/25).
- **Ordered steps**:
  1. `capstone-solid-core/code/` — import the Pass-1 app under a green test suite; write an ADR stating the
     re-engineering goals (22/24/25). Verify the suite passes against the imported baseline.
  2. Refactor the core to SOLID + patterns with a functional core / imperative shell split (16/17/18).
     Verify behavior is unchanged (suite green) and a new variation can be added without editing closed
     classes (OCP).
  3. Make one hot path concurrent (19) and improve one algorithm/query: apply an index guided by
     `EXPLAIN ANALYZE` (23/20). Verify correctness is preserved and a before/after measurement shows the
     improvement.
  4. Wrap it in the workflow: clean conventional-commit history + a CI pipeline gate (lint→test→build) +
     ADRs; attach the product brief + delivery plan (22/24/25). Verify CI gates the change green and fails
     on a bad commit.
- **Acceptance criteria**: a reader on a clean machine builds and tests the re-engineered app, confirms the
  SOLID/functional-core refactor preserved behavior, sees the measured concurrency/SQL/algorithm
  improvements, and finds the CI gate, clean history, ADRs, and product/delivery artifacts in place — end
  to end, no hidden setup.
- **Done bar**: runnable end-to-end (clean-machine reproduction) + produces the decision artifacts +
  web-verified.

---

← Previous: [24 · Software Product Engineering](./24-software-product-engineering.md) · Next: [26 · NoSQL Databases](./26-nosql-databases.md) →
