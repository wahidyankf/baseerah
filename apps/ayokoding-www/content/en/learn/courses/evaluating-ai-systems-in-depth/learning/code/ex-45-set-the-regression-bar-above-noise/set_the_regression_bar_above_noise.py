"""Worked Example 45: Set a Regression Bar From the Measured Noise Floor -- Not From a Guess."""  # => co-24: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

MEASURED_NOISE_FLOOR = 0.032  # => co-24: the standard deviation ex-44 measured across five repeated, unchanged runs (3.2%)
BASELINE_PASS_RATE = 0.86  # => co-24: the mean pass rate ex-44 measured on the unchanged suite

NOISE_MULTIPLE = 2.0  # => co-23: how many noise-floor units below baseline a change must fall before it counts as a REAL regression, not wobble


def set_regression_bar(baseline: float, noise_floor: float, *, multiple: float = NOISE_MULTIPLE) -> float:  # => co-23: derives the bar FROM measured noise, never a round-number guess
    """Return the pass-rate floor below which a run counts as a real regression -- `baseline - multiple * noise_floor`."""  # => co-23: documents set_regression_bar's contract -- no runtime output, just sets its __doc__
    return baseline - multiple * noise_floor  # => co-23: returns this computed value to the caller


def is_real_regression(observed_pass_rate: float, *, bar: float) -> bool:  # => co-23: the CI gate's own decision function
    """Pass (return True) iff `observed_pass_rate` falls below `bar` -- a genuine regression, not noise."""  # => co-23: documents is_real_regression's contract -- no runtime output, just sets its __doc__
    return observed_pass_rate < bar  # => co-23: returns this computed value to the caller


if __name__ == "__main__":  # => co-23: entry point -- runs only when this file executes directly, not on import
    bar = set_regression_bar(BASELINE_PASS_RATE, MEASURED_NOISE_FLOOR)  # => co-23: derive the bar from ex-44's measured noise floor
    within_noise_run = 0.83  # => co-24: a run that dipped, but is still WITHIN the measured noise floor -- ordinary wobble
    real_regression_run = 0.76  # => co-23: a run that dipped FAR below the noise floor -- a genuine regression
    print(f"Baseline: {BASELINE_PASS_RATE:.1%}, noise floor: {MEASURED_NOISE_FLOOR:.1%}, regression bar: {bar:.1%}")  # => co-23: prints the derived bar

    verdict_within_noise = is_real_regression(within_noise_run, bar=bar)  # => co-23: should NOT be flagged -- just wobble
    verdict_real_regression = is_real_regression(real_regression_run, bar=bar)  # => co-23: SHOULD be flagged -- a real drop
    print(f"Run at {within_noise_run:.1%} (within noise): flagged as regression = {verdict_within_noise}")  # => co-23
    print(f"Run at {real_regression_run:.1%} (real drop): flagged as regression = {verdict_real_regression}")  # => co-23

    assert verdict_within_noise is False, "a run that only dips within the measured noise floor must NOT be flagged -- it never wrongly blocks a merge"  # => co-23: the rule this example proves
    assert verdict_real_regression is True, "a run that dips far below the noise floor must BE flagged as a genuine regression"  # => co-23: the rule this example proves
    print(f"MATCH: the regression bar ({bar:.1%}), derived from the MEASURED noise floor, clears the within-noise run and correctly flags the real regression")  # => co-23
    # => co-23: ex-46 next wires this exact bar into a CI gate that actually blocks a merge when a real regression is detected
