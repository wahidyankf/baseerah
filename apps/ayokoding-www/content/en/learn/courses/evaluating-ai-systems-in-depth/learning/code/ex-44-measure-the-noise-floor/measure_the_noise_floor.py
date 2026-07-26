"""Worked Example 44: Run an Unchanged Suite Repeatedly to Establish Its Own Score Variance."""  # => co-24: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import statistics  # => co-24: stdlib mean/stdev -- no external dependency needed for a noise-floor measurement

# The SAME suite, on the SAME unchanged code, run five separate times. Non-determinism in the
# generator model (sampling temperature, etc.) makes the pass rate wobble even with zero changes.
REPEATED_RUN_PASS_RATES = (0.84, 0.88, 0.86, 0.90, 0.82)  # => co-24: five real pass rates from five repeated runs of one unchanged suite


def measure_noise_floor(repeated_pass_rates: tuple[float, ...]) -> tuple[float, float]:  # => co-24: turns raw repeated-run data into a (mean, spread) noise-floor summary
    """Return `(mean, standard_deviation)` of `repeated_pass_rates` -- the suite's own inherent wobble."""  # => co-24: documents measure_noise_floor's contract -- no runtime output, just sets its __doc__
    mean = statistics.mean(repeated_pass_rates)  # => co-24: the central pass rate across repeated, unchanged runs
    spread = statistics.stdev(repeated_pass_rates)  # => co-24: how much a single run typically deviates from that mean
    return mean, spread  # => co-24: returns this computed value to the caller


if __name__ == "__main__":  # => co-24: entry point -- runs only when this file executes directly, not on import
    mean_rate, noise_floor = measure_noise_floor(REPEATED_RUN_PASS_RATES)  # => co-24: compute the suite's own noise floor
    print(f"Repeated pass rates on an UNCHANGED suite: {REPEATED_RUN_PASS_RATES}")  # => co-24: prints the raw repeated-run data
    print(f"Mean pass rate: {mean_rate:.1%}")  # => co-24: prints the mean
    print(f"Noise floor (standard deviation): {noise_floor:.1%}")  # => co-24: prints the measured noise floor

    assert 0.80 <= mean_rate <= 0.90, "the mean pass rate across repeated runs must land in a plausible range for this fixture"  # => co-24
    assert noise_floor > 0.0, "an inherently non-deterministic suite must show a NONZERO noise floor across repeated, unchanged runs"  # => co-24: the rule this example proves
    print(f"MATCH: running the SAME unchanged suite {len(REPEATED_RUN_PASS_RATES)} times yields a mean of {mean_rate:.1%} with a {noise_floor:.1%} noise floor -- score wobble that exists with zero real changes")  # => co-24
    # => co-24: ex-45 next sets a regression bar ABOVE this measured noise floor, so a within-noise wobble never wrongly blocks a merge
