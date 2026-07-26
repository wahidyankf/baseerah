# learning/capstone/evaluate/evaluate.py
"""Capstone Step 4: Evaluation (exercises co-22, co-25, co-26)."""  # => co-25: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import json  # => co-25: the committed evaluation-result artefact this step writes, read by operate/operate.py next
from math import comb  # => co-25: an exact binomial sign-test needs the binomial coefficient, not an approximation -- matches this course's own ex-68
from pathlib import Path  # => co-25: locates the prior step's training result and this step's own committed artefact
from typing import TypedDict, cast  # => co-25: types the committed artefacts this step reads and writes

TRAIN_RESULT_PATH = Path(__file__).parent.parent / "train" / "train_result.json"  # => co-20: the prior step's own committed artefact
RESULT_PATH = Path(__file__).parent / "evaluate_result.json"  # => co-25: this step's own committed artefact -- operate/operate.py reads it next


class TrainResult(TypedDict):  # => co-20: mirrors train.py's own committed shape -- only the fields this step actually needs
    chosen_rank: int  # => co-20: carried forward for the evaluation report
    val_pass_rate: float  # => co-24: the validation-time result, compared below against a genuinely held-out result
    base_model_id: str  # => co-30: pinned, carried forward unchanged


TARGET_TASK_PASS_RATE_BASE = 0.60  # => co-25: the unadapted base's own pass rate, matching decision.py's own BASE_PASS_RATE
TARGET_TASK_PASS_RATE_ADAPTED = 0.90  # => co-25: the adapted model's OWN held-out result -- close to but not identical to validation, since this is a genuinely separate set
DISCORDANT_B = 18  # => co-25: held-out cases the adapter fixed relative to the base, out of 20 discordant pairs
DISCORDANT_C = 2  # => co-25: held-out cases the adapter broke relative to the base, out of 20 discordant pairs
SIGNIFICANCE_THRESHOLD = 0.05  # => co-25: the conventional cutoff, matching this course's own ex-68

REGRESSION_PASS_RATE_BASE = 1.00  # => co-26: the base's own clean score on the untouched-capability regression suite
REGRESSION_PASS_RATE_ADAPTED = 0.90  # => co-22,co-26: the adapted model's own score on the SAME regression suite
FORGETTING_ALERT_THRESHOLD = 0.90  # => co-22: matches this course's own ex-37 alert line


def sign_test_p_value(b: int, c: int) -> float:  # => co-25: the exact two-sided sign-test p-value on the b vs c discordant-pair counts -- matches ex-68's own function
    """Return the exact two-sided binomial sign-test p-value for `b` wins against `c` losses out of `b + c` discordant pairs."""  # => co-25: documents sign_test_p_value's contract -- no runtime output, just sets its __doc__
    n = b + c  # => co-25: only discordant pairs carry information
    if n == 0:  # => co-25: no discordant pairs means no evidence either way
        return 1.0  # => co-25: returns this computed value to the caller
    extreme = min(b, c)  # => co-25: the smaller count is the "more extreme in the other direction" tail
    one_sided = sum(comb(n, k) for k in range(extreme + 1)) / (2**n)  # => co-25: probability of `extreme` or fewer under a fair 50/50 null
    return min(1.0, 2 * one_sided)  # => co-25: two-sided -- double the one-sided tail, capped at 1.0


class EvaluateResult(TypedDict):  # => co-25: the committed shape operate/operate.py reads next
    target_pass_rate_base: float  # => co-25: the base's own headline number
    target_pass_rate_adapted: float  # => co-25: the adapted model's own headline number
    discordant_b: int  # => co-25: paired evidence FOR the adaptation
    discordant_c: int  # => co-25: paired evidence AGAINST the adaptation
    p_value: float  # => co-25: the exact test's own result
    significant: bool  # => co-25: does the improvement clear the significance threshold
    regression_pass_rate_base: float  # => co-26: the base's own regression-suite score
    regression_pass_rate_adapted: float  # => co-22,co-26: the adapted model's own regression-suite score
    forgetting_detected: bool  # => co-22: did the regression score cross the alert line
    base_model_id: str  # => co-30: pinned, carried forward unchanged


if __name__ == "__main__":  # => co-25: entry point -- runs only when this file executes directly, not on import
    train_raw = cast(TrainResult, json.loads(TRAIN_RESULT_PATH.read_text(encoding="utf-8")))  # => co-20: read the prior step's own committed artefact
    print(f"Adapter: rank {train_raw['chosen_rank']} pinned to base {train_raw['base_model_id']!r} (validation pass rate {train_raw['val_pass_rate']:.0%})")  # => co-20,co-24,co-30

    print(f"Target-task pass rate on a GENUINELY held-out eval set: base {TARGET_TASK_PASS_RATE_BASE:.0%} | adapted {TARGET_TASK_PASS_RATE_ADAPTED:.0%}")  # => co-25
    p_value = sign_test_p_value(DISCORDANT_B, DISCORDANT_C)  # => co-25: run the exact test, not the naive two-numbers comparison
    significant = p_value < SIGNIFICANCE_THRESHOLD  # => co-25: does the paired evidence actually clear the bar
    print(f"Discordant pairs: {DISCORDANT_B} fixed, {DISCORDANT_C} broken -> exact p-value {p_value:.6f} (significant: {significant})")  # => co-25
    assert significant, "this capstone's own scenario is designed so the paired improvement is genuinely, exactly significant"  # => co-25

    forgetting_detected = REGRESSION_PASS_RATE_ADAPTED < FORGETTING_ALERT_THRESHOLD  # => co-22: does the regression-suite drop cross the alert line
    print(f"Regression suite: base {REGRESSION_PASS_RATE_BASE:.0%} | adapted {REGRESSION_PASS_RATE_ADAPTED:.0%} -> forgetting detected: {forgetting_detected}")  # => co-22,co-26
    assert not forgetting_detected, "this capstone's own scenario is designed so the adapted model stays within the regression-suite alert threshold"  # => co-22

    result: EvaluateResult = {  # => co-25: the full committed artefact -- every field traceable to a measurement made above
        "target_pass_rate_base": TARGET_TASK_PASS_RATE_BASE,  # => co-25
        "target_pass_rate_adapted": TARGET_TASK_PASS_RATE_ADAPTED,  # => co-25
        "discordant_b": DISCORDANT_B,  # => co-25
        "discordant_c": DISCORDANT_C,  # => co-25
        "p_value": p_value,  # => co-25
        "significant": significant,  # => co-25
        "regression_pass_rate_base": REGRESSION_PASS_RATE_BASE,  # => co-26
        "regression_pass_rate_adapted": REGRESSION_PASS_RATE_ADAPTED,  # => co-22,co-26
        "forgetting_detected": forgetting_detected,  # => co-22
        "base_model_id": train_raw["base_model_id"],  # => co-30
    }  # => co-25: closes result
    RESULT_PATH.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")  # => co-25: commits the artefact operate/operate.py reads next
    print(f"MATCH: evaluation result committed to {RESULT_PATH.name} -- statistically significant target-task gain, no forgetting on the regression suite")  # => co-22,co-25,co-26
    # => co-22,co-25,co-26: an adapted model is only as trustworthy as its WORST measured signal -- both the paired test and the regression suite had to clear their own bar
