# 61 · Engineering Management ‡ (Annotated-concept, no-code)

**prd row**: Pass 5 · Lead at Altitude · Annotated-concept · ‡ no-code · Learn 161 / Drill 261 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `‡` leadership/no-code — the final topic and journey capstone anchor: leading engineers and
engineering — the IC→manager transition, 1:1s & feedback & growth, delivery/planning/estimation at team
scale, technical strategy & prioritization, org health & culture, and leading through influence. The
[topic 22 Software Engineering Practices](./22-software-engineering-practices.md) thread arrives here. Deliverables are **leadership/
decision artifacts**, not code — and file 61 anchors the whole-journey inter-topic capstone.

## Prerequisites

- **Prior topics**: [topic 22 Software Engineering Practices](./22-software-engineering-practices.md) (the
  engineering practices a lead upholds and scales across a team), [topic 24 Software Product Engineering](./24-software-product-engineering.md)
  (strategy, prioritization, product partnership), and [topic 25 Project Management](./25-project-management.md)
  (planning, delivery, and the team process a manager stewards).
- **Tools & environment**: no toolchain — a text editor for the leadership artifacts (a growth plan, a
  strategy doc, a prioritization/decision record); Neovim/VSCode (DD-17). No paid account, no code (DD-20).
- **Assumed knowledge**: mature engineering practices (topic 22); business/product judgment (topic 24);
  project planning + delivery process (topic 25).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: IC→manager transition (e.g. Fournier's _The Manager's Path_), 1:1s, feedback,
  coaching, growth plans, competency ladders, and the **DORA four keys** (deployment frequency, lead time,
  change-failure rate, time-to-restore — still tracked via Google Cloud's annual State of DevOps Report)
  are current/evergreen. No-code topic, nothing version-pinned — nothing to correct.

## Items

- The IC → manager transition: from building software to building teams that build software.
- People: 1:1s, feedback (positive + corrective), coaching, growth plans, competency ladders.
- Delivery: planning, estimation at team scale, prioritization, managing WIP, delivery metrics (DORA
  intuition).
- Strategy: technical strategy, roadmap partnership with product, making + communicating trade-offs.
- Org health: culture, psychological safety, hiring intuition, leading through influence not authority.
- Leading the journey forward: making continuous learning a team norm — closing the whole program by
  turning the individual relearn-and-drill habit into an organizational one.

## Worked examples

Colocated under `engineering-management/learning/artifacts/` (no `code/` — leadership deliverables per the
`‡` shape, DD-27/DD-30).

- **beginner** — a growth-plan artifact for a hypothetical report (strengths, gaps, next-level behaviours).
- **intermediate** — a prioritization/trade-off decision record for a team facing competing demands.
- **advanced** — a one-page technical strategy tying team goals to product outcomes with explicit trade-offs.

## Capstone spec — intra-topic (leadership → decision artifact, no code)

- **Goal**: produce a **leadership decision set** a new engineering lead would actually use — a growth plan
  for a report, a team prioritization/trade-off decision record, and a one-page technical strategy linking
  team goals to product outcomes — demonstrating leadership through structured judgment and clear
  communication. **No code.**
- **Concepts exercised**: [ ] a growth plan (strengths/gaps/next-level behaviours + a feedback frame) [ ] a
  prioritization/trade-off decision record [ ] a one-page technical strategy tying team → product outcomes
  [ ] explicit, communicated trade-offs [ ] leading through influence.
- **Ordered steps**:
  1. `.../learning/capstone/artifacts/growth-plan.md` — a growth plan for a hypothetical report with a
     concrete feedback frame. Verify it names strengths, gaps, and observable next-level behaviours.
  2. `prioritization.md` — a decision record for competing team demands. Verify it states the options, the
     trade-offs, the decision, and the communication plan.
  3. `strategy.md` — a one-page technical strategy linking team goals to product outcomes. Verify every
     technical bet traces to a product outcome and its trade-off is explicit.
- **Acceptance criteria**: the growth plan is actionable and specific; the prioritization record makes
  trade-offs explicit and communicable; the strategy ties team work to product outcomes; the set reads as
  usable leadership judgment. No code.
- **Done bar**: complete leadership artifact set + internally coherent + web-verified.

## Capstone spec — inter-topic: capstone-lead-at-altitude (whole-journey)

> **Weight**: 715 (whole-journey inter-topic capstone — the program's final synthesis). Anchored here as the
> journey closer. Kind: **leadership → design/decision artifact, no code** (per DD-27: leadership `‡`
> capstones produce decision artifacts, not runnable code).

- **Goal**: step into the shoes of a **new engineering lead** and produce the decision artifact set for a
  realistic scenario — take a system through a full arc of judgment spanning the whole program: an
  architecture/technical-direction decision (Pass 3), a reliability + security + governance posture
  (Passes 3–5), a delivery/prioritization plan (Pass 2 collaboration + Pass 5 management), and a
  people-growth and technical-strategy narrative (Passes 2 + 5) — showing that senior engineering leadership
  integrates the technical, the operational, and the human. **No code** — the deliverable is a coherent set
  of leadership/decision artifacts.
- **Concepts exercised**: [ ] a technical-direction / architecture decision record with trade-offs
  [topics 30/32] [ ] a reliability + security + GRC posture summary [topics 38/40/59/60] [ ] a delivery +
  prioritization plan at team scale [topics 25/61] [ ] a people-growth + technical-strategy narrative
  [topics 22/24/61] [ ] an explicit tie back to continuous learning as a team norm.
- **Ordered steps**:
  1. `.../capstone/capstone-lead-at-altitude/adr.md` — an architecture/technical-direction decision record
     for the scenario system, with options + trade-offs + decision. Verify it is a real decision with a
     defensible rationale, not a description.
  2. `posture.md` — a combined reliability (SLO/error-budget), security (controls/threats), and GRC
     (risk/compliance) posture for the system. Verify it integrates all three and traces to concrete risks.
  3. `delivery-and-people.md` — a team-scale delivery/prioritization plan **and** a growth + technical-
     strategy narrative that ties team goals to product outcomes and back to continuous learning. Verify
     every commitment has an owner, a trade-off, and a product rationale.
- **Acceptance criteria**: the architecture decision is defensible with explicit trade-offs; the posture
  integrates reliability + security + governance; the delivery/people narrative is concrete, owned, and
  product-linked; the whole set reads as senior engineering leadership integrating technical + operational +
  human judgment. No code.
- **Done bar**: complete whole-journey leadership artifact set + internally coherent + web-verified.

---

← Previous: [60 · IT Governance & GRC](./60-it-governance-grc.md) · Next: [Syllabus overview](./overview.md) →
