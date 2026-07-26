"""Worked Example 20: A Sample-Size Plan for a Real Eval Set."""  # => co-24: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import math  # => co-06: ceil -- a sample size must be a whole number of cases
from dataclasses import dataclass, field  # => co-24: a typed, structurally-required sampling-plan record

from statsmodels.stats.proportion import samplesize_confint_proportion  # => co-06: the pinned library's own sample-size solver


@dataclass(frozen=True)  # => co-24: frozen -- a written plan is a commitment made BEFORE collecting data, not something later code can quietly edit
class SamplingPlan:  # => co-24: the four fields a real sampling plan must state, per this theme's own discipline
    target_effect: str  # => co-06: what the plan is trying to be able to detect or estimate
    target_precision: float  # => co-06: the interval half-width the plan is designed to achieve
    strata: tuple[str, ...] = field(default_factory=tuple)  # => co-08: named strata this plan deliberately oversamples, if any
    required_n: int = 0  # => co-06: the resulting sample size -- computed, not guessed


def build_plan(*, anticipated_rate: float, target_precision: float, target_effect: str, strata: tuple[str, ...]) -> SamplingPlan:  # => co-24: the ONE function that produces a real plan
    """Solve for the required n and package it with the plan's stated effect, precision, and strata."""  # => co-24: documents build_plan's contract -- no runtime output, just sets its __doc__
    raw_n = samplesize_confint_proportion(proportion=anticipated_rate, half_length=target_precision, alpha=0.05, method="normal")  # => co-06: the SAME solver ex-08 used, now inside a reusable plan builder
    required_n = math.ceil(raw_n)  # => co-06: round UP -- a fractional case cannot be collected
    return SamplingPlan(target_effect=target_effect, target_precision=target_precision, strata=strata, required_n=required_n)  # => co-24: returns this computed value to the caller -- every field populated


if __name__ == "__main__":  # => co-24: entry point -- runs only when this file executes directly, not on import
    plan = build_plan(  # => co-24: a plan for a real, concrete eval-set decision
        anticipated_rate=0.82,  # => co-06: a pilot run's own observed rate, used to anticipate the target eval's rate
        target_precision=0.06,  # => co-06: "I need the interval within +/-6 points to make this ship decision"
        target_effect="baseline-vs-candidate pass-rate comparison for the eval-set decision",  # => co-06: states WHAT the plan is trying to support
        strata=("rare-failure-mode-A", "rare-failure-mode-B"),  # => co-08: two rare failure modes this plan will deliberately oversample, per ex-15/ex-17
    )  # => co-24: closes the build_plan call
    print(f"Target effect: {plan.target_effect}")  # => co-06: states the effect explicitly, first
    print(f"Target precision (half-width): {plan.target_precision}")  # => co-06: states the precision explicitly
    print(f"Deliberately oversampled strata: {plan.strata}")  # => co-08: states the strata explicitly
    print(f"Required n: {plan.required_n}")  # => co-06: the resulting number -- computed, not chosen by feel

    assert plan.target_effect != "", "a sampling plan must state its target effect"  # => co-24: the four required-fields check, one assertion per field
    assert plan.target_precision > 0, "a sampling plan must state a positive target precision"  # => co-24
    assert len(plan.strata) > 0, "this plan must name its deliberately oversampled strata"  # => co-24
    assert plan.required_n > 0, "a sampling plan must state a computed, justified n"  # => co-24
    print("MATCH: this plan states target effect, precision, strata, and n -- BEFORE a single case is drawn")  # => co-24
    # => co-06,co-08,co-24: this is the written artifact the capstone's own sampling-plan.md step produces for a real shipping decision
