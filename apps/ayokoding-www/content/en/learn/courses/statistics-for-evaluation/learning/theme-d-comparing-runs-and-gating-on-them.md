---
title: "Theme D: Comparing Runs and Gating on Them"
date: 2026-07-26T00:00:00+07:00
draft: false
weight: 40
---

Examples 35-46 build the case that "is B better than A" is a hypothesis test, not a comparison of
two printed numbers (co-17): a bare gap between two point estimates is itself noisy (Example 35),
and the actual test that answers the question is a two-proportion z-test (Example 36) or, when
baseline and candidate score the SAME items, a far more sensitive paired test (co-18, Examples
37-38). Statistical significance and practical importance are then shown to be two separate axes
(co-19, Examples 39-40), the bootstrap extends to any statistic with no closed form (co-20,
Examples 41-42), testing many criteria at once manufactures apparent wins unless corrected (co-21,
Examples 43-44), and an unchanged system's own repeat runs decompose into two distinct variance
sources (co-22, co-23, Example 45). The theme closes on one typed `ShipDecision` that gathers every
number this whole topic built into a single defensible verdict (co-24, Example 46). Every worked
example's real, runnable **Python 3.13** file lives under `learning/code/ex-NN-*/`, run for real
against the pinned statistical toolchain to capture genuine output.

---

### Worked Example 35: Two Point Estimates Are Not a Comparison

_ex-35 &middot; exercises co-17, co-01_

**Context**: **co-17 -- comparing two runs** opens by showing what a bare gap between two point
estimates actually is: one noisy draw. This example re-runs the identical comparison between two
fixed systems five times, at a typical small sample size, and shows the printed gap itself moves
around -- sometimes even pointing the wrong direction.

```python
# learning/code/ex-35-two-point-estimates-are-not-a-comparison/two_point_estimates.py
"""Worked Example 35: Two Point Estimates Are Not a Comparison."""  # => co-17: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-17: draws independent samples of two systems whose true rates are fixed but unobserved

TRUE_BASELINE_RATE = 0.75  # => co-17: baseline's real, unobservable pass rate
TRUE_CANDIDATE_RATE = 0.80  # => co-17: candidate's real, unobservable pass rate -- genuinely 5 points better
SAMPLE_SIZE = 30  # => co-17: a typical small eval-run size
TRIALS = 5  # => co-01: repeat the SAME comparison several times, to see how much the printed gap itself moves


if __name__ == "__main__":  # => co-17: entry point -- runs only when this file executes directly, not on import
    print(f"True rates (never observed directly): baseline={TRUE_BASELINE_RATE:.2f} candidate={TRUE_CANDIDATE_RATE:.2f} (candidate is genuinely better by 0.05)")  # => co-17: the ground truth this whole example is checking against
    gaps: list[float] = []  # => co-01: collects each trial's own observed gap, for the instability check below
    for trial in range(1, TRIALS + 1):  # => co-01: FIVE separate re-runs of the identical comparison, nothing else changes
        baseline_rng = random.Random(trial * 100 + 1)  # => co-17: this trial's own independent baseline sample
        candidate_rng = random.Random(trial * 100 + 2)  # => co-17: this trial's own independent candidate sample
        baseline_sample = [baseline_rng.random() < TRUE_BASELINE_RATE for _ in range(SAMPLE_SIZE)]  # => co-17: baseline's observed outcomes this trial
        candidate_sample = [candidate_rng.random() < TRUE_CANDIDATE_RATE for _ in range(SAMPLE_SIZE)]  # => co-17: candidate's observed outcomes this trial
        baseline_rate = sum(baseline_sample) / SAMPLE_SIZE  # => co-17: baseline's point estimate, this trial
        candidate_rate = sum(candidate_sample) / SAMPLE_SIZE  # => co-17: candidate's point estimate, this trial
        gap = candidate_rate - baseline_rate  # => co-01: the NAIVE "comparison" -- just subtracting two bare numbers
        gaps.append(gap)  # => co-01: stored for the spread check below
        print(f"trial {trial}: baseline={baseline_rate:.4f} candidate={candidate_rate:.4f} gap={gap:+.4f}")  # => co-01: the number a report that skips a formal test would ship

    wrong_direction_trials = sum(1 for g in gaps if g < 0)  # => co-01: how many trials showed candidate LOOKING worse, despite being genuinely better
    print(f"Trials where candidate LOOKED worse despite being genuinely better: {wrong_direction_trials} of {TRIALS}")  # => co-01
    assert wrong_direction_trials >= 1, "at least one small-sample trial must show the WRONG-direction gap, to make the instability concrete"  # => co-01: the claim this example demonstrates
    print("MATCH: the same two systems, compared the same way, produce a gap that sometimes points the WRONG direction at this sample size")  # => co-01
    # => co-01,co-17: 'candidate scored X points higher than baseline' is not yet a comparison -- it is one noisy draw from a distribution of possible gaps, and ex-36 builds the test that actually answers 'is B better than A'
```

**Run**: `python3 two_point_estimates.py`

**Output**:

```text
True rates (never observed directly): baseline=0.75 candidate=0.80 (candidate is genuinely better by 0.05)
trial 1: baseline=0.8000 candidate=0.9333 gap=+0.1333
trial 2: baseline=0.8667 candidate=0.7667 gap=-0.1000
trial 3: baseline=0.5667 candidate=0.7667 gap=+0.2000
trial 4: baseline=0.7000 candidate=0.8667 gap=+0.1667
trial 5: baseline=0.8000 candidate=0.8333 gap=+0.0333
Trials where candidate LOOKED worse despite being genuinely better: 1 of 5
MATCH: the same two systems, compared the same way, produce a gap that sometimes points the WRONG direction at this sample size
```

**Verify**: across 5 independent trials of the identical two systems, `1` trial shows candidate
scoring LOWER than baseline (a `-0.1000` gap) despite candidate's true rate being genuinely `0.05`
higher, satisfying co-01's rule that a bare point-estimate gap is one noisy draw, not yet a
comparison.

**Key takeaway**: "candidate scored higher than baseline" is a fact about one sample, not yet a
fact about the two systems -- the same comparison run again can, and sometimes does, disagree.

**Why It Matters**: this instability is exactly why eval reports that ship on "candidate beat
baseline by N points" without a test are gambling on which draw they happened to get -- ex-36
builds the formal test that actually answers the question this bare gap cannot.

---

### Worked Example 36: The Actual Test -- Two-Proportion Z-Test

_ex-36 &middot; exercises co-17_

**Context**: continuing co-17, this example runs the SAME two systems from ex-35 through
`statsmodels`' own two-proportion z-test, at both the small sample size ex-35 used and a properly
powered one, showing the formal test's own verdict changes with sample size even though the true
gap does not.

```python
# learning/code/ex-36-the-actual-test-two-proportion-z-test/two_proportion_z_test.py
"""Worked Example 36: The Actual Test -- Two-Proportion Z-Test."""  # => co-17: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-17: draws independent baseline and candidate samples, at two different sample sizes

from statsmodels.stats.proportion import proportions_ztest  # => co-17: the pinned library's own unpaired two-sample test for comparing two proportions

TRUE_BASELINE_RATE = 0.75  # => co-17: baseline's real, unobservable pass rate -- the SAME systems as ex-35
TRUE_CANDIDATE_RATE = 0.80  # => co-17: candidate's real, unobservable pass rate -- genuinely 5 points better


def sample_outcomes(true_rate: float, n: int, *, seed: int) -> list[bool]:  # => co-17: one independent sample of one system's outcomes
    """Draw n independent Bernoulli outcomes from true_rate."""  # => co-17: documents sample_outcomes's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-17: one fixed generator per sample
    return [rng.random() < true_rate for _ in range(n)]  # => co-17: one Bernoulli draw per case


if __name__ == "__main__":  # => co-17: entry point -- runs only when this file executes directly, not on import
    for n in (30, 300):  # => co-17: the SAME true systems, sampled at a typical small size and a properly powered size
        baseline = sample_outcomes(TRUE_BASELINE_RATE, n, seed=1)  # => co-17: baseline's own sample at this n
        candidate = sample_outcomes(TRUE_CANDIDATE_RATE, n, seed=2)  # => co-17: candidate's own sample at this n
        baseline_rate = sum(baseline) / n  # => co-17: baseline's point estimate at this n
        candidate_rate = sum(candidate) / n  # => co-17: candidate's point estimate at this n

        count = [sum(candidate), sum(baseline)]  # => co-17: successes for each group, in the order the test compares them
        nobs = [n, n]  # => co-17: each group's own sample size
        z_stat, p_value = proportions_ztest(count, nobs)  # => co-17: THE actual hypothesis test -- is this gap distinguishable from sampling noise
        print(f"n={n}: baseline={baseline_rate:.4f} candidate={candidate_rate:.4f} gap={candidate_rate - baseline_rate:+.4f} z={z_stat:.4f} p={p_value:.4f}")  # => co-17: the test's own verdict, not just the bare gap

        if n == 30:  # => co-17: the small-sample case -- the SAME sample size ex-35 showed as unstable
            assert p_value > 0.05, "at n=30, this test must fail to reach significance -- the sample is too small to distinguish this gap from noise"  # => co-17: the underpowered claim
        else:  # => co-17: the properly powered case
            assert p_value < 0.05, "at n=300, this test must reach significance -- the same true gap is now detectable"  # => co-17: the properly-powered claim

    print("MATCH: 'is B better than A' has a real answer -- a p-value from an actual test -- and that answer depends on whether the sample is large enough to detect the true gap")  # => co-17
    # => co-17: this is the test ex-35's bare gap was missing -- a formal hypothesis test, not a comparison of two printed numbers, is what actually answers 'is candidate better than baseline'
```

