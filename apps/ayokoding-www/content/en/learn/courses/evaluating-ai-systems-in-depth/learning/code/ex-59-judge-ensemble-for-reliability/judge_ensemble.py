"""Worked Example 59: Average Verdicts From Two Different Judge Models on the Same Case."""  # => co-12: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import TypedDict  # => co-10: LabeledCase types every field this ensemble measurement reads


class LabeledCase(TypedDict):  # => co-10: one case, human-labeled, scored by TWO INDEPENDENT judge models
    human_verdict: bool  # => co-08: the adjudicated, human-agreed correct verdict
    judge_alpha_verdict: bool  # => co-12: a different model from the generator, per co-12
    judge_beta_verdict: bool  # => co-12: a SECOND, genuinely different judge model


CASES: list[LabeledCase] = [  # => co-10: eight cases, each independently scored by two different judge models
    {"human_verdict": True, "judge_alpha_verdict": True, "judge_beta_verdict": True},  # => co-10: both agree, both correct
    {"human_verdict": True, "judge_alpha_verdict": True, "judge_beta_verdict": True},  # => co-10: both agree, both correct
    {"human_verdict": True, "judge_alpha_verdict": True, "judge_beta_verdict": True},  # => co-10: both agree, both correct
    {"human_verdict": False, "judge_alpha_verdict": True, "judge_beta_verdict": False},  # => co-10: alpha's ONE false positive; beta correctly says False
    {"human_verdict": False, "judge_alpha_verdict": False, "judge_beta_verdict": True},  # => co-10: beta's ONE false positive; alpha correctly says False
    {"human_verdict": False, "judge_alpha_verdict": False, "judge_beta_verdict": False},  # => co-10: both agree, both correct
    {"human_verdict": False, "judge_alpha_verdict": False, "judge_beta_verdict": False},  # => co-10: both agree, both correct
    {"human_verdict": True, "judge_alpha_verdict": True, "judge_beta_verdict": True},  # => co-10: both agree, both correct
]  # => co-10: closes CASES -- alpha and beta each make ONE independent false-positive mistake, never on the same case


def single_judge_agreement(cases: list[LabeledCase], *, judge_key: str) -> float:  # => co-10: agreement for either judge ALONE
    """Return agreement between `cases[judge_key]` and human_verdict."""  # => co-10: documents single_judge_agreement's contract -- no runtime output, just sets its __doc__
    return sum(1 for c in cases if c[judge_key] == c["human_verdict"]) / len(cases)  # type: ignore[literal-required]  # => co-10: dynamic key access, deliberately narrow


def ensemble_verdict(alpha: bool, beta: bool) -> bool:  # => co-12: the ensemble decision rule -- BOTH must agree to pass
    """Return True only if BOTH judges independently agree the case passes -- a conservative ensemble rule."""  # => co-12: documents ensemble_verdict's contract -- no runtime output, just sets its __doc__
    return alpha and beta  # => co-12: requiring unanimity cancels each judge's SOLO false positive, since the other judge correctly withholds True


def ensemble_agreement(cases: list[LabeledCase]) -> float:  # => co-10: agreement for the COMBINED ensemble
    """Return agreement between the ensemble_verdict of both judges and human_verdict."""  # => co-10: documents ensemble_agreement's contract -- no runtime output, just sets its __doc__
    return sum(1 for c in cases if ensemble_verdict(c["judge_alpha_verdict"], c["judge_beta_verdict"]) == c["human_verdict"]) / len(cases)  # => co-10


if __name__ == "__main__":  # => co-10: entry point -- runs only when this file executes directly, not on import
    alpha_agreement = single_judge_agreement(CASES, judge_key="judge_alpha_verdict")  # => co-10: judge alpha's own solo agreement
    beta_agreement = single_judge_agreement(CASES, judge_key="judge_beta_verdict")  # => co-10: judge beta's own solo agreement
    ensemble_agreement_rate = ensemble_agreement(CASES)  # => co-10: the combined ensemble's agreement
    print(f"Judge alpha alone: {alpha_agreement:.0%}")  # => co-10
    print(f"Judge beta alone: {beta_agreement:.0%}")  # => co-10
    print(f"Ensemble (both must agree): {ensemble_agreement_rate:.0%}")  # => co-10

    assert ensemble_agreement_rate > max(alpha_agreement, beta_agreement), "the ensemble must exceed EITHER individual judge's solo agreement"  # => co-10: the rule this example proves
    print(f"MATCH: the two-judge ensemble ({ensemble_agreement_rate:.0%}) beats both individual judges ({alpha_agreement:.0%}, {beta_agreement:.0%}) because their mistakes never coincide on the same case")  # => co-10
    # => co-12: this only works because alpha and beta are genuinely DIFFERENT models -- two judges sharing the same blind spot would cancel nothing, per co-12's own lesson
