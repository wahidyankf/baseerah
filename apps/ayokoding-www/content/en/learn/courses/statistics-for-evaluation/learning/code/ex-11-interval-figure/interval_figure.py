"""Worked Example 11: A Pass-Rate Figure With Intervals."""  # => co-01: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from statsmodels.stats.proportion import proportion_confint  # => co-04: the pinned library's own binomial-interval function

VARIANTS = {  # => co-24: three prompt variants, each with its own pass count and its own n -- eval sets are rarely all the same size
    "prompt-v1 (baseline)": (30, 40),  # => co-24: (passes, n) -- a genuinely small pilot eval set
    "prompt-v2": (36, 40),  # => co-24: same n as the baseline -- directly comparable
    "prompt-v3": (58, 70),  # => co-24: a DIFFERENT n -- collected later, on a larger dataset
}  # => co-24: closes VARIANTS


def render_row(name: str, passes: int, n: int) -> str:  # => co-01: renders ONE labeled row of the figure -- estimate, bar, interval, n, method
    """Return one printable row: the point estimate, an ASCII bar, and the labeled Wilson interval."""  # => co-01: documents render_row's contract -- no runtime output, just sets its __doc__
    p_hat = passes / n  # => co-04: this variant's own point estimate
    lo, hi = proportion_confint(passes, n, method="wilson")  # => co-05: Wilson -- the small-n-appropriate method every row uses, uniformly
    bar = "#" * round(p_hat * 40)  # => co-01: a plain-text bar proportional to the point estimate
    return f"{name:<22} | {bar:<40} | {p_hat:.2%}  CI=[{lo:.2%}, {hi:.2%}]  n={n}  method=wilson"  # => co-24: EVERY field the reporting discipline demands, in one row


if __name__ == "__main__":  # => co-01: entry point -- runs only when this file executes directly, not on import
    print("Pass rate by prompt variant (95% Wilson interval, labeled with n and method):")  # => co-24: states the method up front, once, for the whole figure
    rows = [render_row(name, passes, n) for name, (passes, n) in VARIANTS.items()]  # => co-01: one rendered row per variant
    for row in rows:  # => co-01: prints the whole figure, one labeled row per variant
        print(row)  # => co-01: the figure itself -- this IS the artifact a reader would see
    for row in rows:  # => co-24: every row must carry n and method, never just a bar and a percentage
        assert "n=" in row and "method=wilson" in row, "every row must state its own n and its own method"  # => co-24
    print("MATCH: every bar is labeled with its own point estimate, interval, n, and method -- no row is a bare number")  # => co-01
    # => co-01,co-24: a figure of bars with no interval invites reading noise as a real ranking; this figure structurally cannot omit the interval
