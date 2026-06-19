# Exploratory Testing Findings — Cost-of-Living Calculator

**Plan type**: Exploratory testing backlog (tester output)

**Status**: In Progress — exploratory findings filed; awaiting `plan-maker` to add `tech-docs.md` + TDD `delivery.md`

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

| Dimension                              | Status      | Notes                                                           |
| -------------------------------------- | ----------- | --------------------------------------------------------------- |
| Functional flows (happy path)          | Covered     | All three tabs, all filter modes                                |
| Functional math verification           | Covered     | Dubai, Jakarta, Bengaluru, Singapore independently verified     |
| Behavioural consistency (within page)  | Covered     | Desktop vs mobile inconsistency found (F4)                      |
| Behavioural consistency (cross-locale) | Covered     | /id/ vs /en/ tested; divergences found (F3, F4, F5)             |
| Forms & validation                     | Covered     | Household controls, salary inputs, filter selects               |
| Input edge cases                       | Covered     | Negative salary, zero, very large, XSS-ish in number fields     |
| Navigation & links                     | Covered     | City links, country links, back navigation                      |
| Responsive: 375, 768, 1440 px          | Covered     | Screenshots captured in `local-temp/`                           |
| Accessibility: WCAG 2.2 AA             | Partial     | Automated + structural; no full contrast audit; keyboard tested |
| Performance (Core Web Vitals)          | Not covered | Dev server performance not representative of production         |
| Cross-browser                          | Not covered | Chromium only; Firefox/Safari/Edge not exercised                |
| Security headers (passive)             | Covered     | Curl-checked response headers                                   |

### Responsive not covered

- 320 px breakpoint not tested (smallest mobile)
- 1024 px and 1280 px intermediate breakpoints not tested

### Specs coverage — scenario-by-scenario result

All 38 Gherkin scenarios in
`specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`
were exercised. Results:

| Bucket                             | Count | IDs                    |
| ---------------------------------- | ----- | ---------------------- |
| Covered + passing                  | 32    | See passing list below |
| Covered + diverging (spec defects) | 2     | EWT-001, EWT-002       |
| Spec gaps (uncovered by specs)     | 4     | SG-001 – SG-004        |

**Passing scenarios** (32):

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
- Low-confidence cells flagged ✓ (flags appear for proxy/moderate data on min-role tab)
- School type toggle hidden without school-age children ✓
- Private school raises expenses ✓
- Rural area lowers housing vs city center ✓
- No Israeli cities ✓
- Data snapshot date shown ✓

**Diverging scenarios** (2): → EWT-001, EWT-002 in `findings.md`

---

## Risk Summary

**Overall impression**: The calculator's core math, functional flows, and tab navigation are
solid and well-tested. Most spec scenarios pass. The most impactful risks are in locale
handling: `html lang="en"` on Indonesian pages is a WCAG 3.1.1 violation that affects all
screen-reader users of the /id/ locale, and the desktop table hardcoding `.name.en` means
country and city names are always displayed in English on the Indonesian locale (contradicting
the spec requirement that all labels be in Indonesian).

**Top risks by severity**:

1. **Major — Locale: html lang="en" on /id/ pages** (EWT-003): affects all screen readers and
   translation tools for Indonesian users.
2. **Major — Locale: Desktop table city/country names hardcoded to English** (EWT-004):
   Indonesian users see English names on desktop but Indonesian names on mobile — direct
   internal inconsistency + spec divergence.
3. **Major — URL deep link: geo filter dropdowns not pre-selected** (EWT-001 + EWT-002):
   country and city URL params correctly scope the visible data but leave the filter dropdowns
   showing "All regions / All countries / All cities" — violates spec and breaks the
   stated user experience of the deep-link feature.
4. **Minor — Sort button touch target** (EWT-005): 20 px height is below WCAG 2.5.8
   recommended 24 × 24 px minimum — affects touch users.
5. **Minor — Negative salary accepted without validation** (EWT-006): type="number" with no
   `min="0"` allows negative input, producing semantically nonsensical output.

---

## Document Map

| File                    | Purpose                                                                      |
| ----------------------- | ---------------------------------------------------------------------------- |
| `README.md` (this file) | Testing context, charters, coverage map, risk summary                        |
| `brd.md`                | Business framing: who is affected, cost of defects, success metrics          |
| `prd.md`                | Personas, user stories, Gherkin acceptance criteria for corrected behaviours |
| `findings.md`           | Defect catalog (EWT-001 – EWT-006) with full steps to reproduce              |
| `spec-gaps.md`          | Unspecced correct behaviours observed on the live target (SG-001 – SG-004)   |

`tech-docs.md` and `delivery.md` are **not authored here** — they are produced when this plan
is promoted to `plans/in-progress/` via `plan-maker` (which adds TDD-shaped delivery steps and
the Specs/Gherkin completeness coverage steps).
