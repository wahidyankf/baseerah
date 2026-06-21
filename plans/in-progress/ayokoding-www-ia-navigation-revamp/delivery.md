# Delivery Checklist — AyoKoding IA & Navigation Revamp

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase is
> not complete until its gate is green; do not start phase N+1 while any gate check fails.

This is a UI-bearing, web-UI feature-change plan. Definition of done includes full rule-15 delivery
hardening: both locales (`en`, `id`), breakpoints 320/375/768/1280 px, Phase-1 committed mockups as
visual-parity ground truth, and a near-end three-tester retest. Push target: `origin main` (direct,
Trunk Based Development). Evidence lands in [`evidence/`](./evidence/).

## Worktree

Worktree path: `worktrees/ayokoding-www-ia-navigation-revamp/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree ayokoding-www-ia-navigation-revamp
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before
deleting the worktree after the plan is archived and pushed.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [ ] [AI] Install dependencies in the root worktree: `npm install`
      — acceptance: exits 0, `node_modules/` synchronized
- [ ] [AI] Converge the full polyglot toolchain in the root worktree: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift
- [ ] [AI] **Verify the prerequisite landed on `main`**: confirm
      `plans/in-progress/ayokoding-calculator-test-fixing` is archived to `plans/done/` on `origin/main`
      (`git log origin/main --oneline -- plans/done | grep -i calculator-test-fixing`) OR its delivery
      is complete on `main` — acceptance: prerequisite evidence found on `main`; if NOT found, STOP and
      surface to the user (this is a hard dependency per `tech-docs.md §A-1`)
- [ ] [AI] **Verify shared-file overlap state on `main`**: confirm the calculator renders on the shared
      `Breadcrumb` primitive (`grep -n "Breadcrumb" apps/ayokoding-www/src/features/cost-of-living-calculator/shell/*.tsx`)
      and the tools-index polish is present (`apps/ayokoding-www/src/app/[locale]/tools/page.tsx` has the
      calc link description) — acceptance: both confirmed on `main`
- [ ] [AI] Sync/rebase this worktree on the latest `origin/main`: `git fetch origin && git rebase origin/main`
      — acceptance: worktree is on top of the prerequisite's commits, no conflicts
- [ ] [AI] Establish the test baseline:
      `npx nx run ayokoding-www:typecheck && npx nx run ayokoding-www:lint && npx nx run ayokoding-www:test:unit && npx nx run ayokoding-www:specs:coverage`
      — acceptance: baseline pass/fail recorded; all preexisting failures documented
- [ ] [AI] Resolve all preexisting failures before proceeding
      — acceptance: no preexisting failures remain unresolved

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift
- [ ] [AI] Prerequisite `ayokoding-calculator-test-fixing` confirmed landed on `main`; worktree rebased on it
- [ ] [AI] `npx nx affected -t typecheck lint test:unit specs:coverage` baseline recorded and every
      preexisting failure resolved (zero unresolved)

> **Pause Safety**: only the local toolchain was verified, the prerequisite confirmed, and the
> baseline recorded — no feature work exists yet. Safe to stop indefinitely. To resume: re-run the
> baseline command and confirm it is still clean.

---

## Phase 1: Mockups + Copy

> _Suggested executor: `web-researcher` (R7 prior art) + `swe-typescript-dev` (copy strings) +
> `swe-ui-maker` (mockups)_

- [ ] [AI] Survey existing UI (R5): read `libs/web-ui` component inventory + tokens + Storybook and
      the `apps/ayokoding-www` shell + sibling screens — acceptance: net-new components (if any) named in
      `tech-docs.md`; confirm section cards reuse existing card/border/`bg-accent` tokens (no net-new primitive)
- [ ] [AI] Prior art (R7): delegate a `web-researcher` survey of developer-content homepages
      (MDN, web.dev, Tailwind docs) — acceptance: cited summary captured to inform the alternatives
- [ ] [AI] Diverge: confirm/extend the ≥3 named low-fi ASCII alternatives per screen already drafted in
      `assets/ui-low-fi-alternatives.md` — acceptance: `grep -c "Option [ABC]" plans/in-progress/ayokoding-www-ia-navigation-revamp/assets/ui-low-fi-alternatives.md` ≥ 6
- [ ] [AI] Narrow (landing): **validate/refine** the committed Option-A finalists
      `assets/landing-{320,375,768,1280}.{svg,png}` against the surveyed primitives/tokens — acceptance:
      `ls assets/landing-{320,375,768,1280}.svg` all exist and render (PNGs present)
- [ ] [AI] Narrow (`/c` browse): **validate/refine** the committed `assets/browse-{375,768,1280}.{svg,png}`
      — acceptance: browse finalist files exist under `assets/`
- [ ] [AI] Narrow (nav chrome): **validate/refine** the committed `assets/chrome-{375,1280}.{svg,png}`
      (`chrome-375` includes the open MobileNav drawer) — acceptance: nav finalist files exist under `assets/`
- [ ] [AI] Select + Justify: confirm the named selection + rationale table is present in `prd.md §UI Design Funnel`
      and the selection record + token table in `assets/README.md` — acceptance: `grep -c "Selected:" prd.md` ≥ 3
- [ ] [AI] Responsive: confirm `prd.md` states the selected design's responsive strategy per breakpoint and the
      low-fi tier shows mobile↔desktop reflow — acceptance: `grep -ci "responsive" prd.md` ≥ 1
- [ ] [AI] Draft placeholder copy: add `en`+`id` keys for hero heading/intro, hero CTAs, section blurbs
      (fallback), `/c` browse title, and nav labels (`navLearn`, `navTools`, footer column headings) into
      `apps/ayokoding-www/src/features/i18n/core/translations.ts` — acceptance: keys present for both locales;
      `npx nx run ayokoding-www:typecheck` exits 0
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [HUMAN] Refine final bilingual copy wording (hero, blurbs, nav labels) in `translations.ts`
      — handoff: agent leaves placeholder strings tagged `// TODO(copy): refine`; **observable resume signal**:
      the maintainer replaces the placeholders and removes the TODO markers, then says "copy refined".
      (Per `tech-docs.md §A-2`; may be deferred until just before archival.)

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] All funnel finalist `.svg`+`.png` files exist under `assets/` (landing, browse, nav)
- [ ] [AI] `npx nx run ayokoding-www:typecheck` exits 0 with the new translation keys
- [ ] [AI] `git status` shows only `plans/` + `apps/ayokoding-www/src/features/i18n/core/translations.ts` changes

> **Pause Safety**: only mockups (docs) and additive translation keys exist — no routing or behavior
> changed; the site still renders as before. Safe to stop. To resume:
> `npx nx run ayokoding-www:typecheck`.

---

## Phase 2: `/c` Route + `contentUrl` Helper + Redirects + `/c` Browse Index

> _Suggested executor: `swe-typescript-dev`; e2e steps `swe-e2e-dev`_

### `contentUrl` helper (core)

- [ ] [AI] **RED**: write failing unit test for `contentUrl(locale, slug)` in
      `apps/ayokoding-www/src/features/content/core/content-url.test.ts` (new) — assert
      `contentUrl("en","learn/software-engineering") === "/en/c/learn/software-engineering"`,
      `contentUrl("id","belajar/ikhtisar") === "/id/c/belajar/ikhtisar"`,
      `contentUrl("en","about-ayokoding") === "/en/about-ayokoding"` (loose, no `/c/`),
      `contentUrl("id","tentang-ayokoding") === "/id/tentang-ayokoding"`
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: test fails (`contentUrl` undefined)

  **Gherkin (binds) →** "English content resolves under the /c namespace"

  ```gherkin
  Scenario: English content resolves under the /c namespace
    Given the en learn content exists on disk under content/en/learn
    When a visitor navigates to "/en/c/learn/software-engineering"
    Then the content page renders with status 200
    And the breadcrumb reflects the "/c/" prefixed path
  ```

- [ ] [AI] **GREEN**: implement `contentUrl` + the per-locale loose-page allowlist in
      `apps/ayokoding-www/src/features/content/core/content-url.ts` (new) per `tech-docs.md §DD-1`
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: the new test passes, no others broken
- [ ] [AI] **REFACTOR**: extract the loose-page allowlist constant + add JSDoc
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: all tests still pass

### `c/[...slug]` route + narrow the legacy `[...slug]`

- [ ] [AI] Derive the per-locale moved-section list: `ls apps/ayokoding-www/content/en/` and
      `apps/ayokoding-www/content/id/` — acceptance: section list recorded in notes (expected
      `en: learn, rants`; `id: belajar, celoteh, konten-video`), resolving `tech-docs.md §A-4`
- [ ] [AI] **RED**: add e2e scenario asserting `/en/c/learn/software-engineering` returns 200 in
      `apps/ayokoding-www-fe-e2e/src/` (new spec, sibling to existing fe-e2e specs) — command:
      `npx nx run ayokoding-www-fe-e2e:test:e2e` — acceptance: fails (route does not exist yet)

  **Gherkin (binds) →** "English content resolves under the /c namespace"

  ```gherkin
  Scenario: English content resolves under the /c namespace
    Given the en learn content exists on disk under content/en/learn
    When a visitor navigates to "/en/c/learn/software-engineering"
    Then the content page renders with status 200
    And the breadcrumb reflects the "/c/" prefixed path
  ```

  - _Suggested executor: `swe-e2e-dev`_

- [ ] [AI] **GREEN**: create `apps/ayokoding-www/src/app/[locale]/(content)/c/[...slug]/page.tsx`
      (+ `layout.tsx`/`error.tsx`/`not-found.tsx` mirroring the sibling `[...slug]/` route) that strips the
      leading `c/`-free slug, calls `getBySlug(locale, rest)`, sets `dynamicParams = false`, and enumerates
      content slugs in `generateStaticParams` — command: `npx nx run ayokoding-www-fe-e2e:test:e2e` — acceptance:
      `/en/c/learn/...` and `/id/c/belajar/...` return 200
- [ ] [AI] **GREEN**: narrow `apps/ayokoding-www/src/app/[locale]/(content)/[...slug]/page.tsx`
      `generateStaticParams` to the per-locale loose-page allowlist only (about/terms/\_index), per
      `tech-docs.md §DD-2` — command: `npx nx run ayokoding-www-fe-e2e:test:e2e` — acceptance:
      `/en/about-ayokoding` still 200; old content paths now fall through to redirects
- [ ] [AI] **REFACTOR**: deduplicate shared slug-splitting logic between the two catch-alls into a
      `core` helper — command: `npx nx run ayokoding-www:test:unit` — acceptance: all tests pass

### Redirects module (per-locale, per-section, 308)

- [ ] [AI] **RED**: add e2e scenario asserting `GET /en/learn/software-engineering` → 308 with
      `Location: /en/c/learn/software-engineering` and `GET /id/belajar/ikhtisar` → 308 →
      `/id/c/belajar/ikhtisar` — command: `npx nx run ayokoding-www-fe-e2e:test:e2e` — acceptance: fails (no redirect)

  **Gherkin (binds) →** "Old English learn URL permanently redirects to the /c namespace"

  ```gherkin
  Scenario: Old English learn URL permanently redirects to the /c namespace
    Given an external bookmark points at "/en/learn/software-engineering"
    When a client requests that URL
    Then the server responds 308 with Location "/en/c/learn/software-engineering"
  ```

  - _Suggested executor: `swe-e2e-dev`_

- [ ] [AI] **GREEN**: create `apps/ayokoding-www/src/redirects/content-namespace.ts` exporting
      `contentNamespaceRedirects` (per-locale `:path*` wildcard rules with `permanent: true` for
      en `learn`/`rants` and id `belajar`/`celoteh`/`konten-video`, per `tech-docs.md §DD-3`) and spread it
      into `apps/ayokoding-www/next.config.ts` `redirects()` after `learnReorgRedirects` — command:
      `npx nx run ayokoding-www-fe-e2e:test:e2e` — acceptance: 308 scenarios pass

  > **Note (overlap check)**: Before spreading, grep the existing `learnReorgRedirects` array for
  > any sources that overlap with the new `contentNamespaceRedirects` entries:
  > `grep -n "source" apps/ayokoding-www/src/redirects/learn-reorg.ts` — deduplicate any conflicting
  > or redundant rules before adding the new array to avoid Next.js redirect-precedence surprises.

- [ ] [AI] **RED**: add e2e scenario asserting `/en/about-ayokoding`, `/id/syarat-dan-ketentuan`, and
      `/en/tools` are 200 and NOT redirected — command: `npx nx run ayokoding-www-fe-e2e:test:e2e` — acceptance:
      passes immediately if rules are correctly scoped (guards against over-broad wildcards)

  **Gherkin (binds) →** "About page keeps its top-level URL and is not redirected"

  ```gherkin
  Scenario: About page keeps its top-level URL and is not redirected
    Given a visitor opens "/en/about-ayokoding"
    When the server handles the request
    Then the response is 200 and not a redirect
    And the URL remains "/en/about-ayokoding"
  ```

- [ ] [AI] **REFACTOR**: add a unit test asserting the redirect-array shape (every entry
      `permanent: true`, `source`/`destination` non-empty) in
      `apps/ayokoding-www/src/redirects/content-namespace.test.ts` (new) — command:
      `npx nx run ayokoding-www:test:unit` — acceptance: passes

### `/c` browse index page

- [ ] [AI] **RED**: write failing unit/component test for the `/c` browse index rendering section cards
      for every top-level section in
      `apps/ayokoding-www/src/app/[locale]/(content)/c/page.test.tsx` (new) — command:
      `npx nx run ayokoding-www:test:unit` — acceptance: fails (page does not exist)

  **Gherkin (binds) →** "The /c browse index lists all content sections"

  ```gherkin
  Scenario: The /c browse index lists all content sections
    Given the content tree has top-level sections for the en locale
    When a visitor navigates to "/en/c"
    Then the page shows a browse index of section cards for every top-level section
    And the page shows a breadcrumb beginning at Home
  ```

- [ ] [AI] **GREEN**: create `apps/ayokoding-www/src/app/[locale]/(content)/c/page.tsx` rendering the
      restyled section-card browse index (Option A) from `getTree(locale)`, with a `Home > Browse`
      breadcrumb — command: `npx nx run ayokoding-www:test:unit` — acceptance: test passes
- [ ] [AI] **REFACTOR**: extract the shared SectionCard into
      `apps/ayokoding-www/src/features/content/shell/section-card.tsx` (reused by landing in P4) — command:
      `npx nx run ayokoding-www:test:unit` — acceptance: all tests pass

### Companion Gherkin (two-path rule)

- [ ] [AI] Add `specs/apps/ayokoding/behavior/ayokoding-www/gherkin/navigation/content-namespace-redirects.feature`
      (_New file_) and `.../ia-navigation-revamp.feature` (_New file_) covering the `/c` route,
      308 redirects, about/terms-not-redirected, and `/c` browse index scenarios from `prd.md`
      — command: `npx nx run ayokoding-www:specs:coverage`
      — acceptance: exits 0 (every new scenario has a backing step)
  - _Suggested executor: `specs-maker`_

### Phase 2 Gate

> All checks below must pass before starting Phase 3 / Phase 4.

- [ ] [AI] `npx nx run ayokoding-www:typecheck lint test:unit specs:coverage` — all exit 0
- [ ] [AI] `npx nx run ayokoding-www-fe-e2e:test:e2e` — `/c` content 200, 308 redirects, about/terms not-redirected scenarios pass
- [ ] [AI] Commit and push to origin main (thematic commits per Commit Guidelines below)

> **Pause Safety**: content is now reachable at `/c/...`, old URLs 308-redirect, about/terms/tools
> stay top-level, and the `/c` browse index renders — a coherent, shippable state even though the
> landing page and nav chrome are not yet updated (the homepage still shows the old tree, header/footer
> still have no nav). Safe to stop. To resume: `npx nx run ayokoding-www-fe-e2e:test:e2e`.

---

## Phase 3: Header + Footer + Mobile Nav

> _Suggested executor: `swe-typescript-dev` (TSX) / `swe-ui-maker`_

- [ ] [AI] **RED**: extend `apps/ayokoding-www/src/features/app-shell/shell/header.tsx`'s test (or add
      `header.test.tsx`) asserting the header renders a "Learn" link to `/en/c` and a "Tools" link to
      `/en/tools` — command: `npx nx run ayokoding-www:test:unit` — acceptance: fails (no nav links)

  **Gherkin (binds) →** "Header shows primary nav links on desktop"

  ```gherkin
  Scenario: Header shows primary nav links on desktop
    Given a visitor is on any "/en" page at desktop width
    When the header renders
    Then the header shows a "Learn" link to "/en/c" and a "Tools" link to "/en/tools"
  ```

- [ ] [AI] **GREEN**: add the inline primary nav (`Learn` → `/${locale}/c`, `Tools` → `/${locale}/tools`,
      labels via `t(locale, "navLearn"/"navTools")`) to `header.tsx`, hidden on mobile (`hidden md:flex`)
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: header test passes
- [ ] [AI] **RED**: extend `apps/ayokoding-www/src/features/app-shell/shell/mobile-nav.test.tsx` asserting the
      mobile menu shows the same Learn/Tools links — command: `npx nx run ayokoding-www:test:unit` — acceptance: fails

  **Gherkin (binds) →** "Mobile navigation mirrors the header links"

  ```gherkin
  Scenario: Mobile navigation mirrors the header links
    Given a visitor is on an "/en" page at mobile width
    When the visitor opens the mobile navigation menu
    Then the menu shows a "Learn" link to "/en/c" and a "Tools" link to "/en/tools"
  ```

- [ ] [AI] **GREEN**: add the Learn/Tools links to `mobile-nav.tsx` — command:
      `npx nx run ayokoding-www:test:unit` — acceptance: mobile-nav test passes
- [ ] [AI] **RED**: add a `footer.test.tsx` asserting the footer renders Learn / Tools / About columns with
      localized labels and About links to `/${locale}/about-ayokoding` (en) and `/${locale}/tentang-ayokoding`
      (id) via the loose-page allowlist — command: `npx nx run ayokoding-www:test:unit` — acceptance: fails

  **Gherkin (binds) →** "Footer shows grouped navigation with localized labels"

  ```gherkin
  Scenario: Footer shows grouped navigation with localized labels
    Given a visitor is on any "/id" page
    When the footer renders
    Then the footer shows a Learn column, a Tools column, and an About column
    And the About column links to "/id/tentang-ayokoding" and "/id/syarat-dan-ketentuan"
  ```

- [ ] [AI] **GREEN**: rebuild `footer.tsx` into a multi-column nav (Learn · Tools · About/Terms) using
      per-locale loose-page slugs + `contentUrl`/allowlist, keeping the copyright + license row — command:
      `npx nx run ayokoding-www:test:unit` — acceptance: footer test passes
- [ ] [AI] **REFACTOR**: extract a shared `NavLinks` list config (label key + href builder) reused by header,
      mobile-nav, and footer — command: `npx nx run ayokoding-www:test:unit` — acceptance: all tests pass
- [ ] [AI] Add companion Gherkin for header/footer/mobile nav presence + targets into
      `specs/apps/ayokoding/behavior/ayokoding-www/gherkin/navigation/ia-navigation-revamp.feature` — command:
      `npx nx run ayokoding-www:specs:coverage` — acceptance: exits 0
  - _Suggested executor: `specs-maker`_

### Phase 3 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `npx nx run ayokoding-www:typecheck lint test:unit specs:coverage` — all exit 0
- [ ] [AI] Commit and push to origin main

> **Pause Safety**: global header/footer/mobile nav now link Learn and Tools on every page — a
> coherent, shippable state (the homepage may still be the old tree until P4, but navigation works
> everywhere). Safe to stop. To resume: `npx nx run ayokoding-www:test:unit`.

---

## Phase 4: Landing Homepage (Hero + Section Cards + Tools Teaser)

> _Suggested executor: `swe-typescript-dev` / `swe-ui-maker`_

- [ ] [AI] **RED**: write failing unit test for `landing-sections` derivation+override in
      `apps/ayokoding-www/src/features/content/core/landing-sections.test.ts` (new) — assert order/hide/icon
      overrides apply and title/blurb fall back to `_index.md` — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: fails (module undefined)

  **Gherkin (binds) →** "Section cards derive from the content tree with curated overrides"

  ```gherkin
  Scenario: Section cards derive from the content tree with curated overrides
    Given the content tree exposes top-level sections via the content service
    When the landing page builds its section cards from the curated-override config
    Then each visible card shows the section title and a blurb from its _index.md or an override
    But sections marked hidden in the config do not render a card
  ```

- [ ] [AI] **GREEN**: implement `apps/ayokoding-www/src/features/content/core/landing-sections.ts` (curated
      override config + pure merge) per `tech-docs.md §DD-4` — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: test passes
- [ ] [AI] **REFACTOR**: tidy the override-merge + add JSDoc — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: all tests pass
- [ ] [AI] **RED**: write failing component test for the landing page in
      `apps/ayokoding-www/src/app/[locale]/page.test.tsx` (new) asserting hero heading+intro, section cards
      (including Rants/Celoteh), and a Tools teaser linking `/${locale}/tools/cost-of-living-calculator`
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: fails (still the old tree)

  **Gherkin (binds) →** "Landing homepage renders hero, sections, and tools teaser in English"

  ```gherkin
  Scenario: Landing homepage renders hero, sections, and tools teaser in English
    Given the AyoKoding site is running with the en locale
    When a visitor navigates to "/en"
    Then the page shows the hero heading and intro
    And the page shows curated section cards including "Rants"
    And the page shows a Tools teaser card linking "/en/tools/cost-of-living-calculator"
  ```

- [ ] [AI] **GREEN**: rewrite `apps/ayokoding-www/src/app/[locale]/page.tsx` into the homepage (hero via
      `t()`, section cards via `landing-sections` + `SectionCard` from P2, Tools teaser card) matching the
      selected Option A mockups — command: `npx nx run ayokoding-www:test:unit` — acceptance: landing test passes
- [ ] [AI] **REFACTOR**: extract the hero + Tools-teaser into
      `apps/ayokoding-www/src/features/app-shell/shell/{hero,tools-teaser}.tsx` — command:
      `npx nx run ayokoding-www:test:unit` — acceptance: all tests pass
- [ ] [AI] Add companion Gherkin for landing hero/sections/teaser (both locales) into
      `.../navigation/ia-navigation-revamp.feature` — command: `npx nx run ayokoding-www:specs:coverage`
      — acceptance: exits 0
  - _Suggested executor: `specs-maker`_

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `npx nx run ayokoding-www:typecheck lint test:unit specs:coverage` — all exit 0
- [ ] [AI] Commit and push to origin main

> **Pause Safety**: the homepage is now a real homepage (hero + cards + Tools teaser) and the old
> bare tree lives at `/c`. Combined with P2/P3 this is the full IA — a coherent, shippable state.
> Internal content-link emitters are swept in P5 but the site is navigable now. Safe to stop. To
> resume: `npx nx run ayokoding-www:test:unit`.

---

## Phase 5: SEO + Internals Sweep

> _Suggested executor: `swe-typescript-dev`; e2e `swe-e2e-dev`_

- [ ] [AI] **RED**: extend `apps/ayokoding-www/src/features/navigation/shell/breadcrumb.test.tsx` asserting
      ancestor crumbs link to `/c/` URLs (via `contentUrl`) — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: fails (breadcrumb still emits bare slug hrefs)

  **Gherkin (binds) →** "Breadcrumb segments link to /c URLs"

  ```gherkin
  Scenario: Breadcrumb segments link to /c URLs
    Given a visitor is on "/en/c/learn/software-engineering/data"
    When the breadcrumb renders its ancestor segments
    Then each ancestor crumb links to a "/c/" prefixed URL
  ```

- [ ] [AI] **GREEN**: route breadcrumb hrefs through `contentUrl` in `breadcrumb.tsx` and its callers
      (content page `c/[...slug]/page.tsx` `buildBreadcrumbs`) — command: `npx nx run ayokoding-www:test:unit`
      — acceptance: breadcrumb test passes
- [ ] [AI] **RED**: add failing test asserting `sidebar-tree.tsx` and `prev-next.tsx` emit `/c/` hrefs via
      `contentUrl` — command: `npx nx run ayokoding-www:test:unit` — acceptance: fails

  **Gherkin (binds) →** "Internal content links emit /c URLs directly without relying on redirects"

  ```gherkin
  Scenario: Internal content links emit /c URLs directly without relying on redirects
    Given the sidebar tree, breadcrumb, prev-next, and search results render content links
    When their hrefs are computed via the central content URL helper
    Then every content link resolves directly to a "/c/" URL with status 200
    And no internal content link resolves through a 308 redirect
  ```

- [ ] [AI] **GREEN**: update `sidebar-tree.tsx` and `prev-next.tsx` to build hrefs via `contentUrl(locale, slug)`
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: tests pass
- [ ] [AI] **RED**: add failing test in `apps/ayokoding-www/src/features/search/shell/search-dialog.test.tsx` (new)
      asserting `apps/ayokoding-www/src/features/search/shell/search-dialog.tsx` result links use `contentUrl`
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: fails (search links emit bare slug hrefs)

  **Gherkin (binds) →** "Internal content links emit /c URLs directly without relying on redirects"

  ```gherkin
  Scenario: Internal content links emit /c URLs directly without relying on redirects
    Given the sidebar tree, breadcrumb, prev-next, and search results render content links
    When their hrefs are computed via the central content URL helper
    Then every content link resolves directly to a "/c/" URL with status 200
    And no internal content link resolves through a 308 redirect
  ```

- [ ] [AI] **GREEN**: update search results rendering (`apps/ayokoding-www/src/features/search/shell/search-dialog.tsx`)
      to link via `contentUrl` — command: `npx nx run ayokoding-www:test:unit` — acceptance: search-result links use `/c/`
- [ ] [AI] **RED**: add failing test asserting `sitemap.ts` emits `/c/` for content + bare for loose pages, in
      `apps/ayokoding-www/src/app/sitemap.test.ts` (new) — command: `npx nx run ayokoding-www:test:unit` — acceptance: fails

  **Gherkin (binds) →** "Sitemap lists only the new /c content URLs"

  ```gherkin
  Scenario: Sitemap lists only the new /c content URLs
    Given the sitemap is generated from the content index
    When the sitemap entries are produced
    Then every moved-content entry uses a "/c/" prefixed URL
    But top-level pages (about, terms, tools) are not prefixed with "/c/"
  ```

- [ ] [AI] **GREEN**: update `apps/ayokoding-www/src/app/sitemap.ts` to build URLs via `contentUrl` — command:
      `npx nx run ayokoding-www:test:unit` — acceptance: sitemap test passes
- [ ] [AI] **RED**: add failing test asserting `apps/ayokoding-www/src/app/feed.xml/route.ts` item links use
      `contentUrl` in `apps/ayokoding-www/src/app/feed.xml/route.test.ts` (new) — command:
      `npx nx run ayokoding-www:test:unit` — acceptance: fails (feed links emit bare slug hrefs)

  **Gherkin (binds) →** "RSS feed item links use the new /c content URLs"

  ```gherkin
  Scenario: RSS feed item links use the new /c content URLs
    Given the feed is generated from the content index
    When the feed items are produced
    Then every content item link uses a "/c/" prefixed URL
  ```

- [ ] [AI] **GREEN**: update `apps/ayokoding-www/src/app/feed.xml/route.ts` item links via `contentUrl` — command:
      `npx nx run ayokoding-www:test:unit` — acceptance: feed item links use `/c/`
- [ ] [AI] **RED**: add failing test in `apps/ayokoding-www/src/app/[locale]/(content)/c/[...slug]/page.test.tsx` (new)
      asserting `apps/ayokoding-www/src/app/[locale]/(content)/c/[...slug]/page.tsx`
      `generateMetadata` sets `alternates.canonical` to the `/c/` URL and includes `alternates.languages`
      — command: `npx nx run ayokoding-www:test:unit` — acceptance: fails (canonical/languages absent)

  **Gherkin (binds) →** "Canonical link for moved content points to the /c URL"

  ```gherkin
  Scenario: Canonical link for moved content points to the /c URL
    Given the content page at "/en/c/learn/software-engineering"
    When its metadata is generated
    Then the canonical alternate is "/en/c/learn/software-engineering"
    And the language alternates include en, id, and x-default
  ```

- [ ] [AI] **GREEN**: update `c/[...slug]/page.tsx` `generateMetadata` so `alternates.canonical` is the `/c/` URL and
      add `alternates.languages` (`en`, `id`, `x-default`); set `metadataBase` for relative alternates — command:
      `npx nx run ayokoding-www:test:unit` — acceptance: canonical + languages present
- [ ] [AI] **RED**: add e2e scenario asserting no internal content link resolves through a 308 (crawl rendered
      links, assert each returns 200 directly) — command: `npx nx run ayokoding-www-fe-e2e:test:e2e` — acceptance: fails if any redirect-dependent link remains

  **Gherkin (binds) →** "Internal content links emit /c URLs directly without relying on redirects"

  ```gherkin
  Scenario: Internal content links emit /c URLs directly without relying on redirects
    Given the sidebar tree, breadcrumb, prev-next, and search results render content links
    When their hrefs are computed via the central content URL helper
    Then every content link resolves directly to a "/c/" URL with status 200
    And no internal content link resolves through a 308 redirect
  ```

  - _Suggested executor: `swe-e2e-dev`_

- [ ] [AI] **GREEN**: fix any remaining emitter still producing a bare-slug URL — command:
      `npx nx run ayokoding-www-fe-e2e:test:e2e` — acceptance: no-308-hop scenario passes
- [ ] [AI] **REFACTOR**: confirm every content-URL construction in `apps/ayokoding-www` routes through
      `contentUrl` (`grep -rn "/\${locale}/" apps/ayokoding-www/src` audited) — command:
      `npx nx run ayokoding-www:test:unit` — acceptance: no stray hand-built content URLs remain
- [ ] [AI] Add companion Gherkin for canonical/sitemap/feed/no-broken-links into
      `.../navigation/ia-navigation-revamp.feature` — command: `npx nx run ayokoding-www:specs:coverage`
      — acceptance: exits 0
  - _Suggested executor: `specs-maker`_

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [ ] [AI] `npx nx run ayokoding-www:typecheck lint test:unit specs:coverage` — all exit 0
- [ ] [AI] `npx nx run ayokoding-www-fe-e2e:test:e2e` — canonical/sitemap/feed + no-308-hop scenarios pass
- [ ] [AI] Commit and push to origin main

> **Pause Safety**: every internal emitter and SEO surface now emits `/c/` URLs directly; no internal
> link depends on a redirect; canonical/sitemap/feed are consistent. The IA + SEO are complete and
> shippable. Safe to stop. To resume: `npx nx run ayokoding-www-fe-e2e:test:e2e`.

---

## Phase 6: Full A11y + Responsive + Both-Locale Verification

> _Suggested executor: `swe-e2e-dev` for automated a11y; manual Playwright MCP for evidence_

### Local Quality Gates (Before Push)

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck`
- [ ] [AI] Run affected linting: `npx nx affected -t lint`
- [ ] [AI] Run affected quick/unit tests: `npx nx affected -t test:unit`
- [ ] [AI] Run affected spec coverage: `npx nx affected -t specs:coverage`
- [ ] [AI] Fix ALL failures — including preexisting issues not caused by your changes
- [ ] [AI] Re-run failing checks to confirm resolution; verify zero failures before pushing

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes.
> This follows the root cause orientation principle — proactively fix preexisting errors encountered
> during work. Commit preexisting fixes separately with appropriate conventional commit messages.

### Manual UI Verification (Playwright MCP) — all locales × all breakpoints

- [ ] [AI] Confirm supported locales from `apps/ayokoding-www/src/features/i18n/core/config.ts` — acceptance: `en`, `id` listed
- [ ] [AI] Start dev server: `nx dev ayokoding-www` (port 3101)
- [ ] [AI] For EACH locale (`en`, `id`) × EACH breakpoint (320 / 375 / 768 / 1280 px): navigate to
      `/en` and `/id` via `browser_navigate` + `browser_resize` — acceptance: homepage renders, no horizontal overflow at 320 px
- [ ] [AI] Inspect DOM via `browser_snapshot`: verify `html[lang]` matches the locale, hero+cards+teaser+nav
      strings are translated (no untranslated keys) — acceptance: correct language, `lang` attribute correct
- [ ] [AI] Keyboard a11y: Tab from page top — first focus is the skip link; header nav links reachable/operable
      via keyboard — acceptance: skip link first, nav keyboard-operable
- [ ] [AI] Exercise nav flows via `browser_click`: header Learn → `/c`, header Tools → `/tools`, Tools teaser →
      calculator, footer About → loose page; mobile hamburger opens MobileNav with Learn/Tools — acceptance: each lands on the expected URL
- [ ] [AI] Check JS errors via `browser_console_messages` — acceptance: zero errors per locale
- [ ] [AI] Verify network via `browser_network_requests`: old `/en/learn/...` typed in addressbar 308s to `/c/...`
      — acceptance: 308 observed
- [ ] [AI] Capture one screenshot per locale per breakpoint of `/`, `/c`, and a `/c/...` content page via
      `browser_take_screenshot` to `evidence/phase-6-<page>-<locale>-<breakpoint>px.png` — acceptance: files exist in `evidence/`
- [ ] [AI] Document evidence in this checklist: reference each screenshot (`![alt](./evidence/...)`) and note
      console/network status per locale

### Visual-parity sign-off (against Phase-1 mockups)

- [ ] [AI] Compare each captured screenshot against the matching committed Option-A mockup
      (`assets/landing-*`, `assets/browse-*`, `assets/chrome-*`) per
      breakpoint/locale — acceptance: layout matches the selected Option A within reasonable tolerance; deviations noted/fixed

### Phase 6 Gate

> All checks below must pass before starting Phase 7.

- [ ] [AI] `npx nx affected -t typecheck lint test:unit specs:coverage` — all exit 0
- [ ] [AI] Both locales × 4 breakpoints captured to `evidence/` with zero console errors and no 320 px overflow
- [ ] [AI] Visual-parity sign-off recorded against Phase-1 mockups
- [ ] [AI] Commit and push to origin main

> **Pause Safety**: the IA is implemented, accessible, responsive, and verified in both locales with
> committed evidence. Safe to stop. To resume: re-run `nx dev ayokoding-www` and re-check the
> evidence screenshots.

---

## Phase 7: Rule-15 Three-Tester Retest (before archival)

> _Executors: `web-exploratory-tester`, `web-usability-tester`, `web-design-tester`_

- [ ] [AI] Run the three live-site testers (the `web-ux-test-fixing-planning` workflow:
      `web-exploratory-tester` + `web-usability-tester` + `web-design-tester`, each invoked with
      `output-mode: delivery` and this plan's `plan-path`) against the running site
      (`/en`, `/id`, `/en/c`, `/id/c`, a `/c/...` page, header/footer/mobile nav, Tools teaser) across ALL
      supported locales — acceptance: EWT/UWT/DWT findings + any spec-gaps recorded
- [ ] [AI] Append each finding below as a new unchecked checkbox, source-attributed
      (`- [ ] EWT-NNN:` / `- [ ] UWT-NNN:` / `- [ ] DWT-NNN: <defect> — fix before archival`); append each
      SG-### spec-gap into the specs steps — acceptance: every finding captured as a task item

### Rule-15 retest follow-ups

_(Populated by the three-tester run; every EWT/UWT/DWT defect finding is FIXED before archival.
Deferral is allowed ONLY with explicit user permission and only when fixing is genuinely
impossible. SG-### spec-gap proposals may be triaged.)_

- [ ] [AI] Fix **every** rule-15 EWT/UWT/DWT finding and re-run the relevant gate
      — command: `npx nx run ayokoding-www:typecheck lint test:unit specs:coverage` (+ `ayokoding-www-fe-e2e:test:e2e` where runtime proof is needed)
      — acceptance: all defect findings fixed (no deferral without explicit user permission for a genuinely-impossible fix), gates green

### Phase 7 Gate

> All checks below must pass before archival.

- [ ] [AI] All rule-15 EWT/UWT/DWT findings fixed (deferral only with explicit user permission for a genuinely-impossible fix)
- [ ] [AI] `npx nx affected -t typecheck lint test:unit specs:coverage` — all exit 0
- [ ] [AI] Confirm no `// TODO(copy):` markers remain — command:
      `grep -c "TODO(copy)" apps/ayokoding-www/src/features/i18n/core/translations.ts`
      — acceptance: returns 0
- [ ] [AI] Commit and push to origin main

> **Pause Safety**: the revamp is fully verified including the live-site tester retest. Safe to stop.
> To resume: re-run the three testers if any code changed since the last run.

---

## Commit Guidelines

- [ ] [AI] Commit changes thematically — group related changes into logically cohesive commits
- [ ] [AI] Follow Conventional Commits: `<type>(ayokoding-www): <description>`
- [ ] [AI] Split different domains/concerns into separate commits (routing vs nav vs landing vs SEO vs specs)
- [ ] [AI] Preexisting fixes get their own commits, separate from plan work
- [ ] [AI] Do NOT bundle unrelated changes into a single commit

## Post-Push CI Verification (after every push)

- [ ] [AI] Push changes to `origin main`
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push (poll every ~3 min; do NOT use `gh run watch`)
- [ ] [AI] Verify ALL CI checks pass — no exceptions
- [ ] [AI] If any CI check fails, fix immediately and push a follow-up commit; repeat until green
- [ ] [AI] Do NOT proceed to the next delivery phase until CI is fully green

## Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked
- [ ] [AI] Verify ALL quality gates pass (local + CI)
- [ ] [AI] Verify ALL manual assertions pass (Playwright MCP) with committed evidence in `evidence/`
- [ ] [AI] Verify ALL supported locales (`en`, `id`) were exercised at all 4 breakpoints (not just the default)
- [ ] [AI] Verify every rule-15 three-tester finding (EWT/UWT/DWT) is fixed (deferral only with explicit user permission for a genuinely-impossible fix)
- [ ] [AI] Verify the `[HUMAN]` copy-refinement step is complete (no `TODO(copy)` markers remain)
- [ ] [AI] Move: `git mv plans/in-progress/ayokoding-www-ia-navigation-revamp/ plans/done/YYYY-MM-DD__ayokoding-www-ia-navigation-revamp/`
      using today's date as the completion date (the `evidence/` subfolder moves with it)
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date
- [ ] [AI] Update any other READMEs that reference this plan
- [ ] [AI] Commit the archival: `chore(plans): move ayokoding-www-ia-navigation-revamp to done`
