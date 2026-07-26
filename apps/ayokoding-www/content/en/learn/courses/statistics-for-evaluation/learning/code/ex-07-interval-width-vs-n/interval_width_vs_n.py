"""Worked Example 7: Interval Width vs. n."""  # => co-04: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from statsmodels.stats.proportion import proportion_confint  # => co-04: the pinned library's own binomial-interval function

PASS_RATE = 0.85  # => co-06: held fixed across every n below -- only the sample size changes
SAMPLE_SIZES = (10, 20, 40, 80, 160, 320, 640)  # => co-06: doubling each step, spanning a small eval set to a large one


if __name__ == "__main__":  # => co-06: entry point -- runs only when this file executes directly, not on import
    widths: list[float] = []  # => co-04: one Wilson-interval width per sample size, for the diminishing-return check below
    for n in SAMPLE_SIZES:  # => co-06: one interval computation per candidate sample size
        passes = round(PASS_RATE * n)  # => co-06: keep the observed rate fixed near 0.85 at every n
        lo, hi = proportion_confint(passes, n, method="wilson")  # => co-04: the well-behaved small-n interval from ex-06
        width = hi - lo  # => co-04: the interval's full width -- what a reader actually cares about
        widths.append(width)  # => co-04: recorded for the shrink-rate check below
        bar = "#" * round(width * 200)  # => co-04: a plain-text bar -- longer bar means a wider, less informative interval
        print(f"n={n:>4} | width={width:.4f} | {bar}")  # => co-04: prints size, width, and its ASCII-bar visualization
    assert widths[0] > widths[-1], "interval width must shrink as n grows"  # => co-04: the qualitative claim
    halving_index = next(i for i in range(1, len(widths)) if widths[i] < widths[0] / 2)  # => co-06: where width first drops below half its n=10 value
    print(f"Width first drops below half its n=10 value at n={SAMPLE_SIZES[halving_index]}")  # => co-06: names the diminishing-return point
    last_gain = widths[-2] - widths[-1]  # => co-06: the width saved by the LAST doubling (320 -> 640)
    first_gain = widths[0] - widths[1]  # => co-06: the width saved by the FIRST doubling (10 -> 20)
    print(f"Width saved by 10->20: {first_gain:.4f} | width saved by 320->640: {last_gain:.4f}")  # => co-06: shows diminishing returns
    assert first_gain > last_gain, "the first doubling must buy more width reduction than the last doubling"  # => co-06: the diminishing-return claim itself
    print("MATCH: each doubling of n buys a shrinking amount of extra precision -- diminishing, not free")  # => co-06
    # => co-04,co-06: this shape is WHY ex-08 asks 'how many cases for THIS effect' instead of 'as many cases as possible'
