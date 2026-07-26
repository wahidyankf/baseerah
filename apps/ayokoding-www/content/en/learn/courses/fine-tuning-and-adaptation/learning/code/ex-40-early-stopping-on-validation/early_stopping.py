# learning/code/ex-40-early-stopping-on-validation/early_stopping.py
"""Worked Example 40: Early Stopping on Validation."""  # => co-23: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-23: one immutable row per epoch, validation score tracked to pick a stopping point


class EpochCheckpoint(NamedTuple):  # => co-24: a saved model snapshot at one epoch, plus what would happen if training stopped here
    epoch: int  # => co-23: which epoch this checkpoint is from
    val_pass_rate: float  # => co-23: validation pass rate at this epoch -- the signal early stopping watches
    held_out_test_pass_rate: float  # => co-15: a THIRD, still-unseen split -- what actually matters, checked only ONCE at the end


CHECKPOINTS: list[EpochCheckpoint] = [  # => co-23: the same 5-epoch run as ex-39/ex-67, now with a held-out TEST score attached to each checkpoint
    EpochCheckpoint(epoch=1, val_pass_rate=0.70, held_out_test_pass_rate=0.68),  # => co-23: epoch 1
    EpochCheckpoint(epoch=2, val_pass_rate=0.84, held_out_test_pass_rate=0.83),  # => co-23: epoch 2
    EpochCheckpoint(epoch=3, val_pass_rate=0.91, held_out_test_pass_rate=0.90),  # => co-23: epoch 3 -- validation's peak
    EpochCheckpoint(epoch=4, val_pass_rate=0.88, held_out_test_pass_rate=0.85),  # => co-23: epoch 4 -- validation already falling
    EpochCheckpoint(epoch=5, val_pass_rate=0.83, held_out_test_pass_rate=0.79),  # => co-23: epoch 5 -- trained to completion, validation's worst point
]  # => co-23: closes CHECKPOINTS


def stop_early(checkpoints: list[EpochCheckpoint]) -> EpochCheckpoint:  # => co-23: pick the checkpoint validation says is best, not the last one trained
    """Return the `EpochCheckpoint` with the highest `val_pass_rate` in `checkpoints`, the early-stopping choice."""  # => co-23: documents stop_early's contract -- no runtime output, just sets its __doc__
    return max(checkpoints, key=lambda checkpoint: checkpoint.val_pass_rate)  # => co-23: returns this computed value to the caller


if __name__ == "__main__":  # => co-23: entry point -- runs only when this file executes directly, not on import
    trained_to_completion = CHECKPOINTS[-1]  # => co-24: the naive choice -- whatever epoch count was originally configured
    early_stopped = stop_early(CHECKPOINTS)  # => co-23: the validation-driven choice
    print(f"Trained-to-completion checkpoint: epoch {trained_to_completion.epoch} (val {trained_to_completion.val_pass_rate:.0%})")  # => co-24
    print(f"Early-stopped checkpoint: epoch {early_stopped.epoch} (val {early_stopped.val_pass_rate:.0%})")  # => co-23
    assert early_stopped.epoch == 3, "early stopping must select epoch 3, the true validation peak, in this scenario"  # => co-23
    assert trained_to_completion.epoch == 5, "the naive trained-to-completion choice must be epoch 5, the last epoch run"  # => co-24
    test_gain_from_stopping_early = early_stopped.held_out_test_pass_rate - trained_to_completion.held_out_test_pass_rate  # => co-15: the REAL result, on the still-unseen test split
    print(f"Held-out test pass rate: early-stopped {early_stopped.held_out_test_pass_rate:.0%} vs. trained-to-completion {trained_to_completion.held_out_test_pass_rate:.0%}")  # => co-15
    assert test_gain_from_stopping_early > 0, "the early-stopped checkpoint must beat the trained-to-completion checkpoint on the held-out test split too"  # => co-15,co-23
    print(f"MATCH: stopping on validation gains {test_gain_from_stopping_early:.0%} on the still-unseen test split versus training to the configured epoch count")  # => co-23,co-24
    # => co-23,co-24: epoch count is a schedule, not a target -- validation is the signal that decides when to stop, and it must be checked, not assumed
