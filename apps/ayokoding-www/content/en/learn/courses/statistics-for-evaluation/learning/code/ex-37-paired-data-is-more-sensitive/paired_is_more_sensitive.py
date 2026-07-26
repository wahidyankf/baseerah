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
