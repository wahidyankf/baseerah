# Business Requirements Document

## Business Goal

Fix two confirmed defects in the AyoKoding cost-of-living calculator that produce incorrect or
misleading output for real users:

1. **Zero savings target shows no guidance** — A user who clears the savings target to 0 (or types
   0 explicitly) receives no feedback from the Minimum role tab. The table renders without a marker
   or divider, silently giving wrong information. Every role actually clears a zero bar, so the
   lowest role should be marked as the minimum. The current behaviour contradicts the Gherkin spec
   and misleads users who are exploring the "what does any savings require?" starting point.

2. **Browser tab title doubles the site name** — The page title reads
   "Cost of Living Calculator | AyoKoding | AyoKoding". This is a metadata authoring error that
   affects every user who lands on the calculator and every search-engine crawler that indexes it.
   A duplicated site name dilutes SEO title-tag value and looks unprofessional.

## Who Is Affected

- **All users of the Minimum role tab** who enter a zero savings target (EWT-001). This includes
  first-time users experimenting with the baseline before committing to a real target, and users who
  clear a previous target value.
- **All users and search engines** that load the calculator page (EWT-002). The duplicate title
  appears in the browser tab and in search-result snippets.

## Cost of Leaving These Defects Unfixed

- EWT-001: The zero-target state is a natural first interaction with the Minimum role tab. Leaving
  it broken means users see a table they cannot interpret — no marker, no divider — and may conclude
  the tool is broken or that no role qualifies, which is the opposite of the truth. It erodes
  confidence in the tool's correctness.
- EWT-002: Duplicate title tags reduce click-through rates from search results and may cause search
  engines to truncate the title unpredictably. The calculator is a flagship tool for the site's
  "software engineering career" positioning.

## Business-Level Success Metrics

1. EWT-001 resolved: entering a savings target of 0 USD causes the qualifying divider to appear and
   the lowest-ranked role to receive the minimum marker, with all roles shown above the divider.
2. EWT-002 resolved: the `<title>` of the calculator page reads
   `"Cost of Living Calculator | AyoKoding"` (once per locale, translated correctly for `/id/`).
3. No regression in any passing Gherkin scenario from the calculator spec.
