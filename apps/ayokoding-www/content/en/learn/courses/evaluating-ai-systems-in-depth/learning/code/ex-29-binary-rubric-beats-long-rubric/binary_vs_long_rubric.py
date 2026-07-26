"""Worked Example 29: Contrast a Single-Question Binary Rubric With a Multi-Dimensional Scoring Sheet."""  # => co-15: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import TypedDict  # => co-15: LabeledCase types every field this measurement reads


class LabeledCase(TypedDict):  # => co-15: one case, human-labeled AND scored under BOTH rubric designs
    reply: str  # => co-15: the model reply under evaluation
    human_verdict: bool  # => co-08: the adjudicated, human-agreed correct verdict


CASES: list[LabeledCase] = [  # => co-15: five cases, scored under two different rubric DESIGNS on the same underlying criterion
    {"reply": "Deleted files stay in trash for exactly 30 days.", "human_verdict": True},  # => co-15
    {"reply": "Trash retention is around a month or so.", "human_verdict": False},  # => co-15
    {"reply": "The retention period is 30 days.", "human_verdict": True},  # => co-15
    {"reply": "Files are kept for a while before removal.", "human_verdict": False},  # => co-15
    {"reply": "30 days is the exact trash retention window.", "human_verdict": True},  # => co-15
]  # => co-15: closes CASES


def binary_rubric(reply: str) -> bool:  # => co-15: a SHORT, single-question rubric -- "does it state an exact number of days"
    """Binary rubric: does `reply` state an EXACT number of days, unhedged?"""  # => co-15: documents binary_rubric's contract -- no runtime output, just sets its __doc__
    return "30 days" in reply and not any(w in reply.lower() for w in ("around", "or so", "a while"))  # => co-15: ONE precise, checkable question


def long_multidim_rubric(reply: str) -> bool:  # => co-15: a LONGER rubric scoring several loosely-related dimensions, then averaging
    """Long rubric: average five loosely-defined dimensions (clarity, tone, precision, completeness, warmth), pass if average >= 3."""  # => co-15: documents long_multidim_rubric's contract -- no runtime output, just sets its __doc__
    clarity = 4 if len(reply) < 50 else 3  # => co-15: dimension 1 -- a vague, loosely-defined proxy
    tone = 4 if "!" not in reply else 3  # => co-15: dimension 2 -- another vague, loosely-defined proxy
    precision = 5 if "30 days" in reply else 2  # => co-15: dimension 3 -- the ONE dimension that actually matters here, diluted among four others
    completeness = 3  # => co-15: dimension 4 -- constant, contributes noise rather than signal
    warmth = 3  # => co-15: dimension 5 -- constant, contributes noise rather than signal
    average = (clarity + tone + precision + completeness + warmth) / 5  # => co-15: the diluted, averaged verdict
    return average >= 3.0  # => co-15: passes even when precision itself is genuinely weak, because other dimensions prop up the average


if __name__ == "__main__":  # => co-15: entry point -- runs only when this file executes directly, not on import
    binary_correct = sum(1 for c in CASES if binary_rubric(c["reply"]) == c["human_verdict"])  # => co-15: binary rubric's agreement count
    long_correct = sum(1 for c in CASES if long_multidim_rubric(c["reply"]) == c["human_verdict"])  # => co-15: long rubric's agreement count
    binary_agreement = binary_correct / len(CASES)  # => co-15: binary rubric's agreement rate
    long_agreement = long_correct / len(CASES)  # => co-15: long rubric's agreement rate
    print(f"Binary single-question rubric agreement: {binary_agreement:.0%} ({binary_correct}/{len(CASES)})")  # => co-15
    print(f"Long multi-dimensional rubric agreement: {long_agreement:.0%} ({long_correct}/{len(CASES)})")  # => co-15

    assert binary_agreement > long_agreement, "the binary rubric must agree with humans MORE than the diluted multi-dimensional one"  # => co-15: the rule this example proves
    print("MATCH: a short, single-question rubric agrees with humans better than a longer sheet where the relevant signal gets averaged away")  # => co-15
    # => co-15: this is why ex-30's rubric ITERATION converges toward fewer, sharper questions -- not toward more dimensions
