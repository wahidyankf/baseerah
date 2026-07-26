"""Worked Example 15: A Stratified Sample."""  # => co-08: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-07: builds each stratum's population and draws both sampling strategies
from collections import Counter  # => co-07: tallies per-stratum representation in the random sample

STRATA_SIZES = {"formatting": 1400, "factual": 500, "tone": 100}  # => co-08: population counts per failure-mode stratum -- very unequal sizes
STRATA_PASS_RATE = {"formatting": 0.90, "factual": 0.75, "tone": 0.60}  # => co-08: each stratum has its own true pass rate
STRATIFIED_N_PER_STRATUM = 20  # => co-08: fixed count drawn from EACH stratum, regardless of the stratum's own population size


def build_stratum(pass_rate: float, size: int, *, seed: int) -> list[bool]:  # => co-08: one failure-mode stratum's own population
    """Build a stratum population of size `size`, each True (pass) with probability pass_rate."""  # => co-08: documents build_stratum's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-08: one fixed generator per stratum
    return [rng.random() < pass_rate for _ in range(size)]  # => co-08: one Bernoulli draw per case in this stratum


if __name__ == "__main__":  # => co-08: entry point -- runs only when this file executes directly, not on import
    population_by_stratum = {s: build_stratum(STRATA_PASS_RATE[s], n, seed=4) for s, n in STRATA_SIZES.items()}  # => co-08: three separate, fixed strata populations
    pooled = [(s, c) for s, cases in population_by_stratum.items() for c in cases]  # => co-07: the SAME population, viewed as one pooled list -- what plain random sampling would draw from
    for s, cases in population_by_stratum.items():  # => co-08: prints each stratum's own size and true rate up front
        print(f"{s:<12} population={len(cases):>5}  true_rate={sum(cases) / len(cases):.4f}")  # => co-08

    random_sample = random.Random(60).sample(pooled, 60)  # => co-07: an honest, proportional random sample of the SAME total size as the stratified one below
    random_counts = Counter(s for s, _ in random_sample)  # => co-07: how many of each stratum a plain random sample happened to include
    print(f"Random sample (n=60) per-stratum coverage: {dict(random_counts)}")  # => co-07: the tiny "tone" stratum barely appears
    assert random_counts["tone"] < STRATIFIED_N_PER_STRATUM, "a proportional random sample must under-represent the small 'tone' stratum relative to a stratified draw"  # => co-08

    stratified_counts = {}  # => co-08: one deliberate sample size per stratum
    for s, cases in population_by_stratum.items():  # => co-08: draws EXACTLY STRATIFIED_N_PER_STRATUM from every stratum, regardless of its own size
        sample = random.Random(70).sample(cases, STRATIFIED_N_PER_STRATUM)  # => co-08: the deliberate, guaranteed-coverage draw
        stratified_counts[s] = len(sample)  # => co-08: records this stratum's own guaranteed count
    print(f"Stratified sample per-stratum coverage: {stratified_counts}")  # => co-08: every stratum, equally represented
    assert all(count == STRATIFIED_N_PER_STRATUM for count in stratified_counts.values()), "every stratum must be equally represented under stratified sampling"  # => co-08
    print("MATCH: stratified sampling guarantees every stratum's coverage; random sampling leaves it to chance")  # => co-08
    # => co-07,co-08: random and stratified sampling literally estimate DIFFERENT things when strata are unequal -- ex-16 shows how to combine stratified counts back into one honest population estimate
