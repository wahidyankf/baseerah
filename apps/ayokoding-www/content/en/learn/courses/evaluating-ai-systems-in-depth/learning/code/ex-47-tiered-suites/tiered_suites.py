"""Worked Example 47: A Fast Deterministic Tier Per Commit, a Slower Judged Tier on Merge."""  # => co-26: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-26: EvalTier is a typed record describing one tier's own cost/runtime profile


class EvalTier(NamedTuple):  # => co-26: one tier of the eval suite -- distinct scope, cost, and trigger
    name: str  # => co-26: the tier's own name
    case_count: int  # => co-26: how many cases run in this tier
    uses_llm_judge: bool  # => co-25: whether this tier calls a judge model at all -- the main cost driver
    estimated_runtime_seconds: float  # => co-26: how long this tier takes to run
    runs_on: str  # => co-26: WHEN this tier triggers -- "every-commit" or "pre-merge"


FAST_TIER = EvalTier(name="fast-deterministic", case_count=40, uses_llm_judge=False, estimated_runtime_seconds=8.0, runs_on="every-commit")  # => co-26: exact-match and reference-based scorers only -- cheap enough to run constantly
JUDGED_TIER = EvalTier(name="llm-judged", case_count=20, uses_llm_judge=True, estimated_runtime_seconds=95.0, runs_on="pre-merge")  # => co-26: fewer cases, but each needs a judge call -- reserved for merge time


def choose_tiers_for_trigger(trigger: str, tiers: tuple[EvalTier, ...]) -> tuple[EvalTier, ...]:  # => co-26: routes a CI trigger to the RIGHT subset of tiers, not always everything
    """Return the tiers in `tiers` whose `runs_on` matches `trigger`, where "pre-merge" also includes "every-commit" tiers."""  # => co-26: documents choose_tiers_for_trigger's contract -- no runtime output, just sets its __doc__
    if trigger == "every-commit":  # => co-26: a routine commit -- only the fast tier runs
        return tuple(t for t in tiers if t.runs_on == "every-commit")  # => co-26: returns this computed value to the caller
    return tiers  # => co-26: a pre-merge trigger runs EVERYTHING -- fast tier plus judged tier


if __name__ == "__main__":  # => co-26: entry point -- runs only when this file executes directly, not on import
    all_tiers = (FAST_TIER, JUDGED_TIER)  # => co-26: the full, two-tier suite definition
    commit_tiers = choose_tiers_for_trigger("every-commit", all_tiers)  # => co-26: what runs on an ordinary commit
    merge_tiers = choose_tiers_for_trigger("pre-merge", all_tiers)  # => co-26: what runs before a merge
    commit_runtime = sum(t.estimated_runtime_seconds for t in commit_tiers)  # => co-26: total runtime for the commit-time trigger
    merge_runtime = sum(t.estimated_runtime_seconds for t in merge_tiers)  # => co-26: total runtime for the pre-merge trigger
    print(f"Every-commit tiers: {[t.name for t in commit_tiers]}, runtime: {commit_runtime:.0f}s")  # => co-26: prints the commit-time selection
    print(f"Pre-merge tiers: {[t.name for t in merge_tiers]}, runtime: {merge_runtime:.0f}s")  # => co-26: prints the pre-merge selection

    assert commit_tiers == (FAST_TIER,), "every-commit must run ONLY the fast, deterministic tier -- no judge calls on every commit"  # => co-26: the rule this example proves
    assert JUDGED_TIER in merge_tiers, "the judged tier must run before a merge, catching what the fast tier structurally cannot"  # => co-26: the rule this example proves
    assert merge_runtime > commit_runtime, "the pre-merge trigger, running both tiers, must take longer than the every-commit trigger running only the fast one"  # => co-26
    print(f"MATCH: every commit runs only the {commit_runtime:.0f}s fast tier; merging additionally runs the {JUDGED_TIER.estimated_runtime_seconds:.0f}s judged tier -- {merge_runtime:.0f}s total")  # => co-26
    # => co-26: ex-48 next puts an explicit dollar BUDGET on the judged tier's own judge calls, since those are what cost real money
