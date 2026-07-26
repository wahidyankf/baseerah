# learning/code/ex-55-comparing-alternatives-side-by-side/alternatives_side_by_side.py
"""Worked Example 55: Comparing Alternatives Side by Side."""  # => co-06: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-06: a comparison table beats three separately-run, hard-to-compare scripts


@dataclass(frozen=True)  # => co-06: frozen -- one row of a comparison report should not mutate once recorded
class AlternativeResult:  # => co-06: one alternative's measured result against the SAME gap
    technique: str  # => co-06: which alternative this row reports on
    pass_rate: float  # => co-06: measured against the identical fixed eval set, per co-03's fixed-dataset discipline
    cost_usd: float  # => co-08: what it cost to try this alternative
    time_to_result_hours: float  # => co-06: how long it took to know whether the alternative worked


BASELINE_RATE = 0.55  # => co-06: the gap this comparison is trying to close, measured once, up front

RESULTS = [  # => co-03,co-06: three alternatives, tried against the IDENTICAL gap, so the comparison is fair
    AlternativeResult(technique="prompting (co-03)", pass_rate=0.78, cost_usd=40.0, time_to_result_hours=2.0),  # => co-06: meaningful lift, cheap, fast
    AlternativeResult(technique="retrieval (co-04)", pass_rate=0.61, cost_usd=600.0, time_to_result_hours=16.0),  # => co-06: little lift here -- this gap isn't knowledge-shaped
    AlternativeResult(technique="scoping (co-05)", pass_rate=0.94, cost_usd=80.0, time_to_result_hours=3.0),  # => co-06: the biggest lift, still cheap
]  # => co-06: closes RESULTS


if __name__ == "__main__":  # => co-06: entry point -- runs only when this file executes directly, not on import
    print(f"Baseline pass rate: {BASELINE_RATE:.0%}")  # => co-06: the gap every alternative below is measured against
    best_result = max(RESULTS, key=lambda r: r.pass_rate)  # => co-06: which alternative closed the MOST of the gap
    for result in RESULTS:  # => co-06: print every alternative's row, side by side
        lift = result.pass_rate - BASELINE_RATE  # => co-06: how much THIS alternative improved on the baseline
        marker = " <- best" if result is best_result else ""  # => co-06: highlight the winner without hiding the others
        print(f"  {result.technique}: {result.pass_rate:.0%} (+{lift:.0%}), ${result.cost_usd:.0f}, {result.time_to_result_hours:.0f}h{marker}")  # => co-06
    assert best_result.technique.startswith("scoping"), "scoping must be the best-performing alternative in this scenario"  # => co-06
    assert best_result.pass_rate >= 0.9, "the winning alternative must close the gap to a genuinely usable level"  # => co-06
    print(f"MATCH: {best_result.technique} closes the gap to {best_result.pass_rate:.0%} -- no fine-tune is justified while this alternative works")  # => co-06
    # => co-03,co-04,co-05,co-06: running all three alternatives side by side, against the SAME fixed gap, is what makes co-06's gate a genuine comparison instead of trying one thing and giving up
