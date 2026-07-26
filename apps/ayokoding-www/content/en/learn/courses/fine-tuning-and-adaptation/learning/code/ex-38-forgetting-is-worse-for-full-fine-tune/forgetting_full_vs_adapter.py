# learning/code/ex-38-forgetting-is-worse-for-full-fine-tune/forgetting_full_vs_adapter.py
"""Worked Example 38: Forgetting Is Worse for Full Fine-Tune."""  # => co-22: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-18: a small, self-documenting record for each training strategy's regression damage


@dataclass(frozen=True)  # => co-18: frozen -- a measured regression profile is a fact once run, not a mutable running total
class RegressionProfile:  # => co-22: one training strategy's target-task gain against its regression-suite cost
    strategy: str  # => co-18: "full fine-tune" or "LoRA adapter", the two strategies ex-27/ex-28 and ex-29/ex-30 already introduced
    target_task_pass_rate: float  # => co-25: pass rate on the triage target task after this strategy's training
    regression_suite_pass_rate: float  # => co-22: pass rate on ex-36's untouched-capability suite after this strategy's training


BASE_REGRESSION_PASS_RATE = 1.00  # => co-22: the unadapted base's own regression-suite score, the reference point both strategies are compared against

FULL_FINE_TUNE = RegressionProfile(  # => co-17: updates ALL parameters, per ex-27/ex-28's lineage
    strategy="full fine-tune",  # => co-17
    target_task_pass_rate=0.96,  # => co-25: matches ex-27's own measured target-task result
    regression_suite_pass_rate=0.40,  # => co-22: a much larger regression-suite drop -- updating every parameter reshapes far more than the target behaviour
)  # => co-22: closes FULL_FINE_TUNE

LORA_ADAPTER = RegressionProfile(  # => co-18: trains a small added parameter set with the base FROZEN, per ex-29/ex-30's lineage
    strategy="LoRA adapter",  # => co-19
    target_task_pass_rate=0.94,  # => co-25: matches ex-29's own measured target-task result, nearly as good as the full fine-tune
    regression_suite_pass_rate=0.60,  # => co-22: matches ex-37's own measured adapted-model regression score
)  # => co-22: closes LORA_ADAPTER


if __name__ == "__main__":  # => co-22: entry point -- runs only when this file executes directly, not on import
    for profile in (FULL_FINE_TUNE, LORA_ADAPTER):  # => co-22: compare both strategies side by side
        regression_damage = BASE_REGRESSION_PASS_RATE - profile.regression_suite_pass_rate  # => co-22: how far below the base's clean 100% this strategy fell
        print(f"  {profile.strategy}: target {profile.target_task_pass_rate:.0%} | regression suite {profile.regression_suite_pass_rate:.0%} | damage {regression_damage:.0%}")  # => co-22
    full_ft_damage = BASE_REGRESSION_PASS_RATE - FULL_FINE_TUNE.regression_suite_pass_rate  # => co-22: full fine-tune's regression damage
    adapter_damage = BASE_REGRESSION_PASS_RATE - LORA_ADAPTER.regression_suite_pass_rate  # => co-22: adapter's regression damage
    target_task_gap = FULL_FINE_TUNE.target_task_pass_rate - LORA_ADAPTER.target_task_pass_rate  # => co-25: how much target-task quality the adapter gives up
    print(f"Full fine-tune causes {full_ft_damage / adapter_damage:.1f}x the regression damage of the adapter, for a {target_task_gap:.0%} target-task gain")  # => co-22,co-18
    assert full_ft_damage > adapter_damage, "the full fine-tune must damage untouched capability MORE than the parameter-efficient adapter"  # => co-22,co-18
    assert target_task_gap <= 0.03, "the full fine-tune's target-task edge over the adapter must be small in this scenario"  # => co-25
    print("MATCH: full fine-tuning buys 2 target-task points at the cost of far worse regression damage -- the adapter is the better trade here")  # => co-22,co-18
    # => co-22,co-18: this is the concrete evidence behind the tension note -- a full fine-tune should have to argue for itself against a measured adapter baseline
