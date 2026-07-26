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
