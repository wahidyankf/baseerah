# 89 · Platform Engineering & Developer Experience (Annotated-concept, ‡ no-code)

**prd row**: Pass 5 · Internals & Lead at Altitude · Annotated-concept · ‡ no-code · Learn 189 / Drill 289 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: paving the road for other engineers — internal developer platforms (Backstage-style portals
and IDPs), golden paths, self-service infrastructure, and measuring developer productivity (DORA, SPACE)
without weaponizing the numbers. `‡ no-code`: this is a concept/practice topic; its deliverables are golden-
path templates, a platform contract, and a measurement dashboard rather than an application. It builds on
the operational substrate — [`50-containers-and-orchestration`](./50-containers-and-orchestration.md),
[`51-cloud-and-iac`](./51-cloud-and-iac.md), and [`52-cicd-and-release-engineering`](./52-cicd-and-release-engineering.md)
— and treats those as the platform's raw material.

## Why this exists · the big idea

- **The problem before the solution**: as an org grows, every team reinvents CI, deploy, secrets, and infra
  glue slightly differently; cognitive load explodes and the same problems get solved badly N times. Platform
  engineering exists to factor that shared work out once, as a product, so stream-aligned teams can ship
  without becoming part-time infra experts.
- **Keep-this-if-you-forget-everything**: treat the platform as a product with internal customers — the win
  is a paved golden path that is genuinely easier than the DIY route, offered as self-service, not mandated.
  If the paved road is worse than going off-road, you have built a toll booth, not a platform.
- **Big ideas touched**: `mechanism-vs-policy` (the platform provides mechanism — self-service infra, golden
  paths — while leaving product teams to decide policy; a good platform is opinionated defaults, not a
  straitjacket), `coupling-vs-cohesion` (Team Topologies' platform/stream-aligned split is coupling-and-
  cohesion applied to the org chart — reduce inter-team coupling by giving teams a well-bounded platform
  interface).

## Prerequisites

- **Prior topics**: [topic 50 Containers & Orchestration](./50-containers-and-orchestration.md) (the runtime
  substrate a platform abstracts), [topic 51 Cloud & IaC](./51-cloud-and-iac.md) (the self-service infra a
  platform provisions), and [topic 52 CI/CD & Release Engineering](./52-cicd-and-release-engineering.md) (the
  delivery pipeline golden paths automate).
- **Tools & environment**: no application to build; a developer portal / IDP concept (Backstage-style catalog
  - scaffolder templates), an IaC + CI stack from the prior topics for the golden path to sit on, and a
    DORA/SPACE metrics source. Any scripting for scaffolders or dashboards that uses Python is fully
    type-annotated (DD-34). Neovim/VSCode (DD-17).
- **Assumed knowledge**: containers/orchestration basics (topic 50); IaC and cloud provisioning (topic 51);
  CI/CD pipelines and DORA metrics (topic 52); team/organizational structure trade-offs (topic 33).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: the core frames — Team Topologies' platform-as-a-product and platform/stream-aligned
  team types, golden paths, self-service IDPs, and the DORA four keys plus the SPACE framework for developer
  productivity — are current, widely adopted, and correctly left tool-version-unpinned. Backstage is named as
  the representative open portal but the topic stays tool-agnostic about the specific IDP.
- 2026-07-12 — verified: the CNCF Platforms White Paper is the industry-consensus definition of an internal
  developer platform and is the right anchor for the "what is a platform" framing; no version pin needed.
  (tag-app-delivery.cncf.io/whitepapers/platforms)

## Items

- The platform-as-a-product mindset: internal customers, a platform team, and Team Topologies' interaction
  modes (X-as-a-service vs collaboration).
- Golden paths: the paved, opinionated-default route from `git init` to production that is easier than DIY.
- Internal developer platforms/portals: a software catalog, scaffolder templates, and self-service infra
  (Backstage-style, tool-agnostic).
- Self-service infrastructure: exposing IaC/CI capabilities as safe, guard-railed, self-serve building blocks.
- Measuring developer experience: the DORA four keys and the SPACE framework — what they do and do not capture.
- Metrics without weaponization: measuring the system, not ranking individuals; leading vs lagging signals.

## Tensions & trade-offs — when NOT to reach for this

- **Platform before pain**: a dedicated platform team and an IDP are overhead that a small org cannot amortize.
  Build the platform when repeated, org-wide friction is measurable — not because "platform engineering" is
  fashionable. Prematurely, it is a team maintaining abstractions nobody needed yet.
- **Golden path vs golden cage**: an opinionated path is only a gift if it is genuinely easier and remains
  escapable. Mandate it, or make it worse than the DIY route, and teams route around it — you have added
  coupling and cognitive load instead of removing them. The paved road must win on merit.
