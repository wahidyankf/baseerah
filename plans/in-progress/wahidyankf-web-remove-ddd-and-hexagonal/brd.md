# Business Requirements — Remove DDD and Hexagonal from wahidyankf-web

## Business Goal

Remove architectural ceremony from `wahidyankf-web` that adds maintenance cost
and cognitive load without delivering proportional value, so the app's structure
honestly reflects what it is: a small static content site.

## Business Rationale

`wahidyankf-web` is a personal portfolio with three pages and no business logic,
no IO ports, and no domain rules. [Repo-grounded] Two architectural patterns were
adopted from sibling apps where they earn their keep, but here they are dead
weight:

- The **DDD bounded-context registry** (`bounded-contexts.yaml` +
  ubiquitous-language glossaries) plus its two `rhino-cli ddd` pre-push gates
  describe a domain model that does not exist for a static portfolio.
- The **hexagonal `contexts/` layout** mandates four layers per context, yet two
  of them (`domain/`, `infrastructure/`) are empty one-line stubs across all five
  contexts. [Repo-grounded] The structure forces contributors to navigate empty
  folders and barrel indirection for trivial component code.

Removing both makes the codebase match the principle of **Simplicity Over
Complexity** (minimum viable abstraction) and **Explicit Over Implicit** — a flat
`src/features/<name>/` layout states plainly where each page's code lives.

## Business Impact

**Pain points addressed** (qualitative reasoning):

- Contributors (the solo maintainer, and any AI agent editing this app) waste
  attention on empty `domain/`/`infrastructure/` folders and a DDD vocabulary
  with no referent. [Judgment call]
- The two `ddd bc`/`ddd ul` pre-push commands add rhino-cli build + run latency
  to every `test:quick` for an app that gains nothing from DDD validation.
  [Repo-grounded — the commands are wired into `test:quick`]
- The DDD spec tree (~9 files) and the four-layer folder convention create a
  false impression that this app has a modelled domain. [Judgment call]

**Expected benefits** (qualitative reasoning):

- Fewer files and folders to maintain; flatter, more navigable structure.
- Faster `test:quick` (two rhino-cli `ddd` invocations removed). [Judgment call —
  magnitude not measured]
- Governance honesty: the hexagonal-web convention gains an explicit, documented
  opt-out for trivially-small static sites, so future small sites need not adopt
  ceremony they cannot use.

## Affected Roles

This is a solo-maintainer repository; no sign-off ceremonies apply.

- **Maintainer (architect hat)** — approves the governance opt-out clause and the
  decision to deviate from the hexagonal-web convention for this app.
- **Maintainer (developer hat)** — performs the refactor and import rewrites.
- **AI agents that consume the files** — `swe-typescript-dev` (TS refactor),
  `swe-rust-dev` (rhino-cli allowlist), `repo-rules-maker`/governance-aware agents
  (the hexagonal-web convention edit), `readme-maker` (README rewrite). All are
  existing agents. [Repo-grounded]

## Business-Level Success Metrics

- **Observable fact** — `specs/apps/wahidyankf/ddd/` no longer exists after
  completion (verified by `test ! -d`).
- **Observable fact** — `grep -rn "@/contexts" apps/wahidyankf-web` returns no
  matches after completion.
- **Observable fact** — `grep -rn "apps_with_ddd" apps/rhino-cli` no longer lists
  `"wahidyankf"`.
- **Observable fact** — `nx affected -t typecheck lint test:quick spec-coverage`
  is green for both affected projects.
- **Qualitative** — the hexagonal-web governance doc contains a documented
  opt-out clause for static content sites.

No fabricated numeric KPIs are claimed.

## Business-Scope Non-Goals

- Not changing user-visible behavior, routes, styling, or content of the site.
- Not removing or altering C4 specs or Gherkin behavior specs.
- Not removing the `rhino-cli ddd` subcommands (other apps still use them).
- Not migrating sibling apps (`organiclever-web`, `ose-web`, `ayokoding-web`)
  away from hexagonal — they remain on the convention.

## Business Risks and Mitigations

| Risk                                                                             | Likelihood | Mitigation                                                                                                                        |
| -------------------------------------------------------------------------------- | ---------- | --------------------------------------------------------------------------------------------------------------------------------- |
| Flattening deviates from the hexagonal-web convention, creating governance drift | Certain    | Add an explicit, vendor-neutral opt-out clause to the convention doc (workstream C) so the deviation is sanctioned and documented |
| Import rewrites break the build or tests                                         | Medium     | Behavior-preserving refactor; full unit suite guards each per-context move; every phase gate runs typecheck + lint + unit tests   |
| Removing `wahidyankf` from the rhino-cli allowlist breaks the membership test    | Certain    | Update the `assert_eq!(v.len(), …)` count and rebuild + retest rhino-cli in the same phase                                        |
| `spec-coverage` gate breaks after folder rename                                  | Low        | Verified the gate scans `{projectRoot}/**/*.{ts,tsx}` and the app dir, not `contexts/` paths; phase gate re-runs `spec-coverage`  |

See [prd.md](./prd.md) for the product specification and testable acceptance
criteria, and [tech-docs.md](./tech-docs.md) for the technical design.
