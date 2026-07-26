---
title: "Launch Criteria"
date: 2026-07-26T00:00:00+07:00
draft: false
weight: 6
---

> Meridian Claims Assistant -- scoping, ship, staged-rollout, and rollback criteria.

## Scoping decision

The assistant's original proposed scope -- recommend a decision on any claim type, including
total-loss and liability-dispute claims -- measured 81% pass rate against the eval set, too low for
adjuster trust. Narrowed scope: standard collision and weather-damage claims only, the two damage
types with the most consistent policy language and photo evidence; total-loss and liability-dispute
claims are routed to manual assessment with no assistant recommendation at all. Measured pass rate
on the narrowed scope: 96.1%. This scoping decision required no model change -- only removing the
claim types the model was measurably unreliable on.

## Ship criteria

- **Primary threshold**: pass rate ≥ 95% on the 150-case narrowed-scope eval set, with a 95%
  confidence interval no wider than ±3 percentage points.
- **Named acceptable worst case**: of the failing cases, none may be a confidently-wrong
  recommendation with no citation attached -- every failing case must at minimum have triggered the
  uncertainty badge (co-06) or carried a citation Dana could have used to catch it (co-09).
- **Measured result at this review**: pass rate 96.1%, interval ±2.4 points; zero failing cases
  lacked a citation or an uncertainty signal. Source: the narrowed-scope eval report, produced
  following [Evaluating AI Output -- Essentials](../../../evaluating-ai-output-essentials/overview.md)'s
  pass-rate method, with the confidence interval computed per this course's own ship-criteria
  practice (co-21).
- **Verdict**: criteria met, cleared for staged rollout.

## Staged rollout with guardrails

Ramp: 1 adjuster team (week 1) → 25% of adjuster teams (weeks 2-3) → 100% (week 4+), each stage
gated on the guardrails below staying within threshold for the full prior stage.

- Correction-affordance usage rate (measured against a 3% eval-set baseline) exceeding 9% of
  recommendations halts the ramp immediately.
- Any single week's confirmed-incorrect-payout count exceeding 1 halts the ramp immediately.
- Median review time exceeding 3 minutes for two consecutive weeks halts the ramp and triggers a
  redesign review.

None of these three metrics exist in the eval set -- they are observable only once real claim
traffic, with its unpredictable mix of damage types and policy edge cases, starts flowing through
the assistant.

## Rollback criteria (agreed before rollout begins)

- **Automatic rollback** (no discussion required): any guardrail above breaching its threshold for
  two consecutive weeks without a shipped fix; any single confirmed incident of an approved payout
  later found to be outside the policy's actual coverage.
- **Discussion-required rollback** (decision within one business day by the named on-call product
  lead): correction-affordance usage elevated but not yet breaching the hard threshold; adjuster-
  reported confusion trending upward without a specific confirmed incident.
- **Sign-off**: Product Lead, Claims Operations Lead, Compliance stakeholder -- all three signed
  before rollout began.

## Feedback routing into error analysis

Corrections captured by the correction affordance ([`review-and-recovery.md`](./review-and-recovery.md))
are aggregated weekly, grouped by failure mode ([`failure-catalogue.md`](./failure-catalogue.md)).
Any pattern recurring three or more times in a week is promoted to a permanent case in the
narrowed-scope eval dataset, closing the loop this dossier opened.

**Verify**: every ship criterion states a threshold, an interval, and a named acceptable worst case,
each checkable against the actual eval report (co-21); every guardrail metric names a real-time
threshold that would halt the ramp and could not have been measured pre-launch (co-22); rollback
criteria are agreed and signed before rollout, distinguishing automatic from discussion-required
conditions unambiguously (co-23); the scoping decision states both the narrowed task's measured rate
and the excluded capability (co-20).

**Key takeaway**: This dossier's launch decision rests on a measured distribution with a named worst
case, a rollout that watches for exactly the signals an eval set cannot produce, and rollback
criteria written by people with no stake yet in keeping the feature live -- the same three moves
Theme D taught on Nimbus, applied here to close the Meridian Claims Assistant's design dossier.
