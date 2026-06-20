# Business Requirements Document

## Plan

Usability Evaluation: ayokoding-www Cost-of-Living / Salary Calculator
`plans/in-progress/ayokoding-www-calc-usability-findings/`

## Problem Statement

The ayokoding-www Salary Savings Calculator is a high-effort, data-rich tool designed to help
software professionals evaluate international relocation options. Its value proposition is
comparative clarity across cities and currencies. However, several usability friction points
reduce that value in practice:

1. A first-time visitor cannot tell from the page title, the URL slug, or the tab structure what
   the tool does in the first few seconds.
2. The Indonesian-locale page returns `html[lang]="en"`, meaning assistive technologies and
   browser auto-translate treat the page as English — breaking accessibility for Bahasa Indonesia
   readers.
3. Shared or bookmarked URLs (e.g. `?tab=cost&country=sg`) do not restore the visible filter state,
   so the URL appears to carry context that it silently discards.
4. The cost-of-living table uses unadorned numbers with no currency units — readers cannot tell
   whether 25,000 means Thai baht or another currency without scanning the surrounding context.
5. Two identically-labelled fields ("Total") in the summary card and the comparison table show
   different values for the same city, with no explanation. This erodes trust in the data.
6. Touch targets for the filter selects and the area toggle fall below the WCAG 2.5.8 minimum
   (24 × 24 CSS px) on mobile — primary workflows become hard to use on phones.
7. "Minimum role", "Baseline source", "Liquidity reserve", "Relocation (sunk)", "Net (monthly)",
   and "Savings after lifestyle" are presented as column headers or control labels with no
   definitions — forcing expert-only comprehension of the tool's most distinctive outputs.

## Who Is Affected

- **First-time international-relocation researchers** (primary users): confused by the H1/URL
  mismatch, unable to orient to the tab structure or trust the "Total" figures.
- **Mobile users** (secondary): disadvantaged by small touch targets on filter controls.
- **Bahasa Indonesia readers** (secondary): impaired by the incorrect `html[lang]` attribute,
  which breaks assistive technology and browser translation.
- **Users who share or bookmark calculator URLs**: the URL contains state that appears to filter
  but does not update the visible controls, so collaborators see the same default view the URL
  claims to have customised.

## Cost of Friction

- **Abandoned tasks**: a user who cannot decode "Baseline source" or "Net (monthly)" is unlikely
  to complete the Minimum Role or Savings workflows.
- **Trust erosion**: the "Total" mismatch between summary card (SGD 4,328) and comparison table
  (SGD 4,578) for the same city signals a possible calculation error — even when there is likely
  a principled difference. Eroded trust discourages return visits.
- **Accessibility liability**: `html[lang]` mismatch is a WCAG 2 Level AA failure (SC 3.1.1 —
  Language of Page), a testable criterion.
- **Lost shareability**: URLs that carry state parameters but do not restore the filter UI
  discourage social sharing of specific calculator views, reducing organic referral.
- **Mobile abandonment**: filter controls below touch-target minimums on a 375 px device mean
  users may miss-tap repeatedly, increasing frustration before they see any results.

## Business-Level Success Metrics

Success is defined as resolving all severity-3 and severity-4 findings and verifying the
clarified behaviour by a fresh naive walkthrough (a person who has not seen this evaluation) at
every breakpoint (320, 375, 768, 1280) and both locales (en, id):

1. A first-time visitor can state the tool's purpose within 10 seconds of landing without reading
   any help text — verified by a cognitive walkthrough (Q1 pass on every task).
2. The `html[lang]` attribute on the Indonesian locale matches `id` — verified by automated check.
3. Sharing a URL with `?tab=cost&country=sg` restores the visible Country filter to "Singapore" —
   verified by direct observation.
4. All table numbers include an unambiguous currency unit or the table provides a per-row
   currency column that is visible without horizontal scroll on mobile.
5. The two fields called "Total" either share the same definition or are given distinct, readable
   labels that tell a first-timer what each one covers.
6. All interactive controls meet WCAG 2.5.8 (24 × 24 CSS px minimum) on the 375 px viewport.
7. All severity-2 findings (unexplained jargon columns, missing "lifestyle" definition, etc.)
   are addressed with inline definitions or tooltips visible without interaction.