**Run**: `python3 two_proportion_z_test.py`

**Output**:

```text
n=30: baseline=0.7333 candidate=0.8000 gap=+0.0667 z=0.6105 p=0.5416
n=300: baseline=0.7367 candidate=0.8067 gap=+0.0700 z=2.0424 p=0.0411
MATCH: 'is B better than A' has a real answer -- a p-value from an actual test -- and that answer depends on whether the sample is large enough to detect the true gap
```

**Verify**: at `n=30`, `proportions_ztest()` returns `p=0.5416` (not significant), while at `n=300`
the identical `~0.07` true gap returns `p=0.0411` (significant), satisfying co-17's rule that "is B
better than A" resolves through a formal test whose own power depends on sample size.

**Key takeaway**: the two-proportion z-test is the actual answer to "is B better than A" -- a
p-value earned from a real comparison, not a bare difference between two point estimates.

**Why It Matters**: this is the SAME test underlying every "candidate beat baseline" claim in a
responsible eval report -- ex-37 and ex-38 next show that when baseline and candidate score the
SAME items, this unpaired test is leaving real sensitivity on the table.

---

### Worked Example 37: Paired Data Is More Sensitive

_ex-37 &middot; exercises co-18_

**Context**: **co-18 -- paired comparison** shows that when baseline and candidate score the SAME
items (not separate samples), a paired test can detect a difference the unpaired test from ex-36
misses entirely, because pairing removes item-to-item difficulty as a source of noise.

```python
# learning/code/ex-37-paired-data-is-more-sensitive/paired_is_more_sensitive.py
"""Worked Example 37: Paired Data Is More Sensitive."""  # => co-18: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-18: builds a paired dataset -- baseline and candidate scored on the SAME items

from statsmodels.stats.contingency_tables import mcnemar  # => co-18: the paired test -- uses only the discordant pairs
from statsmodels.stats.proportion import proportions_ztest  # => co-18: the SAME unpaired test ex-36 used, run here on paired data to make the contrast concrete


def build_paired_dataset(
    n: int, *, seed: int, baseline_rate: float, candidate_rate: float, correlation: float
) -> tuple[list[bool], list[bool]]:  # => co-18: SAME items, two verdicts each -- baseline and candidate are correlated because they share per-item difficulty
    """Build paired baseline/candidate outcomes over n SHARED items, correlated by per-item difficulty."""  # => co-18: documents build_paired_dataset's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-18: drives each item's shared difficulty draw
    baseline: list[bool] = []  # => co-18: baseline's verdict, one per item
    candidate: list[bool] = []  # => co-18: candidate's verdict on the SAME item, one per item
    for _ in range(n):  # => co-18: one shared item at a time
        difficulty_draw = rng.random()  # => co-18: this item's own shared difficulty draw -- read by BOTH systems below
        baseline_pass = difficulty_draw < baseline_rate  # => co-18: baseline's verdict on this exact item
        if rng.random() < correlation:  # => co-18: most of the time, candidate's verdict is driven by the SAME difficulty draw -- easy items stay easy for both
            candidate_pass = difficulty_draw < candidate_rate  # => co-18: correlated verdict -- shares baseline's own notion of "hard" vs "easy" for this item
        else:  # => co-18: occasionally, candidate's verdict is an independent draw instead -- real systems are correlated but not identical
            candidate_pass = rng.random() < candidate_rate  # => co-18: an independent verdict, uncorrelated with baseline's difficulty read
        baseline.append(baseline_pass)  # => co-18: records this item's baseline verdict
        candidate.append(candidate_pass)  # => co-18: records this item's candidate verdict, SAME item, SAME index
    return baseline, candidate  # => co-18: two same-length, index-aligned lists -- item i's baseline and candidate verdicts


if __name__ == "__main__":  # => co-18: entry point -- runs only when this file executes directly, not on import
    n = 50  # => co-18: fifty shared items -- reused unchanged in ex-38
    baseline, candidate = build_paired_dataset(n, seed=3, baseline_rate=0.70, candidate_rate=0.84, correlation=0.85)  # => co-18: the paired fixture this whole (ex-37, ex-38) pair reuses
    baseline_rate = sum(baseline) / n  # => co-18: baseline's own pass rate
    candidate_rate = sum(candidate) / n  # => co-18: candidate's own pass rate
    print(f"Baseline rate: {baseline_rate:.4f} | Candidate rate: {candidate_rate:.4f} | gap: {candidate_rate - baseline_rate:+.4f}")  # => co-18: the SAME kind of bare gap ex-35 warned about

    unpaired_count = [sum(candidate), sum(baseline)]  # => co-18: treats the two columns as if they were TWO INDEPENDENT samples -- discards the pairing entirely
    unpaired_nobs = [n, n]  # => co-18: same nominal sample size either way
    _z, unpaired_p = proportions_ztest(unpaired_count, unpaired_nobs)  # => co-18: the unpaired test, run on data that is secretly paired
    print(f"Unpaired two-proportion z-test (pairing discarded): p={unpaired_p:.4f}")  # => co-18: this test cannot see WHICH items changed, only the two marginal rates

    both_pass = sum(1 for b, c in zip(baseline, candidate) if b and c)  # => co-18: items both systems got right -- uninformative about which system is better
    both_fail = sum(1 for b, c in zip(baseline, candidate) if not b and not c)  # => co-18: items both systems got wrong -- also uninformative
    baseline_only = sum(1 for b, c in zip(baseline, candidate) if b and not c)  # => co-18: items where candidate REGRESSED relative to baseline
    candidate_only = sum(1 for b, c in zip(baseline, candidate) if not b and c)  # => co-18: items where candidate IMPROVED relative to baseline -- the informative cell
    table = [[both_pass, baseline_only], [candidate_only, both_fail]]  # => co-18: the 2x2 contingency table McNemar's test actually uses
    paired_result = mcnemar(table, exact=False, correction=True)  # => co-18: the PAIRED test -- uses only the discordant pairs (baseline_only, candidate_only)
    print(f"Paired McNemar test (same discordant items): p={paired_result.pvalue:.4f}")  # => co-18: this test sees EXACTLY which items flipped, and in which direction

    assert unpaired_p > 0.05, "the unpaired test, discarding the pairing, must fail to reach significance on this data"  # => co-18: the less-sensitive claim
    assert paired_result.pvalue < 0.05, "the paired test, using the SAME data's pairing, must reach significance"  # => co-18: the more-sensitive claim
    print(f"MATCH: the identical underlying data is NOT significant unpaired (p={unpaired_p:.4f}) but IS significant paired (p={paired_result.pvalue:.4f}) -- pairing is where the sensitivity comes from")  # => co-18
    # => co-18: throwing away which items changed and keeping only the two marginal rates is throwing away exactly the information a paired test needs -- ex-38 builds that paired test from its own definition
```

**Run**: `python3 paired_is_more_sensitive.py`

**Output**:

```text
Baseline rate: 0.6600 | Candidate rate: 0.8000 | gap: +0.1400
Unpaired two-proportion z-test (pairing discarded): p=0.1149
Paired McNemar test (same discordant items): p=0.0455
MATCH: the identical underlying data is NOT significant unpaired (p=0.1149) but IS significant paired (p=0.0455) -- pairing is where the sensitivity comes from
```

**Verify**: on the identical 50-item paired dataset, the unpaired z-test returns `p=0.1149` (not
significant) while the paired McNemar test returns `p=0.0455` (significant), satisfying co-18's
rule that a paired test is far more sensitive than treating the same paired data as independent
samples.

**Key takeaway**: discarding which items flipped and keeping only the two marginal rates throws
away exactly the information a paired comparison needs -- when items are shared, use a paired test.

**Why It Matters**: most real eval comparisons DO score baseline and candidate on the same eval
set, making this the common case, not the exception -- treating that data as if it were two
independent samples routinely under-detects real improvements.

---

### Worked Example 38: McNemar's Test, From Definition

_ex-38 &middot; exercises co-18_

**Context**: continuing co-18, this example derives ex-37's McNemar statistic from its own textbook
formula -- using only the two discordant counts -- and verifies it against both the pinned
library and an independent exact test.

