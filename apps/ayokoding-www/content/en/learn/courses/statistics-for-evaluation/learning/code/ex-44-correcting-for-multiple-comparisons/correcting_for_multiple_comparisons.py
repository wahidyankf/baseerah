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
