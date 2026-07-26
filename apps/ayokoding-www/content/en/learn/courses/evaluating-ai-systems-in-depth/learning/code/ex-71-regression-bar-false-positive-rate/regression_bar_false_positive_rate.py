"""Worked Example 71: Measure How Often a Badly-Set Regression Bar Produces a False-Positive Block."""  # => co-24: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

# Twenty repeated runs of an UNCHANGED suite -- every "regression" this bar flags here is
# necessarily a false positive, since nothing actually changed between runs.
REPEATED_UNCHANGED_RUNS = (0.84, 0.88, 0.86, 0.90, 0.82, 0.85, 0.87, 0.83, 0.89, 0.86, 0.84, 0.91, 0.85, 0.87, 0.82, 0.88, 0.86, 0.84, 0.89, 0.85)  # => co-24: 20 genuine repeated-run pass rates on a totally unchanged suite

TOO_TIGHT_BAR = 0.855  # => co-23: a bar set carelessly close to the mean -- looks "strict" but is not derived from measured noise
PROPERLY_DERIVED_BAR = 0.796  # => co-23: ex-45's actual noise-derived bar (86.0% - 2 * 3.2%), reused here for direct comparison


def false_positive_rate(runs: tuple[float, ...], bar: float) -> float:  # => co-24: the fraction of UNCHANGED runs a given bar would have wrongly blocked
    """Return the fraction of `runs` that fall below `bar`, given that `runs` all came from an unchanged suite."""  # => co-24: documents false_positive_rate's contract -- no runtime output, just sets its __doc__
    blocked = sum(1 for r in runs if r < bar)  # => co-24: every one of these is, by construction, a false positive
    return blocked / len(runs)  # => co-24: returns this computed value to the caller


if __name__ == "__main__":  # => co-24: entry point -- runs only when this file executes directly, not on import
    too_tight_rate = false_positive_rate(REPEATED_UNCHANGED_RUNS, TOO_TIGHT_BAR)  # => co-24: how often the carelessly-tight bar wrongly blocks
    derived_rate = false_positive_rate(REPEATED_UNCHANGED_RUNS, PROPERLY_DERIVED_BAR)  # => co-24: how often the properly-derived bar wrongly blocks
    print(f"Too-tight bar ({TOO_TIGHT_BAR:.1%}) false-positive rate across {len(REPEATED_UNCHANGED_RUNS)} unchanged runs: {too_tight_rate:.0%}")  # => co-24
    print(f"Properly-derived bar ({PROPERLY_DERIVED_BAR:.1%}) false-positive rate: {derived_rate:.0%}")  # => co-24

    assert too_tight_rate > 0.0, "a bar set too close to the mean, without deriving it from measured noise, must produce SOME false positives on unchanged runs"  # => co-23: the rule this example proves
    assert derived_rate == 0.0, "a bar properly derived from the measured noise floor must produce ZERO false positives across these same unchanged runs"  # => co-24: the rule this example proves
    assert too_tight_rate > derived_rate, "the carelessly-tight bar must false-positive strictly more often than the noise-derived one"  # => co-23
    print(  # => co-24: opens the final MATCH print, reached only if all three asserts above passed
        f"MATCH: the too-tight bar wrongly blocks {too_tight_rate:.0%} of unchanged runs, while the noise-derived bar blocks {derived_rate:.0%} -- deriving the bar from measured noise is not optional polish, it is the mechanism that prevents false-positive blocks"
    )  # => co-24
    # => co-24: ex-72 next moves from the regression bar itself to budgeting the TIME each tier is allowed to consume
