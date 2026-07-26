---
title: "Interface Design"
date: 2026-07-26T00:00:00+07:00
draft: false
weight: 3
---

> Meridian Claims Assistant -- interface design. Every element below traces to a failure mode named
> in [`failure-catalogue.md`](./failure-catalogue.md).

## First-contact expectation setting

- **Rejected copy**: "Meridian's assistant makes accurate claim decisions instantly." (Implies
  certainty; contradicted the moment the first confidently-wrong recommendation occurs -- the exact
  failure this dossier's failure catalogue documents.)
- **Shipped copy**: "The assistant drafts a recommendation and cites the policy clauses it relied
  on -- always open the cited clause before you approve or deny." Names what the tool actually does
  (drafts, cites) and the specific adjuster behaviour (opening the citation) that survives a wrong
  draft without contradicting the copy itself.

**Traces to**: the confidently-wrong, no-citation failure mode -- this copy sets the expectation
that citations are load-bearing, not decorative, before the adjuster ever sees a recommendation.

## Selective uncertainty and provenance policy

**Policy**: an uncertainty badge fires on a recommendation when any of the following holds:

1. The recommendation's calibrated confidence bucket falls below the 92nd-percentile accuracy
   threshold measured against Meridian's held-out eval set.
2. The cited policy clause requires cross-referencing more than one section (a common source of
   the subtly-wrong failure mode).
3. The claim's damage-photo classification confidence is itself below its own threshold -- a
   second-order signal specific to this feature's multi-modal input.

**Provenance**: every recommendation's rationale is restructured from free text into per-clause
fields (`decision`, `primary_clause`, `supporting_clauses`, `exclusions_checked`), each linked
directly to the exact policy section it cites, highlighted -- not merely named. Verifying the
Section 3.1-versus-3.4 confusion from the failure catalogue's silent-failure trace now takes under
ten seconds: the adjuster opens the highlighted clause link and reads the actual exclusion language
directly.

**Accessibility**: the uncertainty badge carries a text label ("Verify before approving"), a
distinct triangle icon independent of colour, and a screen-reader-announced `aria-label`; its text
contrast measures 6.3:1, clearing the WCAG AA 4.5:1 minimum for normal text.

**Verify**: the uncertainty policy's three trigger conditions are each a concrete, measurable
property rather than a subjective judgment, satisfying co-06 and co-07; the provenance links open
directly to the cited clause, satisfying co-09 and co-10's practical-verifiability rule; the badge
survives a colour-blind and screen-reader reading, satisfying co-06's accessibility requirement.

**Key takeaway**: Restructuring the rationale into per-clause fields with direct citations is the
single change that converts the failure catalogue's most damaging failure mode (confidently wrong,
uncited) into a recommendation Dana can check in under ten seconds instead of accepting on faith.
