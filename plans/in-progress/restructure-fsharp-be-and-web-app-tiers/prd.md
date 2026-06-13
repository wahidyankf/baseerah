---
title: Product Requirements — Restructure Backends to F# and Split Web Tiers
description: Personas, user stories, Gherkin acceptance criteria, and product scope for the generic F# backend rewrite (organiclever-be in place, ose-app-be → ose-be) with preserved OpenRouter, crane removal, organiclever web split + rename, the repo-wide -www public-site renames (ose-web → ose-www, wahidyankf-web → wahidyankf-www, ayokoding-web → ayokoding-www), shared design-system lib, and new-marketing-site simplification.
---

# Product Requirements: Restructure Backends to F# and Split Web Tiers

## Product Overview

Two changes ship together:

1. **Backend tier → F# (generic `<product>-be`).** `ose-app-be` is renamed → `ose-be` and ported
   Rust→F# behaviorally equivalent on its preserved contract (minus media; OpenRouter LLM integration
   preserved as core). `organiclever-be` is rewritten **in place** (name kept) into a **real** F#
   backend with minimal `journal` CRUD (mirroring the existing PGlite client schema). Crane and the
   PDF-to-Markdown feature disappear from the product.
2. **Web tier → two-tier parity + `-www` naming.** OrganicLever's single `organiclever-web` splits
   into a simple marketing site (`organiclever-www`) and a CSR app (`organiclever-app-web`). The
   repo-wide rule — `-www` = public website at the domain root (deployment role), `-app-web` = app web
   client at `app.*` — renames the existing public-website sites `ose-web` → `ose-www`,
   `wahidyankf-web` → `wahidyankf-www`, and `ayokoding-web` → `ayokoding-www`. A shared `libs/ts-ui`
   design system feeds the app web clients + the new `organiclever-www`, which collapses to the
   lightweight `wahidyankf-www` pattern; established content platforms (`ose-www`, `ayokoding-www`)
   keep their existing internals.

Each F# backend mirrors `ose-primer/apps/crud-be-fsharp-giraffe`: Giraffe over a hexagonal `Contexts/`
layout, EF Core 10 on Npgsql, DbUp run-on-boot migrations, and a NATS.Net JetStream durable demo.

The organiclever app stays **local-first (PGlite)** in this plan; the journal CRUD on
`organiclever-be` ships **unconsumed but contract-smoke-tested**. The production cutover for the new
domains (and the `prod-ose-web` → `prod-ose-www` / `prod-wahidyankf-web` → `prod-wahidyankf-www` /
`prod-ayokoding-web` → `prod-ayokoding-www` prod-branch renames) is **deferred downstream**.

```mermaid
%% Color-blind-friendly palette: blue #0173B2, orange #DE8F05, green #029E73, purple #CC78BC, grey #808080
flowchart TB
  subgraph BE["Backend tier (F#)"]
    SPEC["OpenAPI contracts<br/>(media removed)"]
    GEN["generated-contracts<br/>(F# types)"]
    H["Giraffe handlers"]
  end
  subgraph FE["Web tier"]
    UI["libs/ts-ui<br/>shared design system"]
    WWW["marketing sites<br/>(wahidyankf pattern)"]
    APP["app sites (CSR)"]
  end

  SPEC --> GEN --> H
  UI --> WWW
  UI --> APP

  linkStyle default stroke:#808080,stroke-width:1px

  style BE fill:#FFFFFF,stroke:#000000,color:#000000
  style FE fill:#FFFFFF,stroke:#000000,color:#000000
  style SPEC fill:#0173B2,stroke:#000000,color:#FFFFFF
  style GEN fill:#DE8F05,stroke:#000000,color:#000000
  style H fill:#029E73,stroke:#000000,color:#000000
  style UI fill:#CC78BC,stroke:#000000,color:#000000
  style WWW fill:#0173B2,stroke:#000000,color:#FFFFFF
  style APP fill:#029E73,stroke:#000000,color:#000000
```

## Personas

This solo-maintainer repo's personas are the hats worn and the agents/services that consume the output.

