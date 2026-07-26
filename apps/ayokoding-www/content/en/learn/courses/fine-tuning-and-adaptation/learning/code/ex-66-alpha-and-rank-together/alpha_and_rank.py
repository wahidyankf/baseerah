# learning/code/ex-66-alpha-and-rank-together/alpha_and_rank.py
"""Worked Example 66: Alpha and Rank Together."""  # => co-20: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-20: one immutable, self-documenting row per (rank, alpha) combination tried


class ScaledRun(NamedTuple):  # => co-20: LoRA's own update is scaled by alpha / rank, so the two never act independently
    rank: int  # => co-20: the adapter rank r, from ex-31's own sweep
    alpha: int  # => co-20: LoRA's scaling numerator -- the update is applied as (alpha / rank) * (B @ A)
    scaling_ratio: float  # => co-20: alpha / rank -- this is the number that actually governs update MAGNITUDE
    pass_rate: float  # => co-20: measured pass rate on Vantage's held-out triage cases at this (rank, alpha) pair


# => co-20: three runs holding rank FIXED at 8 while alpha varies -- this changes the scaling ratio, and the result moves with it
RANK_8_ALPHA_SWEEP: list[ScaledRun] = [  # => co-20: rank held constant, alpha varied
    ScaledRun(rank=8, alpha=8, scaling_ratio=1.0, pass_rate=0.94),  # => co-20: alpha == rank, ex-29's own original setting
    ScaledRun(rank=8, alpha=16, scaling_ratio=2.0, pass_rate=0.95),  # => co-20: doubling alpha strengthens the update
    ScaledRun(rank=8, alpha=32, scaling_ratio=4.0, pass_rate=0.89),  # => co-20: too strong -- the update overshoots and quality drops
]  # => co-20: closes RANK_8_ALPHA_SWEEP

# => co-20: three DIFFERENT (rank, alpha) pairs that all share the SAME scaling ratio of 1.0 -- do they behave alike?
MATCHED_RATIO_RUNS: list[ScaledRun] = [  # => co-20: same ratio, different rank
    ScaledRun(rank=4, alpha=4, scaling_ratio=1.0, pass_rate=0.92),  # => co-20: small rank, matched ratio
    ScaledRun(rank=8, alpha=8, scaling_ratio=1.0, pass_rate=0.94),  # => co-20: ex-31's own r=8 point, matched ratio
    ScaledRun(rank=16, alpha=16, scaling_ratio=1.0, pass_rate=0.95),  # => co-20: larger rank, matched ratio
]  # => co-20: closes MATCHED_RATIO_RUNS -- results cluster tightly (92-95%) when the ratio is held fixed


if __name__ == "__main__":  # => co-20: entry point -- runs only when this file executes directly, not on import
    print("Rank fixed at 8, alpha varied:")  # => co-20
    for run in RANK_8_ALPHA_SWEEP:  # => co-20: show how pass rate moves as ONLY alpha changes
        print(f"  rank={run.rank}, alpha={run.alpha}, ratio={run.scaling_ratio:.1f} -> pass rate {run.pass_rate:.0%}")  # => co-20
    alpha_32_regression = RANK_8_ALPHA_SWEEP[0].pass_rate - RANK_8_ALPHA_SWEEP[2].pass_rate  # => co-20: alpha=8 baseline minus alpha=32 result
    assert alpha_32_regression > 0.03, "an over-strong scaling ratio must measurably HURT pass rate versus the well-scaled baseline"  # => co-20
    print("\nRatio fixed at 1.0, rank varied:")  # => co-20
    for run in MATCHED_RATIO_RUNS:  # => co-20: show that different ranks at the SAME ratio land close together
        print(f"  rank={run.rank}, alpha={run.alpha}, ratio={run.scaling_ratio:.1f} -> pass rate {run.pass_rate:.0%}")  # => co-20
    matched_ratio_spread = max(r.pass_rate for r in MATCHED_RATIO_RUNS) - min(r.pass_rate for r in MATCHED_RATIO_RUNS)  # => co-20: spread across the matched-ratio runs
    assert matched_ratio_spread <= 0.03, "runs that share a scaling ratio must cluster tightly, even at very different rank values"  # => co-20
    print(f"\nMATCH: pass rate tracks the alpha/rank RATIO ({matched_ratio_spread:.0%} spread when matched) more than either number alone")  # => co-20
    # => co-20,co-24: sweeping rank (ex-31) or alpha in isolation hides this interaction -- the two hyperparameters must be read TOGETHER, as one ratio
