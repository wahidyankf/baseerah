"""Worked Example 50: A Miniature End-to-End Validated Eval System -- co-01 Through co-28's Arc, in One File."""  # => co-01: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import statistics  # => co-24: stdlib mean/stdev for the noise-floor step, reused from ex-44's pattern
from typing import NamedTuple  # => co-01: every stage below uses a typed record, not a bare dict


class FailureCase(NamedTuple):  # => co-01: a single, real observed failure -- error analysis's raw material (co-02)
    request: str  # => co-01: the real request
    failure_mode: str  # => co-03: the taxonomy mode this failure belongs to


class GroundTruthCase(NamedTuple):  # => co-11: an adjudicated case with a human-agreed label (co-12/co-13)
    request: str  # => co-11
    human_label: bool  # => co-11: True means "passes the derived criterion"


class JudgeVerdict(NamedTuple):  # => co-09: a judge's own machine-parseable verdict on one case
    case_request: str  # => co-09
    judge_label: bool  # => co-09: the judge's own True/False call


# STAGE 1 (co-01/co-02/co-03): a small batch of REAL failures, already open-coded and clustered.
OBSERVED_FAILURES = (  # => co-01: the course's very first step, replayed here in miniature
    FailureCase("Move this to done.", failure_mode="skips-clarifying-question"),  # => co-03
    FailureCase("Close ticket #12.", failure_mode="wrong-object-acted-on"),  # => co-03
)  # => co-01: closes OBSERVED_FAILURES

# STAGE 2 (co-08): a derived, operationalized criterion FROM the dominant failure mode above.
DERIVED_CRITERION = "The reply must ask a clarifying question before acting when the request names no specific target."  # => co-08

# STAGE 3 (co-11/co-14): a small, adjudicated ground-truth set, labeled against the derived criterion.
GROUND_TRUTH = (  # => co-14
    GroundTruthCase("Move this to done.", human_label=False),  # => co-14: acted without asking -- FAILS the criterion
    GroundTruthCase("Sure -- which board should I move it on?", human_label=True),  # => co-14: asked first -- PASSES
    GroundTruthCase("Close ticket #12.", human_label=False),  # => co-14: acted on an unconfirmed target -- FAILS
    GroundTruthCase("Which ticket number, exactly?", human_label=True),  # => co-14: confirmed first -- PASSES
)  # => co-14: closes GROUND_TRUTH


def mock_judge(request: str) -> JudgeVerdict:  # => co-09: a mocked judge model -- a DIFFERENT model than the one under test, per co-16
    """Return a mocked judge verdict for `request`, checking for a clarifying-question pattern."""  # => co-09: documents mock_judge's contract -- no runtime output, just sets its __doc__
    asks_first = "which" in request.lower() or "sure" in request.lower()  # => co-09: the judge's own read of DERIVED_CRITERION
    return JudgeVerdict(case_request=request, judge_label=asks_first)  # => co-09: returns this computed value to the caller


def measure_agreement(ground_truth: tuple[GroundTruthCase, ...], verdicts: tuple[JudgeVerdict, ...]) -> float:  # => co-17: co-17's measured judge-human agreement rate, not an assumed one
    """Return the fraction of cases where `verdicts[i].judge_label == ground_truth[i].human_label`."""  # => co-17: documents measure_agreement's contract -- no runtime output, just sets its __doc__
    matches = sum(g.human_label == v.judge_label for g, v in zip(ground_truth, verdicts, strict=True))  # => co-17: counts genuine agreements, not assumed ones
    return matches / len(ground_truth)  # => co-17: returns this computed value to the caller


if __name__ == "__main__":  # => co-01: entry point -- runs only when this file executes directly, not on import
    print(f"Stage 1 -- observed failures: {[f.failure_mode for f in OBSERVED_FAILURES]}")  # => co-01
    print(f"Stage 2 -- derived criterion: {DERIVED_CRITERION!r}")  # => co-08
    judge_verdicts = tuple(mock_judge(c.request) for c in GROUND_TRUTH)  # => co-09: Stage 3 -- run the judge on every ground-truth case
    agreement_rate = measure_agreement(GROUND_TRUTH, judge_verdicts)  # => co-17: Stage 4 -- measure judge-human agreement, the validation gate
    print(f"Stage 3/4 -- judge-human agreement on {len(GROUND_TRUTH)} ground-truth cases: {agreement_rate:.0%}")  # => co-17

    # STAGE 5 (co-24/co-23): a CI regression bar derived from a measured noise floor, exactly as ex-45 built it.
    repeated_pass_rates = (0.90, 0.94, 0.88, 0.92)  # => co-24: illustrative repeated-run pass rates for THIS validated judge
    noise_floor = statistics.stdev(repeated_pass_rates)  # => co-24: the measured wobble
    baseline = statistics.mean(repeated_pass_rates)  # => co-24: the measured baseline
    regression_bar = baseline - 2 * noise_floor  # => co-23: the derived bar, same formula as ex-45
    within_noise_candidate = 0.89  # => co-24: a candidate that dips, but stays within noise
    real_regression_candidate = 0.72  # => co-23: a candidate that genuinely regressed
    print(f"Stage 5 -- regression bar: {regression_bar:.1%} (baseline {baseline:.1%}, noise {noise_floor:.1%})")  # => co-23
    print(f"  within-noise candidate ({within_noise_candidate:.0%}) blocked: {within_noise_candidate < regression_bar}")  # => co-23
    print(f"  real-regression candidate ({real_regression_candidate:.0%}) blocked: {real_regression_candidate < regression_bar}")  # => co-23

    assert agreement_rate >= 0.75, "the validated judge must clear a real agreement threshold against ground truth before it is trusted in CI"  # => co-17: the rule this example proves
    assert within_noise_candidate >= regression_bar, "the within-noise candidate must NOT be blocked -- it is ordinary wobble, not a real regression"  # => co-23: the rule this example proves
    assert real_regression_candidate < regression_bar, "the real-regression candidate MUST be blocked"  # => co-23: the rule this example proves
    print(f"MATCH: error analysis (co-01) through a CI-gated, noise-aware regression bar (co-23/co-24), routed through a MEASURED {agreement_rate:.0%}-agreement judge (co-17) -- one integrated system, not disconnected pieces")  # => co-01
    # => co-01: this miniature system is exactly what the course's own five-step capstone builds out in full, end to end
