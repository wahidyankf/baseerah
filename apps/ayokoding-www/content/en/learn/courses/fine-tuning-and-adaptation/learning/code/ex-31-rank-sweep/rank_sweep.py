# learning/code/ex-31-rank-sweep/rank_sweep.py
"""Worked Example 31: Rank Sweep."""  # => co-20: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-20: one row per swept rank, one comment per row


class RankResult(NamedTuple):  # => co-20: (rank, param_count, eval_pass_rate) -- the capacity/quality trade-off curve
    rank: int  # => co-20: the adapter rank tried
    param_count: int  # => co-20: how many trainable parameters this rank produces
    eval_pass_rate: float  # => co-20: the resulting model's measured pass rate on ex-08's fixed eval


NUM_TARGET_MATRICES = 48  # => co-19: identical to ex-29
HIDDEN_DIM = 896  # => co-19: identical to ex-29

# => co-20: a sweep across seven ranks -- the SAME eval, SAME dataset, ONLY rank varies
SWEEP: list[RankResult] = [  # => co-20: one row per rank, ruff-format-stable (one item per line, own comment)
    RankResult(rank=1, param_count=NUM_TARGET_MATRICES * 2 * 1 * HIDDEN_DIM, eval_pass_rate=0.78),  # => co-20: too little capacity
    RankResult(rank=2, param_count=NUM_TARGET_MATRICES * 2 * 2 * HIDDEN_DIM, eval_pass_rate=0.85),  # => co-20: meaningful lift
    RankResult(rank=4, param_count=NUM_TARGET_MATRICES * 2 * 4 * HIDDEN_DIM, eval_pass_rate=0.90),  # => co-20: still climbing
    RankResult(rank=8, param_count=NUM_TARGET_MATRICES * 2 * 8 * HIDDEN_DIM, eval_pass_rate=0.94),  # => co-20: ex-29's chosen rank
    RankResult(rank=16, param_count=NUM_TARGET_MATRICES * 2 * 16 * HIDDEN_DIM, eval_pass_rate=0.95),  # => co-20: diminishing returns begin
    RankResult(rank=32, param_count=NUM_TARGET_MATRICES * 2 * 32 * HIDDEN_DIM, eval_pass_rate=0.955),  # => co-20: nearly flat now
    RankResult(rank=64, param_count=NUM_TARGET_MATRICES * 2 * 64 * HIDDEN_DIM, eval_pass_rate=0.955),  # => co-20: fully plateaued -- no more lift, only more size
]  # => co-20: closes SWEEP


if __name__ == "__main__":  # => co-20: entry point -- runs only when this file executes directly, not on import
    for row in SWEEP:  # => co-20: print the whole capacity/quality curve
        print(f"  rank={row.rank:>2}: {row.param_count:>9,} params -> {row.eval_pass_rate:.1%} pass rate")  # => co-20
    best_lift_per_rank_doubling = SWEEP[3].eval_pass_rate - SWEEP[2].eval_pass_rate  # => co-20: r=4 -> r=8's lift
    plateau_lift = SWEEP[6].eval_pass_rate - SWEEP[5].eval_pass_rate  # => co-20: r=32 -> r=64's lift -- should be near zero
    print(f"Lift from r=4 to r=8: +{best_lift_per_rank_doubling:.1%} | Lift from r=32 to r=64: +{plateau_lift:.1%}")  # => co-20
    assert best_lift_per_rank_doubling > plateau_lift * 3, "the early ranks must show far more lift per doubling than the plateaued high ranks"  # => co-20
    assert SWEEP[6].param_count > SWEEP[3].param_count * 7, "r=64 must cost far more parameters than r=8 despite the near-zero extra lift"  # => co-20
    print("MATCH: capacity keeps growing linearly with rank, but quality plateaus -- rank 8 is where THIS gap's trade-off curve bends")  # => co-20
    # => co-20: this sweep is what justifies ex-29's r=8 choice with a curve, not a default -- the capstone reuses exactly this discipline
