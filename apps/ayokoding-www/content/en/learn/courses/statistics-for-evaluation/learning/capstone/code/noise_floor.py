"""Capstone Step 4: Noise Floor and Regression Bar."""  # => co-22,co-23: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import math  # => co-22: sqrt -- converts variance into a stdev-scaled regression bar
import random  # => co-22: builds the population and draws both the generation-noise and case-sampling simulations
import statistics  # => co-22: pvariance -- population variance over each simulated distribution
from dataclasses import dataclass  # => co-24: forces the final noise-floor report through one typed shape

POPULATION_SIZE = 400  # => co-22: a realistic-sized pool of real cases, for THIS capstone's own candidate system
N = 50  # => co-22: the SAME eval-set size compare.py used for the "correctness" criterion
K_REGENERATIONS = 1000  # => co-23: how many times the SAME fixed cases are re-generated
M_RESAMPLES = 1000  # => co-22: how many DIFFERENT case samples are drawn


@dataclass(frozen=True)  # => co-24: immutable -- a measured noise floor is a fact about the suite, not something later code edits
class NoiseFloorReport:  # => co-24: EVERY field the noise-floor measurement needs, gathered together
    n: int  # => co-06: how many cases each simulated eval run draws
    generation_variance: float  # => co-23: variance from stochastic generation alone, cases held fixed
    case_sampling_variance: float  # => co-22: variance from which cases got sampled alone
    total_variance: float  # => co-22,co-23: the SPREAD a team would actually observe re-running the whole eval
    noise_floor_stdev: float  # => co-22: the standard deviation of that spread
    regression_bar: float  # => co-22: the derived threshold -- a gap below this could be ordinary noise, not a real regression


