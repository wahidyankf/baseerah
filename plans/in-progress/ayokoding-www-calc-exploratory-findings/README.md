# Exploratory Testing Findings — Cost-of-Living Calculator

**Plan type**: Exploratory testing backlog (tester output)

**Status**: In Progress — exploratory findings filed and **re-verified (2026-06-19 re-run)**; awaiting `plan-maker` to add `tech-docs.md` + TDD `delivery.md`

**Re-run note**: A second spec-aware exploratory pass on 2026-06-19 re-verified every prior finding
(EWT-001 – EWT-007 all **STILL-PRESENT** — see the re-verification table in `findings.md`) and added six
new defects (EWT-008 – EWT-013) plus two new spec gaps (SG-005, SG-006).

**Next steps**: `plan-maker` adds `tech-docs.md` and a TDD-shaped `delivery.md` with the
Specs/Gherkin coverage steps required by the Specs & Gherkin Completeness rule. `spec-gaps.md`
seeds `specs-maker` scenario work. `findings.md` seeds developer fixes.

---

## Target URL and Environment

- **URL tested**: `http://localhost:3101/en/tools/cost-of-living-calculator`
- **Secondary URL**: `http://localhost:3101/id/tools/cost-of-living-calculator`
- **App**: `ayokoding-www` (Next.js 16, TypeScript)
- **Dev server port**: 3101 (confirmed HTTP 200)
- **Test date**: 2026-06-19
- **Browser**: Chromium via Playwright
- **Dataset snapshot date**: 2026-06-18 (shown on page)
- **FX snapshot date**: 2026-06-17 (from `fx.ts`)

---

## Testing Goal

Thoroughly explore the cost-of-living-calculator tool page for defects across all
dimensions: functional correctness (calculator math/logic, inputs, outputs), behavioural
consistency, UI/UX, responsive layout (mobile/tablet/desktop breakpoints), accessibility
(WCAG AA, keyboard nav, ARIA, contrast), performance, and safe non-destructive
security/input-validation. Compare against existing `specs/**` Gherkin and flag spec gaps.

---

## Charters Run

| #   | Charter                                                      | Tours Applied       | Dimensions Covered       |
| --- | ------------------------------------------------------------ | ------------------- | ------------------------ |
| C1  | Explore baseline page load — structure, metadata, disclaimer | Landmark            | Structure, Function      |
| C2  | Explore cost-of-living table structure and cascading filters | Money, FedEx        | Function, Interface      |
| C3  | Explore deep-link routing (city, country, both params)       | Money               | Function, Operations     |
| C4  | Explore savings tab math and sorting                         | Intellectual        | Function, Data           |
| C5  | Explore min-role tab all three baseline modes                | Money, Intellectual | Function, Data           |
| C6  | Explore household controls and area switching                | FedEx, Intellectual | Function, Data           |
| C7  | Explore Indonesian locale (`/id/`)                           | Supermodel          | Localizability, Platform |
| C8  | Explore responsive behaviour at 375, 768, 1440 px            | Supermodel          | Platform, Compatibility  |
| C9  | Explore accessibility: keyboard nav, ARIA, skip link, labels | Supermodel          | Usability (a11y)         |
| C10 | Explore edge inputs: negative salary, zero, large, XSS-ish   | Antisocial          | Data, Security (passive) |
| C11 | Explore security headers (passive)                           | Back Alley          | Security (passive)       |

---

## Coverage Map

### Dimensions covered

| Dimension                                      | Status      | Notes                                                             |
| ---------------------------------------------- | ----------- | ----------------------------------------------------------------- |
| Functional flows (happy path)                  | Covered     | All three tabs, all filter modes                                  |
| Functional math verification                   | Covered     | Dubai, Jakarta, Bengaluru, Singapore independently verified       |
| Behavioural consistency (within page)          | Covered     | Desktop vs mobile inconsistency found (F4)                        |
| Behavioural consistency (cross-locale)         | Covered     | /id/ vs /en/ tested; divergences found (F3, F4, F5)               |
| Forms & validation                             | Covered     | Household controls, salary inputs, filter selects                 |
| Input edge cases                               | Covered     | Negative salary, zero, very large, XSS-ish in number fields       |
| Navigation & links                             | Covered     | City links, country links, back navigation                        |
| Responsive: 320, 375, 768, 1024, 1280, 1440 px | Covered     | Re-run extended to 320/1024/1280; 320 px overflow found (EWT-011) |
| Accessibility: WCAG 2.2 AA                     | Partial     | Automated + structural; no full contrast audit; keyboard tested   |
| Performance (Core Web Vitals)                  | Not covered | Dev server performance not representative of production           |
| Cross-browser                                  | Not covered | Chromium only; Firefox/Safari/Edge not exercised                  |
| Security headers (passive)                     | Covered     | Curl-checked response headers                                     |

### Responsive not covered

- All standard breakpoints (320, 375, 768, 1024, 1280, 1440 px) exercised across the two passes — no
  responsive breakpoint outstanding.

### Specs coverage — scenario-by-scenario result

All 38 Gherkin scenarios in
`specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`
were exercised. Results (updated after the 2026-06-19 re-run):

| Bucket                             | Count | IDs                                |
| ---------------------------------- | ----- | ---------------------------------- |
| Covered + passing                  | 30    | See passing list below             |
| Covered + diverging (spec defects) | 4     | EWT-001, EWT-002, EWT-008, EWT-013 |
| Spec gaps (uncovered by specs)     | 6     | SG-001 – SG-006                    |