```python
# learning/code/ex-38-mcnemars-test-from-definition/mcnemar_from_definition.py
"""Worked Example 38: McNemar's Test, From Definition."""  # => co-18: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import math  # => co-18: isclose -- verifies the from-definition and library statistics agree
import random  # => co-18: rebuilds the SAME paired dataset ex-37 introduced

from scipy.stats import binomtest, chi2  # => co-18: an EXACT alternative test, for cross-checking the chi2 approximation at this small discordant-pair count
from statsmodels.stats.contingency_tables import mcnemar  # => co-18: the pinned library's own paired test


def build_paired_dataset(n: int, *, seed: int, baseline_rate: float, candidate_rate: float, correlation: float) -> tuple[list[bool], list[bool]]:  # => co-18: the identical fixture-building function from ex-37
    """Build paired baseline/candidate outcomes over n SHARED items, correlated by per-item difficulty."""  # => co-18: documents build_paired_dataset's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-18: drives each item's shared difficulty draw
    baseline: list[bool] = []  # => co-18: baseline's verdict, one per item
    candidate: list[bool] = []  # => co-18: candidate's verdict on the SAME item, one per item
    for _ in range(n):  # => co-18: one shared item at a time
        difficulty_draw = rng.random()  # => co-18: this item's own shared difficulty draw -- read by BOTH systems below
        baseline_pass = difficulty_draw < baseline_rate  # => co-18: baseline's verdict on this exact item
        if rng.random() < correlation:  # => co-18: most of the time, candidate's verdict is driven by the SAME difficulty draw
            candidate_pass = difficulty_draw < candidate_rate  # => co-18: correlated verdict
        else:  # => co-18: occasionally, candidate's verdict is an independent draw instead
            candidate_pass = rng.random() < candidate_rate  # => co-18: an independent verdict
        baseline.append(baseline_pass)  # => co-18: records this item's baseline verdict
        candidate.append(candidate_pass)  # => co-18: records this item's candidate verdict, SAME item, SAME index
    return baseline, candidate  # => co-18: two same-length, index-aligned lists


def mcnemar_from_definition(baseline_only: int, candidate_only: int) -> tuple[float, float]:  # => co-18: the textbook chi2-corrected McNemar formula, in code
    """Return (statistic, p_value) for the continuity-corrected McNemar chi2 test on the two discordant counts."""  # => co-18: documents mcnemar_from_definition's contract -- no runtime output, just sets its __doc__
    statistic = (abs(baseline_only - candidate_only) - 1) ** 2 / (baseline_only + candidate_only)  # => co-18: Yates' continuity correction -- the "-1" before squaring
    p_value = 1 - chi2.cdf(statistic, df=1)  # => co-18: the chi-squared survival function at 1 degree of freedom -- ONLY the two discordant cells ever enter this formula
    return statistic, p_value  # => co-18: returns this computed value to the caller


if __name__ == "__main__":  # => co-18: entry point -- runs only when this file executes directly, not on import
    n = 50  # => co-18: the SAME fifty shared items as ex-37
    baseline, candidate = build_paired_dataset(n, seed=3, baseline_rate=0.70, candidate_rate=0.84, correlation=0.85)  # => co-18: reproduces ex-37's exact fixture

    both_pass = sum(1 for b, c in zip(baseline, candidate) if b and c)  # => co-18: concordant -- both right, uninformative for this test
    both_fail = sum(1 for b, c in zip(baseline, candidate) if not b and not c)  # => co-18: concordant -- both wrong, uninformative for this test
    baseline_only = sum(1 for b, c in zip(baseline, candidate) if b and not c)  # => co-18: discordant -- candidate REGRESSED on this item
    candidate_only = sum(1 for b, c in zip(baseline, candidate) if not b and c)  # => co-18: discordant -- candidate IMPROVED on this item
    print(f"Discordant pairs: baseline-only (regressions)={baseline_only} | candidate-only (improvements)={candidate_only}")  # => co-18: the ONLY two numbers McNemar's test actually uses

    stat_def, p_def = mcnemar_from_definition(baseline_only, candidate_only)  # => co-18: computed from the formula directly
    print(f"McNemar (from definition): statistic={stat_def:.4f} p={p_def:.4f}")  # => co-18: the chi2-corrected coefficient this file derives itself

    table = [[both_pass, baseline_only], [candidate_only, both_fail]]  # => co-18: the full 2x2 table the library's own function expects
    lib_result = mcnemar(table, exact=False, correction=True)  # => co-18: the SAME computation, called from the pinned library
    print(f"McNemar (statsmodels library): statistic={lib_result.statistic:.4f} p={lib_result.pvalue:.4f}")  # => co-18: the library's answer to the identical question
    assert math.isclose(stat_def, lib_result.statistic, abs_tol=1e-9), "the from-definition and library statistics must agree"  # => co-18
    assert math.isclose(p_def, lib_result.pvalue, abs_tol=1e-9), "the from-definition and library p-values must agree"  # => co-18

    exact = binomtest(min(baseline_only, candidate_only), n=baseline_only + candidate_only, p=0.5, alternative="two-sided")  # => co-18: cross-checks the chi2 approximation with an EXACT test -- appropriate given only 9 discordant pairs
    print(f"McNemar (exact binomial cross-check): p={exact.pvalue:.4f}")  # => co-18: a second, independent confirmation this is genuinely significant

    assert p_def < 0.05, "McNemar's test must reach significance on this paired data"  # => co-18: the claim this example demonstrates
    print(f"MATCH: the from-definition statistic ({stat_def:.4f}), the library's statistic, and an independent exact test all agree this paired difference is significant")  # => co-18
    # => co-18: McNemar's test is nothing more than 'compare the two discordant counts to each other' -- everything about the items where both systems agreed is thrown away on purpose, because it carries no information about which system is better
```

**Run**: `python3 mcnemar_from_definition.py`

**Output**:

```text
Discordant pairs: baseline-only (regressions)=1 | candidate-only (improvements)=8
McNemar (from definition): statistic=4.0000 p=0.0455
McNemar (statsmodels library): statistic=4.0000 p=0.0455
McNemar (exact binomial cross-check): p=0.0391
MATCH: the from-definition statistic (4.0000), the library's statistic, and an independent exact test all agree this paired difference is significant
```

**Verify**: `mcnemar_from_definition()` and the statsmodels library both return `statistic=4.0000`
and `p=0.0455`, with an independent exact binomial cross-check returning `p=0.0391` (also
significant), satisfying co-18's rule that McNemar's test is fully derivable from just the two
discordant counts.

**Key takeaway**: McNemar's test uses only the discordant pairs -- every item both systems agreed
on, right or wrong, carries zero information about which system is better.

**Why It Matters**: knowing McNemar's test is "just the two discordant counts" makes it easy to
sanity-check any paired-comparison library call by hand -- if the discordant counts printed in a
report do not match the p-value's own direction, something upstream is broken.

---

### Worked Example 39: Statistically Significant, Practically Irrelevant

_ex-39 &middot; exercises co-19_

**Context**: **co-19 -- significance vs. practical importance** shows the two are separate axes: at
a large enough sample size, even a genuinely tiny true gap becomes statistically detectable, while
staying below any reasonable materiality bar.

```python
# learning/code/ex-39-statistically-significant-practically-irrelevant/statistically_significant_practically_irrelevant.py
"""Worked Example 39: Statistically Significant, Practically Irrelevant."""  # => co-19: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-19: draws two large independent samples of two near-identical systems

from statsmodels.stats.proportion import proportions_ztest  # => co-19: the SAME two-proportion test ex-36 introduced

TRUE_A_RATE = 0.850  # => co-19: system A's real pass rate
TRUE_B_RATE = 0.862  # => co-19: system B's real pass rate -- only 1.2 points higher, genuinely
N = 8000  # => co-19: a LARGE eval run -- big enough to detect even a small true gap
MATERIALITY_THRESHOLD = 0.03  # => co-19: this team's own stated bar for "worth acting on" -- a 3-point gap or larger


def sample_outcomes(true_rate: float, n: int, *, seed: int) -> list[bool]:  # => co-19: one independent sample of one system's outcomes
    """Draw n independent Bernoulli outcomes from true_rate."""  # => co-19: documents sample_outcomes's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-19: one fixed generator per sample
    return [rng.random() < true_rate for _ in range(n)]  # => co-19: one Bernoulli draw per case


if __name__ == "__main__":  # => co-19: entry point -- runs only when this file executes directly, not on import
    a = sample_outcomes(TRUE_A_RATE, N, seed=31)  # => co-19: system A's own large sample
    b = sample_outcomes(TRUE_B_RATE, N, seed=32)  # => co-19: system B's own large sample
    a_rate = sum(a) / N  # => co-19: A's point estimate
    b_rate = sum(b) / N  # => co-19: B's point estimate
    gap = b_rate - a_rate  # => co-19: the observed gap
    print(f"N={N}: A={a_rate:.4f} B={b_rate:.4f} gap={gap:+.4f}")  # => co-19: a genuinely small gap

    count = [sum(b), sum(a)]  # => co-19: successes for each group
    nobs = [N, N]  # => co-19: each group's sample size
    z_stat, p_value = proportions_ztest(count, nobs)  # => co-19: the SAME formal test ex-36 built
    print(f"Two-proportion z-test: z={z_stat:.4f} p={p_value:.4f}")  # => co-19: the test's own verdict

    assert p_value < 0.05, "at this large n, the test must reach statistical significance despite the small true gap"  # => co-19: the "statistically significant" half of the claim
    assert abs(gap) < MATERIALITY_THRESHOLD, "the observed gap must fall BELOW this team's own materiality threshold, despite being statistically significant"  # => co-19: the "practically irrelevant" half of the claim
    print(f"MATCH: p={p_value:.4f} < 0.05 (statistically significant) AND gap={gap:+.4f} is below the {MATERIALITY_THRESHOLD:.2f} materiality threshold (practically irrelevant)")  # => co-19
    # => co-19: a large enough sample can make even a genuinely trivial difference statistically detectable -- 'significant' answers 'is there a real difference,' never 'is this difference worth acting on'
```