- **Metrics as weapons (hard boundary)**: DORA/SPACE measure the delivery _system_. The moment they become
  individual performance rankings or targets, Goodhart's law takes over — people optimize the number, not the
  outcome, and the signal dies. When NOT to use them: never as a stack-ranking or a stick.

## Lineage — why it beat the alternative

- Platform engineering is the current synthesis of two prior swings. First, siloed Dev-throws-to-Ops created
  the friction DevOps set out to remove; but "you build it, you run it" pushed so much operational surface
  onto every product team that cognitive load became the new bottleneck. Platform engineering answers that by
  re-centralizing the _undifferentiated_ heavy lifting — as a self-service product, not a gatekeeping ops
  silo — so teams keep autonomy without each rebuilding the same infra. Team Topologies gave the
  organizational vocabulary (platform vs stream-aligned teams), Accelerate/DORA gave the measurement base,
  and the CNCF codified the definition. What it hands forward: the reliability and operational rigor of the
  paved road feed directly into [`90-site-reliability-engineering`](./90-site-reliability-engineering.md),
  where SLOs and error budgets govern the services the platform helps ship.

## Worked examples

Colocated under `platform-engineering-and-devex/learning/`; the deliverables are platform artifacts —
golden-path templates, a platform contract, and a metrics dashboard — not an application (DD-20/DD-30). Any
scaffolder/dashboard scripting in Python is fully type-annotated (DD-34).

- **beginner** — write a golden-path scaffolder template (a new service pre-wired with CI, containerization,
  and a deploy pipeline from topics 50–52); verify a fresh service generated from it builds and deploys.
- **intermediate** — define the platform contract for one self-service capability (e.g. "request a database"):
  inputs, guard-rails, defaults, and the escape hatch; verify a developer can self-serve it without a ticket.
- **advanced** — build a DORA/SPACE dashboard from real delivery signals and write the anti-weaponization
  policy that governs how it is read; verify the metrics describe the system and cannot be traced to rank an
  individual.

## Capstone spec — intra-topic (subject → paved golden path)

- **Goal**: design and stand up a minimal internal developer platform slice — one golden-path scaffolder that
  takes a new service from nothing to a deployed, monitored state using the topics-50–52 substrate; one
  self-service capability with guard-rails and an escape hatch; and a DORA/SPACE dashboard plus the policy
  that keeps it a system-measurement, not an individual scorecard.
- **Concepts exercised**: [ ] a golden-path scaffolder template [ ] self-service infra with guard-rails +
  escape hatch [ ] a software-catalog/portal entry [ ] a DORA/SPACE dashboard [ ] a metrics anti-weaponization
  policy [ ] platform-as-a-product framing (internal customer + contract).
- **Ordered steps**:
  1. `.../learning/capstone/golden-path/` — a scaffolder template producing a new service pre-wired with CI +
     container + deploy (topics 50–52). Verify a generated service builds and deploys with no hand-editing.
  2. `.../learning/capstone/self-service/` — one capability (e.g. database provisioning) as a guard-railed,
     self-serve building block with a documented escape hatch. Verify a developer provisions it without a
     ticket and the guard-rails block an unsafe request.
  3. `.../learning/capstone/devex-metrics/` — a DORA/SPACE dashboard from delivery signals + a written policy
     on how it may and may not be used. Verify the dashboard reflects real signals and the policy forbids
     individual ranking.
- **Acceptance criteria**: the golden path is genuinely easier than DIY and escapable; the self-service
  capability is guard-railed and ticket-free; the metrics measure the system with an explicit
  anti-weaponization policy; every piece is framed as a product for an internal customer.
- **Done bar**: the golden path produces a deployed service end-to-end + the platform contract and metrics
  policy are documented + web-verified.

## Read more

**Books**

- **Team Topologies** — Matthew Skelton, Manuel Pais (2019). The field-defining model for organizing teams
  (including platform teams) for fast flow; foundational to platform-engineering practice.
- **Platform Engineering: A Guide for Technical, Product, and People Leaders** — Camille Fournier, Ian
  Nowland (2024). The current canonical practitioner book on building internal platforms as products.
- **Accelerate** — Nicole Forsgren, Jez Humble, Gene Kim (2018). The empirical research base (DORA metrics)
  behind modern platform-engineering and developer-experience investment decisions.

**Papers & articles**

- **CNCF Platforms White Paper** — CNCF TAG App Delivery Platforms Working Group (2023). The
  industry-consensus definition and framing of internal developer platforms; free official paper.
  <https://tag-app-delivery.cncf.io/whitepapers/platforms/>

---

← Previous: [88 · Build Your Own Raft / Replicated KV](./88-build-your-own-raft.md) · Next: [90 · Site Reliability Engineering](./90-site-reliability-engineering.md) →
