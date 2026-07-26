# learning/code/ex-67-early-warning-signs-of-overfitting/early_warning_signs.py
"""Worked Example 67: Early Warning Signs of Overfitting."""  # => co-23: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-23: one immutable row per training epoch, read in sequence as the run progresses


class EpochSnapshot(NamedTuple):  # => co-23: what a training loop can observe about itself, epoch by epoch, WHILE it is running
    epoch: int  # => co-23: which epoch this snapshot is from
    train_pass_rate: float  # => co-23: pass rate on the training set at this epoch -- keeps climbing under overfitting
    val_pass_rate: float  # => co-23: pass rate on a held-out validation set at this epoch -- the signal that catches overfitting EARLY


# => co-23: the SAME run ex-39 later shows only in hindsight, here observed epoch-by-epoch AS it happens
TRAINING_CURVE: list[EpochSnapshot] = [  # => co-24: per-epoch snapshots, in order, for one LoRA fine-tuning run
    EpochSnapshot(epoch=1, train_pass_rate=0.71, val_pass_rate=0.70),  # => co-23: epoch 1 -- train and val move together
    EpochSnapshot(epoch=2, train_pass_rate=0.85, val_pass_rate=0.84),  # => co-23: epoch 2 -- still tracking closely
    EpochSnapshot(epoch=3, train_pass_rate=0.93, val_pass_rate=0.91),  # => co-23: epoch 3 -- val is at its PEAK here
    EpochSnapshot(epoch=4, train_pass_rate=0.97, val_pass_rate=0.88),  # => co-23,co-24: epoch 4 -- train KEEPS climbing, val starts falling -- the warning sign
    EpochSnapshot(epoch=5, train_pass_rate=0.99, val_pass_rate=0.83),  # => co-23,co-24: epoch 5 -- the gap keeps widening
]  # => co-23: closes TRAINING_CURVE

EARLY_WARNING_GAP_THRESHOLD = 0.05  # => co-23: once train exceeds val by more than 5 points, treat it as a live warning, not noise


def first_warning_epoch(curve: list[EpochSnapshot], threshold: float) -> int | None:  # => co-23: which epoch FIRST crosses the threshold, if any
    """Return the epoch number of the first `EpochSnapshot` in `curve` whose train/val gap exceeds `threshold`, or None if none does."""  # => co-23: documents first_warning_epoch's contract -- no runtime output, just sets its __doc__
    for snapshot in curve:  # => co-23: walk the curve IN epoch order, exactly as a real training loop would observe it live
        gap = snapshot.train_pass_rate - snapshot.val_pass_rate  # => co-23: this epoch's train/val gap
        if gap > threshold:  # => co-23: the FIRST epoch where the gap crosses the line is the earliest actionable warning
            return snapshot.epoch  # => co-23: returns this computed value to the caller
    return None  # => co-23: no epoch in this curve crossed the threshold


def best_val_epoch(curve: list[EpochSnapshot]) -> int:  # => co-23,co-24: which epoch's checkpoint a real run should actually keep
    """Return the epoch number with the highest `val_pass_rate` in `curve`."""  # => co-23: documents best_val_epoch's contract -- no runtime output, just sets its __doc__
    return max(curve, key=lambda snapshot: snapshot.val_pass_rate).epoch  # => co-23: returns this computed value to the caller


if __name__ == "__main__":  # => co-23: entry point -- runs only when this file executes directly, not on import
    for snapshot in TRAINING_CURVE:  # => co-23: print the curve exactly as a monitoring dashboard would show it, epoch by epoch
        gap = snapshot.train_pass_rate - snapshot.val_pass_rate  # => co-23: this epoch's gap, for display
        print(f"  epoch {snapshot.epoch}: train {snapshot.train_pass_rate:.0%} | val {snapshot.val_pass_rate:.0%} | gap {gap:+.0%}")  # => co-23
    warning_epoch = first_warning_epoch(TRAINING_CURVE, EARLY_WARNING_GAP_THRESHOLD)  # => co-23: run the LIVE detector
    print(f"First epoch crossing the {EARLY_WARNING_GAP_THRESHOLD:.0%} gap threshold: epoch {warning_epoch}")  # => co-23
    assert warning_epoch == 4, "the widening gap must first cross the threshold at epoch 4 in this scenario, one epoch after the true val peak"  # => co-23,co-24
    peak_epoch = best_val_epoch(TRAINING_CURVE)  # => co-23,co-24: which epoch's weights are actually best
    print(f"Best validation epoch: {peak_epoch}")  # => co-23
    assert peak_epoch == 3, "the true validation peak must land one epoch BEFORE the gap crosses the warning threshold"  # => co-23,co-24
    print("MATCH: monitoring the train/val gap epoch-by-epoch flags trouble by epoch 4 -- one epoch after the peak, but four epochs before training would normally stop")  # => co-23,co-24
    # => co-23,co-24: this is the LIVE version of what ex-39 only shows in hindsight -- catching the gap as it opens is what makes ex-40's early stopping possible