**Run**: `python3 statistically_significant_practically_irrelevant.py`

**Output**:

```text
N=8000: A=0.8474 B=0.8650 gap=+0.0176
Two-proportion z-test: z=3.1767 p=0.0015
MATCH: p=0.0015 < 0.05 (statistically significant) AND gap=+0.0176 is below the 0.03 materiality threshold (practically irrelevant)
```

**Verify**: at `N=8000`, the observed gap (`+0.0176`) returns `p=0.0015` (clearly significant)
while sitting below this team's own `0.03` materiality threshold, satisfying co-19's rule that
statistical significance and practical importance are two separate axes.

**Key takeaway**: "significant" answers "is there a real difference," never "is this difference
worth acting on" -- those two questions need two separate, explicitly stated bars.

**Why It Matters**: at scale, nearly every comparison eventually becomes statistically significant
if the sample grows large enough -- a stated materiality threshold, decided BEFORE looking at the
result, is what keeps a report from chasing statistically real but practically meaningless wins.

---

### Worked Example 40: Not Yet Significant, but Practically Important

_ex-40 &middot; exercises co-19, co-06_

**Context**: continuing co-19, this example shows the reverse failure: a genuinely large, practically
important 8-point gap is invisible at a typical small sample size, and **co-06**'s own power
calculation gives the exact n needed to detect it.

```python
# learning/code/ex-40-not-yet-significant-but-practically-important/not_yet_significant.py
"""Worked Example 40: Not Yet Significant, but Practically Important."""  # => co-19: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import math  # => co-06: ceil -- a sample size must be a whole number of cases
import random  # => co-19: draws samples of two systems with a genuinely large true gap

from statsmodels.stats.power import NormalIndPower  # => co-06: the pinned library's own power-based sample-size solver
from statsmodels.stats.proportion import proportion_effectsize, proportions_ztest  # => co-06: Cohen's h -- the effect-size input the power solver needs; co-19: the SAME test ex-36/ex-39 used

TRUE_A_RATE = 0.70  # => co-19: system A's real pass rate
TRUE_B_RATE = 0.78  # => co-19: system B's real pass rate -- a genuinely large, practically important 8-point gap
SMALL_N = 25  # => co-19: a typical small eval run -- the size a team might reach for first


def sample_outcomes(true_rate: float, n: int, *, seed: int) -> list[bool]:  # => co-19: one independent sample of one system's outcomes
    """Draw n independent Bernoulli outcomes from true_rate."""  # => co-19: documents sample_outcomes's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-19: one fixed generator per sample
    return [rng.random() < true_rate for _ in range(n)]  # => co-19: one Bernoulli draw per case


if __name__ == "__main__":  # => co-06: entry point -- runs only when this file executes directly, not on import
    a_small = sample_outcomes(TRUE_A_RATE, SMALL_N, seed=11)  # => co-19: A's sample at the small, typical size
    b_small = sample_outcomes(TRUE_B_RATE, SMALL_N, seed=12)  # => co-19: B's sample at the small, typical size
    count_small = [sum(b_small), sum(a_small)]  # => co-19: successes for each group, small sample
    _z_small, p_small = proportions_ztest(count_small, [SMALL_N, SMALL_N])  # => co-19: the formal test, applied at the small sample size
    print(f"n={SMALL_N}: A={sum(a_small) / SMALL_N:.4f} B={sum(b_small) / SMALL_N:.4f} p={p_small:.4f}")  # => co-19: looks unremarkable -- NOT significant

    effect_size = proportion_effectsize(TRUE_B_RATE, TRUE_A_RATE)  # => co-06: Cohen's h -- how large this gap is in the units the power solver needs
    power_analysis = NormalIndPower()  # => co-06: the pinned library's own two-sample power calculator
    required_n_raw = power_analysis.solve_power(effect_size=effect_size, alpha=0.05, power=0.80, ratio=1.0, alternative="two-sided")  # => co-06: how many cases PER GROUP are needed to detect this exact gap 80% of the time
    required_n = math.ceil(required_n_raw)  # => co-06: round UP -- a fractional case cannot be collected
    print(f"Effect size (Cohen's h): {effect_size:.4f} | Required n per group for 80% power: {required_n}")  # => co-06: the computed, justified target -- not a guess

    a_big = sample_outcomes(TRUE_A_RATE, required_n, seed=11)  # => co-19: A's sample at the COMPUTED required size, same underlying true rate
    b_big = sample_outcomes(TRUE_B_RATE, required_n, seed=12)  # => co-19: B's sample at the COMPUTED required size, same underlying true rate
    count_big = [sum(b_big), sum(a_big)]  # => co-19: successes for each group, at the required sample size
    _z_big, p_big = proportions_ztest(count_big, [required_n, required_n])  # => co-19: the SAME test, now properly powered
    print(f"n={required_n}: A={sum(a_big) / required_n:.4f} B={sum(b_big) / required_n:.4f} p={p_big:.4f}")  # => co-19: the SAME real gap, now clearly detected

    assert p_small > 0.05, "at n=25, this genuinely large gap must still fail to reach significance"  # => co-19: the "not yet significant" half of the claim
    assert p_big < 0.05, "at the computed required n, the SAME true gap must reach significance"  # => co-19: the "practically important, and now detected" half of the claim
    print(f"MATCH: the identical 8-point true gap is invisible at n={SMALL_N} (p={p_small:.4f}) but clearly detected at the computed n={required_n} (p={p_big:.4f})")  # => co-19
    # => co-06,co-19: 'not significant' at a small sample does not mean 'not real' -- it can mean the sample was never large enough to see a gap that a power calculation would have said needed a specific, computable n
```

**Run**: `python3 not_yet_significant.py`

**Output**:

```text
n=25: A=0.8000 B=0.8400 p=0.7128
Effect size (Cohen's h): 0.1829 | Required n per group for 80% power: 470
n=470: A=0.6894 B=0.7936 p=0.0003
MATCH: the identical 8-point true gap is invisible at n=25 (p=0.7128) but clearly detected at the computed n=470 (p=0.0003)
```

**Verify**: the identical `0.08` true gap returns `p=0.7128` at `n=25` but `p=0.0003` at the
power-computed `n=470`, satisfying co-19's and co-06's rule that a real, practically important
effect can stay statistically invisible until the sample reaches its own computed required size.

**Key takeaway**: "not significant" at a small sample is not evidence "not real" -- it can simply
mean the sample was never large enough, and a power calculation says exactly how large is enough.

**Why It Matters**: this pairs directly with ex-39's opposite failure -- together they show
significance and materiality are two separate, both-necessary checks, and co-06's power
calculation is what turns "we need more data" from a vague feeling into a specific, justified n.

---

### Worked Example 41: Bootstrap for a Statistic With No Closed Form

_ex-41 &middot; exercises co-20_

**Context**: **co-20 -- bootstrap resampling** extends beyond agreement coefficients (ex-30) to any
named statistic with no textbook standard-error formula. This example puts a bootstrap interval
around a median latency, and checks the interval has already stabilized at a modest resample count.

