"""Worked Example 21: Measure One Judge's Agreement on Two Different Criteria."""  # => co-11: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import TypedDict  # => co-11: LabeledCase types every field this measurement reads


class LabeledCase(TypedDict):  # => co-11: one case, human-labeled AND judge-scored, for ONE specific criterion
    reply: str  # => co-11: the model reply under evaluation
    human_verdict: bool  # => co-08: the adjudicated, human-agreed correct verdict
    judge_verdict: bool  # => co-11: the SAME judge model, scoring this criterion


# The SAME judge model, scoring two DIFFERENT criteria on two different case sets.
CLARIFICATION_CASES: list[LabeledCase] = [  # => co-11: criterion 1 -- "did it ask before acting" (ex-17's criterion)
    {"reply": "Sure -- which board?", "human_verdict": True, "judge_verdict": True},  # => co-11: agree
    {"reply": "Done, moved it.", "human_verdict": False, "judge_verdict": False},  # => co-11: agree
    {"reply": "Which project did you mean?", "human_verdict": True, "judge_verdict": True},  # => co-11: agree
    {"reply": "Handled already!", "human_verdict": False, "judge_verdict": False},  # => co-11: agree
    {"reply": "On it, which board though?", "human_verdict": True, "judge_verdict": True},  # => co-11: agree
]  # => co-11: closes CLARIFICATION_CASES -- an easy, structural criterion

TONE_CASES: list[LabeledCase] = [  # => co-11: criterion 2 -- "is the tone appropriately reassuring for an anxious user" -- SUBJECTIVE
    {"reply": "Your data is safe, nothing was lost.", "human_verdict": True, "judge_verdict": True},  # => co-11: agree
    {"reply": "No data was lost during that operation.", "human_verdict": True, "judge_verdict": False},  # => co-11: disagree -- correct but flat, judge under-reads reassurance
    {"reply": "That's technically expected behavior.", "human_verdict": False, "judge_verdict": True},  # => co-11: disagree -- judge over-reads calm tone as reassuring
    {"reply": "I understand this is stressful -- your work is safe.", "human_verdict": True, "judge_verdict": True},  # => co-11: agree
    {"reply": "Error handled per spec.", "human_verdict": False, "judge_verdict": True},  # => co-11: disagree -- judge misses the cold, unreassuring tone
]  # => co-11: closes TONE_CASES -- a genuinely harder, subjective criterion


def agreement(cases: list[LabeledCase]) -> float:  # => co-11: the SAME measurement function, applied per criterion
    """Return the fraction of `cases` where judge_verdict matches human_verdict."""  # => co-11: documents agreement's contract -- no runtime output, just sets its __doc__
    return sum(1 for c in cases if c["judge_verdict"] == c["human_verdict"]) / len(cases)  # => co-11


if __name__ == "__main__":  # => co-11: entry point -- runs only when this file executes directly, not on import
    clarification_agreement = agreement(CLARIFICATION_CASES)  # => co-11: this judge's agreement on the STRUCTURAL criterion
    tone_agreement = agreement(TONE_CASES)  # => co-11: the SAME judge's agreement on the SUBJECTIVE criterion
    print(f"Judge agreement on 'asks before acting' (structural): {clarification_agreement:.0%}")  # => co-11
    print(f"Judge agreement on 'reassuring tone' (subjective): {tone_agreement:.0%}")  # => co-11

    assert clarification_agreement == 1.0, "the structural criterion must show perfect agreement for this judge"  # => co-11
    assert tone_agreement == 0.4, "the subjective criterion must show much lower agreement for the SAME judge"  # => co-11: the rule this example proves
    assert clarification_agreement != tone_agreement, "the same judge's scope must differ sharply between these two criteria"  # => co-11
    print("MATCH: one judge model scores 100% on a structural criterion and only 40% on a subjective one -- scope is per-question, not global")  # => co-11
    # => co-11: a team that only ever validated this judge on the structural criterion would wrongly trust it on tone too
