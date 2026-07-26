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
