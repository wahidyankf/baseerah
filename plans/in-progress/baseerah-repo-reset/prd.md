# PRD — Baseerah Repo Reset

Product Requirements Document for the [Baseerah Repo Reset](./README.md) plan.

## Product Overview

Two products are in scope, and they are deliberately unequal in ambition.

**The repository itself** is the primary product of this plan. Its users are AI agents and one
maintainer. Its "features" are an accurate instruction surface, a green CI harness, and a project
graph with nothing dead in it. Nearly all the work in `delivery.md` serves this.

**The four `baseerah-*` apps are hello world.** Not a walking skeleton of a future feature — hello
world. A health endpoint, a greeting endpoint, and one page that renders the greeting. Their entire
job is to prove that the four-project quad wires up correctly: that `baseerah-contracts` generates,
that `baseerah-be` builds and serves, that `baseerah-fe` renders and reaches the backend, and that
both E2E suites run green against a real Docker stack.

Choosing hello world over a real feature is the point. A feature would smuggle product decisions —
what an assistant note _is_, whether it persists, what shape the domain takes — into a plan whose
subject is repository structure. Those decisions belong to their own plans, made deliberately.
Everything the apps _are_ here is scaffolding that a later plan replaces without regret.

## Personas

Solo-maintainer repo. These are hats and agent consumers, not external stakeholders.

| Persona                                                  | Who / what                                  | What they need from this plan                                                    |
| -------------------------------------------------------- | ------------------------------------------- | -------------------------------------------------------------------------------- |
| **Maintainer (platform)**                                | The human, wearing the repo-owner hat       | A repo whose instructions match its contents; a green `main` after each push     |
| **Maintainer (product)**                                 | The human, wearing the Baseerah-product hat | A named product, a vision doc, and a running quad to build the real thing into   |
| **Executing agent**                                      | Whatever agent runs `delivery.md`           | Unambiguous steps: exact paths, verbatim commands, observable acceptance         |
| **`swe-fsharp-dev`**                                     | Implements `baseerah-be`                    | A spec tree and contracts project to code against                                |
| **`swe-typescript-dev`**                                 | Implements `baseerah-fe`                    | Design tokens, `web-ui` primitives, and a typed client from `baseerah-contracts` |
| **`swe-e2e-dev`**                                        | Implements both E2E suites                  | Gherkin under `specs/apps/baseerah/behavior/**` and a local Docker stack         |
| **`plan-checker` / `repo-rules-checker` / `ci-checker`** | Validate the result                         | `repo-config.yml`, workflows, and governance prose that describe reality         |

## User Stories

### US-1 — Purge the old product

**As a** maintainer,
**I want** every app except `rhino-cli` and its supporting libs removed along with all their CI,
infra, specs, agents, and config entries,
**So that** no agent plans, searches, or validates against code that does not exist here.

```gherkin
Feature: Retired project surface is fully removed

  Scenario: The project graph contains only the kept and new projects
    Given the purge phases have completed
    When I run "npx nx show projects"
    Then the output lists rhino-cli, rust-commons, web-ui, web-ui-token, baseerah-contracts, baseerah-be, baseerah-be-e2e, baseerah-fe and baseerah-fe-e2e
    And the output also lists fsharp-crane-core if and only if the Phase 2 audit kept it
    And no other project name appears

  Scenario: No retired app name survives outside the plan folder
    Given the purge phases have completed
    When I run "rg -n --hidden -g '!.git' -g '!plans/in-progress/baseerah-repo-reset' 'ayokoding|organiclever|wahidyankf|crane-cli|ose-www|ose-app-web|ose-cli'"
    Then the command exits non-zero with no matches

  Scenario: The repo-config coverage registry matches the project graph
    Given the purge phases have completed
    When I run "npm run validate:config"
    Then the command exits 0
    And every coverage.projects entry names a project that "npx nx show projects" also lists
```

### US-2 — Keep the engineering harness intact

**As a** maintainer,
**I want** `rhino-cli`, the generic agent fleet, the skills, and `repo-governance/` preserved
through the purge,
**So that** I keep the SDLC machinery that makes this repo productive instead of rebuilding it.

