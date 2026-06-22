# Business Requirements — Calculator UX Hardening

## Problem

The ayokoding-www cost-of-living calculator is a flagship public tool. A three-lens live-site test pass
(spec-aware correctness, spec-blind usability, design fidelity) over both locales and six breakpoints
surfaced **26 findings**, including one Major functional/visual regression already shipped (all three tab
descriptions render at once), accessibility gaps below WCAG 2.5.8 touch-target size, jargon labels that
block first-time comprehension, and design-fidelity drift (native dropdown chrome, low-hierarchy
annotations). Left unfixed, these erode trust, accessibility, and comprehension on the tool that most
new visitors meet first.

## Goal

Resolve every confirmed finding so the calculator is **correct, accessible, comprehensible to a
first-time user, and faithful to its committed design** in both English and Indonesian, across mobile,
tablet, and desktop — without regressing the URL-as-single-source-of-truth, scroll-preservation, or
debounce behaviours that the same test pass confirmed are working.

## Scope

**In scope** (all confirmed findings — they are small, well-understood, and serve the hardening intent):

- Functional/visual: tab-description visibility regression (EWT-001/DWT-001); foreigner-school flag
  parity in city-detail (EWT-003); CSP/GA console error (EWT-006).
- Accessibility: 44px touch targets on tabs + segmented radios (EWT-002); `aria-sort` on the sort header
  (EWT-005); `aria-pressed` on the Area toggle (UWT-008); `aria-describedby`/`aria-disabled` on disabled
  school-type buttons (UWT-011).
- Comprehension / i18n: relabel "Baseline source" (UWT-001); gloss jargon headers — OOP (EWT-004/UWT-014),
  Relocation(sunk)/Liquidity reserve (UWT-009), P25/Median/P75 (UWT-010), ic/mgmt (UWT-013), Non-salary
  comp (UWT-003); translate/expand region names incl. MENA/Nordics (UWT-004); sentence-case healthcare
  scheme badges (UWT-012); clarify the foreigner-school flag wording (UWT-002).
- UX states: Savings-tab empty-state prominence / auto-focus (UWT-005); Min-role example-panel labelling
  (UWT-006); at-field salary currency on Savings (UWT-007).
- Design fidelity: styled select chrome on household + currency selects (DWT-002/003); baseline-source
  segmented wrap ≤375px (DWT-004); foreigner-flag visual hierarchy (DWT-006); salary-currency toggle
  bottom-alignment (DWT-007).
- Specs: fold the accepted USS-001…004 (usability) and SG-001…003 (design) proposals into the calculator
  Gherkin; add regression scenarios for every behavioural fix per the regression-test mandate.

**Out of scope** (recorded, not addressed this round): dark-mode runtime audit, cross-browser pass,
Lighthouse CWV, formal color-contrast audit, throttled-network loading states. These are logged in the
README coverage map as untested, to be picked up in a future round if prioritized.

## Success criteria

- All 26 findings resolved or explicitly deferred with rationale; no Major/sev-3 finding deferred.
- Every behavioural fix lands with a reproducing test (regression-test mandate) and, where it changes or
  specifies behaviour, a companion `specs/**` Gherkin scenario (feature-change-completeness).
- `nx run ayokoding-www:typecheck`, `:lint`, `:test:unit`, and `:specs:coverage` all green.
- Both locales and all six breakpoints visually signed off against the design before archival.
- The Rule-15 three-tester retest round runs after the fixes land and its findings are resolved.

## Stakeholders

- Site owner (calculator is a primary acquisition surface).
- First-time visitors in en and id locales (comprehension + accessibility).
- Maintainers (spec coverage prevents the shipped-regression class from recurring).

## Risks

- **Scope creep**: 26 confirmed findings could expand during execution (new surface discovered
  during fixes, regressions introduced by enlarging controls). Mitigation: treat each cluster as
  atomic; run the Rule-15 three-tester retest at Phase 10 to catch new findings before archival.
- **Regression from touch-target enlargement**: enlarging segmented controls to 44px may shift
  layout at tablet breakpoints. Mitigation: visual sign-off across all breakpoints in Phase 9
  before CI push.
- **CSP decision deferred to human (EWT-006)**: if the `[HUMAN]` decision at Phase 7.1 is not
  recorded, the GA console error persists. Mitigation: default is keep + whitelist (the tag
  already ships); executor proceeds with the default if no human response is received.
- **i18n correctness**: Indonesian translations added in Phase 4 are authored by the maintainer
  without a dedicated id-language reviewer. Mitigation: existing `healthcareOutOfPocket` key
  ("bayar sendiri") is already shipped and approved; new keys follow the same pattern.
