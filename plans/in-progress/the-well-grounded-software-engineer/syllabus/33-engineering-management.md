# 33 · Engineering Management ‡ (Annotated-concept, no-code)

**prd row**: Pass 2 · Depth, Design & Craft · Annotated-concept · ‡ no-code · Learn 133 / Drill 233 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `‡` leadership/no-code — leading engineers and engineering — the IC→manager transition, 1:1s
& feedback & growth, delivery/planning/estimation at team scale, technical strategy & prioritization, org
health & culture, and leading through influence. The
[topic 30 Software Engineering Practices](./30-software-engineering-practices.md) thread arrives here. Deliverables are **leadership/
decision artifacts**, not code. **Closes Pass 2** and anchors the `capstone-solid-core` inter-topic capstone
that re-engineers the Pass-1 app with everything Pass 2 taught (the whole-journey `capstone-lead-at-altitude`
now anchors at the journey's true close, [`90-site-reliability-engineering`](./90-site-reliability-engineering.md)).

## Why this exists · the big idea

- **The problem before the solution**: the best IC gets promoted and keeps solving every problem
  personally — and the team stalls behind the new bottleneck. Management is a different job, not
  senior-IC-plus: the scarce resource stops being your code and becomes other people's judgment.
- **Keep-this-if-you-forget-everything**: you now succeed _through_ others — your output is the team's
  decisions, growth, and trust, measured in outcomes you no longer type yourself.
- **Big ideas touched**: `correctness-vs-pragmatism` — leadership is disciplined compromise, every
  prioritization, estimate, and staffing call trading an ideal for what ships and holds;
  `mechanism-vs-policy` — a lead sets policy (what matters, who decides) and delegates the mechanism
  rather than owning every _how_.

## Prerequisites

- **Prior topics**: [topic 30 Software Engineering Practices](./30-software-engineering-practices.md) (the
  engineering practices a lead upholds and scales across a team), [topic 32 Software Product Engineering](./32-software-product-engineering.md)
  (strategy, prioritization, product partnership), and [topic 9 Project Management](./09-project-management.md)
  (planning, delivery, and the team process a manager stewards).
- **Tools & environment**: no toolchain — a text editor for the leadership artifacts (a growth plan, a
  strategy doc, a prioritization/decision record); Neovim/VSCode (DD-17). No paid account, no code (DD-20).
- **Assumed knowledge**: mature engineering practices (topic 30); business/product judgment (topic 32);
  project planning + delivery process (topic 09).

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

## Tensions & trade-offs — when NOT to reach for this

- **Autonomy vs alignment**: over-direct and you get a team of hands, not minds; under-direct and effort
  scatters. Set the _what_ and _why_, delegate the _how_ — the lever is context, not control.
- **Delivery vs growth**: shipping this quarter competes with growing people who ship every quarter.
  Over-index on delivery and you spend the team down, stopping the learning that compounds into next
  year's velocity.
- **Metrics vs trust**: DORA keys and velocity focus a team, but any metric is gameable, and measuring
  people as throughput corrodes the trust that actually drives delivery. Metrics inform judgment; they
  don't replace it.
- **Manager vs maker**: staying hands-on keeps technical credibility, but coding on the critical path
  makes you the bottleneck you were promoted to remove — the hardest habit to unlearn.

## Lineage — why it beat the alternative

- Engineering management professionalized as teams outgrew the heroic-lead / player-coach model that
  doesn't scale past a handful of people. Command-and-control factory management (Taylorism) treated
  engineers as interchangeable throughput and failed on creative knowledge work; the pure servant-leader
  reaction under-set direction and drifted. Modern practice converged on a middle — set clear direction
  with high trust, measure outcomes not activity, grow people as the durable asset — evidenced by DORA's
  research base and codified in Fournier's _The Manager's Path_ and Larson's systems view. Conway's Law
  made org design a technical concern (team boundaries become system boundaries), which is why this closes
  Pass 2 on `coupling-vs-cohesion` at org scale, pairs with [`32-software-product-engineering`](./32-software-product-engineering.md)
  (what to build) and [`09-project-management`](./09-project-management.md) (deliver it), and matures into
  the org-level reliability trade-offs of [`90-site-reliability-engineering`](./90-site-reliability-engineering.md).

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

<!-- Inter-topic capstone spec block: this file (last topic of Pass 2) anchors the Pass-2 boundary capstone -->

## Capstone spec — inter-topic: capstone-solid-core (Pass-2 boundary)

- **Weight**: `capstone-solid-core/_index.md` = **435** (section root, after Pass 2 / topic 33). Kind:
  **pass-boundary**, integrating Pass 2 topics 19–33 (design + paradigms + concurrency + algorithms +
  advanced SQL + practices + product/delivery discipline).
- **Goal**: take the **`capstone-first-working-software`** app from Pass 1 and **re-engineer it to a
  professional core**: apply SOLID + patterns (21), choose paradigms deliberately with a functional core
  (22/23), make a hot path concurrent and correct (24), improve an algorithm/complexity (25), tune the
  data layer with `EXPLAIN`-driven indexing (26), wrap it in an engineering workflow — clean git history,
  CI gate, ADRs (30) — and frame the work with product/delivery discipline (32/33). CS-foundations
  reasoning (19) justifies the performance choices.
- **Concepts integrated**: [ ] SOLID + patterns refactor (21) [ ] deliberate paradigm choice + functional
  core (22/23) [ ] safe concurrency on a hot path (24) [ ] an algorithm/complexity improvement (25/19)
  [ ] `EXPLAIN`-driven SQL tuning (26) [ ] CI gate + clean history + ADR (30) [ ] a product brief + delivery
  plan framing the work (32/33).
- **Ordered steps**:
  1. `capstone-solid-core/code/` — import the Pass-1 app under a green test suite; write an ADR stating the
     re-engineering goals (30/32/33). Verify the suite passes against the imported baseline.
  2. Refactor the core to SOLID + patterns with a functional core / imperative shell split (21/22/23).
     Verify behavior is unchanged (suite green) and a new variation can be added without editing closed
     classes (OCP).
  3. Make one hot path concurrent (24) and improve one algorithm/query: apply an index guided by
     `EXPLAIN ANALYZE` (26/25). Verify correctness is preserved and a before/after measurement shows the
     improvement.
  4. Wrap it in the workflow: clean conventional-commit history + a CI pipeline gate (lint→test→build) +
     ADRs; attach the product brief + delivery plan (30/32/33). Verify CI gates the change green and fails
     on a bad commit.
- **Acceptance criteria**: a reader on a clean machine builds and tests the re-engineered app, confirms the
  SOLID/functional-core refactor preserved behavior, sees the measured concurrency/SQL/algorithm
  improvements, and finds the CI gate, clean history, ADRs, and product/delivery artifacts in place — end
  to end, no hidden setup.
- **Done bar**: runnable end-to-end (clean-machine reproduction) + produces the decision artifacts +
  web-verified.

## Read more

**Books**

- **The Manager's Path** — Camille Fournier (2017). The standard reference guide for engineers moving into technical leadership and management roles.
- **An Elegant Puzzle: Systems of Engineering Management** — Will Larson (2019). Widely cited systems-thinking guide to engineering organization design and management.
- **Peopleware: Productive Projects and Teams** — Tom DeMarco & Timothy Lister (1987; 3rd ed. 2013). Classic text on the human and organizational factors that determine software team productivity.
- **Radical Candor: Be a Kick-Ass Boss Without Losing Your Humanity** — Kim Scott (2017; revised ed. 2019). Standard reference framework for direct, caring feedback and effective one-on-ones.

---

← Previous: [32 · Software Product Engineering](./32-software-product-engineering.md) · Next: [34 · NoSQL Databases](./34-nosql-databases.md) →
