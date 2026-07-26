# learning/code/ex-39-overfitting-invisible-in-training-loss/overfitting_invisible_in_loss.py
"""Worked Example 39: Overfitting Invisible in Training Loss."""  # => co-23: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-23: one immutable row per epoch, training loss and held-out score recorded TOGETHER


class LossSnapshot(NamedTuple):  # => co-23: what the training loop's own loss curve says, versus what a held-out check would say
    epoch: int  # => co-23: which epoch this snapshot is from
    training_loss: float  # => co-23: the loss the optimizer is minimizing -- computed ONLY on training data
    held_out_pass_rate: float  # => co-15: pass rate on a split the model never trains on -- the only honest signal


# => co-23,co-15: the same underlying overfitting run as ex-67, this time read through TRAINING LOSS instead of the val gap
LOSS_CURVE: list[LossSnapshot] = [  # => co-23: per-epoch snapshots, in order
    LossSnapshot(epoch=1, training_loss=1.82, held_out_pass_rate=0.70),  # => co-23: epoch 1 -- loss falling, held-out rising, both look healthy
    LossSnapshot(epoch=2, training_loss=1.10, held_out_pass_rate=0.84),  # => co-23: epoch 2 -- loss keeps falling, held-out keeps rising
    LossSnapshot(epoch=3, training_loss=0.61, held_out_pass_rate=0.91),  # => co-23: epoch 3 -- loss still falling, held-out at its PEAK
    LossSnapshot(epoch=4, training_loss=0.29, held_out_pass_rate=0.88),  # => co-23: epoch 4 -- loss STILL falling, held-out has started falling
    LossSnapshot(epoch=5, training_loss=0.08, held_out_pass_rate=0.83),  # => co-23: epoch 5 -- loss at its BEST-LOOKING value, held-out down 8 points from peak
]  # => co-23: closes LOSS_CURVE


def loss_says_improving(curve: list[LossSnapshot]) -> bool:  # => co-23: what a loss-only dashboard would report at the final epoch
    """Return whether `training_loss` strictly decreases across every consecutive pair of epochs in `curve`."""  # => co-23: documents loss_says_improving's contract -- no runtime output, just sets its __doc__
    return all(curve[i].training_loss < curve[i - 1].training_loss for i in range(1, len(curve)))  # => co-23: monotonic decrease, epoch over epoch


def held_out_peak_epoch(curve: list[LossSnapshot]) -> int:  # => co-15: the epoch a held-out check would have flagged as best
    """Return the epoch number with the highest `held_out_pass_rate` in `curve`."""  # => co-15: documents held_out_peak_epoch's contract -- no runtime output, just sets its __doc__
    return max(curve, key=lambda snapshot: snapshot.held_out_pass_rate).epoch  # => co-15: returns this computed value to the caller


if __name__ == "__main__":  # => co-23: entry point -- runs only when this file executes directly, not on import
    for snapshot in LOSS_CURVE:  # => co-23: print exactly what a training log would show, epoch by epoch
        print(f"  epoch {snapshot.epoch}: training loss {snapshot.training_loss:.2f} | held-out pass rate {snapshot.held_out_pass_rate:.0%}")  # => co-23
    loss_verdict = loss_says_improving(LOSS_CURVE)  # => co-23: what the loss curve ALONE would conclude
    print(f"Training loss says 'still improving' at every epoch: {loss_verdict}")  # => co-23
    assert loss_verdict, "training loss must decrease monotonically across all 5 epochs in this scenario -- that is exactly the trap"  # => co-23
    peak_epoch = held_out_peak_epoch(LOSS_CURVE)  # => co-15: what the held-out check ACTUALLY shows
    print(f"Held-out pass rate actually peaks at epoch {peak_epoch}, then falls for two more epochs")  # => co-15
    assert peak_epoch == 3, "the held-out peak must land at epoch 3, two epochs before the loss curve's own best-looking point"  # => co-15,co-23
    final_epoch_loss_looks_best = LOSS_CURVE[-1].training_loss == min(s.training_loss for s in LOSS_CURVE)  # => co-23: is the FINAL epoch's loss the best of the whole run
    assert final_epoch_loss_looks_best, "the final epoch must have the lowest training loss of the whole run, even though it is NOT the best model"  # => co-23
    print("MATCH: training loss recommends epoch 5 -- the worst held-out result of the run -- because loss alone cannot see memorization happening")  # => co-15,co-23
    # => co-15,co-23: this is why co-15's held-out discipline exists -- a metric computed only on training data will always look like it is improving
