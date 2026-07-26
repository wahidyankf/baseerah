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
