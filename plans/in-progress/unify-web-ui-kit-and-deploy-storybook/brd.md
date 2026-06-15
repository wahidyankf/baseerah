# Business Requirements Document — Unify Web UI Kit and Deploy Storybook

## Business Goal

Establish **one shared, brand-themed UI component kit** as the single source of visual truth for
every frontend web application in the repository, and make that kit **publicly browsable** through a
deployed Storybook site at `web-ui.oseplatform.com`.

## Business Rationale (WHY)

The repository today runs six FE web apps with three different UI-kit-adoption postures
[Repo-grounded — verified against `apps/*/package.json` and local `ui/` directories]:

- Two apps (`organiclever-www`, `ose-app-web`) consume both `web-ui` and `web-ui-token`.
- Two apps (`organiclever-app-web`, `wahidyankf-www`) consume `web-ui` but **not** the token lib.
- Two content sites (`ose-www`, `ayokoding-www`) consume **neither** — each maintains its own copy
  of shadcn primitives (11 files and 8 files respectively [Repo-grounded]).

This fragmentation produces concrete maintenance pain:

- **Triplicated primitives**: the same `dropdown-menu.tsx`, `dialog.tsx`, `sheet.tsx`, `tabs.tsx`,
  etc. are hand-maintained in `web-ui`, `ose-www`, and `ayokoding-www`. A fix to one does not
  propagate. [Repo-grounded]
- **No single visual reference**: there is no deployed catalogue of the kit, so a contributor
  cannot see what already exists before building a new component — encouraging yet more duplication.
- **Brand-theming inconsistency**: only the OrganicLever brand has a token file
  (`organiclever.css` is the only brand file today [Repo-grounded]); the other three brands have no
  formal token surface, so their colours live ad-hoc inside app code.

## Business Impact

| Dimension                  | Before                                                    | After                                             |
| -------------------------- | --------------------------------------------------------- | ------------------------------------------------- |
| Primitive maintenance      | 3 hand-maintained copies of overlapping shadcn primitives | 1 canonical copy in `web-ui/src/primitives/`      |
| Brand token surfaces       | 1 of 4 brands has a token file                            | 4 of 4 brands have a token file                   |
| Discoverability of the kit | No deployed reference                                     | Public Storybook with a live brand-theme switcher |
| New-component review       | Contributor cannot easily check what exists               | Contributor browses Storybook first               |

## Affected Roles (hats the solo maintainer wears; agents that consume the artifacts)

- **Frontend maintainer** — migrates the two content sites and wires tokens; primary beneficiary of
  de-duplication.
- **Design-system maintainer** — owns `web-ui` and `web-ui-token`; gains a deployed catalogue.
- **Release/deploy maintainer** — operates the new `prod-web-ui` branch and Vercel project.
- **Consuming agents** — `swe-ui-maker`, `swe-typescript-dev`, `apps-ose-www-content-maker`,
  `apps-ayokoding-www-*` makers reference the Storybook catalogue and the unified kit; the new
  `apps-web-ui-storybook-deployer` agent performs on-demand deploys. [Repo-grounded — agents exist
  in `.claude/agents/`]

## Business-Level Success Metrics

- **Zero duplicated primitive files across apps** — observable fact: after Phase 6, the directories
  `apps/ose-www/src/features/app-shell/presentation/ui/` and
  `apps/ayokoding-www/src/contexts/app-shell/presentation/ui/` no longer exist. [Observable check]
- **All six FE apps import `web-ui-token`** — observable fact: each app's entry CSS imports the base
  token sheet plus its brand sheet. [Observable check]
- **Storybook reachable at `web-ui.oseplatform.com`** — observable fact: an HTTP request to
  `https://web-ui.oseplatform.com` returns `200`. [Observable check]
- **No visual regression in either migrated content site** — observable fact:
  `nx run ose-www-fe-e2e:test:e2e` and `nx run ayokoding-www-fe-e2e:test:e2e` pass after migration.
  [Observable check — these projects expose `test:e2e`; they do NOT expose a separate `test:visual`
  target, Repo-grounded against `apps/*-fe-e2e/project.json`]
- **Reduced cognitive overhead for new UI work** — _Judgment call_: a single deployed catalogue
  plus one canonical kit lowers the chance of re-implementing an existing component. No numeric
  target is asserted.

## Business-Scope Non-Goals

- No visual redesign of any app (the unification is invisible to end users by mandate).
- No change to backend behaviour or any F# service.
- No new third-party dependency version (cost/risk avoidance — reuse exact repo-consistent pins).
- No migration of the four apps that already consume `web-ui` away from their current components
  (only token wiring is added where missing).

## Business Risks and Mitigations

| Risk                                                                | Likelihood | Impact | Mitigation                                                                                                 |
| ------------------------------------------------------------------- | ---------- | ------ | ---------------------------------------------------------------------------------------------------------- |
| Migration silently changes a content site's appearance              | Medium     | High   | Gate every migration phase on that app's `fe-e2e:test:e2e`; treat any diff as a stop-the-line defect.      |
| Vercel auto-detects Next.js and runs `next build` instead of static | Medium     | High   | `vercel.json` sets `framework: null` and an explicit static `outputDirectory` (see `tech-docs.md`).        |
| A newly-pinned dependency carries an unpatched CVE                  | Low        | High   | Reuse only versions already present and CVE-cleared in the repo (Path A); record clearance in tech-docs.   |
| DNS/SSL go-live blocked on human credentials                        | High       | Low    | Cluster all human Vercel/DNS steps in the final phase; everything else completes independently first.      |
| Deployer agent introduces vendor-specific governance content        | Low        | Medium | Author the agent vendor-neutral; resync bindings via `npm run generate:bindings` (never hand-edit mirror). |
