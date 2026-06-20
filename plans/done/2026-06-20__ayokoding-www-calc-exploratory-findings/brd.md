# Business Requirements — Cost-of-Living Calculator Defect Fix

**Source**: Exploratory testing session 2026-06-19
**Plan**: `plans/backlog/2026-06-19__ayokoding-www-calc-exploratory-findings/`

---

## Business Context

The cost-of-living calculator is a primary tool on ayokoding.com — a public educational platform for
software engineers and tech workers. The calculator lets users compare living costs and salary
adequacy across 31 cities to inform relocation, negotiation, and financial planning decisions.
Traffic comes from both English-speaking and Indonesian-speaking audiences; the site explicitly
ships an Indonesian locale (`/id/`) as a supported language.

---

## Who Is Affected

| Persona                    | Affected by               | Severity of impact                                                          |
| -------------------------- | ------------------------- | --------------------------------------------------------------------------- |
| Indonesian users (mobile)  | EWT-004 (desktop only)    | None at mobile; encounter English names on desktop                          |
| Indonesian users (any)     | EWT-003, EWT-004, EWT-005 | Screen readers mispronounce; desktop shows English; min-role always English |
| Screen reader users        | EWT-003                   | Screen reader announces wrong language for all `/id/` page content          |
| Users following city links | EWT-001, EWT-002          | Filter dropdowns show no active state; confusing UX                         |
| Touch/mobile users         | EWT-006                   | 20 px sort button below WCAG minimum touch target                           |
| All users (edge input)     | EWT-007                   | Negative salary accepted; output misleading                                 |

---

## Cost of Leaving Defects Unfixed

- **EWT-003 (html lang)**: WCAG 3.1.1 is Level A — the lowest threshold. Assistive technology
  (screen readers, Braille displays, translation services) relies on it to select the correct
  language model. Indonesian-locale users using screen readers hear content announced in the wrong
  language, impairing comprehension of every label, category, and number on the page.

- **EWT-004/005 (locale names)**: The site ships an Indonesian translation. If the desktop view
  systematically shows English city and country names while mobile shows Indonesian names, the
  translation is functionally incomplete for desktop users and creates an internal contradiction that
  erodes trust in the tool's correctness.

- **EWT-001/002 (geo filter sync)**: The deep-link feature (country and city links in the table) is
  a deliberate sharing mechanism — it enables users to link colleagues to a specific-city or
  specific-country view. When the filter dropdowns do not reflect the URL state, the feature appears
  broken to users who navigate to a linked URL and see "All regions / All countries / All cities"
  while the data is actually filtered.

- **EWT-006 (touch target)**: WCAG 2.5.8 (Level AA) — the sort button affects discoverability of
  the savings ranking feature on touch devices.

- **EWT-007 (negative salary)**: Minor UX issue; negative salary produces negative "annual gross"
  which is meaningless and could mislead a user who accidentally enters a negative value.

---

## Why Fixing Matters

The calculator's stated purpose is to help software engineers make accurate, data-driven relocation
and salary decisions. Defects in locale correctness (EWT-003/004/005) and URL state sync
(EWT-001/002) directly undermine that purpose and violate the platform's commitment to supporting
Indonesian-speaking users as a first-class audience.

Fixing EWT-003 also removes a WCAG Level A violation, which is a non-negotiable accessibility
baseline per the repo's Accessibility First principle.

---

## Business Success Metrics

Fix is complete when:

1. All Major findings (EWT-001 through EWT-004) are resolved and re-verified on the live
   dev server across both `/en/` and `/id/` locales at 375 px (mobile) and 1440 px (desktop).
2. No WCAG 3.1.1 violation on `/id/` pages (automated check: `html[lang]` attribute equals `"id"`).
3. Geo filter dropdowns match URL-param state on deep-link navigation for both `?country=` and
   `?city=` params.
4. City and country names on the `/id/` locale desktop table match the Indonesian translations in
   `translations.ts`.
5. Minor findings (EWT-005, EWT-006, EWT-007) resolved or formally deferred with a written rationale.
6. All related Gherkin scenarios in
   `specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`
   pass the `specs:coverage` Nx target after fixes are applied.
