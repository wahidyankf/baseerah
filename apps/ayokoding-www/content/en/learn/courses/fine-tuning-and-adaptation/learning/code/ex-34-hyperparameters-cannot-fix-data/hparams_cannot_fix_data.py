# learning/code/ex-34-hyperparameters-cannot-fix-data/hparams_cannot_fix_data.py
"""Worked Example 34: Hyperparameters Cannot Fix Data."""  # => co-24: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-24: one row per hyperparameter config, swept against the SAME noisy dataset


class NoisyDataSweepResult(NamedTuple):  # => co-10,co-24: every config below trains on ex-18's SAME noisy 10,000-example dataset
    config_label: str  # => co-24: a short label for this hyperparameter combination
    eval_pass_rate: float  # => co-24: the resulting model's measured pass rate on ex-08's fixed eval


CLEAN_DATASET_RESULT = 0.91  # => co-11,co-18: ex-18's CLEAN_SMALL_RUN result -- the bar every config below is measured against

# => co-24: an exhaustive-feeling sweep, every combination trained on ex-18's SAME noisy, 35%-inconsistent dataset
NOISY_DATA_SWEEP: list[NoisyDataSweepResult] = [  # => co-10: the dataset never changes across this entire sweep -- only hyperparameters do
    NoisyDataSweepResult(config_label="default lr, 3 epochs", eval_pass_rate=0.68),  # => co-24: ex-18's original noisy-data result
    NoisyDataSweepResult(config_label="lower lr, 3 epochs", eval_pass_rate=0.66),  # => co-24: no better
    NoisyDataSweepResult(config_label="default lr, 10 epochs", eval_pass_rate=0.70),  # => co-24: marginal gain, then plateaus
    NoisyDataSweepResult(config_label="default lr, 25 epochs", eval_pass_rate=0.69),  # => co-24: more epochs makes it WORSE -- memorizing noise now
    NoisyDataSweepResult(config_label="rank 32 instead of rank 8", eval_pass_rate=0.72),  # => co-20,co-24: the best of this entire sweep
    NoisyDataSweepResult(config_label="rank 64, 10 epochs", eval_pass_rate=0.71),  # => co-24: still nowhere close
]  # => co-24: closes NOISY_DATA_SWEEP


if __name__ == "__main__":  # => co-24: entry point -- runs only when this file executes directly, not on import
    for row in NOISY_DATA_SWEEP:  # => co-24: print every configuration tried against the SAME noisy dataset
        print(f"  {row.config_label}: {row.eval_pass_rate:.0%}")  # => co-24
    best_noisy_result = max(row.eval_pass_rate for row in NOISY_DATA_SWEEP)  # => co-24: the best ANY hyperparameter config achieved
    print(f"Best achievable on the noisy dataset, across every config tried: {best_noisy_result:.0%}")  # => co-24
    print(f"Clean, small dataset's result (ex-18), UNTOUCHED hyperparameters: {CLEAN_DATASET_RESULT:.0%}")  # => co-11,co-24
    gap_remaining = CLEAN_DATASET_RESULT - best_noisy_result  # => co-10,co-24: what NO hyperparameter config could close
    assert best_noisy_result < CLEAN_DATASET_RESULT - 0.15, "even the best hyperparameter config on noisy data must fall well short of the clean-data result"  # => co-10,co-24
    print(f"Gap remaining after exhausting hyperparameters: {gap_remaining:.0%}")  # => co-24
    print("MATCH: no hyperparameter configuration recovered the clean-data result -- the dataset was always the actual problem")  # => co-10,co-24
    # => co-10,co-24: this closes the loop on co-10's central claim -- when the dataset is the bottleneck, tuning knobs is time spent on the wrong lever
