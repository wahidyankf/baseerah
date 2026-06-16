# Product Requirements Document — Unify Web UI Kit and Deploy Storybook

## Product Overview

A unified frontend UI system: one component kit (`@open-sharia-enterprise/web-ui`) layered as
**primitives** (shadcn base) plus **composites** (app-level components), themed per brand through
`@open-sharia-enterprise/web-ui-token`, and showcased by a publicly deployed Storybook with a live
brand-theme switcher at `web-ui.oseplatform.com`.

## Personas

- **UI engineer (maintainer hat)** — builds and migrates app screens; wants one place to import
  every component and one place to see them rendered.
- **Design-system owner (maintainer hat)** — curates primitives, composites, and brand tokens; wants
  a deployed catalogue and theme switcher to validate brand parity visually.
- **Release operator (maintainer hat)** — deploys the Storybook site; wants a deterministic,
  one-action deploy path mirroring the existing `prod-*` Vercel model.
- **Consuming agent** — `swe-ui-maker`, content makers, and the new deployer agent
  [Repo-grounded — these agents exist in `.claude/agents/`].

## User Stories

- **US-1** — As a UI engineer, I want all shadcn primitives to live in `web-ui/src/primitives/` so
  that I import them from one package instead of copying files into each app.
- **US-2** — As a design-system owner, I want a brand token CSS file for each of the four brands so
  that brand differences are expressed only in tokens, not in component code.
- **US-3** — As a UI engineer, I want `ose-www` and `ayokoding-www` migrated onto `web-ui` with
  **zero visual change** so that users see no difference while duplication is removed.
- **US-4** — As a design-system owner, I want a deployed Storybook with a brand-theme toolbar so
  that I can switch OSE / AyoKoding / wahidyankf / OrganicLever themes live and confirm parity.
- **US-5** — As a release operator, I want a `prod-web-ui` environment branch and a deployer agent so
  that I can publish the Storybook the same way I publish the other Vercel sites.
- **US-6** — As a UI engineer, I want every primitive dependency pinned to the exact version already
  used in the repo so that no new version risk is introduced.

## Acceptance Criteria (Gherkin)

Each scenario uses exactly one primary `Given`, one `When`, and one `Then`; extras chain with
`And`/`But` per the step-keyword cardinality HARD rule.

```gherkin
Feature: Unified primitives layer in web-ui

Scenario: Primitives layer exposes the superset of content-site primitives
  Given libs/web-ui has a new src/primitives/ directory
  When the maintainer builds the web-ui barrel export
  Then libs/web-ui/src/index.ts re-exports button, badge, sheet, command, dialog, dropdown-menu, tabs, card, tooltip, scroll-area, and separator
  And nx run web-ui:typecheck exits 0
  And nx run web-ui:test:unit passes

Scenario: Primitive dependencies are pinned to exact repo-consistent versions
  Given libs/web-ui/package.json declares the primitive dependencies
  When the maintainer inspects each primitive dependency version string
  Then every version is an exact pin with no caret or tilde
  And each pin equals the version resolved in the repo-root package-lock.json
```

```gherkin
Feature: Brand token parity across all four brands

Scenario: All four brand token files exist
  Given libs/web-ui-token/src/ already contains organiclever.css
  When the maintainer adds the remaining brand token files
  Then libs/web-ui-token/src/ contains ose.css, ayokoding.css, and wahidyankf.css
  And each new brand file defines the same token custom-property names as organiclever.css

Scenario: All six FE apps import the token library
  Given the four brand token files exist
  When the maintainer wires each app's entry CSS
  Then ose-www, ayokoding-www, organiclever-www, ose-app-web, organiclever-app-web, and wahidyankf-www each import the base token sheet plus their brand sheet
  And nx affected -t typecheck exits 0
```

```gherkin
Feature: Zero-visual-change content-site migration

Scenario: ose-www renders identically after migrating onto web-ui
  Given ose-www imported primitives from features/app-shell/presentation/ui/
  When the maintainer repoints ose-www imports to @open-sharia-enterprise/web-ui
  Then nx run ose-www-fe-e2e:test:e2e passes with no rendering assertion failures
  And the directory apps/ose-www/src/features/app-shell/presentation/ui/ no longer exists

Scenario: ayokoding-www renders identically after migrating onto web-ui
  Given ayokoding-www imported primitives from contexts/app-shell/presentation/ui/
  When the maintainer repoints ayokoding-www imports to @open-sharia-enterprise/web-ui
  Then nx run ayokoding-www-fe-e2e:test:e2e passes with no rendering assertion failures
  And the directory apps/ayokoding-www/src/contexts/app-shell/presentation/ui/ no longer exists
```

```gherkin
Feature: Storybook catalogue with brand-theme switcher

Scenario: Stories cover every primitive and composite
  Given web-ui exposes primitives and composites
  When the maintainer authors stories for the kit
  Then each primitive and each composite has a corresponding .stories.tsx file
  And nx run web-ui:build-storybook produces libs/web-ui/storybook-static/index.html

Scenario: Brand theme switcher offers all four brands
  Given the Storybook preview uses withThemeByClassName
  When a viewer opens the brand-theme toolbar in the built Storybook
  Then the toolbar lists OSE, AyoKoding, wahidyankf, and OrganicLever
  And selecting a brand applies that brand's token class to the html element
```

```gherkin
Feature: Storybook deployed to Vercel at the custom domain

Scenario: Static Storybook deploys via the prod-web-ui branch
  Given a Vercel project is connected to the prod-web-ui branch with framework set to Other
  When the deployer force-pushes main to prod-web-ui
  Then Vercel builds the static Storybook from the configured output directory
  And an HTTP request to https://web-ui.oseplatform.com returns status 200

Scenario: Deep links to a story do not 404
  Given vercel.json defines an SPA rewrite to index.html
  When a viewer opens a deep story URL on the deployed site
  Then the Storybook application loads instead of a 404 page
```

## Product Scope

### In Scope

- Primitives layer (`web-ui/src/primitives/`) covering the superset of primitives used by the two
  content sites, exported from the barrel alongside composites.
- Four brand token CSS files and token wiring for all six apps.
- Zero-visual-change migration of `ose-www` and `ayokoding-www`.
- Stories for every primitive and composite, plus a four-brand theme switcher.
- `vercel.json`, a deployer agent, a CI gate workflow, and a `prod-web-ui` branch.

### Out of Scope

- Any visual redesign or look-and-feel change.
- Migrating the four already-on-`web-ui` apps' components (token wiring only).
- New dependency versions or a Storybook upgrade.
- Backend / F# changes.

## Product Risks

| Risk                                                 | Mitigation                                                                          |
| ---------------------------------------------------- | ----------------------------------------------------------------------------------- |
| A primitive's props differ subtly between app copies | Build the superset from the content-site copies; gate on `fe-e2e:test:e2e` per app. |
| Theme switcher class does not map to token sheets    | Reuse the repo's existing `withThemeByClassName` pattern (already in `preview.ts`). |
| Story coverage drifts from the kit over time         | Author one story file per primitive and composite as a Phase 3 acceptance gate.     |
