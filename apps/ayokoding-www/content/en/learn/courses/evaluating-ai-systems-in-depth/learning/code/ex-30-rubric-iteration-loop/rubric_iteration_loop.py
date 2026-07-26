"""Worked Example 30: Iterate a Rubric Until Agreement Clears the Stated Threshold."""  # => co-15: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import Callable, TypedDict  # => co-15: RubricVersion types each iteration; Callable types a rubric function


class LabeledCase(TypedDict):  # => co-15: one case, human-labeled, re-scored across every rubric iteration
    reply: str  # => co-15: the model reply under evaluation
    human_verdict: bool  # => co-08: the adjudicated, human-agreed correct verdict


CASES: list[LabeledCase] = [  # => co-15: five cases the rubric must correctly classify by the final iteration
    {"reply": "Your data is safe -- nothing was lost in that error.", "human_verdict": True},  # => co-15
    {"reply": "An error occurred during that operation.", "human_verdict": False},  # => co-15: cold, no reassurance
    {"reply": "I understand this is worrying -- everything has been fully recovered.", "human_verdict": True},  # => co-15: no safety-word match, only the acknowledgment path can catch this
    {"reply": "That's expected behavior per the error-handling spec.", "human_verdict": False},  # => co-15: cold, no reassurance
    {"reply": "Don't worry, nothing was lost -- your work is safe.", "human_verdict": True},  # => co-15
]  # => co-15: closes CASES

RubricFn = Callable[[str], bool]  # => co-15: every rubric iteration below has this exact shape


def rubric_v1(reply: str) -> bool:  # => co-15: v1 -- checks for a generic positive word, TOO LOOSE
    """v1: does `reply` contain any generically positive word?"""  # => co-15: documents rubric_v1's contract -- no runtime output, just sets its __doc__
    return any(w in reply.lower() for w in ("safe", "good", "fine", "expected"))  # => co-15: "expected" wrongly triggers on the cold, unreassuring reply


def rubric_v2(reply: str) -> bool:  # => co-15: v2 -- narrows to reassurance-specific words, still imperfect
    """v2: does `reply` contain a reassurance-specific word, excluding neutral technical terms?"""  # => co-15: documents rubric_v2's contract -- no runtime output, just sets its __doc__
    reassures = any(w in reply.lower() for w in ("safe", "intact", "don't worry"))  # => co-15: tighter than v1 -- drops "expected"
    return reassures  # => co-15: still misses the case that reassures WITHOUT using any of these exact words


def rubric_v3(reply: str) -> bool:  # => co-15: v3 -- adds explicit empathy-acknowledgment as a second, equally valid path
    """v3: reassures via an explicit safety word OR explicitly acknowledges the user's concern before reassuring."""  # => co-15: documents rubric_v3's contract -- no runtime output, just sets its __doc__
    safety_word = any(w in reply.lower() for w in ("safe", "intact", "don't worry"))  # => co-15: path 1 -- same as v2
    acknowledges_concern = "understand this is" in reply.lower()  # => co-15: path 2 -- catches the empathy-led case v2 missed
    return safety_word or acknowledges_concern  # => co-15: EITHER path satisfies the rubric


ITERATIONS: list[RubricFn] = [rubric_v1, rubric_v2, rubric_v3]  # => co-15: three successive iterations, in order


def measure(rubric: RubricFn, cases: list[LabeledCase]) -> float:  # => co-15: the SAME measurement applied to every iteration
    """Return the fraction of `cases` where `rubric(reply)` matches human_verdict."""  # => co-15: documents measure's contract -- no runtime output, just sets its __doc__
    return sum(1 for c in cases if rubric(c["reply"]) == c["human_verdict"]) / len(cases)  # => co-15


if __name__ == "__main__":  # => co-15: entry point -- runs only when this file executes directly, not on import
    agreements = [measure(rubric, CASES) for rubric in ITERATIONS]  # => co-06: re-measures agreement after EVERY iteration, not just once
    for version, agreement in enumerate(agreements, start=1):  # => co-15: prints each version's measured agreement
        print(f"Rubric v{version}: {agreement:.0%} agreement")  # => co-15: one line per iteration

    assert agreements[0] < agreements[1] < agreements[2], "each rubric iteration must measurably improve on the last"  # => co-15: the rule this example proves
    assert agreements[2] == 1.0, "the final iteration must clear a 100% agreement threshold on this fixture"  # => co-06: the stated threshold this loop iterates toward
    print("MATCH: three measured iterations, each an improvement, converge on a rubric that fully agrees with human labels")  # => co-15
    # => co-15,co-06: iterating toward a SHARPER, still-binary rubric -- not toward MORE dimensions -- is what ex-29 already showed wins