```gherkin
Feature: The engineering harness survives the purge

  Scenario: rhino-cli still builds and passes its own gates
    Given the purge phases have completed
    When I run "npx nx run rhino-cli:test:quick"
    Then the command exits 0

  Scenario: Platform bindings regenerate without drift
    Given app-scoped agents have been deleted from ".claude/agents/"
    When I run "npm run generate:bindings" followed by "npm run validate:sync"
    Then validate:sync reports zero drift
    And ".opencode/agents/" contains exactly the same basenames as ".claude/agents/" minus README.md

  Scenario: The governance principles remain identical to the OSE siblings
    Given the purge and identity phases have completed
    When I run "diff -r /Users/wkf/ose-projects/ose-public/repo-governance/principles /Users/wkf/ose-projects/baseerah/repo-governance/principles"
    Then the command exits 0 with no output
```

### US-3 — Establish Baseerah's identity inside the OSE ecosystem

**As a** maintainer,
**I want** the root instruction surface to describe Baseerah as a personal-assistant product that
belongs to the Open Sharia Enterprise ecosystem,
**So that** an agent reading `AGENTS.md` understands both what this repo builds and what family it
belongs to.

```gherkin
Feature: Repository identity names Baseerah within the OSE ecosystem

  Scenario: The canonical instruction file describes Baseerah
    Given Phase 4 has completed
    When I read "AGENTS.md"
    Then its Repository Overview names Baseerah as a personal-assistant product
    And it states that Baseerah is a product within the Open Sharia Enterprise ecosystem
    And its Web Sites table lists only baseerah-fe and baseerah-be

  Scenario: The vision layer carries both the ecosystem and the product
    Given Phase 4 has completed
    When I list "repo-governance/vision/"
    Then it contains "open-sharia-enterprise.md" unchanged
    And it contains a new "baseerah.md" describing the product vision
    And "repo-governance/vision/README.md" links to both

  Scenario: The rewritten instruction surface stays inside its size budget
    Given Phase 4 has completed
    When I run "npx nx run rhino-cli:instruction-size:validation"
    Then the command exits 0
```

### US-4 — Serve hello world from `baseerah-be`

**As a** maintainer,
**I want** the backend to answer a liveness check and return a greeting,
**So that** the F# / Giraffe quad is proven to build, run, and serve HTTP before any real domain
logic is designed.

```gherkin
Feature: Backend hello world

  Background:
    Given the baseerah-be service is running on port 19320

  Scenario: The service reports liveness
    Given the service has finished starting
    When I send a GET request to "/api/v1/health"
    Then the response status is 200
    And the response body field "status" equals "ok"

  Scenario: The service returns a greeting
    Given the service has finished starting
    When I send a GET request to "/api/v1/hello"
    Then the response status is 200
    And the response body field "message" equals "Hello from Baseerah"

  Scenario: An unknown route is refused
    Given the service has finished starting
    When I send a GET request to "/api/v1/does-not-exist"
    Then the response status is 404
    And the response body field "error" is a non-empty string
```

### US-5 — Render hello world in `baseerah-fe`

**As a** maintainer,
**I want** one page that names the product and displays the greeting fetched from the backend,
**So that** the Next.js quad is proven to build, render, and reach `baseerah-be` over HTTP.

```gherkin
Feature: Frontend hello world

  Background:
    Given the baseerah-fe app is running on port 19310 against a live baseerah-be

  Scenario: The landing page names the product and shows the backend greeting
    Given I have not visited the site before
    When I navigate to "/"
    Then the page shows a level-one heading containing "Baseerah"
    And the page shows the text "Hello from Baseerah" sourced from the backend

  Scenario: The landing page meets the baseline accessibility bar
    Given I am on "/"
    When an automated accessibility scan runs against the rendered page
    Then it reports zero serious violations
    And it reports zero critical violations
```

### US-6 — Gate Baseerah content and deploys with dedicated agents

**As a** maintainer,
**I want** `apps-baseerah-*` maker/checker/fixer and deployer agents to exist from the start,
**So that** Baseerah's own quality gates are in place before there is much to gate.

