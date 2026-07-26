"""Worked Example 56: Measure Agreement for a 1-Question vs. a 5-Question Rubric on the Same Criterion."""  # => co-15: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import TypedDict  # => co-15: LabeledCase types every field this measurement reads


class LabeledCase(TypedDict):  # => co-15: one case, human-labeled, scored under BOTH rubric lengths
    reply: str  # => co-15: the model reply under evaluation
    human_verdict: bool  # => co-08: the adjudicated, human-agreed correct verdict


CASES: list[LabeledCase] = [  # => co-15: six cases, testing "does this reply cite a specific, checkable ticket ID"
    {"reply": "I've updated ticket #4821 as requested.", "human_verdict": True},  # => co-15
    {"reply": "I've updated the ticket as requested.", "human_verdict": False},  # => co-15: no specific ID cited
    {"reply": "Ticket #77 is now marked resolved.", "human_verdict": True},  # => co-15
    {"reply": "That ticket is now marked resolved.", "human_verdict": False},  # => co-15: no specific ID cited
    {"reply": "#903 has been closed per your request.", "human_verdict": True},  # => co-15
    {"reply": "It has been closed per your request.", "human_verdict": False},  # => co-15: no specific ID cited
]  # => co-15: closes CASES


def one_question_rubric(reply: str) -> bool:  # => co-15: a single, binary rubric question
    """ONE question: does `reply` cite a specific ticket ID (a '#' followed by digits)?"""  # => co-15: documents one_question_rubric's contract -- no runtime output, just sets its __doc__
    return "#" in reply and any(c.isdigit() for c in reply)  # => co-15: exactly the one property that decides this criterion


def five_question_rubric(reply: str) -> bool:  # => co-15: five loosely-related dimensions, averaged -- structurally identical to ex-29's long rubric
    """FIVE questions (clarity, politeness, ID citation, length, tense), pass if 3+ of 5 individually pass."""  # => co-15: documents five_question_rubric's contract -- no runtime output, just sets its __doc__
    clarity = len(reply) < 60  # => co-15: dimension 1 -- a vague proxy, unrelated to the real criterion
    politeness = "please" not in reply.lower()  # => co-15: dimension 2 -- a vague proxy, unrelated to the real criterion
    cites_id = "#" in reply and any(c.isdigit() for c in reply)  # => co-15: dimension 3 -- the ONE dimension that actually matters
    right_length = 10 < len(reply) < 80  # => co-15: dimension 4 -- another vague proxy
    past_tense = any(w in reply.lower() for w in ("has been", "updated", "closed", "marked", "resolved"))  # => co-15: dimension 5 -- another vague proxy
    passing_dimensions = sum([clarity, politeness, cites_id, right_length, past_tense])  # => co-15: how many of the five dimensions pass
    return passing_dimensions >= 3  # => co-15: three-of-five can pass WITHOUT cites_id ever being true, diluting the real signal


if __name__ == "__main__":  # => co-15: entry point -- runs only when this file executes directly, not on import
    one_q_correct = sum(1 for c in CASES if one_question_rubric(c["reply"]) == c["human_verdict"])  # => co-15: one-question rubric's agreement count
    five_q_correct = sum(1 for c in CASES if five_question_rubric(c["reply"]) == c["human_verdict"])  # => co-15: five-question rubric's agreement count
    one_q_agreement = one_q_correct / len(CASES)  # => co-15: one-question rubric's agreement rate
    five_q_agreement = five_q_correct / len(CASES)  # => co-15: five-question rubric's agreement rate
    print(f"1-question rubric agreement: {one_q_agreement:.0%} ({one_q_correct}/{len(CASES)})")  # => co-15
    print(f"5-question rubric agreement: {five_q_agreement:.0%} ({five_q_correct}/{len(CASES)})")  # => co-15

    assert one_q_agreement > five_q_agreement, "the single-question rubric must agree with humans MORE than the five-question one on THIS criterion"  # => co-15: the rule this example proves
    print(f"MATCH: the shorter rubric wins by {(one_q_agreement - five_q_agreement):.0%} agreement -- fewer questions, more precisely aimed, beats more questions, diluted")  # => co-15
    # => co-15: this reproduces ex-29's finding on a genuinely DIFFERENT criterion (ticket-ID citation, not tone) -- the pattern generalizes, it is not a one-off fixture artifact
