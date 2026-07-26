"""Worked Example 8: How Many Cases Do I Need For This Effect."""  # => co-06: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import math  # => co-06: ceil -- a sample size must be a whole number of cases

from statsmodels.stats.proportion import proportion_confint, samplesize_confint_proportion  # => co-06: the pinned library's sample-size solver

ANTICIPATED_RATE = 0.85  # => co-06: the pass rate the team expects, from a pilot run or a prior eval
TARGET_HALF_WIDTH = 0.05  # => co-06: the precision demanded -- "I want the interval within +/-5 points"


if __name__ == "__main__":  # => co-06: entry point -- runs only when this file executes directly, not on import
    raw_n = samplesize_confint_proportion(proportion=ANTICIPATED_RATE, half_length=TARGET_HALF_WIDTH, alpha=0.05, method="normal")  # => co-06: solves the CI-width formula for n
    print(f"Solved n (continuous): {raw_n:.2f}")  # => co-06: the exact real-valued solution to the width equation
    required_n = math.ceil(raw_n)  # => co-06: round UP -- a fractional case cannot be collected, and rounding down would miss the target
    print(f"Required n (rounded up to a whole case count): {required_n}")  # => co-06: the number a team actually plans to collect
    assert required_n == 196, "the required n for this anticipated rate and target half-width must be 196"  # => co-06: pins the exact planning number

    passes_at_required_n = round(ANTICIPATED_RATE * required_n)  # => co-06: what collecting exactly required_n cases would look like
    lo, hi = proportion_confint(passes_at_required_n, required_n, method="normal")  # => co-06: the SAME normal method this n was solved for
    achieved_half_width = (hi - lo) / 2  # => co-06: verify the PLANNED n actually delivers the target precision
    print(f"Achieved half-width at n={required_n}: {achieved_half_width:.4f} (target was {TARGET_HALF_WIDTH})")  # => co-06
    assert achieved_half_width <= TARGET_HALF_WIDTH + 1e-3, "collecting the solved n must actually achieve the target half-width"  # => co-06: closes the loop

    smaller_n = required_n - 100  # => co-06: a plausible temptation -- "let's just collect fewer cases"
    lo_small, hi_small = proportion_confint(round(ANTICIPATED_RATE * smaller_n), smaller_n, method="normal")  # => co-06: what a shortcut n actually buys
    print(f"With only n={smaller_n}, half-width would be: {(hi_small - lo_small) / 2:.4f} -- misses the {TARGET_HALF_WIDTH} target")  # => co-06
    assert (hi_small - lo_small) / 2 > TARGET_HALF_WIDTH, "an under-sized n must miss the precision target"  # => co-06: names the shortcut's real cost
    print("MATCH: the required n is a design question answered BEFORE collecting, not a guess made after")  # => co-06
    # => co-06: this is the number a sampling plan (ex-20) states and justifies, in writing, before a single case is drawn