```python
# learning/code/ex-41-bootstrap-for-a-statistic-with-no-closed-form/bootstrap_median_latency.py
"""Worked Example 41: Bootstrap for a Statistic With No Closed Form."""  # => co-20: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-20: builds a right-skewed latency sample -- a realistic shape for real request timings

import numpy as np  # => co-20: scipy's bootstrap operates on numpy arrays
from scipy.stats import bootstrap  # => co-20: no closed-form standard error exists for a median -- bootstrap resampling instead


def build_latency_sample(n: int, *, seed: int) -> list[float]:  # => co-20: a right-skewed distribution of per-request latencies, in milliseconds
    """Build n simulated request latencies (ms), log-normally distributed -- a realistic right-skewed shape."""  # => co-20: documents build_latency_sample's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-20: one fixed generator -- reproducible latency draws
    return [round(rng.lognormvariate(mu=6.0, sigma=0.5)) for _ in range(n)]  # => co-20: most requests are fast, a long tail is slow -- exactly the shape a median (not a mean) is meant to summarize


def median_statistic(x: np.ndarray, axis: int = -1) -> np.ndarray | float:  # => co-20: the statistic scipy's bootstrap resamples -- unlike a proportion or a mean, a median has no textbook standard-error formula
    """Compute the median of x, vectorized over resample rows when axis is given."""  # => co-20: documents median_statistic's contract -- no runtime output, just sets its __doc__
    return np.median(x, axis=axis)  # => co-20: numpy's own median -- works identically whether x is one sample or a whole batch of resamples


if __name__ == "__main__":  # => co-20: entry point -- runs only when this file executes directly, not on import
    sample = build_latency_sample(50, seed=42)  # => co-20: fifty simulated latencies -- a realistic eval-run size for a latency check
    observed_median = float(np.median(sample))  # => co-20: the point estimate -- the ONLY number a bare report would show
    print(f"n=50 observed median latency: {observed_median:.1f}ms")  # => co-20: no formula exists to hand-derive this number's own standard error

    arr = np.array(sample, dtype=float)  # => co-20: scipy's bootstrap resamples this array with replacement
    for n_resamples in (1000, 5000):  # => co-20: checks whether the interval has already STABILIZED at a modest resample count
        result = bootstrap(
            (arr,), median_statistic, vectorized=True, confidence_level=0.95, n_resamples=n_resamples, method="percentile", rng=np.random.default_rng(42)
        )  # => co-20: resamples the 50 latencies with replacement, recomputes the median each time
        low, high = result.confidence_interval  # => co-20: unpacks the interval's two ends
        print(f"n_resamples={n_resamples}: bootstrap 95% CI on median = [{low:.1f}, {high:.1f}]ms")  # => co-20: the honest report -- a range around the median, not a bare point

    assert low < observed_median < high, "the observed median must sit inside its own bootstrap interval"  # => co-20: a basic sanity check on the interval itself
    print("MATCH: the bootstrap interval on the median stays essentially stable between 1000 and 5000 resamples -- 1000 was already enough to trust")  # => co-20
    # => co-20: any named statistic with no closed-form formula for its own uncertainty -- a median, a trimmed mean, a custom aggregate metric -- can still get an honest interval via resampling; the statistic itself never has to change
```

**Run**: `python3 bootstrap_median_latency.py`

**Output**:

```text
n=50 observed median latency: 441.5ms
n_resamples=1000: bootstrap 95% CI on median = [334.0, 524.0]ms
n_resamples=5000: bootstrap 95% CI on median = [332.0, 529.5]ms
MATCH: the bootstrap interval on the median stays essentially stable between 1000 and 5000 resamples -- 1000 was already enough to trust
```

**Verify**: at `n_resamples=1000`, the bootstrap interval is `[334.0, 524.0]`, and at `5000` it is
`[332.0, 529.5]` -- essentially unchanged -- satisfying co-20's rule that a modest resample count
is enough to trust the interval once it has stabilized.

**Key takeaway**: bootstrap resampling puts an honest interval around any statistic, whether or not
a closed-form formula exists for its uncertainty -- the median needed no special-case math.

**Why It Matters**: real eval metrics are rarely limited to proportions and kappas -- latency
percentiles, trimmed means, and custom composite scores all lack closed-form standard errors, and
the bootstrap is the one technique that covers all of them with the same procedure.

---

### Worked Example 42: Honest Small-n Uncertainty

_ex-42 &middot; exercises co-20_

**Context**: continuing co-20, this example reuses ex-41's latency fixture at a small vs. a much
larger sample size, showing the bootstrap interval's own width shrinks dramatically as n grows --
making the small-sample uncertainty visible rather than hidden.

```python
# learning/code/ex-42-honest-small-n-uncertainty/honest_small_n_uncertainty.py
"""Worked Example 42: Honest Small-n Uncertainty."""  # => co-20: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-20: rebuilds the SAME latency-generating fixture at two very different sample sizes

import numpy as np  # => co-20: scipy's bootstrap operates on numpy arrays
from scipy.stats import bootstrap  # => co-20: the SAME bootstrap machinery ex-41 introduced


def build_latency_sample(n: int, *, seed: int) -> list[float]:  # => co-20: the identical fixture-building function from ex-41
    """Build n simulated request latencies (ms), log-normally distributed -- a realistic right-skewed shape."""  # => co-20: documents build_latency_sample's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-20: one fixed generator -- reproducible latency draws
    return [round(rng.lognormvariate(mu=6.0, sigma=0.5)) for _ in range(n)]  # => co-20: most requests are fast, a long tail is slow


def median_statistic(x: np.ndarray, axis: int = -1) -> np.ndarray | float:  # => co-20: the identical statistic function from ex-41
    """Compute the median of x, vectorized over resample rows when axis is given."""  # => co-20: documents median_statistic's contract -- no runtime output, just sets its __doc__
    return np.median(x, axis=axis)  # => co-20: numpy's own median


if __name__ == "__main__":  # => co-20: entry point -- runs only when this file executes directly, not on import
    widths: dict[int, float] = {}  # => co-20: collects each sample size's own interval width, for the shrinkage check below
    for n in (15, 150):  # => co-20: the SAME latency-generating process, sampled a small vs. a much larger number of times
        sample = build_latency_sample(n, seed=42)  # => co-20: this sample size's own draw
        arr = np.array(sample, dtype=float)  # => co-20: scipy's bootstrap resamples this array with replacement
        median = float(np.median(sample))  # => co-20: this sample's own point estimate
        result = bootstrap((arr,), median_statistic, vectorized=True, confidence_level=0.95, n_resamples=3000, method="percentile", rng=np.random.default_rng(7))  # => co-20: the SAME bootstrap procedure, only n changes
        low, high = result.confidence_interval  # => co-20: this sample size's own interval bounds
        width = high - low  # => co-20: how WIDE the honest uncertainty band is, at this sample size
        widths[n] = width  # => co-20: stored for the comparison below
        print(f"n={n}: median={median:.1f}ms bootstrap 95% CI=[{low:.1f}, {high:.1f}]ms width={width:.1f}ms")  # => co-20: the full, honest report at each sample size

    assert widths[15] > 3 * widths[150], "the n=15 interval must be dramatically wider than the n=150 interval -- small-n uncertainty is not a rounding error"  # => co-20: the claim this example demonstrates
    print(f"MATCH: the n=15 interval ({widths[15]:.1f}ms wide) is over {widths[15] / widths[150]:.1f}x wider than the n=150 interval ({widths[150]:.1f}ms wide), on the SAME underlying latency distribution")  # => co-20
    # => co-20: a bootstrap interval does not manufacture precision a small sample does not have -- it reports, honestly, exactly how little a 15-case sample can actually pin down, which a bare median alone would have hidden entirely
```

**Run**: `python3 honest_small_n_uncertainty.py`

**Output**:

```text
n=15: median=535.0ms bootstrap 95% CI=[320.0, 856.0]ms width=536.0ms
n=150: median=377.0ms bootstrap 95% CI=[356.0, 455.5]ms width=99.5ms
MATCH: the n=15 interval (536.0ms wide) is over 5.4x wider than the n=150 interval (99.5ms wide), on the SAME underlying latency distribution
```

**Verify**: at `n=15`, the bootstrap interval is `536.0ms` wide, over `5.4x` wider than the `99.5ms`
interval at `n=150`, on the same underlying latency distribution, satisfying co-20's rule that a
bootstrap interval reports small-sample uncertainty honestly rather than hiding it.

**Key takeaway**: a bootstrap interval does not fabricate precision a small sample never had -- a
wide interval at low n is the correct, honest answer, not a flaw in the method.

**Why It Matters**: a 15-case latency check that reports only its median, with no interval, hides
exactly the fact this example makes visible -- that the true median could plausibly be anywhere in
a 536ms-wide band, information no ship decision should be made without.

---

### Worked Example 43: The Multiple Comparisons Trap

_ex-43 &middot; exercises co-21_

**Context**: **co-21 -- multiple comparisons** shows that testing many criteria at once manufactures
apparent wins by pure chance. This example runs twenty separate two-proportion tests where baseline
and candidate share the IDENTICAL true rate on every criterion, and finds a "significant" result
anyway.

