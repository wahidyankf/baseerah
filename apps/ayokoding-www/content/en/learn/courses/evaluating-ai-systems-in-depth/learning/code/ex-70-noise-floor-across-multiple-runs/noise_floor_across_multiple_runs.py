"""Worked Example 70: Check Whether the Measured Noise Floor Itself Is Stable Across Many Runs."""  # => co-24: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import statistics  # => co-24: stdlib mean/stdev, reused from ex-44's pattern

# Two SEPARATE batches of five repeated, unchanged runs each, collected a week apart. If the
# noise floor itself is stable, both batches should yield a similar standard deviation.
BATCH_ONE_PASS_RATES = (0.84, 0.88, 0.86, 0.90, 0.82)  # => co-24: the same five runs ex-44 already measured
BATCH_TWO_PASS_RATES = (0.87, 0.83, 0.89, 0.85, 0.91)  # => co-24: a second, independent batch of five repeated runs


def noise_floor(pass_rates: tuple[float, ...]) -> float:  # => co-24: the same noise-floor computation as ex-44, applied to any batch
    """Return the standard deviation of `pass_rates`."""  # => co-24: documents noise_floor's contract -- no runtime output, just sets its __doc__
    return statistics.stdev(pass_rates)  # => co-24: returns this computed value to the caller


def noise_floor_is_stable(floor_a: float, floor_b: float, *, tolerance: float = 0.02) -> bool:  # => co-24: checks that two independently-measured noise floors are close enough to trust as ONE stable figure
    """Pass iff `floor_a` and `floor_b` differ by no more than `tolerance`."""  # => co-24: documents noise_floor_is_stable's contract -- no runtime output, just sets its __doc__
    return abs(floor_a - floor_b) <= tolerance  # => co-24: returns this computed value to the caller


if __name__ == "__main__":  # => co-24: entry point -- runs only when this file executes directly, not on import
    floor_one = noise_floor(BATCH_ONE_PASS_RATES)  # => co-24: measure the noise floor from batch one
    floor_two = noise_floor(BATCH_TWO_PASS_RATES)  # => co-24: measure the noise floor from batch two, independently
    stable = noise_floor_is_stable(floor_one, floor_two)  # => co-24: check whether the two measurements agree closely enough
    print(f"Batch one noise floor: {floor_one:.1%}")  # => co-24: prints batch one's measured noise floor
    print(f"Batch two noise floor: {floor_two:.1%}")  # => co-24: prints batch two's measured noise floor
    print(f"Noise floor is stable across batches: {stable}")  # => co-24: prints the stability verdict

    assert stable is True, "two independently-measured noise floors, a week apart, must agree closely enough to trust as one stable figure for setting a regression bar"  # => co-24: the rule this example proves
    assert abs(floor_one - floor_two) < 0.02, "the two batches' noise floors must differ by less than 2 percentage points"  # => co-24
    print(f"MATCH: batch one ({floor_one:.1%}) and batch two ({floor_two:.1%}) agree closely enough that the regression bar derived from either batch is trustworthy, not a one-off artifact")  # => co-24
    # => co-24: ex-71 next asks the INVERSE question -- how often does an UNSTABLE, badly-set regression bar produce a false positive?