The re-run reclassified two previously-passing scenarios as diverging: "Rural area lowers housing vs city
center" (Housing **column** shows the base rate while the total is discounted — EWT-008) and "Low-confidence
cells flagged" (flags appear on the min-role tab but not on the Cost-of-Living or Savings tabs — EWT-013).

**Passing scenarios** (30 — two former passes reclassified diverging below):

- Cost-of-living breakdown lists category expenses per city ✓
- Region narrows country filter, country narrows city filter ✓
- Clicking a city name opens city detail ✓
- Clicking a country opens cost-of-living filtered to that country (table filters correctly; geo UI
  state is the divergence — EWT-001) ✓/✗ partial
- City link takes precedence over country link ✓
- Healthcare funding scheme always shown ✓
- OOP abbreviation explained on screen ✓
- Relocation reserve shown separately from sunk costs ✓
- Savings tab: gross to net conversion ✓ (math verified)
- Gross monthly shows derived annual ✓ (8000 × 12 = 96,000 ✓)
- Non-salary comp informational only ✓
- Total comp shown for negotiation context ✓
- Sub-national tax lowers net in federal countries (indicator shown) ✓
- Net take-home lower than gross ✓
- Essentials above net show deficit (negative savings in red) ✓
- Savings tab sortable by savings ✓
- Minimum role tab: savings target baseline ✓
- Minimum role tab: qualifying divider and dimmed rows ✓
- No role reaches bar → message shown ✓
- Reference role and my-salary baseline modes ✓
- Dual currency display ✓
- Every money column dual currency ✓ (savings cell shows USD + display + local)
- Geographic filter scopes candidates ✓
- Non-salary comp doesn't change ranking ✓
- Roles labelled as software-engineering roles ✓
- Per-country salary distribution shown ✓
- Low-confidence cells flagged — partial ✗ (min-role tab only; absent on Cost-of-Living/Savings tabs →
  reclassified diverging, EWT-013)
- School type toggle hidden without school-age children ✓
- Private school raises expenses ✓
- Rural area lowers housing vs city center — partial ✗ (total discounts correctly but the Housing column
  shows the base rate → reclassified diverging, EWT-008)
- No Israeli cities ✓
- Data snapshot date shown ✓

**Diverging scenarios** (4): → EWT-001, EWT-002, EWT-008, EWT-013 in `findings.md`

---

## Risk Summary

**Overall impression**: The calculator's core math, functional flows, and tab navigation are
solid and well-tested. Most spec scenarios pass. The most impactful risks are in locale
handling: `html lang="en"` on Indonesian pages is a WCAG 3.1.1 violation that affects all
screen-reader users of the /id/ locale, and the desktop table hardcoding `.name.en` means
country and city names are always displayed in English on the Indonesian locale (contradicting
the spec requirement that all labels be in Indonesian).

**Top risks by severity** (13 findings open after the re-run; all 7 originals STILL-PRESENT):

1. **Major — Locale: html lang="en" on /id/ pages** (EWT-003): affects all screen readers and
   translation tools for Indonesian users. Confirmed present on production.
2. **Major — Locale: Desktop table city/country names hardcoded to English** (EWT-004):
   Indonesian users see English names on desktop but Indonesian names on mobile — direct
   internal inconsistency + spec divergence. (EWT-005 is the same defect on the min-role tab.)
3. **Major — URL deep link: geo filter dropdowns not pre-selected** (EWT-001 + EWT-002):
   country and city URL params correctly scope the visible data but leave the filter dropdowns
   showing "All regions / All countries / All cities" — violates spec and breaks the
   stated user experience of the deep-link feature.
4. **Major — Housing column ignores the area discount** (EWT-008): selecting "Rural" discounts the
   Essentials total but the Housing column still shows the city-center rate — the displayed number
   disagrees with the value used in the total, undermining trust in the calculator.
5. **Major — Confidence flags missing on Cost-of-Living and Savings tabs** (EWT-013): lower-confidence
   estimates are shown unqualified on the two largest tables; the spec's flagging contract is honoured
   only on the min-role tab.
6. **Minor — Sort button touch target** (EWT-006): 20 px height is below WCAG 2.5.8 24 × 24 px minimum.
7. **Minor — Negative salary accepted without validation** (EWT-007): `type="number"` with no `min="0"`
   yields semantically nonsensical negative output.
8. **Minor — IA/SEO** (EWT-009 generic `<title>`, EWT-012 calculator absent from `sitemap.xml`): hurt
   discoverability and link-sharing of the production tool page.
9. **Minor — Responsive/a11y** (EWT-010 footer links below 24 px at mobile, EWT-011 8 px nav overflow at
   320 px): small touch/reflow defects at the smallest viewports.

---

## Document Map

| File                    | Purpose                                                                            |
| ----------------------- | ---------------------------------------------------------------------------------- |
| `README.md` (this file) | Testing context, charters, coverage map, risk summary                              |
| `brd.md`                | Business framing: who is affected, cost of defects, success metrics                |
| `prd.md`                | Personas, user stories, Gherkin acceptance criteria for corrected behaviours       |
| `findings.md`           | Re-verification table + defect catalog (EWT-001 – EWT-013) with steps to reproduce |
| `spec-gaps.md`          | Unspecced correct behaviours observed on the live target (SG-001 – SG-006)         |

`tech-docs.md` and `delivery.md` are **not authored here** — they are produced when this plan
is promoted to `plans/in-progress/` via `plan-maker` (which adds TDD-shaped delivery steps and
the Specs/Gherkin completeness coverage steps).