- **Backend maintainer** — rewrites both backends to F#, builds journal CRUD, removes crane; wants
  idiomatic F#, a preserved ose-app-be contract, and green gates.
- **Frontend maintainer** — splits + renames the organiclever web tier, renames the existing
  public-website sites to the `-www` suffix (`ose-web` → `ose-www`, `wahidyankf-web` →
  `wahidyankf-www`, `ayokoding-web` → `ayokoding-www`), builds `libs/ts-ui`, simplifies the new
  marketing site; wants consistent structure, a legible deployment-role naming, a shared design
  system, no brand drift.
- **Infra operator (ose-infra)** — pulls the two generic public F# GHCR images
  (`organiclever-be`, `ose-be`) early; wants images that build, run DbUp migrations on boot, connect
  to NATS, and serve the contract. Owns the deferred prod cutover.
- **Contract consumer (`ose-app-web` / external)** — calls the preserved `ose-be` (renamed from
  `ose-app-be`) non-media endpoints; must see no breaking change beyond the removed media path (and
  the codegen source pointer updated to `ose-be`).
- **E2E runners** — Playwright black-box clients that drive a running backend (BE e2e) or a built site
  (FE e2e) over real HTTP, asserting preserved behavior, the JetStream demo, and rendered marketing/app
  surfaces.

## User Stories

- As the **backend maintainer**, I want `ose-app-be` renamed to `ose-be` and rewritten to F# with all
  six non-media bounded contexts intact (including the OpenRouter LLM integration) so that no
  functionality is lost and its contract is preserved.
- As the **backend maintainer**, I want `organiclever-be` rewritten in place (name kept) and given
  real `journal` CRUD so that it is a genuine backend, not an empty skeleton.
- As the **backend maintainer**, I want the journal CRUD schema mirrored from the existing PGlite
  client model so that the eventual consumption decision does not force a rewrite.
- As the **backend maintainer**, I want the crane media service and PDF-to-Markdown fully removed so
  that the deploy surface is two generic backends, not three services.
- As the **infra operator**, I want two **bootable** generic public GHCR images
  (`organiclever-be`, `ose-be`) published **early** so that the k3s Phase 0.5 gate unblocks before the
  rest of the restructure finishes.
- As the **frontend maintainer**, I want `organiclever-web` (the app) renamed to `organiclever-app-web`
  and a new simple `organiclever-www` marketing site so that OrganicLever matches OSE's two-tier shape.
- As the **frontend maintainer**, I want the existing public-website sites renamed to the `-www`
  suffix (`ose-web` → `ose-www`, `wahidyankf-web` → `wahidyankf-www`, `ayokoding-web` →
  `ayokoding-www`) so that every public website's deployment role is legible from its name under the
  `-www` / `-app-web` rule.
- As the **frontend maintainer**, I want a shared `libs/ts-ui` consumed by the app web clients + the
  new `organiclever-www` so that brand and components stay consistent across each product's
  `www → app` jump.
- As the **frontend maintainer**, I want the new marketing site (`organiclever-www`) on the simple
  `wahidyankf-www` `features/` pattern (and `ose-www` structure-only simplified) so that marketing
  stays lightweight while the apps keep the heavier CSR/DDD stack and the content platforms
  (`ose-www`, `ayokoding-www`) keep their existing internals.
- As the **spec maintainer**, I want the `specs/` surfaces renamed (organiclever web tier + the new
  `behavior/organiclever-www/` marketing surface; OSE backend `app-be` → `be`; ayokoding-web →
  ayokoding-www), the marketing tier added, and crane-be/media removed so that the specs match the new
  project topology.

## Acceptance Criteria (Gherkin)

> Every scenario uses exactly one primary `Given`, one `When`, one `Then`; extras chain with
> `And`/`But`. These scenarios seed the first failing tests for the matching delivery phases.

### Prerequisite gate

```gherkin
Scenario: the toolchain-parity prerequisite is confirmed done before restructure work
  Given the standardize-repo-toolchain-parity plan is expected to be complete
  When Phase 0 checks for the converged F# Nx targets, the doctor .NET SDK check, and F# coverage tooling
  Then all three are present and pass
  And the restructure phases are cleared to begin
```

