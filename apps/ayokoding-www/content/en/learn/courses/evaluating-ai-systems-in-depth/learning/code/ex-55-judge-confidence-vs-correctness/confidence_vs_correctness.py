"""Worked Example 55: Check Whether a Judge's Stated Confidence Correlates With Its Actual Agreement."""  # => co-10: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import TypedDict  # => co-10: JudgedCase types every field this correlation check reads


class JudgedCase(TypedDict):  # => co-10: one case, with the judge's OWN stated confidence alongside its actual correctness
    judge_verdict: bool  # => co-10: the judge's pass/fail verdict
    human_verdict: bool  # => co-08: the adjudicated, human-agreed correct verdict
    judge_stated_confidence: str  # => co-11: the judge's own self-reported confidence label -- "high" or "low"


# A judge that reports "high" confidence on every verdict, whether or not that verdict is
# actually correct -- confidence and correctness measured completely independently.
JUDGED_CASES: list[JudgedCase] = [  # => co-10: eight cases -- the judge's stated confidence tells you nothing about whether it's RIGHT
    {"judge_verdict": True, "human_verdict": True, "judge_stated_confidence": "high"},  # => co-10: correct, confident
    {"judge_verdict": False, "human_verdict": True, "judge_stated_confidence": "high"},  # => co-10: WRONG, but still confident
    {"judge_verdict": True, "human_verdict": True, "judge_stated_confidence": "high"},  # => co-10: correct, confident
    {"judge_verdict": True, "human_verdict": False, "judge_stated_confidence": "high"},  # => co-10: WRONG, but still confident
    {"judge_verdict": False, "human_verdict": False, "judge_stated_confidence": "low"},  # => co-10: correct, but "unconfident"
    {"judge_verdict": True, "human_verdict": True, "judge_stated_confidence": "low"},  # => co-10: correct, but "unconfident"
    {"judge_verdict": False, "human_verdict": True, "judge_stated_confidence": "low"},  # => co-10: WRONG, and "unconfident"
    {"judge_verdict": True, "human_verdict": True, "judge_stated_confidence": "high"},  # => co-10: correct, confident
]  # => co-10: closes JUDGED_CASES


def agreement_within_confidence_band(cases: list[JudgedCase], *, confidence: str) -> float:  # => co-11: measures agreement SEPARATELY per confidence label
    """Return agreement rate restricted to `cases` whose judge_stated_confidence equals `confidence`."""  # => co-11: documents agreement_within_confidence_band's contract -- no runtime output, just sets its __doc__
    band = [c for c in cases if c["judge_stated_confidence"] == confidence]  # => co-11: filters to only this confidence band
    return sum(1 for c in band if c["judge_verdict"] == c["human_verdict"]) / len(band)  # => co-11: returns this computed value to the caller


if __name__ == "__main__":  # => co-11: entry point -- runs only when this file executes directly, not on import
    high_confidence_agreement = agreement_within_confidence_band(JUDGED_CASES, confidence="high")  # => co-11: agreement WITHIN the "high confidence" cases only
    low_confidence_agreement = agreement_within_confidence_band(JUDGED_CASES, confidence="low")  # => co-11: agreement WITHIN the "low confidence" cases only
    print(f"Agreement when judge states HIGH confidence: {high_confidence_agreement:.0%}")  # => co-11
    print(f"Agreement when judge states LOW confidence: {low_confidence_agreement:.0%}")  # => co-11

    assert high_confidence_agreement < 1.0, "the judge must be genuinely wrong on at least one 'high confidence' case"  # => co-10: high stated confidence is not proof of correctness
    assert low_confidence_agreement >= high_confidence_agreement, "the 'low confidence' band must NOT show worse agreement than 'high confidence' -- proving the label carries no real signal"  # => co-10: the rule this example proves
    print("MATCH: stated confidence does not track actual agreement -- 'low confidence' cases are AT LEAST as accurate as 'high confidence' ones")  # => co-10
    # => co-10,co-11: a judge's own confidence label is not evidence -- only a MEASURED agreement statistic, per co-10, is
