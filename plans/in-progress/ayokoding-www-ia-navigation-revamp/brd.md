# Business Requirements — AyoKoding IA & Navigation Revamp

## Business Goal

Make AyoKoding's surfaces — the **Learn** content library and the **Tools** area (which contains the
cost-of-living calculator) — reachable and discoverable through natural, consistent navigation, so a
visitor who lands on the homepage can flow into any part of the site without typing a URL or guessing.

## Business Rationale (WHY)

AyoKoding is a bilingual educational platform whose value is its breadth: a large `learn/` library
(1,165 `en` markdown files, 124 `id` files) [Repo-grounded — verified 2026-06-21 via
`find apps/ayokoding-www/content/en/learn -name "*.md" | wc -l` = 1165;
`find apps/ayokoding-www/content/id -name "*.md" | wc -l` = 124; counts will drift over time —
treat as "large" rather than exact], a `rants/` section, and a Tools area
with the cost-of-living calculator. Today that value is **structurally hidden**:

- The homepage is a bare slug list — it neither explains what AyoKoding is nor curates an entry
  point into the content, so a first-time visitor cannot tell what the site offers.
- The header and footer carry **no navigation**, so once a reader leaves the homepage there is no
  global way back to Learn or across to Tools. [Repo-grounded]
- The calculator is therefore an **island** — reachable only by direct URL or the Tools index, with
  nothing on the homepage or chrome pointing to it.

Fixing the IA is the highest-leverage change available: it does not require new content or new
tools, only a navigable shell and a real homepage that surface what already exists.

## Business Impact

### Pain points addressed

- **Discoverability gap**: the calculator and the whole Tools area are effectively undiscoverable
  from the homepage. _Judgment call: a homepage that is a bare slug list with no Tools entry gives a
  visitor no signal that Tools exist; this is reasoned from the current `page.tsx` rendering only a
  content tree._ [Repo-grounded]
- **No global wayfinding**: empty header/footer nav means every cross-section move requires the
  browser back button or a manual URL edit. [Repo-grounded]
- **No homepage value proposition**: the bare tree communicates nothing about what AyoKoding is or
  who it is for.

### Expected benefits

- A homepage that states the value proposition and routes visitors into Learn, Tools, and the
  calculator. _Qualitative reasoning: a hero + curated cards + a Tools teaser is the conventional,
  well-understood pattern for turning a content dump into a navigable homepage._
- Persistent header + footer navigation so any page connects to Learn and Tools. _Qualitative
  reasoning: global chrome nav is the baseline expectation for content sites._
- A clean, SEO-safe content URL namespace (`/c/...`) that separates the browsable content library
  from top-level product pages (Tools, About, Terms), with 308 redirects preserving existing
  backlinks and search rankings.

## Affected Roles

This is a solo-maintainer repository — no sign-off ceremonies. The hats the maintainer wears:

- **Site owner / product** — defines the IA, curates the section cards and homepage copy, refines
  final bilingual wording (the `[HUMAN]` copy-refinement step in delivery).
- **Frontend engineer** — implements routing, the `contentUrl` helper, redirects, nav components,
  and the landing page.
- **SEO steward** — verifies canonical/sitemap/feed/redirect correctness on the URL move.
- **Accessibility reviewer** — verifies WCAG AA, keyboard nav, skip link across breakpoints/locales.

Consuming agents: `repo-setup-manager` (Phase 0), `swe-typescript-dev` (TS/TSX implementation),
`swe-e2e-dev` (Playwright e2e), `web-exploratory-tester` / `web-usability-tester` /
`web-design-tester` (rule-15 retest).

## Business-Level Success Metrics

- **Navigability** (observable fact): from the homepage, a visitor can reach Learn, the `/c` browse
  index, the Tools index, and the calculator using only on-page navigation — verified by the e2e
  click-through scenarios in [prd.md](./prd.md). [Judgment call on threshold; observable via tests.]
- **No regression on existing content reachability** (observable fact): every previously reachable
  content URL still resolves — either directly at its new `/c/` URL or via a 308 redirect — verified
  by redirect + sitemap scenarios.
- **SEO continuity** (observable fact): canonical, sitemap, and feed all emit the new `/c/` URLs;
  old URLs return 308 permanent redirects — verified by the SEO scenarios.
- **Accessibility** (observable fact): skip link, keyboard navigation, and WCAG AA contrast pass at
  all four breakpoints in both locales — verified by the a11y scenarios and the rule-15 design
  tester.

## Business-Scope Non-Goals

- Not a content rewrite: markdown bodies are unchanged (only URL namespace + landing copy strings).
- Not a tools expansion: no new tools; calculator internals are owned by the prerequisite plan.
- Not a search-engine change: FlexSearch stays; only emitted URLs change.
- Not a localization expansion: still `en` + `id`.

## Business Risks and Mitigations

| Risk                                                           | Likelihood | Impact | Mitigation                                                                                                                          |
| -------------------------------------------------------------- | ---------- | ------ | ----------------------------------------------------------------------------------------------------------------------------------- |
| URL move breaks external backlinks / search rankings           | Medium     | High   | 308 permanent redirects on every moved namespace, kept indefinitely; canonical + sitemap point only to new URLs.                    |
| Internal links left pointing at old URLs (redirect-dependent)  | Medium     | Medium | P5 sweep updates every emitter via the central `contentUrl` helper; e2e asserts no internal link resolves through a 308.            |
| Locale slug asymmetry causes id content to 404 or mis-redirect | Medium     | High   | `contentUrl` + redirects are per-locale slug-aware; Gherkin covers `id` (`belajar`/`celoteh`) explicitly.                           |
| Prerequisite plan not yet merged → shared-file conflicts       | Medium     | High   | Phase 0 hard-verifies the prerequisite landed on `main` before any work; build on top, no parallel edits to tools-index/breadcrumb. |
| Homepage copy ships as placeholder English-only                | Low        | Medium | Placeholder copy drafted in-plan; a `[HUMAN]` step has the maintainer refine final bilingual wording before archival.               |

See [prd.md §Product Risks](./prd.md#product-risks) for the testable product-level counterparts.
