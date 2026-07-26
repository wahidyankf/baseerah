# learning/code/ex-70-a-fine-tune-that-helped-and-hurt/helped_and_hurt.py
"""Worked Example 70: A Fine-Tune That Helped and Hurt."""  # => co-25: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-25: a small, self-documenting record holding BOTH halves of the decision, together


@dataclass(frozen=True)  # => co-25: frozen -- a completed run's dual result is a fact once measured, not a mutable running total
class DualResult:  # => co-25,co-22: a run that must be judged on target-task gain AND regression-suite cost, never one alone
    target_task_pass_rate_base: float  # => co-25: base model's pass rate on the target triage task
    target_task_pass_rate_adapted: float  # => co-25: adapted model's pass rate on the SAME target task -- genuinely better
    regression_pass_rate_base: float  # => co-22: base model's pass rate on the untouched-capability regression suite
    regression_pass_rate_adapted: float  # => co-22: adapted model's pass rate on the SAME regression suite -- genuinely worse
    regression_alert_threshold: float  # => co-22: below this regression score, the damage is judged too severe to accept


RESULT = DualResult(  # => co-25,co-22: a genuinely mixed outcome -- clearly helped on target, clearly hurt on regression
    target_task_pass_rate_base=0.60,  # => co-25: matches ex-01's own baseline gap
    target_task_pass_rate_adapted=0.93,  # => co-25: a real, evidenced 33-point target-task gain
    regression_pass_rate_base=1.00,  # => co-22: the base's own clean regression score
    regression_pass_rate_adapted=0.72,  # => co-22: a real, evidenced 28-point regression-suite drop
    regression_alert_threshold=0.90,  # => co-22: matches ex-37's own alert line
)  # => co-25: closes RESULT


def decide(result: DualResult) -> str:  # => co-25,co-22: the decision must weigh BOTH halves, never just the flattering one
    """Return 'ship', 'reject', or 'needs mitigation' based on `result`'s target-task gain and regression-suite damage."""  # => co-25: documents decide's contract -- no runtime output, just sets its __doc__
    target_improved = result.target_task_pass_rate_adapted > result.target_task_pass_rate_base  # => co-25: did the target task genuinely improve
    regression_acceptable = result.regression_pass_rate_adapted >= result.regression_alert_threshold  # => co-22: is the regression damage within the accepted line
    if target_improved and regression_acceptable:  # => co-25,co-22: both halves clear -- a clean ship decision
        return "ship"  # => co-25
    if target_improved and not regression_acceptable:  # => co-25,co-22: THIS scenario -- helped on target, hurt on regression, past the accepted line
        return "needs mitigation"  # => co-22: neither an unqualified ship nor an outright reject -- the regression must be fixed or scoped around first
    return "reject"  # => co-25: no target-task improvement at all -- reject outright


if __name__ == "__main__":  # => co-25: entry point -- runs only when this file executes directly, not on import
    target_gain = RESULT.target_task_pass_rate_adapted - RESULT.target_task_pass_rate_base  # => co-25: the real, positive gain
    regression_damage = RESULT.regression_pass_rate_base - RESULT.regression_pass_rate_adapted  # => co-22: the real, negative damage
    print(f"Target-task gain: +{target_gain:.0%} | Regression-suite damage: -{regression_damage:.0%}")  # => co-25,co-22
    assert target_gain > 0.30, "the target-task gain must be large and genuinely positive in this scenario"  # => co-25
    assert regression_damage > 0.25, "the regression-suite damage must be large and genuinely negative in this scenario"  # => co-22
    verdict = decide(RESULT)  # => co-25,co-22: run the full decision, weighing both halves
    print(f"Decision: {verdict}")  # => co-25
    assert verdict == "needs mitigation", "a run that genuinely helps the target task but crosses the regression alert line must land as 'needs mitigation', not a clean ship"  # => co-25,co-22
    print("MATCH: +33 points on target and -28 points on regression is neither a clean ship nor an outright reject -- it needs mitigation before it can go out")  # => co-25,co-22
    # => co-25,co-22: ex-35 and ex-41 were each one-sided (clear win, clear wash) -- real runs often land here, needing BOTH signals read together, not separately