### F# backends build and boot

```gherkin
Scenario: both backends build as F# services
  Given ose-be and organiclever-be have been written in F# Giraffe on net10.0
  When nx build ose-be organiclever-be runs
  Then the .NET release artifacts are produced
  And no Rust toolchain is invoked for either backend
```

```gherkin
Scenario: a backend runs DbUp migrations on boot
  Given a backend starts against an empty database with DATABASE_URL set
  When the backend process boots
  Then DbUp applies all embedded db/migrations/*.sql before serving requests
  And the backend reports healthy after migrations complete
```

### Early bootable images unblock k3s

```gherkin
Scenario: two bootable F# images publish before the feature ports complete
  Given the F# skeletons boot, migrate, connect NATS, and serve /health
  When the publish-images workflow runs at Phase 2
  Then ghcr.io/wahidyankf/ose-be and ghcr.io/wahidyankf/organiclever-be are publicly pullable
  And no crane-be image job exists
  But no web-tier container image is published
```

### ose-be contract preserved minus media (OpenRouter intact)

```gherkin
Scenario: the ose-be OpenAPI contract still validates after media removal
  Given the media path has been removed from the ose-be OpenAPI contract
  When the contract lint and bundle target runs
  Then the contract validates and bundles successfully
  And every previously documented non-media path is still present
```

```gherkin
Scenario: all six ose-be bounded contexts are preserved
  Given ose-be has been renamed from ose-app-be and ported to F#
  When the spec-coverage target runs over its behavior specs
  Then health, ai-orchestration, gap-analysis, internal-policy, and regulatory-source steps are all bound
  And the db migrations.feature steps are bound via DbUp infrastructure
  And no bounded context is missing
```

```gherkin
Scenario: the ose-be OpenRouter LLM integration is preserved as core
  Given ose-be has been ported to F# with the media feature removed
  When the gap-analysis context is inspected
  Then it retains an OpenRouter HTTP client adapter driven by OSE_BE_OPENROUTER_* env vars
  And OSE_BE_OPENROUTER_API_KEY appears only as a placeholder in .env.example
```

### organiclever-be is a real backend

```gherkin
Scenario: organiclever-be serves journal CRUD mirrored from the PGlite client schema
  Given organiclever-be has been rewritten in place in F# with a journal context
  When a contract smoke-probe exercises the journal CRUD endpoints
  Then create, read, update, and delete each return their expected status and shape
  And the journal entity schema matches the existing PGlite client model
```

```gherkin
Scenario: the journal CRUD ships unconsumed in this plan
  Given the web-to-backend consumption decision is deferred
  When organiclever-app-web is inspected
  Then it still uses local-first PGlite for journal data
  And it does not depend on organiclever-be for journal persistence
```

### Crane and media fully removed

```gherkin
Scenario: the crane service and its e2e runner no longer exist
  Given the restructure is complete
  When the apps directory is inspected
  Then apps/crane-be does not exist
  And apps/crane-be-e2e does not exist
```

```gherkin
Scenario: no media or crane references remain in apps or specs
  Given the restructure is complete
  When a search for crane, media, pdf-to-md, and crane.convert runs over apps and specs
  Then no crane_client module is found
  And no /media/pdf-to-md route is found
  And no crane.convert subject usage is found
  But crane-cli and fsharp-crane-core are untouched
```

### Renames applied cleanly

```gherkin
Scenario: the organiclever web tier is renamed to the *-app-* family
  Given the organiclever web-tier rename has been applied as one atomic change
  When nx show projects runs
  Then organiclever-app-web and organiclever-app-web-e2e exist
  And the old app-flavored organiclever-web is gone
  But organiclever-be keeps its name (in-place rewrite, not renamed)
```

```gherkin
Scenario: the OSE backend is renamed to the generic ose-be name
  Given the ose-app-be to ose-be rename has been applied as one atomic change
  When nx show projects runs
  Then ose-be and ose-be-e2e exist
  And the old ose-app-be and ose-app-be-e2e project names are gone
  And the full affected build passes
```

