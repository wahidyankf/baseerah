---
title: "Failure Catalogue"
date: 2026-07-26T00:00:00+07:00
draft: false
weight: 2
---

> Meridian Claims Assistant -- failure catalogue. The assistant reads an incoming auto-insurance
> claim (photos, the policyholder's form, and the policy's coverage terms) and drafts a recommended
> decision -- approve, deny, or escalate -- with a written rationale, for a human adjuster to review
> before any decision is finalized.

## Six user-visible failure modes

| Failure mode                           | User-observable description                                                                   | Concrete example                                                                                                |
| -------------------------------------- | --------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| Confidently wrong                      | A fluent, specific recommendation that is factually incorrect, with no hedging language.      | Recommends "approve" citing a coverage clause that does not actually cover the damage type shown in the photos. |
| Subtly wrong                           | Mostly correct, with one small but consequential detail altered.                              | Correctly recommends "approve" but cites the wrong claim-limit figure in the rationale.                         |
| Refused                                | Declines to produce a recommendation at all.                                                  | "I can't assess claims involving total-loss vehicles." with no further guidance offered.                        |
| Truncated                              | Recommendation is cut off mid-rationale, appearing complete but omitting key reasoning.       | States "approve" but the listed supporting clauses stop at two of the four actually relevant.                   |
| Slow                                   | Correct eventually, but the multi-second wait changes what the adjuster does in the meantime. | 9-second generation with no progress indication, during which the adjuster starts a second claim.               |
| Inconsistent across identical requests | The same claim, re-submitted, returns a different recommendation.                             | Re-running the same claim's photos and form returns "escalate" instead of the original "approve."               |

## Silent-failure trace: confidently wrong, no citation

1. The assistant drafts: "Approve -- damage is consistent with a covered collision event under
   Section 3.1 (Collision Coverage)." The photos actually show weather damage, which the
   policyholder's plan explicitly excludes under Section 3.4.
2. The recommendation renders in the standard drafting panel -- same layout, same styling as every
   correct recommendation the adjuster has approved before.
3. The adjuster, Dana, reviewing her 34th claim of the day, reads the one-line rationale, sees no
   citation link to the actual policy clause, and approves it in line with the assistant's
   recommendation.
4. The claim pays out under a coverage type the policy does not actually provide for this damage.
5. Nothing in steps 1-4 contained a catch point: no uncertainty signal, no clickable citation to the
   actual clause, and no friction proportional to the fact that this action -- an approved payout --
   is effectively irreversible once funds are disbursed.

**Verify**: the trace reaches Dana's actual decision (approving the payout) and confirms, step by
step, that no point along the path -- drafting, rendering, or reviewing -- contained anything that
could have caught the error, satisfying co-19's silent-failure rule and co-02's requirement that
each failure mode be independently catalogued.

**Key takeaway**: The confidently-wrong, no-citation case is this feature's single most consequential
failure mode, because it is indistinguishable from a correct recommendation at every point in the
adjuster's actual workflow -- exactly the gap Step 2's interface design must close.
