---
title: "Review and Recovery"
date: 2026-07-26T00:00:00+07:00
draft: false
weight: 4
---

> Meridian Claims Assistant -- review and recovery design.

## Review cost

Unaided, Dana assesses a claim (reading the policy, the photos, and the form, then deciding) in a
measured average of 5 minutes 50 seconds. With the assistant's structured, per-clause-cited draft
(Step 2), her review -- checking the cited clauses against the photos and confirming or editing the
decision -- averages 1 minute 40 seconds, roughly 3.5x faster. This clears the review-as-product bar:
the review step is measurably faster than the unaided task, not merely present alongside it.

## Vigilance-decrement mitigation

Dana reviews 30-40 claims per shift. Session traces show scrutiny visibly narrowing after
approximately the twelfth claim in a row. Mitigations shipped: batches capped at 10 claims before a
mandatory 5-minute break; every 8th claim in a batch requires an active per-clause confirmation
click rather than a single "approve" action; a random 1-in-12 already-approved claim is silently
re-shown later in the shift for a second, independent check. Each mitigation's cost is named and
accepted: batching adds context-switching overhead; forced confirmation adds friction to every
session; re-audit sampling adds review volume.

## Consequence-scaled friction table

| Action                                    | Reversibility                                                                  | Blast radius                               | Required confirmation                                                                                               |
| ----------------------------------------- | ------------------------------------------------------------------------------ | ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------- |
| View the assistant's draft recommendation | Fully reversible (no state changed)                                            | None.                                      | None.                                                                                                               |
| Edit the draft rationale before deciding  | Fully reversible (edit history retained)                                       | Low -- internal draft only.                | None; auto-saved.                                                                                                   |
| Escalate a claim to a senior adjuster     | Reversible (can be un-escalated)                                               | Low-medium -- reassigns internal work.     | Single click, no modal.                                                                                             |
| Deny a claim                              | Reversible before the policyholder is notified; effectively irreversible after | High -- affects the policyholder directly. | Preview-and-diff of the denial letter (below) plus explicit confirmation.                                           |
| Approve a payout                          | Irreversible once funds are disbursed                                          | Highest -- financial and contractual.      | Preview-and-diff of the payout amount and cited clauses, plus explicit confirmation; no default "approve" shortcut. |

## Preview and diff

Before Dana confirms an approval, the interface shows the exact payout amount, the exact cited
clauses (highlighted against the policy text), and the exact damage-photo regions the assistant
flagged as supporting evidence -- side by side with the claim's actual submitted values. This is
what would have caught the failure catalogue's Section 3.1-versus-3.4 error: the highlighted clause
text, shown directly beside the photo evidence, makes a collision-coverage citation visibly
inconsistent with weather-damage photos, at a glance, before the approval is confirmed.

## Undo

Editing the draft before a decision is fully reversible via edit history. Once a decision is
recorded but before the policyholder is notified (a fixed 2-hour internal review window), any
adjuster can fully reverse it -- decision, rationale, and any internal status change -- with no
partial state left behind. After the notification window closes, reversal requires a separate,
logged claims-correction process (outside this dossier's scope), which is why the friction table
above escalates confirmation specifically at the approve/deny actions rather than treating the
2-hour window as sufficient protection on its own.

## Correction affordance

An always-visible "This citation is wrong" link sits beside every cited clause. Clicking it opens a
short form capturing what the correct clause should have been; submitting immediately flags the
current recommendation for a second adjuster's review (recovering the current case) and logs the
original citation, the correction, and the claim's photo/form context for the weekly error-analysis
pass described in [`launch-criteria.md`](./launch-criteria.md).

**Verify**: the review step is measurably faster than the unaided task (co-11); the vigilance
mitigations each state their own cost (co-12); every friction-table row's confirmation level is
justified by its own reversibility and blast radius (co-13); the preview shows exact values, not a
summary (co-14); undo is complete within its stated window, with no partial state (co-15); the
correction affordance both recovers the current case and logs enough context for later analysis
(co-24).

**Key takeaway**: Every mechanism in this file traces to a Theme C pattern applied fresh to a new
domain -- friction concentrated on the approve/deny actions, complete undo within a real operational
window, and a correction affordance that closes the loop into ongoing error analysis.