### New marketing site

```gherkin
Scenario: a new simple organiclever-www marketing site is created from the landing context
  Given the landing context has been extracted from the former organiclever-web
  When nx build organiclever-www runs
  Then the marketing site builds using the src/features layout matching wahidyankf-www
  And it does not depend on PGlite, Effect, or XState
```

### Public-website tier renamed to -www

```gherkin
Scenario: the existing public-website sites are renamed to the -www suffix
  Given ose-web, wahidyankf-web, and ayokoding-web have been renamed under the repo-wide -www rule
  When nx show projects runs
  Then ose-www, wahidyankf-www, and ayokoding-www exist with their renamed e2e pairs
  And the old ose-web, wahidyankf-web, and ayokoding-web project names are gone
  And the full affected build passes
```

```gherkin
Scenario: ayokoding-www keeps its existing structure and tRPC after the mechanical rename
  Given ayokoding-web has been renamed to ayokoding-www
  When apps/ayokoding-www is inspected
  Then it retains its existing content structure and tRPC pipeline
  And it does not adopt the simple features/ pattern or libs/ts-ui
```

### Shared design system

```gherkin
Scenario: libs/ts-ui is consumed by its three frontend consumers
  Given libs/ts-ui has been created and its consumers have adopted it
  When nx graph is inspected
  Then organiclever-www, organiclever-app-web, and ose-app-web each depend on libs/ts-ui
  And libs/ts-ui builds independently
  But ose-www and ayokoding-www do not depend on libs/ts-ui (content platforms, not forced)
```

### Marketing-site simplification

```gherkin
Scenario: ose-www is simplified structure-only while keeping its content pipeline
  Given ose-www (renamed from ose-web) has been reshaped to the wahidyankf-www features layout
  When nx build ose-www and its fe-e2e run
  Then ose-www uses the src/features layout
  And its tRPC content/updates/feed/rss pipeline still renders
```

### JetStream demo on NATS.Net

```gherkin
@e2e
Scenario: a backend publishes and durably consumes its demo subject with ack
  Given a backend has a JetStream durable stream and consumer for its demo subject
  When the backend publishes a demo message to that subject
  Then the durable consumer receives the message
  And the message is acknowledged
  And the messaging status surface reports the demo delivered and acked
```

### Specs restructured

```gherkin
Scenario: the organiclever specs match the new project topology
  Given the restructure is complete
  When specs/apps/organiclever is inspected
  Then the web behavior and component surfaces reflect organiclever-app-web
  And the behavior/organiclever-be surface is kept (backend name unchanged) with journal Gherkin
  And a marketing-tier surface for organiclever-www exists
  But no media or crane references remain
```

```gherkin
Scenario: the ose backend specs are renamed to match ose-be
  Given the ose-app-be to ose-be rename has been applied
  When specs/apps/ose is inspected
  Then behavior/be and components/be exist (renamed from app-be)
  And platform-web is annotated as ose-www
  But no media or crane references remain
```

```gherkin
Scenario: the ayokoding specs reference ayokoding-www
  Given ayokoding-web has been renamed to ayokoding-www
  When specs/apps/ayokoding is inspected
  Then behavior/ayokoding-www exists (renamed from behavior/ayokoding-web)
  And no ayokoding-web reference remains in the ayokoding specs
```

```gherkin
Scenario: crane-be specs are removed but crane-cli specs are kept
  Given crane-be has been deleted
  When specs/apps/crane is inspected
  Then the crane-be behavior and component surfaces are gone
  And the crane-cli behavior and component surfaces are intact
```

### Dependency preservation

```gherkin
Scenario: the crane-cli and rust-commons dependents are preserved
  Given crane-be has been removed and the backends are F#
  When the dependency graph is inspected
  Then apps/crane-cli still references libs/fsharp-crane-core
  And apps/ayokoding-cli and apps/ose-cli still reference libs/rust-commons
```