```python
# learning/code/ex-43-the-multiple-comparisons-trap/multiple_comparisons_trap.py
"""Worked Example 43: The Multiple Comparisons Trap."""  # => co-21: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-21: draws twenty INDEPENDENT criterion comparisons, none of them actually different

from statsmodels.stats.proportion import proportions_ztest  # => co-21: the SAME two-proportion test used throughout this theme

TRUE_RATE = 0.80  # => co-21: baseline and candidate have the IDENTICAL true rate on EVERY criterion -- no real difference exists anywhere
N = 30  # => co-21: a typical small per-criterion eval-run size
N_CRITERIA = 20  # => co-21: a realistic rubric size -- twenty separate criteria, each tested independently


def sample_outcomes(true_rate: float, n: int, *, seed: int) -> list[bool]:  # => co-21: one independent sample of one system on one criterion
    """Draw n independent Bernoulli outcomes from true_rate."""  # => co-21: documents sample_outcomes's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-21: one fixed generator per (criterion, system) sample
    return [rng.random() < true_rate for _ in range(n)]  # => co-21: one Bernoulli draw per case


if __name__ == "__main__":  # => co-21: entry point -- runs only when this file executes directly, not on import
    pvalues: list[float] = []  # => co-21: one p-value per criterion, from twenty SEPARATE, independent tests
    for criterion in range(N_CRITERIA):  # => co-21: tests EVERY criterion the SAME way -- no criterion is special
        baseline = sample_outcomes(TRUE_RATE, N, seed=criterion * 2 + 500)  # => co-21: this criterion's own baseline sample
        candidate = sample_outcomes(TRUE_RATE, N, seed=criterion * 2 + 501)  # => co-21: this criterion's own candidate sample -- SAME true rate as baseline
        count = [sum(candidate), sum(baseline)]  # => co-21: successes for each group, this criterion
        _z, p = proportions_ztest(count, [N, N])  # => co-21: this criterion's own p-value
        pvalues.append(p)  # => co-21: stored for the sweep below

    significant = [i for i, p in enumerate(pvalues) if p < 0.05]  # => co-21: every criterion that CROSSED the ordinary 0.05 threshold, uncorrected
    print(f"Criteria tested: {N_CRITERIA} | criteria with p < 0.05 (uncorrected): {significant}")  # => co-21: how many "wins" a report that skips correction would claim
    for i in significant:  # => co-21: prints the "winning" criterion's own p-value
        print(f"  criterion {i}: p={pvalues[i]:.4f}")  # => co-21: looks like a real, ordinary significant result

    assert len(significant) >= 1, "at 20 independent tests, at least one must cross p < 0.05 purely by chance, on the SAME true rate for this fixed seed"  # => co-21: the claim this example demonstrates
    print(f"MATCH: {len(significant)} of {N_CRITERIA} criteria appear 'significant' even though baseline and candidate have the IDENTICAL true rate ({TRUE_RATE}) on every single one")  # => co-21
    # => co-21: testing enough criteria at the ordinary 0.05 threshold manufactures apparent wins out of pure noise -- ex-44 corrects for exactly this
```

**Run**: `python3 multiple_comparisons_trap.py`

**Output**:

```text
Criteria tested: 20 | criteria with p < 0.05 (uncorrected): [14]
  criterion 14: p=0.0227
MATCH: 1 of 20 criteria appear 'significant' even though baseline and candidate have the IDENTICAL true rate (0.8) on every single one
```

**Verify**: across `20` independent tests of two systems sharing the IDENTICAL true rate, criterion
`14` returns `p=0.0227` (uncorrected), satisfying co-21's rule that testing enough criteria
manufactures at least one apparent win by chance alone.

**Key takeaway**: testing many criteria at the ordinary 0.05 threshold is not "twenty honest
tests" -- it is one test run twenty times, with a rising chance one of them crosses the line by
luck alone.

**Why It Matters**: a rubric with many criteria (faithfulness, tone, safety, formatting, and more)
is exactly this scenario -- reporting "criterion 14 improved significantly" without correcting for
the other nineteen tests is a phantom win waiting to be believed.

---

### Worked Example 44: Correcting for Multiple Comparisons

_ex-44 &middot; exercises co-21_

**Context**: continuing co-21, this example applies Bonferroni and Benjamini-Hochberg correction to
ex-43's exact twenty p-values, showing both correctly eliminate the phantom win.

```python
# learning/code/ex-44-correcting-for-multiple-comparisons/correcting_for_multiple_comparisons.py
"""Worked Example 44: Correcting for Multiple Comparisons."""  # => co-21: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-21: rebuilds the SAME twenty-criteria fixture ex-43 introduced

from statsmodels.stats.multitest import multipletests  # => co-21: the pinned library's own multiple-comparisons correction -- both Bonferroni and Benjamini-Hochberg
from statsmodels.stats.proportion import proportions_ztest  # => co-21: the SAME two-proportion test used throughout this theme

TRUE_RATE = 0.80  # => co-21: baseline and candidate have the IDENTICAL true rate on EVERY criterion -- the SAME fixture as ex-43
N = 30  # => co-21: a typical small per-criterion eval-run size
N_CRITERIA = 20  # => co-21: the SAME twenty criteria as ex-43


def sample_outcomes(true_rate: float, n: int, *, seed: int) -> list[bool]:  # => co-21: the identical sampling function from ex-43
    """Draw n independent Bernoulli outcomes from true_rate."""  # => co-21: documents sample_outcomes's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-21: one fixed generator per (criterion, system) sample
    return [rng.random() < true_rate for _ in range(n)]  # => co-21: one Bernoulli draw per case


if __name__ == "__main__":  # => co-21: entry point -- runs only when this file executes directly, not on import
    pvalues: list[float] = []  # => co-21: reproduces ex-43's exact twenty p-values
    for criterion in range(N_CRITERIA):  # => co-21: the SAME twenty criteria, in the SAME order
        baseline = sample_outcomes(TRUE_RATE, N, seed=criterion * 2 + 500)  # => co-21: this criterion's own baseline sample -- SAME seed as ex-43
        candidate = sample_outcomes(TRUE_RATE, N, seed=criterion * 2 + 501)  # => co-21: this criterion's own candidate sample -- SAME seed as ex-43
        count = [sum(candidate), sum(baseline)]  # => co-21: successes for each group, this criterion
        _z, p = proportions_ztest(count, [N, N])  # => co-21: this criterion's own p-value
        pvalues.append(p)  # => co-21: stored for correction below

    uncorrected_significant = [i for i, p in enumerate(pvalues) if p < 0.05]  # => co-21: ex-43's own phantom win -- criterion 14, uncorrected
    print(f"Uncorrected significant criteria: {uncorrected_significant}")  # => co-21: reproduces ex-43's result exactly, for direct comparison

    reject_bonferroni, _corrected_p_bonferroni, _, _ = multipletests(pvalues, alpha=0.05, method="bonferroni")  # => co-21: the STRICT correction -- divides alpha by the number of tests
    bonferroni_significant = [i for i, rejected in enumerate(reject_bonferroni) if rejected]  # => co-21: which criteria survive the strict correction
    print(f"Bonferroni-corrected significant criteria: {bonferroni_significant}")  # => co-21: the phantom win should NOT survive

    reject_bh, _corrected_p_bh, _, _ = multipletests(pvalues, alpha=0.05, method="fdr_bh")  # => co-21: the LESS strict Benjamini-Hochberg false-discovery-rate correction
    bh_significant = [i for i, rejected in enumerate(reject_bh) if rejected]  # => co-21: which criteria survive the FDR correction
    print(f"Benjamini-Hochberg-corrected significant criteria: {bh_significant}")  # => co-21: even the more forgiving correction should reject this phantom win

    assert len(uncorrected_significant) >= 1, "the uncorrected sweep must reproduce ex-43's own phantom win"  # => co-21: confirms this file reproduces the SAME fixture
    assert len(bonferroni_significant) == 0, "Bonferroni correction must eliminate the phantom win entirely"  # => co-21: the strict-correction claim
    assert len(bh_significant) == 0, "Benjamini-Hochberg correction must ALSO eliminate the phantom win"  # => co-21: the FDR-correction claim
    print("MATCH: both Bonferroni and Benjamini-Hochberg correction eliminate the criterion that looked significant BEFORE correcting for testing twenty criteria at once")  # => co-21
    # => co-21: the fix for the multiple-comparisons trap is not 'test fewer criteria' -- it is correcting the significance threshold for how many tests were actually run, using a named, citable method
```

**Run**: `python3 correcting_for_multiple_comparisons.py`

**Output**:

```text
Uncorrected significant criteria: [14]
Bonferroni-corrected significant criteria: []
Benjamini-Hochberg-corrected significant criteria: []
MATCH: both Bonferroni and Benjamini-Hochberg correction eliminate the criterion that looked significant BEFORE correcting for testing twenty criteria at once
```

**Verify**: `multipletests()` returns zero surviving criteria under both `bonferroni` and `fdr_bh`
correction, versus one uncorrected criterion, satisfying co-21's rule that a named correction
method eliminates the phantom win multiple testing manufactures.

**Key takeaway**: the fix for the multiple-comparisons trap is a correction applied to the
threshold, not fewer tests -- Bonferroni and Benjamini-Hochberg both name a specific, citable
method for it.

**Why It Matters**: any eval report that tests more than one criterion and quotes uncorrected
p-values is systematically overclaiming wins -- applying a named correction before reporting
"significant" is the discipline that closes this gap.

---

### Worked Example 45: Noise Floor Decomposition

