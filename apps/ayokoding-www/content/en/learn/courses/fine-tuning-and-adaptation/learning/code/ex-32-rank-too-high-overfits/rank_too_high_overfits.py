# learning/code/ex-32-rank-too-high-overfits/rank_too_high_overfits.py
"""Worked Example 32: Rank Too High Overfits."""  # => co-20: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-23: a training run's train-vs-held-out result, on a DELIBERATELY small dataset this time


@dataclass(frozen=True)  # => co-23: frozen -- a completed run's recorded result should not mutate after the fact
class RankRun:  # => co-20,co-23: what training loss alone would NOT reveal -- the train/validation gap, made explicit
    rank: int  # => co-20: the adapter rank used for this run
    dataset_size: int  # => co-23: a SMALL dataset -- exactly the setting where excess capacity can memorize instead of generalize
    train_pass_rate: float  # => co-23: pass rate on the SAME cases the model trained on
    validation_pass_rate: float  # => co-23: pass rate on held-out cases the model never saw during training


SMALL_DATASET_SIZE = 40  # => co-23: deliberately small -- co-20's rank-too-high risk needs a dataset this size to bite

MODEST_RANK_RUN = RankRun(rank=8, dataset_size=SMALL_DATASET_SIZE, train_pass_rate=0.95, validation_pass_rate=0.94)  # => co-20,co-23: generalizes well
EXCESSIVE_RANK_RUN = RankRun(rank=256, dataset_size=SMALL_DATASET_SIZE, train_pass_rate=0.99, validation_pass_rate=0.71)  # => co-20,co-23: memorized instead

if __name__ == "__main__":  # => co-23: entry point -- runs only when this file executes directly, not on import
    modest_gap = MODEST_RANK_RUN.train_pass_rate - MODEST_RANK_RUN.validation_pass_rate  # => co-23: train/validation gap at a modest rank
    excessive_gap = EXCESSIVE_RANK_RUN.train_pass_rate - EXCESSIVE_RANK_RUN.validation_pass_rate  # => co-23: the SAME gap at an excessive rank
    print(f"rank={MODEST_RANK_RUN.rank}: train {MODEST_RANK_RUN.train_pass_rate:.0%}, validation {MODEST_RANK_RUN.validation_pass_rate:.0%} (gap {modest_gap:.0%})")  # => co-20,co-23
    print(f"rank={EXCESSIVE_RANK_RUN.rank}: train {EXCESSIVE_RANK_RUN.train_pass_rate:.0%}, validation {EXCESSIVE_RANK_RUN.validation_pass_rate:.0%} (gap {excessive_gap:.0%})")  # => co-20,co-23
    print(f"Training loss ALONE, on either run, would show a near-perfect fit -- {MODEST_RANK_RUN.train_pass_rate:.0%} and {EXCESSIVE_RANK_RUN.train_pass_rate:.0%}")  # => co-23
    assert excessive_gap > modest_gap * 4, "the excessive-rank run's train/validation gap must be dramatically larger"  # => co-20,co-23
    assert EXCESSIVE_RANK_RUN.validation_pass_rate < MODEST_RANK_RUN.validation_pass_rate, "the excessive-rank run must generalize WORSE despite fitting training data better"  # => co-23
    print("MATCH: rank 256, on a 40-example dataset, memorized training data -- and training pass rate alone never revealed it")  # => co-20,co-23
    # => co-20,co-23: this is co-20's capacity/overfitting trade-off made concrete -- more rank is not free, and the cost is invisible until you check held-out data
