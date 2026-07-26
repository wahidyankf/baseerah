"""Capstone Step 1: Sampling Plan and Reweighted Sample."""  # => co-06,co-07,co-08: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import math  # => co-06: ceil -- a sample size must be a whole number of cases
import random  # => co-07: builds the synthetic population and draws the stratified sample
import statistics  # => co-06: stdev -- verifies the plan's achieved precision by simulation
from dataclasses import dataclass, field  # => co-24: the SAME typed SamplingPlan record as ex-20

from statsmodels.stats.proportion import samplesize_confint_proportion  # => co-06: the pinned library's own sample-size solver, per ex-08/ex-20

# => co-07,co-08: this eval's population is NOT uniform -- one common case type plus two rare, harder edge-case types
STRATA_SIZES = {"general": 5000, "edge-case-formatting": 300, "edge-case-safety": 150}  # => co-08: population counts per stratum -- very unequal, the SAME shape as Theme B's ex-15
STRATA_RATES = {"general": 0.85, "edge-case-formatting": 0.60, "edge-case-safety": 0.55}  # => co-08: each stratum's own true pass rate -- the rare strata are also the HARDER ones
STRATA_N = {"general": 200, "edge-case-formatting": 60, "edge-case-safety": 60}  # => co-08: deliberately WEIGHT-PROPORTIONAL oversampling -- more from the dominant stratum, enough from each rare one to see it


@dataclass(frozen=True)  # => co-24: frozen -- a written plan is a commitment made BEFORE collecting data
class SamplingPlan:  # => co-24: the four required fields, per ex-20
    target_effect: str  # => co-06: what the plan is trying to be able to detect or estimate
    target_precision: float  # => co-06: the interval half-width the plan is designed to achieve
    strata: tuple[str, ...] = field(default_factory=tuple)  # => co-08: named strata this plan deliberately oversamples
    required_n: int = 0  # => co-06: the resulting UNSTRATIFIED reference sample size -- computed, not guessed


def build_stratum(rate: float, size: int, *, seed: int) -> list[bool]:  # => co-08: one stratum's own fixed population
    """Build a stratum population of size `size`, each True (pass) with probability rate."""  # => co-08: documents build_stratum's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-08: one fixed generator per stratum
    return [rng.random() < rate for _ in range(size)]  # => co-08: one Bernoulli draw per case in this stratum


def draw_reweighted_estimate(population_by_stratum: dict[str, list[bool]], n_per_stratum: dict[str, int], *, trial_seed: int) -> float:  # => co-08: one full stratified-draw-then-reweight cycle
    """Draw n_per_stratum[s] cases from each stratum and return the population-share-reweighted estimate."""  # => co-08: documents draw_reweighted_estimate's contract -- no runtime output, just sets its __doc__
    total_pop = sum(len(cases) for cases in population_by_stratum.values())  # => co-08: total population size, across all strata
    stratum_means: dict[str, float] = {}  # => co-08: each stratum's own sample-based estimate, this trial
    for i, (stratum, cases) in enumerate(population_by_stratum.items()):  # => co-08: draws THIS trial's sample from every stratum
        sample = random.Random(trial_seed * 1000 + i).sample(cases, n_per_stratum[stratum])  # => co-08: this stratum's own draw, this trial
        stratum_means[stratum] = sum(sample) / len(sample)  # => co-08: this stratum's own sample mean, this trial
    return sum(stratum_means[s] * (len(population_by_stratum[s]) / total_pop) for s in population_by_stratum)  # => co-08: weight each stratum's mean by its REAL population share


