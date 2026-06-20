# Business Requirements — AyoKoding Calculator Usability Findings

## Problem Statement

The AyoKoding cost-of-living calculator delivers genuine data value: 30+ cities, three distinct
comparison modes, localised Indonesian content. Yet the **Savings** and **Minimum role** tabs both
fail the cold-start test: a first-time user who clicks either tab without reading documentation
sees either all-negative savings or an unexplained ranked list with no guidance on what input to
provide. The resulting confusion drives abandonment before the user reaches the moment of insight
that makes the tool valuable.

Usability friction at a calculator's input step is not a cosmetic concern — it is a conversion
gate. A user who cannot form a correct mental model in the first 10 seconds will navigate away
rather than invest effort in understanding.

## Who Is Confused

- **Relocation-considering engineers** arriving via search or share links — they hit the Savings
  tab, see negative numbers, assume the tool is broken or irrelevant, and leave.
- **Non-English-speaking users in the Indonesian locale** — the tool is translated, but they
  encounter the same cold-start confusion: an all-negative table and three unlabelled mode radio
  buttons on the Minimum Role tab.
- **Mobile users at narrow viewports (320 px)** — hidden horizontal overflow with no scroll
  affordance may cause them to miss comparison data.
- **Any user who bookmarks or shares a specific tab** — tab state is absent from the URL, so
  shared links always drop the recipient on the Cost of living tab.

## Cost of Friction

- **Abandoned task rate**: users who cannot form a mental model at step 1 abandon rather than
  explore. The Savings tab's all-negative empty state is a plausible abandonment trigger for a
  significant fraction of first-time users.
- **Missed insight**: the calculator's core value proposition (salary-portability insight) lives
  in the Savings tab. UWT-001 gates access to that insight behind a confusing empty state.
- **Trust erosion**: an all-negative table on the Savings tab reads as a broken tool to a user
  who does not understand that zero-salary causes the negatives. Trust, once lost, is not recovered
  by adding a correct salary value — the user has already left.
- **Shareability reduction**: a link to `?tab=savings` does not exist (UWT-005). Users who want
  to share a specific comparison cannot do so, reducing organic distribution of the tool.
- **Non-USD user exclusion**: users with non-USD salaries cannot enter their salary in their own
  currency on the Savings tab (UWT-004), effectively excluding a substantial fraction of the
  tool's target audience (the tool covers Asia, Europe, and the Americas).

## Why Clarity Matters

AyoKoding is an educational platform for software engineers. The cost-of-living calculator is a
flagship interactive tool. Its quality shapes the visitor's perception of the site's overall
quality. Friction in a high-visibility tool erodes trust in the platform broadly — the opposite of
the desired outcome for an educational destination.

## Business-Level Success Metrics

- All severity-3 findings (UWT-001, UWT-002, UWT-003) resolved: the Savings tab empty state
  provides a clear call-to-action, the Minimum Role mode selector carries a visible group label,
  and tab sub-descriptions are surfaced as visible text.
- Re-evaluation by a naive walkthrough (a person who has not used the tool before) confirms all
  three tasks (salary savings estimation, minimum-role discovery, filter navigation) are completable
  without support at both desktop and mobile.
- URL state for tab selection is implemented (UWT-005) so shared links restore the active tab.
- Indonesian locale verified to be equivalent in clarity to English — no additional comprehension
  gaps beyond those already found in English.

## Non-Goals (Business Scope)

- Changing the underlying data model or calculation methodology.
- Expanding the city or role dataset.
- Full WCAG 2.2 accessibility audit (contrast, keyboard traps, ARIA wiring) — that is the domain
  of `web-exploratory-tester`.
- Design token fidelity or visual regression — that is the domain of `web-design-tester`.
