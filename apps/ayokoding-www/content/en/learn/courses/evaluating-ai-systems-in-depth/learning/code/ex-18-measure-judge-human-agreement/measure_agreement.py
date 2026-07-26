"""Worked Example 18: Compute Agreement Between a Judge and Ground Truth."""  # => co-10: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import TypedDict  # => co-10: LabeledCase types every field this measurement reads


class LabeledCase(TypedDict):  # => co-10: one case, with BOTH a human ground-truth label and a judge verdict
    reply: str  # => co-10: the model reply under evaluation
    human_verdict: bool  # => co-08: the adjudicated, human-agreed correct verdict
    judge_verdict: bool  # => co-10: the SAME case, scored independently by ex-17's mock judge


LABELED_CASES: list[LabeledCase] = [  # => co-10: ten cases, human-labeled AND judge-scored, ready for a real agreement measurement
    {"reply": "Sure -- which board should I move it on?", "human_verdict": True, "judge_verdict": True},  # => co-10: agree
    {"reply": "Done -- moved to the Done column.", "human_verdict": False, "judge_verdict": False},  # => co-10: agree
    {"reply": "Which project's board did you mean?", "human_verdict": True, "judge_verdict": True},  # => co-10: agree
    {"reply": "I'll take care of it right away.", "human_verdict": False, "judge_verdict": False},  # => co-10: agree
    {"reply": "Happy to help -- which board first?", "human_verdict": True, "judge_verdict": True},  # => co-10: agree
    {"reply": "Consider it done!", "human_verdict": False, "judge_verdict": False},  # => co-10: agree
    {"reply": "On it -- which board, though?", "human_verdict": True, "judge_verdict": True},  # => co-10: agree
    {"reply": "I moved it just now.", "human_verdict": False, "judge_verdict": True},  # => co-10: a genuine DISAGREEMENT -- judge got this one wrong
    {"reply": "Which board do you have in mind?", "human_verdict": True, "judge_verdict": True},  # => co-10: agree
    {"reply": "Moving that over now, no worries!", "human_verdict": False, "judge_verdict": False},  # => co-10: agree
]  # => co-10: closes LABELED_CASES -- nine agreements, one real disagreement


def measure_judge_agreement(cases: list[LabeledCase]) -> float:  # => co-10: the measurement itself -- required before trusting ANY judge
    """Return the fraction of `cases` where judge_verdict matches human_verdict."""  # => co-10: documents measure_judge_agreement's contract -- no runtime output, just sets its __doc__
    matches = sum(1 for case in cases if case["judge_verdict"] == case["human_verdict"])  # => co-10: per-case agreement
    return matches / len(cases)  # => co-10: returns this computed value to the caller


if __name__ == "__main__":  # => co-10: entry point -- runs only when this file executes directly, not on import
    disagreements = [c for c in LABELED_CASES if c["judge_verdict"] != c["human_verdict"]]  # => co-10: exactly which cases the judge got wrong
    for case in disagreements:  # => co-10: prints every disagreement, not just the summary number
        print(f"DISAGREE: {case['reply']!r} -- human={case['human_verdict']}, judge={case['judge_verdict']}")  # => co-10

    agreement = measure_judge_agreement(LABELED_CASES)  # => co-10: the headline agreement statistic
    print(f"Judge-human agreement: {agreement:.0%} ({len(LABELED_CASES) - len(disagreements)}/{len(LABELED_CASES)})")  # => co-10: the number, with its raw counts

    assert len(disagreements) == 1, "this fixture must contain exactly one genuine disagreement"  # => co-10: sanity check on the fixture
    assert agreement == 0.9, "nine of ten matching cases must measure as exactly 90% agreement"  # => co-10: the rule this example proves
    print("MATCH: the judge's value is exactly this MEASURED 90% -- not asserted, not assumed")  # => co-10
    # => co-10: 90% on ten cases is a starting point, not a settled claim -- ex-19 reports the confidence interval around it