```gherkin
Feature: Baseerah-scoped agent fleet

  Scenario: The new agents satisfy the naming convention
    Given Phase 10 has completed
    When I run the agent-naming enforcement command from the Agent Naming Convention
    Then it produces no output beyond the known preexisting violation

  Scenario: The new agents appear in every platform binding
    Given the new agents exist under ".claude/agents/"
    When I run "npm run generate:bindings"
    Then ".opencode/agents/" and ".cursor/agents/" each gain a matching file
    And "git diff --exit-code" reports no unstaged drift afterwards
```

## Product Scope

### In scope

- Removal of 22 apps, their spec trees, CI, infra, and config registrations
- Preservation of `rhino-cli`, four libs, ~59 generic agents, ~27 generic skills, `repo-governance/`,
  with `repo-governance/principles/**` verified byte-identical to `ose-public`
- A Baseerah identity surface: `README.md`, `ROADMAP.md`, `AGENTS.md`, `CONTRIBUTING.md`,
  `SECURITY.md`, and a new `repo-governance/vision/baseerah.md`
- `specs/apps/baseerah/` five-folder C4 tree with the five hello-world scenarios above
- `baseerah-contracts` (OpenAPI 3.1 + bundle target), `baseerah-be`, `baseerah-be-e2e`,
  `baseerah-fe`, `baseerah-fe-e2e`
- **Exactly three endpoints and one page**: `GET /api/v1/health`, `GET /api/v1/hello`, a 404 handler,
  and `/`
- `infra/dev/baseerah-app/` local Docker stack
- Baseerah CI caller workflows in the OSE thin-caller shape, delegating to the untouched
  `_reusable-*.yml` templates, with the four core workflows and their job sets verified unchanged
  against `ose-public`
- `apps-baseerah-fe-content-{maker,checker,fixer}`, `apps-baseerah-fe-deployer`,
  `apps-baseerah-be-deployer`, and the `apps-baseerah-fe-developing-content` skill

### Out of scope

- **Every product feature.** No capture, no notes, no LLM calls, no prompt plumbing, no AI SDK
  dependency, no scheduling, no posting. The apps are hello world and are expected to be rewritten.
- **Any persistence.** No database, no in-memory store, no state of any kind. The greeting is a
  constant. Consequently there is no `docker-compose.integration.yml` database service, no
  `db/migrations/`, and no test-reset hook — the backend is stateless, so E2E scenarios cannot leak
  state into one another and need no isolation mechanism.
- **Any write endpoint.** All three routes are `GET`. No request-body validation is needed or built.
- **Authentication and multi-user.** No login, no session, no user concept.
- **Deploy _provisioning_.** No Vercel project, no GHCR repository, no `prod-*` / `stag-*` branch is
  created. The deployer agents ship and the CI caller workflows ship, but nothing has ever been
  deployed and the first real deploy belongs to its own plan.

  Note the boundary: the **workflow files are in scope** — `baseerah-be-build-deploy-stag.yml`,
  `baseerah-app-test-local-deploy-stag.yml`, `baseerah-app-test-stag.yml`, and the `baseerah-be` job
  in `publish-images.yml` all land in Phases 7 and 9, because omitting them would leave this repo's
  CI architecture inconsistent with its OSE siblings. They land **wired but dormant**: their trigger
  branches do not exist, so they never fire.

- **`infra/k8s/baseerah/`.** Deferred with the deploys.
- **Storybook for `baseerah-fe`.** `web-ui` keeps its own; the app does not get one yet.
- **The DDD bounded-context layering** used by `ose-app-web` (`eslint-plugin-boundaries`, four-layer
  `src/contexts/`). Hello world does not earn it; it is adopted when the first real context lands.

## UI Design Funnel

`baseerah-fe` renders a user-facing page, so this plan carries the funnel per the
[UI Mockups in Plan Docs](../../../repo-governance/conventions/formatting/diagrams.md#placement--the-ui-lives-in-prdmd-hard-rule)
placement rule. The funnel is deliberately small, matching a one-page hello world.

### Grounding

Both alternatives below draw exclusively from what already exists rather than inventing new
primitives: `libs/web-ui`'s current component inventory (`AppShell` header/footer regions,
typography scale, landmark-aware layout primitives) and `libs/web-ui-token`'s existing colour,
spacing, and contrast tokens (rebranded for Baseerah in Phase 4, not newly authored). No component
proposed here requires a `libs/web-ui` addition.

