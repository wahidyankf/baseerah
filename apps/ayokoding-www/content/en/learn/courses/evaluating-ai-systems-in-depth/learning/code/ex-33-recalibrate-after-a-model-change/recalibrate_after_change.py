"""Worked Example 33: Re-Measure Agreement After Swapping the Generator -- and Detect the Drift."""  # => co-16: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import TypedDict  # => co-16: LabeledCase types every field this measurement reads


class LabeledCase(TypedDict):  # => co-16: one case, human-labeled, judge-scored BEFORE and AFTER a generator swap
    reply: str  # => co-16: the model reply under evaluation
    human_verdict: bool  # => co-08: the adjudicated, human-agreed correct verdict
    judge_verdict: bool  # => co-16: the judge's own verdict on THIS reply


# The judge was originally validated against replies from "Generator v1" -- its measured agreement
# was 90% (matches ex-18). The team then silently swapped in "Generator v2" without re-measuring.
CASES_AGAINST_GENERATOR_V1: list[LabeledCase] = [  # => co-16: the ORIGINAL validation set -- Generator v1's replies
    {"reply": "Sure -- which board?", "human_verdict": True, "judge_verdict": True},  # => co-16
    {"reply": "Done, moved it.", "human_verdict": False, "judge_verdict": False},  # => co-16
    {"reply": "Which project did you mean?", "human_verdict": True, "judge_verdict": True},  # => co-16
    {"reply": "Handled already!", "human_verdict": False, "judge_verdict": False},  # => co-16
    {"reply": "On it, which board though?", "human_verdict": True, "judge_verdict": True},  # => co-16
]  # => co-16: closes CASES_AGAINST_GENERATOR_V1 -- 5/5, matches the judge's original validation

CASES_AGAINST_GENERATOR_V2: list[LabeledCase] = [  # => co-16: the SAME judge, applied to Generator v2's DIFFERENT phrasing style, un-re-measured
    {"reply": "Understood, happy to help with that request!", "human_verdict": False, "judge_verdict": True},  # => co-16: v2 never asks, judge missed it
    {"reply": "Consider it handled on my end.", "human_verdict": False, "judge_verdict": True},  # => co-16: v2 never asks, judge missed it
    {"reply": "Let me know if you meant something else!", "human_verdict": False, "judge_verdict": True},  # => co-16: v2 never asks a real clarifying question, judge missed it
    {"reply": "Could you tell me which board you mean?", "human_verdict": True, "judge_verdict": True},  # => co-16: v2 asks -- still correctly caught
    {"reply": "Sounds good, taking care of it now.", "human_verdict": False, "judge_verdict": True},  # => co-16: v2 never asks, judge missed it
]  # => co-16: closes CASES_AGAINST_GENERATOR_V2 -- the judge's phrasing-pattern matching no longer transfers


def agreement(cases: list[LabeledCase]) -> float:  # => co-16: the SAME measurement function, applied before and after the swap
    """Return the fraction of `cases` where judge_verdict matches human_verdict."""  # => co-16: documents agreement's contract -- no runtime output, just sets its __doc__
    return sum(1 for c in cases if c["judge_verdict"] == c["human_verdict"]) / len(cases)  # => co-16


if __name__ == "__main__":  # => co-16: entry point -- runs only when this file executes directly, not on import
    agreement_v1 = agreement(CASES_AGAINST_GENERATOR_V1)  # => co-16: the judge's ORIGINALLY-measured agreement, against Generator v1
    agreement_v2 = agreement(CASES_AGAINST_GENERATOR_V2)  # => co-16: the SAME judge's agreement, re-measured against Generator v2
    print(f"Judge agreement vs. Generator v1 (original validation): {agreement_v1:.0%}")  # => co-16
    print(f"Judge agreement vs. Generator v2 (after a silent swap): {agreement_v2:.0%}")  # => co-16

    assert agreement_v1 == 1.0, "the judge's original validation against Generator v1 must show full agreement"  # => co-16: sanity check on the fixture
    assert agreement_v2 < 0.5, "the SAME judge's agreement must have dropped sharply against Generator v2's different phrasing style"  # => co-16: the rule this example proves
    print(f"MATCH: re-measuring after the generator swap caught a real drift the team would NOT have seen without it ({agreement_v1:.0%} -> {agreement_v2:.0%})")  # => co-16
    # => co-16: this is why co-16 insists agreement is re-measured on a schedule -- a judge validated once is not validated forever