if __name__ == "__main__":  # => co-22: entry point -- runs only when this file executes directly, not on import
    population_rng = random.Random(9)  # => co-22: builds the fixed population every sample below draws from
    population_p = [population_rng.betavariate(7, 3) for _ in range(POPULATION_SIZE)]  # => co-22: each case's own TRUE pass PROBABILITY -- the SAME technique as ex-45
    print(f"Population: {POPULATION_SIZE} cases, mean true pass probability {statistics.mean(population_p):.4f}")  # => co-22

    fixed_sample_rng = random.Random(50)  # => co-23: fixes ONE specific eval set -- reused for the generation-noise measurement below
    fixed_indices = fixed_sample_rng.sample(range(POPULATION_SIZE), N)  # => co-23: the exact 50 cases this candidate's "correctness" eval set contains
    fixed_p = [population_p[i] for i in fixed_indices]  # => co-23: these cases' own true pass probabilities, held fixed

    generation_rates: list[float] = []  # => co-23: one pass rate per re-run of the SAME fixed cases
    for k in range(K_REGENERATIONS):  # => co-23: re-generates the SAME fixed cases, over and over -- exactly like re-running an unchanged candidate
        gen_rng = random.Random(9000 + k)  # => co-23: a fresh generation draw each time
        outcomes = [gen_rng.random() < p for p in fixed_p]  # => co-23: each case's outcome THIS regeneration
        generation_rates.append(sum(outcomes) / N)  # => co-23: this regeneration's own pass rate
    generation_variance_empirical = statistics.pvariance(generation_rates)  # => co-23: pure generation noise, cases held fixed
    generation_variance_closed_form = sum(p * (1 - p) for p in fixed_p) / (N**2)  # => co-23: the EXACT closed form, per ex-45
    print(f"Generation variance: empirical={generation_variance_empirical:.6f} closed-form={generation_variance_closed_form:.6f}")  # => co-23: computed twice, same discipline as every other coefficient in this capstone

    case_sampling_means: list[float] = []  # => co-22: one mean TRUE probability per DIFFERENT case sample
    for m in range(M_RESAMPLES):  # => co-22: draws a DIFFERENT set of 50 cases each time, from the SAME population
        sample_rng = random.Random(8000 + m)  # => co-22: this resample's own case draw
        sample_idx = sample_rng.sample(range(POPULATION_SIZE), N)  # => co-22: which cases this resample happens to contain
        sample_p = [population_p[i] for i in sample_idx]  # => co-22: those cases' own TRUE probabilities -- isolates case-sampling variance alone
        case_sampling_means.append(statistics.mean(sample_p))  # => co-22: this resample's own mean true probability
    case_sampling_variance_empirical = statistics.pvariance(case_sampling_means)  # => co-22: the SPREAD across different case samples, generation noise removed

    total_rates: list[float] = []  # => co-22,co-23: one pass rate per DIFFERENT case sample, EACH ALSO freshly generated
    for m in range(M_RESAMPLES):  # => co-22: a DIFFERENT case sample every time, exactly like real repeated eval runs
        total_sample_rng = random.Random(7000 + m)  # => co-22: this run's own case draw
        total_idx = total_sample_rng.sample(range(POPULATION_SIZE), N)  # => co-22: which cases this run happens to contain
        total_p = [population_p[i] for i in total_idx]  # => co-22: those cases' own true probabilities
        total_gen_rng = random.Random(6000 + m)  # => co-23: this run's own fresh generation
        total_outcomes = [total_gen_rng.random() < p for p in total_p]  # => co-23: each case's actual outcome THIS run
        total_rates.append(sum(total_outcomes) / N)  # => co-22,co-23: this run's own observed pass rate
    total_variance_empirical = statistics.pvariance(total_rates)  # => co-22,co-23: the SPREAD an unchanged, re-run candidate would actually show
    sum_of_components = generation_variance_empirical + case_sampling_variance_empirical  # => co-22,co-23: the decomposition's own internal-consistency check
    print(f"Case-sampling variance: {case_sampling_variance_empirical:.6f}")  # => co-22
    print(f"Total variance: {total_variance_empirical:.6f} | Sum of components: {sum_of_components:.6f}")  # => co-22,co-23

    noise_floor_stdev = math.sqrt(total_variance_empirical)  # => co-22: converts the measured total variance into a standard-deviation-scale figure
    regression_bar = 2 * noise_floor_stdev  # => co-22: a gap smaller than TWICE this system's own noise floor could plausibly be ordinary re-run noise, not a real regression
    report = NoiseFloorReport(  # => co-24: every field populated -- no bare regression bar leaves this function
        n=N,  # => co-06: how many cases each simulated eval run draws
        generation_variance=generation_variance_empirical,  # => co-23: variance from stochastic generation alone
        case_sampling_variance=case_sampling_variance_empirical,  # => co-22: variance from which cases got sampled alone
        total_variance=total_variance_empirical,  # => co-22,co-23: both sources combined
        noise_floor_stdev=noise_floor_stdev,  # => co-22: the standard deviation of that spread
        regression_bar=regression_bar,  # => co-22: the derived threshold, not chosen by feel
    )  # => co-24: closes the NoiseFloorReport constructor -- all six fields supplied, none deferred
    print(f"Noise floor stdev: {report.noise_floor_stdev:.4f} | Derived regression bar: {report.regression_bar:.4f}")  # => co-22: the ONE number a ship decision compares its own gap against

    assert abs(generation_variance_empirical - generation_variance_closed_form) / generation_variance_closed_form < 0.10, "generation variance must match its closed form closely"  # => co-23
    assert abs(total_variance_empirical - sum_of_components) / sum_of_components < 0.20, "total variance must be close to the sum of its two components"  # => co-22,co-23
    assert report.regression_bar < 0.24, "compare.py's own planted real effect (a 0.24 gap on correctness) must clear this measured regression bar"  # => co-22: ties this file's output directly to compare.py's own result
    print("MATCH: this candidate's own measured noise floor derives a regression bar well below compare.py's planted 0.24 real-effect gap -- that gap is not explainable by ordinary re-run noise alone")  # => co-22,co-23
    # => co-22,co-23,co-24: a regression bar CHOSEN by feel is a guess -- this one is DERIVED from a measured decomposition of this exact candidate's own re-run variance, which is what report.md cites