_ex-45 &middot; exercises co-22, co-23_

**Context**: **co-22 -- noise floor of a suite** and **co-23 -- variance from stochastic
generation** together show that re-running an unchanged system produces a spread with (at least)
two distinct sources: which cases got sampled, and how each case's own stochastic generation
turned out. This example measures both separately and confirms they sum back to the observed total.

```python
# learning/code/ex-45-noise-floor-decomposition/noise_floor_decomposition.py
"""Worked Example 45: Noise Floor Decomposition."""  # => co-22: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-22: builds a population where each case has its own true pass PROBABILITY, not a fixed pass/fail
import statistics  # => co-22: pvariance -- population variance, computed over each simulated distribution of pass rates

POPULATION_SIZE = 300  # => co-22: a realistic-sized pool of real cases
N = 30  # => co-22: a typical eval-set size, drawn from that pool
K_REGENERATIONS = 1000  # => co-23: how many times the SAME fixed cases are re-generated, to measure generation noise alone
M_RESAMPLES = 1000  # => co-22: how many DIFFERENT case samples are drawn, to measure case-sampling variance and total variance


if __name__ == "__main__":  # => co-22: entry point -- runs only when this file executes directly, not on import
    population_rng = random.Random(7)  # => co-22: builds the fixed population every sample below draws from
    population_p = [population_rng.betavariate(6, 2) for _ in range(POPULATION_SIZE)]  # => co-22: each case's own TRUE pass PROBABILITY -- some cases are inherently easier or harder than others
    print(f"Population: {POPULATION_SIZE} cases, mean true pass probability {statistics.mean(population_p):.4f}")  # => co-22: the population's own per-case difficulty spread, not a single pass rate

    fixed_sample_rng = random.Random(42)  # => co-23: fixes ONE specific eval set -- the same 30 cases used throughout the generation-noise measurement below
    fixed_indices = fixed_sample_rng.sample(range(POPULATION_SIZE), N)  # => co-23: the exact 30 cases this "eval set" contains, held constant below
    fixed_p = [population_p[i] for i in fixed_indices]  # => co-23: these 30 cases' own true pass probabilities, held fixed

    generation_rates: list[float] = []  # => co-23: one pass rate per re-run of the SAME 30 cases
    for k in range(K_REGENERATIONS):  # => co-23: re-generates the SAME fixed cases, over and over, as an unchanged stochastic system would on repeat runs
        gen_rng = random.Random(1000 + k)  # => co-23: a fresh generation draw each time -- cases do NOT change, only the stochastic outcome does
        outcomes = [gen_rng.random() < p for p in fixed_p]  # => co-23: each case's outcome THIS regeneration -- a fresh Bernoulli draw from its own true probability
        generation_rates.append(sum(outcomes) / N)  # => co-23: this regeneration's own pass rate
    generation_variance_empirical = statistics.pvariance(generation_rates)  # => co-23: the SPREAD of pass rates across regenerations, cases held fixed -- pure generation noise
    generation_variance_closed_form = sum(p * (1 - p) for p in fixed_p) / (N**2)  # => co-23: the EXACT closed form -- mean per-case Bernoulli variance, divided by n
    print(
        f"Generation variance (fixed cases, regenerated {K_REGENERATIONS}x): empirical={generation_variance_empirical:.6f} closed-form={generation_variance_closed_form:.6f}"
    )  # => co-23: computed twice, same as every other named quantity in this course

    case_sampling_means: list[float] = []  # => co-22: one mean TRUE probability per DIFFERENT case sample, with NO generation noise mixed in
    for m in range(M_RESAMPLES):  # => co-22: draws a DIFFERENT set of 30 cases each time, from the SAME population
        sample_rng = random.Random(2000 + m)  # => co-22: this resample's own case draw
        sample_idx = sample_rng.sample(range(POPULATION_SIZE), N)  # => co-22: which 30 cases this resample happens to contain
        sample_p = [population_p[i] for i in sample_idx]  # => co-22: those 30 cases' own TRUE probabilities -- using the true value, not a noisy generation, isolates case-sampling variance alone
        case_sampling_means.append(statistics.mean(sample_p))  # => co-22: this resample's own mean true probability
    case_sampling_variance_empirical = statistics.pvariance(case_sampling_means)  # => co-22: the SPREAD across different case samples, with generation noise removed entirely

    total_rates: list[float] = []  # => co-22,co-23: one pass rate per DIFFERENT case sample, EACH ALSO freshly generated -- both noise sources combined
    for m in range(M_RESAMPLES):  # => co-22: a DIFFERENT case sample every time, exactly like real repeated eval runs
        total_sample_rng = random.Random(3000 + m)  # => co-22: this run's own case draw
        total_idx = total_sample_rng.sample(range(POPULATION_SIZE), N)  # => co-22: which 30 cases this run happens to contain
        total_p = [population_p[i] for i in total_idx]  # => co-22: those 30 cases' own true probabilities
        total_gen_rng = random.Random(4000 + m)  # => co-23: this run's own fresh generation
        total_outcomes = [total_gen_rng.random() < p for p in total_p]  # => co-23: each case's actual outcome THIS run -- both which cases AND their generation vary
        total_rates.append(sum(total_outcomes) / N)  # => co-22,co-23: this run's own observed pass rate, exactly like a real repeated eval run
    total_variance_empirical = statistics.pvariance(total_rates)  # => co-22,co-23: the SPREAD a team would actually observe re-running the whole eval, unchanged system
    sum_of_components = generation_variance_empirical + case_sampling_variance_empirical  # => co-22,co-23: the decomposition's own prediction -- total should equal the sum of its two independent sources
    print(f"Case-sampling variance (different cases, true probabilities): {case_sampling_variance_empirical:.6f}")  # => co-22
    print(f"Total variance (different cases AND fresh generation): {total_variance_empirical:.6f}")  # => co-22,co-23
    print(f"Sum of components (generation + case-sampling): {sum_of_components:.6f}")  # => co-22,co-23: the internal-consistency check this decomposition must pass

    assert abs(generation_variance_empirical - generation_variance_closed_form) / generation_variance_closed_form < 0.10  # => co-23: the empirical and closed-form generation variance must agree closely
    assert abs(total_variance_empirical - sum_of_components) / sum_of_components < 0.20  # => co-22,co-23: total variance must be close to the sum of its two independent components
    print("MATCH: total run-to-run variance decomposes into generation noise (same cases, re-generated) plus case-sampling variance (different cases drawn) -- and the two sum back to the observed total")  # => co-22,co-23
    # => co-22,co-23: the noise floor is not one number -- it is (at least) two distinct sources, and neither one alone is 'the' variance of an unchanged, re-run suite
```

**Run**: `python3 noise_floor_decomposition.py`

**Output**:

```text
Population: 300 cases, mean true pass probability 0.7407
Generation variance (fixed cases, regenerated 1000x): empirical=0.005279 closed-form=0.005219
Case-sampling variance (different cases, true probabilities): 0.000710
Total variance (different cases AND fresh generation): 0.006178
Sum of components (generation + case-sampling): 0.005989
MATCH: total run-to-run variance decomposes into generation noise (same cases, re-generated) plus case-sampling variance (different cases drawn) -- and the two sum back to the observed total
```

**Verify**: the empirical generation variance (`0.005279`) matches its closed form (`0.005219`)
within `10%`, and the total empirical variance (`0.006178`) matches the sum of generation plus
case-sampling variance (`0.005989`) within `20%`, satisfying co-22's and co-23's rule that an
unchanged suite's own run-to-run spread decomposes into two distinct, separately measurable
sources.

**Key takeaway**: an unchanged system's own repeat-run variance is not one mysterious number -- it
splits into "which cases got sampled" and "how each case's own generation turned out," and both
are separately measurable.

**Why It Matters**: this decomposition IS the noise floor ex-46's ship decision checks a gap
against -- a regression bar set without measuring both sources risks flagging ordinary generation
noise as a real regression, or missing a real regression buried inside sampling noise.

---

### Worked Example 46: A Defensible Ship Decision

_ex-46 &middot; exercises co-24, co-01_

**Context**: this theme, and this topic's whole worked-example arc, closes on **co-24 -- reporting
honestly**: one typed `ShipDecision` that gathers the estimate and its interval, the paired
comparison and its significance, the noise floor, and the materiality threshold into a single
verdict -- reprising co-01's opening claim that a number without its context invites a wrong
decision.

