# Business Requirements — AyoKoding Cost-of-Living Calculator Design Findings

## Business Context

The cost-of-living calculator is a high-value, user-facing tool on `ayokoding.com` that helps
software engineers compare salaries and living costs across global cities. It is the primary
conversion driver for the site, exposing the brand to engineering-audience visitors who expect a
polished, professional product. Design drift — especially in dark mode, on mobile, and across the
Indonesian locale — undermines credibility directly.

## Who Is Affected

- **Visitors using Indonesian locale** — the tab overflow (DWT-001) and the baseline SegmentedControl
  overflow (DWT-007) produce a visually broken header bar on mobile devices at 375 px, which is the
  dominant viewport for mobile users in Indonesia.
- **Dark-mode users** — the active-tab colour regression (DWT-002) means the selected tab looks
  nearly invisible on a dark background; the user cannot confidently tell which tab is active.
- **All users on Savings and Min-role tabs** — the unstyled inputs (DWT-003, DWT-004) and
  inconsistently styled filters (DWT-005) present a visually unfinished surface, eroding trust in
  the tool's data quality.
- **Brand / design language** — the bespoke `<select>` for the baseline-source control (DWT-004)
  fragments the design language that the committed hi-fi mockups deliberately established with the
  SegmentedControl pattern.

## Cost of Leaving Unfixed

- A tab label that overflows the viewport is immediately visible to all mobile users in the
  Indonesian locale. It signals an unfinished product to a key growth audience.
- Dark-mode users (growing share of engineering-audience visitors) encounter an ambiguous tab state
  that forces them to guess which section they're viewing.
- The SegmentedControl / `<select>` inconsistency means the Min-role tab looks and behaves
  differently from the mockup the developer team reviewed and approved, degrading confidence in
  correctness of the overall design delivery.

## Business-Level Success Metrics

All of the following must hold after the findings in `findings.md` are resolved:

- No tab element extends past the viewport right edge at 375 px in either locale.
- The active tab carries a visible, on-token fill at all tested breakpoints in both light and dark
  mode.
- The gross-salary input on the Savings tab and the baseline-source control on the Min-role tab
  use styled primitives (or receive styled wrappers) consistent with the design system.
- All geo-filter selects carry consistent border and border-radius tokens.
- Zero DWT-severity-Major or higher findings remain unresolved or undeferred.
