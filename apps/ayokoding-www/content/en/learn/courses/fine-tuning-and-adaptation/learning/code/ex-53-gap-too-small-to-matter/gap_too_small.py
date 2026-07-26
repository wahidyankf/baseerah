# learning/code/ex-53-gap-too-small-to-matter/gap_too_small.py
"""Worked Example 53: A Gap Too Small to Matter."""  # => co-01: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

MINIMUM_GAP_WORTH_FIXING = 0.03  # => co-06: below a 3-point gap, the cost of ANY remedy (ex-13's total-cost lesson) outweighs the benefit

CURRENT_BASELINE_RATE = 0.955  # => co-06: the assistant's current greeting-style compliance rate, measured on a 40-case eval
TARGET_RATE = 0.97  # => co-06: the style guide's stated aspiration -- not a hard requirement anyone has enforced


def gap_size(current: float, target: float) -> float:  # => co-06: the actual, measured distance -- not a feeling
    """Return the absolute gap between `current` and `target`."""  # => co-06: documents gap_size's contract -- no runtime output, just sets its __doc__
    return abs(target - current)  # => co-06: distance, not direction -- either side of target counts as a gap


if __name__ == "__main__":  # => co-06: entry point -- runs only when this file executes directly, not on import
    measured_gap = gap_size(CURRENT_BASELINE_RATE, TARGET_RATE)  # => co-06: measure BEFORE proposing any remedy, per ex-01's discipline
    print(f"Current: {CURRENT_BASELINE_RATE:.1%} | Target: {TARGET_RATE:.1%} | Measured gap: {measured_gap:.1%}")  # => co-06
    print(f"Minimum gap considered worth fixing: {MINIMUM_GAP_WORTH_FIXING:.1%}")  # => co-06: the actual threshold, stated up front
    worth_fixing = measured_gap >= MINIMUM_GAP_WORTH_FIXING  # => co-06: the decision this specific check makes
    print(f"Worth fixing at all: {worth_fixing}")  # => co-06
    assert measured_gap < MINIMUM_GAP_WORTH_FIXING, "this scenario's gap must fall below the minimum-worth-fixing threshold"  # => co-06
    assert not worth_fixing, "a sub-threshold gap must produce a documented decision to do NOTHING"  # => co-06
    print("MATCH: the gap is real but too small to justify ANY remedy -- prompting, retrieval, scoping, or a fine-tune")  # => co-06
    # => co-01,co-06: measuring the gap first (ex-01) sometimes means measuring that there is nothing worth doing -- "do nothing" is a valid, documented outcome of this gate too