### Env drift guard

```gherkin
Scenario: env drift guard passes with renamed/F# vars and without crane vars
  Given the F# backend env vars are annotated and registered for the renamed projects
  And the crane-specific vars have been removed
  When rhino-cli env validate runs
  Then it reports no drift
  And the pre-push and CI env-validate checks pass
```

## Product Scope

### In Scope (Product Features)

- `ose-be` (renamed from `ose-app-be`) rewritten to F#, preserving `/health`, all non-media paths, its
  messaging status surface + JetStream demo, its six bounded contexts, and its OpenRouter LLM
  integration (gap-analysis; core).
- `organiclever-be` (in-place rewrite, name kept) as a real F# backend: `health`, minimal `journal`
  CRUD (PGlite-schema mirror), messaging status + JetStream demo.
- Crane + media fully removed; two bootable backend images (`organiclever-be`, `ose-be`) published
  early; image roster 3 → 2; web tiers ship no images.
- organiclever web split + rename: `organiclever-app-web` (the app) + new simple `organiclever-www`
  (marketing, from `landing`).
- Repo-wide `-www` public-site renames: `ose-web` → `ose-www`, `wahidyankf-web` → `wahidyankf-www`,
  `ayokoding-web` → `ayokoding-www` (plus their e2e pairs).
- `libs/ts-ui` shared design system adopted by its three consumers (`organiclever-www`,
  `organiclever-app-web`, `ose-app-web`); `ose-www`/`ayokoding-www` keep their internals.
- `ose-www` structure-only simplification (keeps tRPC + content pipeline); OSE frontend audit.
- Adapted Playwright e2e (renamed pairs incl. `ose-be-e2e`, the `-www` pairs, the `ayokoding-www`
  pairs, a new marketing pair; media scenarios dropped).
- F# Dockerfiles (backends only); adjusted integration/e2e compose; env-var updates
  (`OSE_APP_BE_*` → `OSE_BE_*`; `ORGANICLEVER_BE_*` kept) + drift-guard registration.
- `specs/` restructure across organiclever, ose, crane, and ayokoding.
- Comprehensive `.md` sweep across `AGENTS.md`, `CLAUDE.md`, `docs/`, every renamed app README, and the
  app-naming convention (documenting the `www` app type).

### Out of Scope (Product Features)

- Production cutover (Vercel/DNS/prod branches) — deferred downstream.
- The organiclever sync-vs-server-authoritative decision and any consumption wiring beyond the
  smoke-probe.
- The PDF-to-Markdown feature in any form (removed).
- New endpoints/business features beyond journal CRUD (organiclever) / non-media parity (ose).
- Authoring the converged toolchain (assumed DONE).
- `libs/fsharp-crane-core`, `apps/crane-cli`, `libs/rust-commons` internals; `wahidyankf-web` and
  `ayokoding-web` content/structure (each renamed to its `-www` form only — mechanical, no content
  work; `ayokoding-www` keeps its tRPC).

## Product Risks

- **Behavioral parity on the ose-be contract**: the F# port must serve every non-media path the Rust
  `ose-app-be` served (and preserve OpenRouter); mitigated by driving the port from the preserved
  behavior specs and asserting via the adapted e2e runner before the gate.
- **Journal CRUD built blind to consumption**: mitigated by mirroring the PGlite client schema and
  keeping it minimal + contract-smoke-tested.
- **EF Core mapping fidelity**: snake_case columns and types must match; mitigated by reusing migration
  SQL and the primer's `UseSnakeCaseNamingConvention` + `[<Column>]`-annotated entities.
- **ts-ui adoption rework**: mitigated by building `ts-ui` before the frontends consume it.
- **ose-www content pipeline regression** (during the `ose-web` → `ose-www` rename + simplification):
  mitigated by structure-only simplification keeping tRPC + content infra and asserting feed/updates
  render at e2e.
- **NATS.Net JetStream demo flakiness**: depends on a real NATS container at e2e; mitigated by keeping
  `test:e2e` non-cacheable and gating on a `/health` healthcheck wait.
