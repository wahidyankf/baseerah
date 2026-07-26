"""Worked Example 60: Iterate a Rubric Across Three Versions and Show Agreement Plateauing."""  # => co-15: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import Callable, TypedDict  # => co-15: types the rubric functions and the case fixture


class LabeledCase(TypedDict):  # => co-15: one case, human-labeled, re-scored across every rubric iteration
    reply: str  # => co-15: the model reply under evaluation
    human_verdict: bool  # => co-08: the adjudicated, human-agreed correct verdict


CASES: list[LabeledCase] = [  # => co-15: five cases -- one genuinely ambiguous case no amount of rubric wordsmithing can resolve
    {"reply": "Escalated to a human agent, ticket #4821.", "human_verdict": True},  # => co-15
    {"reply": "I'll take a look shortly.", "human_verdict": False},  # => co-15
    {"reply": "Escalating now, reference #77.", "human_verdict": True},  # => co-15
    {"reply": "This has been passed along.", "human_verdict": False},  # => co-15: genuinely ambiguous -- could mean escalated OR just noted
    {"reply": "Forwarded to support, ticket #903.", "human_verdict": True},  # => co-15
]  # => co-15: closes CASES

RubricFn = Callable[[str], bool]  # => co-15: every rubric iteration below has this exact shape


def rubric_v1(reply: str) -> bool:  # => co-15: v1 -- checks for the word "escalat"
    """v1: does `reply` contain a form of the word 'escalat'?"""  # => co-15: documents rubric_v1's contract -- no runtime output, just sets its __doc__
    return "escalat" in reply.lower()  # => co-15: catches two of three true cases, misses "forwarded"


def rubric_v2(reply: str) -> bool:  # => co-15: v2 -- adds "forwarded" as a synonym, an improvement over v1
    """v2: does `reply` contain 'escalat' OR 'forwarded', PLUS a specific ticket reference?"""  # => co-15: documents rubric_v2's contract -- no runtime output, just sets its __doc__
    has_synonym = "escalat" in reply.lower() or "forwarded" in reply.lower()  # => co-15: catches all three true positives now
    has_reference = "#" in reply  # => co-15: requires a concrete reference, filtering out vague claims
    return has_synonym and has_reference  # => co-15: fixes v1's blind spot on the "forwarded" case


def rubric_v3(reply: str) -> bool:  # => co-15: v3 -- adds yet MORE synonyms, hoping for further improvement
    """v3: v2's rule, PLUS 'passed along' as an additional synonym for escalation."""  # => co-15: documents rubric_v3's contract -- no runtime output, just sets its __doc__
    has_synonym = any(w in reply.lower() for w in ("escalat", "forwarded", "passed along"))  # => co-15: adds a third synonym
    has_reference = "#" in reply  # => co-15: SAME reference requirement as v2
    return has_synonym and has_reference  # => co-15: "passed along" case still fails -- it has no "#" reference at all


ITERATIONS: list[RubricFn] = [rubric_v1, rubric_v2, rubric_v3]  # => co-15: three successive iterations, in order


def measure(rubric: RubricFn, cases: list[LabeledCase]) -> float:  # => co-15: the SAME measurement applied to every iteration
    """Return the fraction of `cases` where `rubric(reply)` matches human_verdict."""  # => co-15: documents measure's contract -- no runtime output, just sets its __doc__
    return sum(1 for c in cases if rubric(c["reply"]) == c["human_verdict"]) / len(cases)  # => co-15


if __name__ == "__main__":  # => co-15: entry point -- runs only when this file executes directly, not on import
    agreements = [measure(rubric, CASES) for rubric in ITERATIONS]  # => co-06: re-measures agreement after EVERY iteration
    for version, agreement in enumerate(agreements, start=1):  # => co-15: prints each version's measured agreement
        print(f"Rubric v{version}: {agreement:.0%} agreement")  # => co-15: one line per iteration

    assert agreements[0] < agreements[1], "v1 to v2 must show a genuine, measured improvement"  # => co-15: the productive first iteration
    assert agreements[1] == agreements[2], "v2 to v3 must show NO further improvement -- a genuine plateau, not more progress"  # => co-15: the rule this example proves
    print(f"MATCH: agreement improves from v1 to v2 ({agreements[0]:.0%} -> {agreements[1]:.0%}), then plateaus at v3 ({agreements[2]:.0%}) -- the signal to stop iterating, not add a fourth synonym")  # => co-15
    # => co-15: a plateau is information -- it means the REMAINING disagreement is genuinely ambiguous, not that the rubric's wording is still the problem
