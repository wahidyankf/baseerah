"""Worked Example 48: Budget and Report Judge-Call Cost Per CI Run, Flag a Budget Breach."""  # => co-25: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-25: CostReport is a typed record -- the CI log's own cost accounting


class CostReport(NamedTuple):  # => co-25: what a CI run reports about its own judge-call spend
    judge_call_count: int  # => co-25: how many judge calls this run actually made
    cost_per_call_usd: float  # => co-25: the per-call price for the judge model in use
    total_cost_usd: float  # => co-25: the run's total judge-call spend
    budget_usd: float  # => co-25: the ceiling this run was checked against
    over_budget: bool  # => co-25: whether this run breached its own budget


BUDGET_PER_RUN_USD = 5.00  # => co-25: the fixed per-CI-run ceiling for judge-call spend
COST_PER_JUDGE_CALL_USD = 0.018  # => co-25: an illustrative per-call price for the judged tier's judge model


def build_cost_report(judge_call_count: int, *, cost_per_call: float = COST_PER_JUDGE_CALL_USD, budget: float = BUDGET_PER_RUN_USD) -> CostReport:  # => co-25: turns a raw call count into an accountable cost report
    """Return a `CostReport` for `judge_call_count` judge calls, flagging whether it exceeds `budget`."""  # => co-25: documents build_cost_report's contract -- no runtime output, just sets its __doc__
    total = judge_call_count * cost_per_call  # => co-25: the run's actual total spend
    return CostReport(  # => co-25: returns this computed value to the caller
        judge_call_count=judge_call_count,  # => co-25: echoes the input call count
        cost_per_call_usd=cost_per_call,  # => co-25: echoes the per-call price used
        total_cost_usd=total,  # => co-25: the computed total
        budget_usd=budget,  # => co-25: echoes the budget checked against
        over_budget=total > budget,  # => co-25: the flag the CI gate reads
    )  # => co-25: closes the CostReport(...) call


if __name__ == "__main__":  # => co-25: entry point -- runs only when this file executes directly, not on import
    normal_run = build_cost_report(judge_call_count=200)  # => co-25: the judged tier's ordinary call count -- 200 cases needing one judge call each
    runaway_run = build_cost_report(judge_call_count=400)  # => co-25: a run that DOUBLED its judge calls -- a retry storm or a scope creep bug
    print(f"Normal run: {normal_run.judge_call_count} calls, ${normal_run.total_cost_usd:.2f} total, over budget: {normal_run.over_budget}")  # => co-25: prints the normal run's report
    print(f"Runaway run: {runaway_run.judge_call_count} calls, ${runaway_run.total_cost_usd:.2f} total, over budget: {runaway_run.over_budget}")  # => co-25: prints the runaway run's report

    assert normal_run.over_budget is False, "the ordinary judged-tier call count must stay within budget"  # => co-25: the rule this example proves
    assert runaway_run.over_budget is True, "a doubled call count must be flagged as a budget breach, catching a retry storm or scope creep before it silently doubles CI spend"  # => co-25: the rule this example proves
    print(  # => co-25: opens the final MATCH print, reached only if both asserts above passed
        f"MATCH: {normal_run.judge_call_count} judge calls cost ${normal_run.total_cost_usd:.2f} (within the ${BUDGET_PER_RUN_USD:.2f} budget); {runaway_run.judge_call_count} calls cost ${runaway_run.total_cost_usd:.2f} and correctly breaches it"  # => co-25: the message string itself
    )  # => co-25
    # => co-25: ex-49 next routes a CI failure -- whether a real regression or a cost breach -- back into the error-analysis loop ex-01 began