```python
# learning/code/ex-46-a-defensible-ship-decision/defensible_ship_decision.py
"""Worked Example 46: A Defensible Ship Decision."""  # => co-24: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-24: builds one paired baseline/candidate comparison, tying this theme's techniques together
from dataclasses import dataclass  # => co-24: forces the final ship/hold call through one typed, checkable record -- no bare verdict allowed

from statsmodels.stats.contingency_tables import mcnemar  # => co-18: the paired significance test, per ex-37/ex-38
from statsmodels.stats.proportion import proportion_confint  # => co-04,co-05: the small-n-appropriate interval, per Theme A

MEASURED_NOISE_FLOOR = 0.03  # => co-22: this suite's own measured generation-noise floor -- from a decomposition like ex-45's, run previously on this exact eval set
MATERIALITY_THRESHOLD = 0.05  # => co-19: this team's own stated bar for "worth acting on" -- a 5-point gap or larger


def build_paired_dataset(n: int, *, seed: int, baseline_rate: float, candidate_rate: float, correlation: float) -> tuple[list[bool], list[bool]]:  # => co-18: the SAME paired-fixture shape as ex-37/ex-38
    """Build paired baseline/candidate outcomes over n SHARED items, correlated by per-item difficulty."""  # => co-18: documents build_paired_dataset's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-18: drives each item's shared difficulty draw
    baseline: list[bool] = []  # => co-18: baseline's verdict, one per item
    candidate: list[bool] = []  # => co-18: candidate's verdict on the SAME item, one per item
    for _ in range(n):  # => co-18: one shared item at a time
        difficulty_draw = rng.random()  # => co-18: this item's own shared difficulty draw -- read by BOTH systems below
        baseline_pass = difficulty_draw < baseline_rate  # => co-18: baseline's verdict on this exact item
        if rng.random() < correlation:  # => co-18: most of the time, candidate's verdict is driven by the SAME difficulty draw
            candidate_pass = difficulty_draw < candidate_rate  # => co-18: correlated verdict
        else:  # => co-18: occasionally, candidate's verdict is an independent draw instead
            candidate_pass = rng.random() < candidate_rate  # => co-18: an independent verdict
        baseline.append(baseline_pass)  # => co-18: records this item's baseline verdict
        candidate.append(candidate_pass)  # => co-18: records this item's candidate verdict, SAME item, SAME index
    return baseline, candidate  # => co-18: two same-length, index-aligned lists


@dataclass(frozen=True)  # => co-24: immutable -- a ship decision cannot be silently edited after being made
class ShipDecision:  # => co-24: EVERY field this theme built, gathered into the ONE record a real ship/hold call rests on
    n: int  # => co-06: how many paired items this decision rests on
    candidate_rate: float  # => co-02: candidate's own point estimate
    ci_low: float  # => co-04,co-05: candidate's own interval -- small-n-appropriate, per Theme A
    ci_high: float  # => co-04,co-05: candidate's own interval upper bound
    gap: float  # => co-17: the observed gap over baseline
    mcnemar_p: float  # => co-18: the paired significance test's own p-value
    noise_floor: float  # => co-22: this suite's own measured generation-noise floor
    materiality_threshold: float  # => co-19: this team's own stated bar for "worth acting on"
    verdict: str  # => co-24: SHIP or HOLD -- derived from every field above, never chosen by feel


def decide(baseline: list[bool], candidate: list[bool]) -> ShipDecision:  # => co-24: the ONE function that turns raw paired data into a defensible decision
    """Compute every field a ship/hold decision needs, and the resulting verdict."""  # => co-24: documents decide's contract -- no runtime output, just sets its __doc__
    n = len(baseline)  # => co-06: item count
    candidate_rate = sum(candidate) / n  # => co-02: candidate's own point estimate
    ci_low, ci_high = proportion_confint(sum(candidate), n, alpha=0.05, method="wilson")  # => co-04,co-05: candidate's own Wilson interval, per Theme A
    gap = candidate_rate - (sum(baseline) / n)  # => co-17: the observed gap over baseline

    both_pass = sum(1 for b, c in zip(baseline, candidate) if b and c)  # => co-18: concordant pairs -- uninformative for the test
    both_fail = sum(1 for b, c in zip(baseline, candidate) if not b and not c)  # => co-18: concordant pairs -- uninformative for the test
    baseline_only = sum(1 for b, c in zip(baseline, candidate) if b and not c)  # => co-18: discordant -- candidate regressed
    candidate_only = sum(1 for b, c in zip(baseline, candidate) if not b and c)  # => co-18: discordant -- candidate improved
    table = [[both_pass, baseline_only], [candidate_only, both_fail]]  # => co-18: the 2x2 table McNemar's test uses
    mcnemar_p = mcnemar(table, exact=False, correction=True).pvalue  # => co-18: the paired significance test's own p-value

    is_significant = mcnemar_p < 0.05  # => co-18: passes the formal test, per ex-37/ex-38
    beats_noise_floor = gap > MEASURED_NOISE_FLOOR  # => co-22: the gap must exceed what an UNCHANGED system's own re-runs would show, per ex-45
    beats_materiality = gap > MATERIALITY_THRESHOLD  # => co-19: the gap must clear this team's own stated bar for action, per ex-39/ex-40
    verdict = "SHIP" if (is_significant and beats_noise_floor and beats_materiality) else "HOLD"  # => co-24: ALL THREE conditions, never any one number alone

    return ShipDecision(  # => co-24: every field populated -- no bare verdict leaves this function
        n=n,  # => co-06: how many paired items this decision rests on
        candidate_rate=candidate_rate,  # => co-02: candidate's own point estimate
        ci_low=ci_low,  # => co-04,co-05: the interval's lower bound
        ci_high=ci_high,  # => co-04,co-05: the interval's upper bound
        gap=gap,  # => co-17: the observed gap over baseline
        mcnemar_p=mcnemar_p,  # => co-18: the paired test's own p-value
        noise_floor=MEASURED_NOISE_FLOOR,  # => co-22: this suite's own measured noise floor
        materiality_threshold=MATERIALITY_THRESHOLD,  # => co-19: this team's own stated bar for action
        verdict=verdict,  # => co-24: SHIP or HOLD, derived above from all three conditions together
    )  # => co-24: closes the ShipDecision constructor -- all nine fields supplied, none deferred


if __name__ == "__main__":  # => co-24: entry point -- runs only when this file executes directly, not on import
    baseline, candidate = build_paired_dataset(60, seed=15, baseline_rate=0.72, candidate_rate=0.88, correlation=0.85)  # => co-18: one paired eval run over 60 shared items
    decision = decide(baseline, candidate)  # => co-24: the full, typed decision

    print(f"n={decision.n} candidate_rate={decision.candidate_rate:.4f} 95% CI=[{decision.ci_low:.4f}, {decision.ci_high:.4f}]")  # => co-02,co-04,co-05: the estimate and its interval, never the estimate alone
    print(f"gap={decision.gap:.4f} mcnemar_p={decision.mcnemar_p:.4f}")  # => co-17,co-18: the comparison and its actual test
    print(f"noise_floor={decision.noise_floor:.4f} materiality_threshold={decision.materiality_threshold:.4f}")  # => co-19,co-22: the two bars the gap must clear, beyond bare significance
    print(f"VERDICT: {decision.verdict}")  # => co-24: the final, defensible call

    assert decision.verdict == "SHIP", "with a significant, above-noise-floor, above-materiality gap, this decision must resolve to SHIP"  # => co-24: the claim this example demonstrates
    assert decision.ci_low > 0.0 and decision.ci_high < 1.0, "the reported interval must be a genuine range, not a degenerate point"  # => co-04,co-05
    print("MATCH: every field this theme built -- estimate, interval, gap, paired test, noise floor, materiality -- points the SAME direction, which is what makes this verdict defensible")  # => co-24
    # => co-24: a ship decision built on any ONE of these numbers alone -- the bare rate, the bare p-value, the bare gap -- is not yet defensible; this course's own discipline is reporting every field together, or not shipping the claim
```

**Run**: `python3 defensible_ship_decision.py`

**Output**:

```text
n=60 candidate_rate=0.8000 95% CI=[0.6822, 0.8817]
gap=0.1500 mcnemar_p=0.0159
noise_floor=0.0300 materiality_threshold=0.0500
VERDICT: SHIP
MATCH: every field this theme built -- estimate, interval, gap, paired test, noise floor, materiality -- points the SAME direction, which is what makes this verdict defensible
```

**Verify**: `decide()` returns a `ShipDecision` with `candidate_rate=0.8000`, a genuine interval
`[0.6822, 0.8817]`, `mcnemar_p=0.0159` (significant), a `0.1500` gap clearing both the `0.0300`
noise floor and the `0.0500` materiality threshold, and `verdict="SHIP"`, satisfying co-24's rule
that a defensible decision is the full record, not any single field read alone.

**Key takeaway**: a ship decision built on the bare pass rate, the bare p-value, or the bare gap
alone is not yet defensible -- the full `ShipDecision`, every field checked together, is what this
whole topic has been building toward.

**Why It Matters**: this is the exact shape the capstone scales up to a full, real evaluation
report -- a justified sample size, a small-n-appropriate interval, chance-corrected judge
concordance against a measured human ceiling, a paired comparison corrected for multiple
comparisons, and a regression bar derived from a measured noise floor, all reported together.

---

← Previous: [Theme C: Agreement and Judge Concordance](./theme-c-agreement-and-judge-concordance.md)
&middot; Next: [Capstone](./capstone/overview.md) →
