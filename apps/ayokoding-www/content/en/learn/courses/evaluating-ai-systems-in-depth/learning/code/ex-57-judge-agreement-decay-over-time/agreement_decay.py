"""Worked Example 57: Measure Agreement at Two Points in Time After a Silent Prompt Change."""  # => co-16: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import TypedDict  # => co-16: LabeledCase types every field this time-series measurement reads


class LabeledCase(TypedDict):  # => co-16: one case, human-labeled, judge-scored at a specific POINT IN TIME
    judge_verdict: bool  # => co-16: the judge's verdict at this time point
    human_verdict: bool  # => co-08: the adjudicated, human-agreed correct verdict


# Week 1: the judge's prompt template was written and validated. Week 8: the same judge's PROMPT
# was silently edited by a well-meaning engineer, and never re-validated.
WEEK_1_CASES: list[LabeledCase] = [  # => co-16: the judge's ORIGINAL validation, at deployment time
    {"judge_verdict": True, "human_verdict": True},  # => co-16
    {"judge_verdict": False, "human_verdict": False},  # => co-16
    {"judge_verdict": True, "human_verdict": True},  # => co-16
    {"judge_verdict": False, "human_verdict": False},  # => co-16
    {"judge_verdict": True, "human_verdict": True},  # => co-16
]  # => co-16: closes WEEK_1_CASES -- 5/5, matches the judge's original validation

WEEK_8_CASES: list[LabeledCase] = [  # => co-16: the SAME judge, same criterion, re-measured after a silent prompt edit
    {"judge_verdict": True, "human_verdict": True},  # => co-16: still agrees
    {"judge_verdict": True, "human_verdict": False},  # => co-16: NOW disagrees -- the edited prompt drifted
    {"judge_verdict": True, "human_verdict": True},  # => co-16: still agrees
    {"judge_verdict": True, "human_verdict": False},  # => co-16: NOW disagrees -- the edited prompt drifted
    {"judge_verdict": True, "human_verdict": True},  # => co-16: still agrees
]  # => co-16: closes WEEK_8_CASES -- the prompt edit made the judge over-eager to pass everything


def agreement(cases: list[LabeledCase]) -> float:  # => co-16: the SAME measurement function, applied at each time point
    """Return the fraction of `cases` where judge_verdict matches human_verdict."""  # => co-16: documents agreement's contract -- no runtime output, just sets its __doc__
    return sum(1 for c in cases if c["judge_verdict"] == c["human_verdict"]) / len(cases)  # => co-16


if __name__ == "__main__":  # => co-16: entry point -- runs only when this file executes directly, not on import
    week_1_agreement = agreement(WEEK_1_CASES)  # => co-16: the judge's agreement at week 1
    week_8_agreement = agreement(WEEK_8_CASES)  # => co-16: the SAME judge's agreement at week 8, after the silent prompt edit
    print(f"Week 1 (original validation): {week_1_agreement:.0%}")  # => co-16
    print(f"Week 8 (after a silent prompt edit): {week_8_agreement:.0%}")  # => co-16

    decay = week_1_agreement - week_8_agreement  # => co-16: how much agreement was lost over the eight weeks
    print(f"Agreement decay: {decay:.0%}")  # => co-16: the headline number for this example
    assert week_1_agreement == 1.0, "week 1's original validation must show full agreement"  # => co-16: sanity check on the fixture
    assert decay >= 0.3, "a silent prompt edit must produce a substantial, MEASURABLE decay in agreement"  # => co-16: the rule this example proves
    print("MATCH: a scheduled re-measurement at week 8 catches a real decay that a one-time validation at week 1 would never reveal")  # => co-16
    # => co-16: this is exactly what ex-34's prompt-template-edit trigger exists to force -- re-measurement on the SCHEDULE, not after a problem is noticed downstream
