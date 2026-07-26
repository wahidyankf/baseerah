"""Worked Example 16: Reweight a Stratified Estimate."""  # => co-08: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-08: builds each stratum's population and draws the stratified sample

STRATA_SIZES = {"formatting": 1400, "factual": 500, "tone": 100}  # => co-08: population counts per stratum -- the SAME fixture as ex-15
STRATA_PASS_RATE = {"formatting": 0.90, "factual": 0.75, "tone": 0.60}  # => co-08: each stratum's own true pass rate
POPULATION_TOTAL = sum(STRATA_SIZES.values())  # => co-08: the full population size across all strata
N_PER_STRATUM = 20  # => co-08: fixed draw per stratum -- OVERSAMPLES "tone", which is only 5% of the population but 33% of this sample


def build_stratum(pass_rate: float, size: int, *, seed: int) -> list[bool]:  # => co-08: one failure-mode stratum's own population
    """Build a stratum population of size `size`, each True (pass) with probability pass_rate."""  # => co-08: documents build_stratum's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-08: one fixed generator per stratum
    return [rng.random() < pass_rate for _ in range(size)]  # => co-08: one Bernoulli draw per case in this stratum


if __name__ == "__main__":  # => co-08: entry point -- runs only when this file executes directly, not on import
    population_by_stratum = {s: build_stratum(STRATA_PASS_RATE[s], n, seed=4) for s, n in STRATA_SIZES.items()}  # => co-08: the SAME three strata populations ex-15 built
    true_overall_rate = sum(sum(cases) for cases in population_by_stratum.values()) / POPULATION_TOTAL  # => co-08: the population's own TRUE overall pass rate, weighted by real stratum sizes
    print(f"True overall population pass rate: {true_overall_rate:.4f}")  # => co-08: the ground truth every estimate below is compared against

    stratum_samples = {s: random.Random(70 + i).sample(cases, N_PER_STRATUM) for i, (s, cases) in enumerate(population_by_stratum.items())}  # => co-08: one equal-sized sample per stratum
    stratum_means = {s: sum(sample) / len(sample) for s, sample in stratum_samples.items()}  # => co-08: each stratum's own sample-based pass-rate estimate
    print(f"Per-stratum sample means: { {s: round(m, 4) for s, m in stratum_means.items()} }")  # => co-08

    naive_pooled_estimate = sum(sum(sample) for sample in stratum_samples.values()) / sum(len(sample) for sample in stratum_samples.values())  # => co-08: WRONG -- treats every stratum as equally common
    print(f"Naive (unweighted) pooled estimate: {naive_pooled_estimate:.4f}")  # => co-08: biased toward the over-sampled "tone" stratum's lower rate

    reweighted_estimate = sum(stratum_means[s] * (STRATA_SIZES[s] / POPULATION_TOTAL) for s in STRATA_SIZES)  # => co-08: weight each stratum's mean by its REAL population share
    print(f"Reweighted (population-share-weighted) estimate: {reweighted_estimate:.4f}")  # => co-08: recovers the population structure the sample itself distorted

    naive_error = abs(naive_pooled_estimate - true_overall_rate)  # => co-08: how far the naive pooled estimate misses the truth
    reweighted_error = abs(reweighted_estimate - true_overall_rate)  # => co-08: how far the reweighted estimate misses the truth
    print(f"Naive error: {naive_error:.4f} | Reweighted error: {reweighted_error:.4f}")  # => co-08
    assert reweighted_error < naive_error, "reweighting by population share must reduce the error versus the naive unweighted pool"  # => co-08: the reweighting claim itself
    print("MATCH: reweighting by each stratum's TRUE population share recovers a far more honest overall estimate")  # => co-08
    # => co-08: oversampling a rare stratum for statistical power is fine -- but pooling it back WITHOUT reweighting silently distorts the overall number
