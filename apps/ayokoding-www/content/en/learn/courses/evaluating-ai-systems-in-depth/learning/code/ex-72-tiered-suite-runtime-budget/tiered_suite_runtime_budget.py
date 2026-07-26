"""Worked Example 72: Enforce an Explicit Runtime Budget Per Tier, Not Just an Overall CI Timeout."""  # => co-26: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-26: TierRuntimeCheck is a typed record -- one tier's own budget compliance


class TierRuntimeCheck(NamedTuple):  # => co-26: one tier's measured runtime against its OWN budget, not a shared overall figure
    tier_name: str  # => co-26: which tier this check covers
    measured_seconds: float  # => co-26: how long this tier actually took
    budget_seconds: float  # => co-26: this tier's own ceiling
    within_budget: bool  # => co-26: whether this specific tier stayed within ITS OWN budget


FAST_TIER_BUDGET_SECONDS = 15.0  # => co-26: the fast, every-commit tier must stay well under 15s to keep commits fast
JUDGED_TIER_BUDGET_SECONDS = 120.0  # => co-26: the judged, pre-merge tier gets a much larger budget, but still a real ceiling


def check_tier_runtime(tier_name: str, measured_seconds: float, budget_seconds: float) -> TierRuntimeCheck:  # => co-26: checks ONE tier's own measured runtime against ITS OWN budget
    """Return a `TierRuntimeCheck` comparing `measured_seconds` to `budget_seconds` for `tier_name`."""  # => co-26: documents check_tier_runtime's contract -- no runtime output, just sets its __doc__
    return TierRuntimeCheck(tier_name=tier_name, measured_seconds=measured_seconds, budget_seconds=budget_seconds, within_budget=measured_seconds <= budget_seconds)  # => co-26: returns this computed value to the caller


if __name__ == "__main__":  # => co-26: entry point -- runs only when this file executes directly, not on import
    fast_tier_check = check_tier_runtime("fast-deterministic", measured_seconds=8.0, budget_seconds=FAST_TIER_BUDGET_SECONDS)  # => co-26: fast tier ran well within budget
    judged_tier_check = check_tier_runtime("llm-judged", measured_seconds=145.0, budget_seconds=JUDGED_TIER_BUDGET_SECONDS)  # => co-26: judged tier BLEW its own budget, even though it is allowed more time
    print(f"{fast_tier_check.tier_name}: {fast_tier_check.measured_seconds:.0f}s / {fast_tier_check.budget_seconds:.0f}s budget, within budget: {fast_tier_check.within_budget}")  # => co-26
    print(f"{judged_tier_check.tier_name}: {judged_tier_check.measured_seconds:.0f}s / {judged_tier_check.budget_seconds:.0f}s budget, within budget: {judged_tier_check.within_budget}")  # => co-26

    assert fast_tier_check.within_budget is True, "the fast tier must stay within its own tight 15-second budget"  # => co-26: the rule this example proves
    assert judged_tier_check.within_budget is False, (  # => co-26: opens the second assert's multi-line message
        "the judged tier, despite a 120-second budget, must be flagged when it takes 145 seconds -- a per-tier budget, not just a giant shared CI timeout, catches this"  # => co-26: the assertion message itself
    )  # => co-26: the rule this example proves
    print(  # => co-26: opens the final MATCH print, reached only if both asserts above passed
        f"MATCH: each tier is checked against ITS OWN runtime budget -- the fast tier passes at {fast_tier_check.measured_seconds:.0f}s, while the judged tier is flagged at {judged_tier_check.measured_seconds:.0f}s despite having 8x the fast tier's budget"
    )  # => co-26
    # => co-26: ex-73 next turns a blocked CI run into an ANNOTATED failure report, not just a bare pass/fail exit code
