"""Capstone Step 5: Noise-Aware Regression Bar, Tiered Suites, and a Judge-Call Cost Budget."""  # => co-23/co-24/co-26: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import statistics  # => co-24: stdlib mean/stdev, the same tool ex-44 used for the noise floor
from typing import NamedTuple  # => co-23: every record below is typed, not a bare dict


class GateResult(NamedTuple):  # => co-23: the CI gate's own reasoned verdict -- same shape as ex-46
    merge_allowed: bool  # => co-23: the gate's binary decision
    observed_pass_rate: float  # => co-23: what THIS candidate run scored
    bar: float  # => co-23: the regression bar this run was checked against
    reason: str  # => co-23: a human-readable explanation, for the CI log


class TierRun(NamedTuple):  # => co-26: one tier's own execution record
    tier_name: str  # => co-26
    case_count: int  # => co-26
    uses_llm_judge: bool  # => co-25: whether this tier's cost includes judge calls
    pass_rate: float  # => co-26


class CostReport(NamedTuple):  # => co-25: the SAME cost-accounting shape as ex-48
    judge_call_count: int  # => co-25
    cost_per_call_usd: float  # => co-25
    total_cost_usd: float  # => co-25
    budget_usd: float  # => co-25
    over_budget: bool  # => co-25


# Five repeated, unchanged runs of the capstone's OWN combined fast tier (criteria 1/2/4, all above
# threshold per judge.py) -- establishes THIS suite's own measured noise floor, not an assumed one.
REPEATED_UNCHANGED_FAST_TIER_RUNS = (0.90, 0.94, 0.88, 0.92, 0.90)  # => co-24: illustrative repeated pass rates for the capstone's own fast tier

COST_PER_JUDGE_CALL_USD = 0.018  # => co-25: reused from ex-48's illustrative per-call price
BUDGET_PER_RUN_USD = 5.00  # => co-25: reused from ex-48's fixed per-CI-run ceiling


def measure_noise_floor(repeated_pass_rates: tuple[float, ...]) -> tuple[float, float]:  # => co-24: the SAME (mean, stdev) computation as ex-44
    """Return `(mean, standard_deviation)` of `repeated_pass_rates`."""  # => co-24: documents measure_noise_floor's contract -- no runtime output, just sets its __doc__
    return statistics.mean(repeated_pass_rates), statistics.stdev(repeated_pass_rates)  # => co-24: returns this computed value to the caller


def set_regression_bar(baseline: float, noise_floor: float, *, multiple: float = 2.0) -> float:  # => co-23: the SAME derivation as ex-45
    """Return `baseline - multiple * noise_floor`."""  # => co-23: documents set_regression_bar's contract -- no runtime output, just sets its __doc__
    return baseline - multiple * noise_floor  # => co-23: returns this computed value to the caller


def run_eval_gate(candidate_pass_rate: float, *, bar: float) -> GateResult:  # => co-23: the SAME CI-gate decision function as ex-46
    """Return a `GateResult` deciding whether `candidate_pass_rate` clears `bar`."""  # => co-23: documents run_eval_gate's contract -- no runtime output, just sets its __doc__
    if candidate_pass_rate < bar:  # => co-23: below the bar -- a real regression
        return GateResult(merge_allowed=False, observed_pass_rate=candidate_pass_rate, bar=bar, reason=f"pass rate {candidate_pass_rate:.1%} fell below the regression bar {bar:.1%}")  # => co-23: returns this computed value to the caller
    return GateResult(merge_allowed=True, observed_pass_rate=candidate_pass_rate, bar=bar, reason=f"pass rate {candidate_pass_rate:.1%} cleared the regression bar {bar:.1%}")  # => co-23: returns this computed value to the caller


def build_cost_report(judge_call_count: int, *, cost_per_call: float = COST_PER_JUDGE_CALL_USD, budget: float = BUDGET_PER_RUN_USD) -> CostReport:  # => co-25: the SAME cost accounting as ex-48
    """Return a `CostReport` for `judge_call_count` judge calls."""  # => co-25: documents build_cost_report's contract -- no runtime output, just sets its __doc__
    total = judge_call_count * cost_per_call  # => co-25: the run's actual total spend
    return CostReport(judge_call_count=judge_call_count, cost_per_call_usd=cost_per_call, total_cost_usd=total, budget_usd=budget, over_budget=total > budget)  # => co-25: returns this computed value to the caller


if __name__ == "__main__":  # => co-26: entry point -- runs only when this file executes directly, not on import
    baseline, noise_floor = measure_noise_floor(REPEATED_UNCHANGED_FAST_TIER_RUNS)  # => co-24: measure this capstone suite's own noise floor
    bar = set_regression_bar(baseline, noise_floor)  # => co-23: derive the bar from the MEASURED noise floor
    print(f"Fast-tier baseline: {baseline:.1%}, noise floor: {noise_floor:.1%}, regression bar: {bar:.1%}")  # => co-23/co-24

    within_noise_candidate = run_eval_gate(0.89, bar=bar)  # => co-24: a candidate that dips, but stays within noise
    real_regression_candidate = run_eval_gate(0.74, bar=bar)  # => co-23: a candidate that genuinely regressed
    print(f"Within-noise candidate: merge_allowed={within_noise_candidate.merge_allowed} ({within_noise_candidate.reason})")  # => co-24
    print(f"Real-regression candidate: merge_allowed={real_regression_candidate.merge_allowed} ({real_regression_candidate.reason})")  # => co-23

    fast_tier = TierRun(tier_name="fast-deterministic", case_count=13, uses_llm_judge=False, pass_rate=baseline)  # => co-26: the capstone's own 13-case ground-truth-scale fast tier
    judged_tier = TierRun(tier_name="llm-judged-merge", case_count=13, uses_llm_judge=True, pass_rate=0.92)  # => co-26: the same 13 cases, re-checked with the validated judge before merge
    tiers = (fast_tier, judged_tier)  # => co-26: both tiers, wired into one pipeline
    print(f"Tiers: {[(t.tier_name, t.uses_llm_judge) for t in tiers]}")  # => co-26

    judged_call_count = judged_tier.case_count  # => co-25: one judge call per case in the judged tier
    cost_report = build_cost_report(judged_call_count)  # => co-25: report and budget the judged tier's own cost
    print(f"Judged-tier cost: {cost_report.judge_call_count} calls, ${cost_report.total_cost_usd:.2f}, over budget: {cost_report.over_budget}")  # => co-25

    assert real_regression_candidate.merge_allowed is False, "a genuine regression must BLOCK the merge"  # => co-23: the syllabus's own required acceptance check
    assert within_noise_candidate.merge_allowed is True, "a within-noise change must NOT block the merge"  # => co-24: the syllabus's own required acceptance check
    assert any(t.uses_llm_judge for t in tiers) and any(not t.uses_llm_judge for t in tiers), "the pipeline must wire BOTH a fast deterministic tier and a judged merge tier"  # => co-26: the rule this example proves
    assert cost_report.over_budget is False, "the judged tier's cost must be reported AND confirmed within budget, not merely reported"  # => co-25: the syllabus's own required acceptance check
    print(
        f"MATCH: a real regression ({real_regression_candidate.observed_pass_rate:.1%}) blocks the merge, a within-noise change ({within_noise_candidate.observed_pass_rate:.1%}) does not, and the judged tier's ${cost_report.total_cost_usd:.2f} cost is reported and confirmed within its ${cost_report.budget_usd:.2f} budget"
    )  # => co-26
    # => co-28: this closes the capstone's five ordered steps; the overview's own final section states what this suite still cannot catch
