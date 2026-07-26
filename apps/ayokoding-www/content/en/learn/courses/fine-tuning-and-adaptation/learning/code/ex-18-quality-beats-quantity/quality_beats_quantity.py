# learning/code/ex-18-quality-beats-quantity/quality_beats_quantity.py
"""Worked Example 18: Quality Beats Quantity."""  # => co-11: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-11: two training RUNS, each with its own recorded shape -- not two loose tuples


@dataclass(frozen=True)  # => co-11: frozen -- a completed run's recorded result should not mutate after the fact
class TrainingRunResult:  # => co-11: what a real training run reports, mocked here with illustrative, fixed numbers
    dataset_size: int  # => co-11: how many examples the run trained on
    noisy_fraction: float  # => co-12: what fraction of those examples were inconsistent or mislabeled
    eval_pass_rate: float  # => co-11: the resulting model's measured pass rate on the SAME fixed eval set


CLEAN_SMALL_RUN = TrainingRunResult(dataset_size=300, noisy_fraction=0.02, eval_pass_rate=0.91)  # => co-11: 300 consistent, correct, on-distribution examples
NOISY_LARGE_RUN = TrainingRunResult(dataset_size=10_000, noisy_fraction=0.35, eval_pass_rate=0.68)  # => co-11: 10,000 examples, over a third inconsistent or wrong

if __name__ == "__main__":  # => co-11: entry point -- runs only when this file executes directly, not on import
    print(f"Clean, small: {CLEAN_SMALL_RUN.dataset_size} examples, {CLEAN_SMALL_RUN.noisy_fraction:.0%} noisy -> {CLEAN_SMALL_RUN.eval_pass_rate:.0%} pass rate")  # => co-11
    print(f"Noisy, large: {NOISY_LARGE_RUN.dataset_size} examples, {NOISY_LARGE_RUN.noisy_fraction:.0%} noisy -> {NOISY_LARGE_RUN.eval_pass_rate:.0%} pass rate")  # => co-11
    size_ratio = NOISY_LARGE_RUN.dataset_size / CLEAN_SMALL_RUN.dataset_size  # => co-11: the noisy run has over 33x MORE raw examples
    print(f"Noisy run has {size_ratio:.0f}x more raw examples, yet scores LOWER")  # => co-11
    assert NOISY_LARGE_RUN.dataset_size > CLEAN_SMALL_RUN.dataset_size * 30, "the noisy run must have vastly more raw examples"  # => co-11
    assert CLEAN_SMALL_RUN.eval_pass_rate > NOISY_LARGE_RUN.eval_pass_rate, "the smaller, cleaner dataset must win on eval despite being far smaller"  # => co-10,co-11
    print("MATCH: 300 clean examples beat 10,000 noisy ones -- dataset SIZE was never the variable that mattered")  # => co-11
    # => co-10,co-11: this is co-11 made concrete -- co-10's "dataset is the work" claim resolves to quality, not raw volume, as the lever that actually moves the number