### Prior art

A `web-researcher` sweep of comparable hello-world/health-check landing pages (framework
starter-kit default pages — Next.js's own `create-next-app` template, Vercel's platform starters,
and typical `/status` pages for backend services) confirms the two-alternative split below —
content-only versus content-in-a-persistent-shell — is the standard fork for this exact page type;
no third pattern recurs often enough to warrant a Diverge slot. Depth stops here: for a one-page
hello world, a fuller competitive survey would cost more than the decision is worth.

### Diverge — two alternatives

**Alternative A — "Bare Greeting"**: the greeting is the whole page. No chrome, no shell, no
navigation. The smallest thing that renders.

```text
DESKTOP 1280px — Alternative A                MOBILE 390px — Alternative A
┌────────────────────────────────────────┐    ┌──────────────────────┐
│                                        │    │                      │
│                                        │    │                      │
│              Baseerah                  │    │      Baseerah        │
│                                        │    │                      │
│       Hello from Baseerah              │    │  Hello from Baseerah │
│                                        │    │                      │
│                                        │    │                      │
└────────────────────────────────────────┘    └──────────────────────┘

TABLET 768px — Alternative A
┌──────────────────────────────┐
│                              │
│           Baseerah           │
│                              │
│     Hello from Baseerah      │
│                              │
└──────────────────────────────┘
```

**Alternative B — "Shell + Greeting"**: the greeting sits inside a persistent app shell — a header
carrying the product name and a footer carrying the build/version line.

```text
DESKTOP 1280px — Alternative B                MOBILE 390px — Alternative B
┌────────────────────────────────────────┐    ┌──────────────────────┐
│  Baseerah                              │    │ Baseerah             │
├────────────────────────────────────────┤    ├──────────────────────┤
│                                        │    │                      │
│         بصيرة — insight, wawasan       │    │  بصيرة               │
│                                        │    │  insight · wawasan   │
│         Hello from Baseerah            │    │                      │
│                                        │    │  Hello from Baseerah │
│                                        │    │                      │
├────────────────────────────────────────┤    ├──────────────────────┤
│  baseerah-fe · connected to :19320     │    │ connected to :19320  │
└────────────────────────────────────────┘    └──────────────────────┘

TABLET 768px — Alternative B
┌──────────────────────────────┐
│  Baseerah                    │
├──────────────────────────────┤
│                              │
│   بصيرة — insight, wawasan   │
│                              │
│   Hello from Baseerah        │
│                              │
├──────────────────────────────┤
│  connected to :19320         │
└──────────────────────────────┘
```

At all three breakpoints the shell reflows to a single column; only the header/footer padding and
the greeting's font size scale down. No layout restructuring happens between 1280px and 768px — the
first restructuring point (stacking header text) is at 390px, already shown above.

### Narrow — one hi-fi finalist, not two (explicit trade-off)

The convention's Narrow stage calls for carrying the **two** strongest alternatives forward as
high-fidelity mockups before Select happens. This plan deliberately narrows to **one** hi-fi
finalist instead, authored after Select rather than before it:

- **What's skipped**: Alternative A never receives a hi-fi treatment at any point in this plan.
- **Why**: the Select decision below is already lopsided on the low-fi wireframes alone — B wins
  four of six Justify criteria outright, including the two that matter most for a hello-world scope
  (proving the Phase 4 token rebrand actually renders, and giving the accessibility scan real
  landmarks to check). A hi-fi pass on the alternative that isn't going to win would be spent effort
  with no decision left to inform, on a page whose entire purpose is to be small.
- **What still happens**: the low-fi wireframes above are detailed enough (real copy, real
  breakpoints, real token-bearing regions) to Select on with the same confidence a hi-fi pass would
  add. The winner still gets a genuine hi-fi mockup — just after Select, not before it.

The finalist's high-fidelity mockups are authored into this plan's `assets/` folder as the first
step of Phase 8, before any component is written, and embedded here:

