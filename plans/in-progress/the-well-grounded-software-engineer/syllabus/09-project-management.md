# 9 · Project Management ▲ (Annotated-concept, — ‡)

**prd row**: Pass 1 · Core Foundations · Annotated-concept · — ‡ · Learn 109 / Drill 209 · Nvim-ready
Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the **Product & Delivery** track (`▲`) — delivery execution: methodologies, the triple
constraint, planning/estimation, execution mechanics, metrics, and risk/change management. Leadership
topic (`‡`): **no code** — prose, worked design/decision exercises, and diagrams. A **Pass-1** ▲ topic read
early so delivery discipline is available from the start; the Pass-2 boundary `capstone-solid-core` anchors
later at [`33-engineering-management`](./33-engineering-management.md), where people leadership also deepens.

## Why this exists · the big idea

- **The problem before the solution**: work that isn't planned, sequenced, and de-risked slips silently —
  the schedule is already late before anyone notices, because nobody made the constraints and dependencies visible.
- **Keep-this-if-you-forget-everything**: scope, schedule, and cost are one triangle — you can fix any two,
  and pretending you can fix all three is how projects fail. Make the trade-off explicit and chosen.
- **Big ideas touched**: `correctness-vs-pragmatism` (estimation and risk are decisions under uncertainty,
  not precision), `coupling-vs-cohesion` (task dependencies are coupling, and the critical path is the tightest chain).

## Prerequisites

- **Prior topics**: no code prerequisites. Pairs with
  [topic 32 Software Product Engineering](./32-software-product-engineering.md) (what to build → how to
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

## Tensions & trade-offs — when NOT to reach for this

- **Methodology fit**: Scrum's cadence suits evolving product work with an engaged customer; Kanban suits
  continuous-flow and ops; waterfall genuinely fits fixed-scope, high-cost-of-change domains (regulated,
  hardware). Cargo-culting Scrum onto the wrong context adds ceremony without the benefit it was built for.
- **Estimation honesty**: points and velocity beat hour-estimates because they embrace uncertainty instead of
  faking precision — but velocity becomes a lie the instant it's used as a productivity target (Goodhart
  again). Estimates inform a commitment; they don't remove the need for slack.
- **Process weight**: standups, planning, and retros cost real hours; on a two-person project they can exceed
  the coordination they save. Match process to the number of communication paths — which grow with the square
  of team size, not linearly.

## Lineage — why it beat the alternative

- Project management formalized in mid-20th-century large engineering — critical-path method and PERT (1950s,
  US Navy / DuPont) — where sequencing dependencies on enormous projects was the binding constraint. Agile
  (2001 Manifesto) was a reaction against heavyweight plan-everything-upfront PM failing on software's high
  rate of change: it moved the constraint from _following the plan_ to _responding to change_, and Kanban
  imported lean-manufacturing flow. The invariant across all of it is one move — make work, constraints, and
  risk _visible_, so trade-offs are chosen rather than stumbled into. That visibility discipline is exactly
  what [`33-engineering-management`](./33-engineering-management.md) scales up to people and organizations.

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

## Read more

**Books**

- **The Mythical Man-Month** — Frederick P. Brooks Jr. (Anniversary ed., 1995; orig. 1975). Seminal essays on software project management; origin of Brooks's Law.
- **Peopleware: Productive Projects and Teams** — DeMarco, Lister (3rd ed., 2013; orig. 1987). Classic argument that software success is a human/organizational problem first.
- **Software Estimation: Demystifying the Black Art** — Steve McConnell (2006). Standard practical reference for estimation techniques.

**Papers & articles**

- **Manifesto for Agile Software Development** — Kent Beck + 16 co-signatories (2001). Founding document of agile: four values, twelve principles. <https://agilemanifesto.org/>

---

← Previous: [8 · Object-Oriented Programming Essentials](./08-object-oriented-programming-essentials.md) · Next: [10 · SQL Essentials](./10-sql-essentials.md) →