if __name__ == "__main__":  # => co-24: entry point -- runs only when this file executes directly, not on import
    unstratified_raw_n = samplesize_confint_proportion(proportion=0.83, half_length=0.05, alpha=0.05, method="normal")  # => co-06: the SIMPLE reference n, ignoring strata -- an anchor, not the final plan
    unstratified_n = math.ceil(unstratified_raw_n)  # => co-06: round UP -- a fractional case cannot be collected
    plan = SamplingPlan(  # => co-24: the written plan, BEFORE any data is drawn
        target_effect="candidate-vs-baseline overall pass-rate estimate for the ship decision",  # => co-06: states WHAT the plan supports
        target_precision=0.05,  # => co-06: "I need the overall estimate within +/-5 points"
        strata=tuple(STRATA_SIZES.keys()),  # => co-08: the three strata this plan deliberately oversamples
        required_n=unstratified_n,  # => co-06: the unstratified anchor -- the actual stratified plan below allocates MORE than this, on purpose
    )
    print(f"Target effect: {plan.target_effect}")  # => co-06: states the effect explicitly, first
    print(f"Target precision (half-width): {plan.target_precision}")  # => co-06: states the precision explicitly
    print(f"Strata: {plan.strata}")  # => co-08: states the strata explicitly
    print(f"Unstratified reference n: {plan.required_n}")  # => co-06: the anchor figure
    total_planned_n = sum(STRATA_N.values())  # => co-08: the ACTUAL total this plan draws, across all three strata
    print(f"Actual stratified allocation: {STRATA_N} (total n={total_planned_n})")  # => co-08: MORE than the unstratified anchor, because the rare strata need dedicated coverage too

    population_by_stratum = {s: build_stratum(STRATA_RATES[s], size, seed=100 + i) for i, (s, size) in enumerate(STRATA_SIZES.items())}  # => co-08: the fixed synthetic population this whole file draws from
    total_pop = sum(STRATA_SIZES.values())  # => co-08: total population size
    true_overall_rate = sum(sum(cases) for cases in population_by_stratum.values()) / total_pop  # => co-08: the population's own TRUE overall rate -- KNOWN here because this is synthetic data, used to verify recovery below
    print(f"(Synthetic ground truth, for verification only) true overall population rate: {true_overall_rate:.4f}")  # => co-08

    achieved_estimates = [draw_reweighted_estimate(population_by_stratum, STRATA_N, trial_seed=t) for t in range(500)]  # => co-06: simulates 500 independent draws AT the planned allocation, to check the plan's own achieved precision
    achieved_stdev = statistics.stdev(achieved_estimates)  # => co-06: the spread of the reweighted estimate, across those 500 simulated draws
    achieved_half_width = 1.96 * achieved_stdev  # => co-06: the approximate 95% half-width this ALLOCATION actually achieves
    print(f"Simulated achieved 95% half-width at this allocation: {achieved_half_width:.4f}")  # => co-06: verifies the plan's OWN precision claim, by simulation
    assert achieved_half_width <= plan.target_precision * 1.10, "the planned allocation must achieve close to its own stated target precision"  # => co-06: the plan-is-justified claim

    stratum_samples = {s: random.Random(500 + i).sample(cases, STRATA_N[s]) for i, (s, cases) in enumerate(population_by_stratum.items())}  # => co-08: ONE actual observed sample, at the planned allocation
    stratum_means = {s: sum(sample) / len(sample) for s, sample in stratum_samples.items()}  # => co-08: each stratum's own observed mean
    naive_pooled = sum(sum(sample) for sample in stratum_samples.values()) / sum(len(sample) for sample in stratum_samples.values())  # => co-08: the WRONG shortcut -- ignores each stratum's real population share
    reweighted_estimate = sum(stratum_means[s] * (STRATA_SIZES[s] / total_pop) for s in STRATA_SIZES)  # => co-08: the population-share-reweighted estimate, per ex-16
    print(f"Naive pooled estimate: {naive_pooled:.4f} | Reweighted estimate: {reweighted_estimate:.4f}")  # => co-08

    naive_error = abs(naive_pooled - true_overall_rate)  # => co-08: how far the naive estimate misses the KNOWN synthetic truth
    reweighted_error = abs(reweighted_estimate - true_overall_rate)  # => co-08: how far the reweighted estimate misses the KNOWN synthetic truth
    print(f"Naive error: {naive_error:.4f} | Reweighted error: {reweighted_error:.4f}")  # => co-08
    assert reweighted_error < naive_error, "on synthetic data with a known population rate, reweighting must recover it more accurately than naive pooling"  # => co-08: the recovery claim this step's own acceptance criterion requires
    print("MATCH: this plan's allocation achieves its own stated precision by simulation, and the reweighted estimate recovers the known synthetic population rate more accurately than naive pooling")  # => co-08
    # => co-06,co-07,co-08: this is the artifact sampling-plan.md narrates -- a plan justified BEFORE data collection, and a reweighted estimate verified against a KNOWN synthetic truth, not merely asserted to work