- `![Baseerah landing page, desktop 1280px](./assets/landing-desktop-1280.png)`
- `![Baseerah landing page, mobile 390px](./assets/landing-mobile-390.png)`
- `![Baseerah landing page, tablet 768px](./assets/landing-tablet-768.png)`

Until Phase 8 authors those files these are deliberately inert code-fenced paths rather than live
`![]()` embeds, so this document never renders a broken image. Phase 8's first checklist item
converts them to live embeds in the same commit that adds the assets.

### Select

**Alternative B — "Shell + Greeting" — is selected.**

### Justify

| Criterion                              | A — Bare Greeting                                     | B — Shell + Greeting                                           | Winner |
| -------------------------------------- | ----------------------------------------------------- | -------------------------------------------------------------- | ------ |
| Exercises `libs/web-ui` primitives     | Renders raw text; the design system is never touched  | Header, footer, and typography all come from `web-ui`          | **B**  |
| Proves the Phase 4 token rebrand       | No token beyond a text colour is used                 | Surface, border, and text tokens are all exercised and visible | **B**  |
| Gives the a11y scan something to check | One heading; landmark and contrast checks are vacuous | Header/main/footer landmarks make the axe scan meaningful      | **B**  |
| Growth path                            | The shell must be added later anyway                  | The shell the real app needs already exists                    | **B**  |
| Fewest components                      | One text block                                        | Adds `AppShell`                                                | **A**  |
| Least code to throw away               | Almost nothing to discard                             | The shell survives; only the greeting is discarded             | tie    |

B wins four criteria; A wins one. The decisive point is the third: a bare greeting makes the
accessibility gate in US-5 pass trivially without proving anything, whereas a shell with real
landmarks makes the same gate a genuine check. B also happens to throw away _less_, since the shell
is the one part of this hello world a real Baseerah page will keep.

**Accessibility commitments** (binding on Phase 8, verified in Phase 9):

- Colour pairs drawn from `libs/web-ui-token` only, all meeting WCAG AA contrast.
- The page uses real landmarks — `<header>`, `<main>`, `<footer>` — and exactly one `<h1>`.
- The Arabic string `بصيرة` carries `lang="ar"` and `dir="rtl"` on its own element so screen readers
  announce it correctly inside an otherwise `lang="en"` document.
- `@axe-core/playwright` runs in `baseerah-fe-e2e` with zero serious or critical violations.

## Product Risks

| Risk                                                                                            | Severity | Mitigation                                                                                                                                                                     |
| ----------------------------------------------------------------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Hello world is mistaken for a foundation and features are bolted onto it ad hoc                 | Medium   | `README.md`, `prd.md`, and `repo-governance/vision/baseerah.md` all state that the apps are throwaway scaffolding; the first feature plan is expected to replace them          |
| `libs/web-ui` primitives were designed for OrganicLever/OSE brands and look wrong for Baseerah  | Medium   | Phase 4 rebrands `libs/web-ui-token` values and the `swe-developing-frontend-ui` skill's `reference/brand-context.md`; `web-ui` component APIs are brand-neutral and unchanged |
| Choosing `baseerah-fe` over the established `[domain]-app-web` tier breaks naming conventions   | Medium   | Phase 4 amends the app-naming tier table in `AGENTS.md` and `docs/reference/monorepo-structure.md` to add `[domain]-fe` **before** Phase 8 creates the app                     |
| `domain:baseerah` is not in the Nx tag vocabulary, so tagging the new projects is a violation   | High     | Phase 5 amends the vocabulary table in `repo-governance/development/infra/nx-targets.md` before any new `project.json` is authored                                             |
| A stateless hello world under-exercises the quad, hiding wiring defects until the first feature | Medium   | The greeting is fetched from the backend rather than hardcoded in the frontend, so `baseerah-fe-e2e` genuinely exercises the full FE → BE path against the Docker stack        |
| Skipping the DDD `src/contexts/` layering now makes the first real context expensive            | Low      | Accepted deliberately — `tech-docs.md` Decision 9 records the trigger condition for adopting it                                                                                |
