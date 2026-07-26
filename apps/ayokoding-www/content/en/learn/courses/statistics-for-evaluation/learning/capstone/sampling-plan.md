---
title: "Artifact: Sampling Plan"
date: 2026-07-26T00:00:00+07:00
draft: false
weight: 2
---

> The written sampling plan Step 1 of the capstone produces -- committed BEFORE any data is drawn,
> then justified by `sample.py`'s own simulation. Exercises co-06, co-07, co-08.

**Context**: a team preparing to decide whether a candidate change should ship needs an overall
pass-rate estimate precise enough to support that decision, on a population that is not uniform --
most cases are "general" traffic, but two rarer edge-case types are also the harder, more
failure-prone ones. A plan written down before collecting data is a commitment a reviewer can check;
a plan improvised while looking at results is not a plan at all.

## The plan

**Target effect**: candidate-vs-baseline overall pass-rate estimate for the ship decision -- the
single number Step 3's paired comparison (`compare.py`) and Step 4's final report (`report.md`) both
depend on having estimated precisely enough first.

**Target precision**: a 95% interval half-width of **0.05** -- the team's own stated bar for "close
enough to defend a ship/no-ship call."

**Strata**: three, matching this eval's real population shape --

| Stratum                | Population share | True pass rate (unknown to the sampler, known only for this synthetic exercise) |
| ---------------------- | ---------------: | ------------------------------------------------------------------------------: |
| `general`              |          ≈ 91.7% |                                                                          ≈ 0.85 |
| `edge-case-formatting` |           ≈ 5.5% |                                                                          ≈ 0.60 |
| `edge-case-safety`     |           ≈ 2.8% |                                                                          ≈ 0.55 |

The two rare strata are also the two hardest strata -- exactly the pattern that makes rare-mode
sampling dangerous per Theme B's ex-17: a naive random sample at any feasible size would draw so few
`edge-case-safety` items that its own pass rate would be invisible in the noise.

**Allocation**: an unstratified reference calculation (`statsmodels`'
`samplesize_confint_proportion`, at the assumed overall rate 0.83 and half-width 0.05) gives
`n = 217` as an anchor. The actual plan allocates **more** than that anchor, and allocates it
**unevenly** -- `n = 200` from `general`, `n = 60` from each rare stratum (total `n = 320`) -- because
an unstratified n large enough for the overall estimate is not automatically large enough for either
rare stratum's own estimate to be visible; the rare strata each get a fixed minimum for visibility
regardless of their small population share.

## Justification

A plan's stated precision is a claim, not a fact, until it is checked. `sample.py` (Step 1 of the
capstone) simulates 500 independent draws at exactly this allocation and measures the achieved
spread of the reweighted estimate:

```text
Simulated achieved 95% half-width at this allocation: 0.0441
```

`0.0441` sits at or below the plan's own `0.05` target -- the allocation above is not a guess, it is
a plan whose own precision claim was checked by simulation before any real data collection began.

The same script also verifies the reweighting step itself, on a synthetic population with a KNOWN
true rate (available only because the population is synthetic, for this verification purpose):

```text
(Synthetic ground truth, for verification only) true overall population rate: 0.8250
Naive pooled estimate: 0.7250 | Reweighted estimate: 0.8087
Naive error: 0.1000 | Reweighted error: 0.0162
```

Naively pooling the three strata's samples together (ignoring that `general` is 92% of the real
population, not one-third of it) would have understated the true rate by 10 points. Reweighting by
each stratum's real population share cuts that error to under 2 points.

**Verify**: the achieved half-width (`0.0441`) is at or below the stated target (`0.05`), and the
reweighted estimate's error (`0.0162`) is smaller than the naive pooled estimate's error (`0.1000`)
against the known synthetic truth.

**Key takeaway**: a sampling plan earns the word "plan" only when its own precision claim is checked
by simulation before data collection, and a stratified sample only pays off when its estimate is
reweighted by real population share -- both steps this plan takes on purpose, not by accident.

**Why It Matters**: [`report.md`](./report.md) cites this plan's `n = 320` allocation as the basis
for every rate reported downstream -- a report that skipped this step would be reporting numbers
whose own precision was never checked.

---

← Previous: [Capstone](./overview.md) &middot; Next: [Report](./report.md) →
