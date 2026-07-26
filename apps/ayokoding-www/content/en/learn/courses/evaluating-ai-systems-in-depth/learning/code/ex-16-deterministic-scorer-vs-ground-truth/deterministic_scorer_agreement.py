"""Worked Example 16: Measure a Cheap Deterministic Scorer Against Human Labels Before Reaching for a Judge."""  # => co-08: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import re  # => co-17: a deterministic scorer backed by a plain regex, no model call needed
from typing import TypedDict  # => co-08: GroundTruthCase types the ground-truth set this scorer is measured against


class GroundTruthCase(TypedDict):  # => co-08: ex-14's schema, minimal fields needed for this measurement
    answer: str  # => co-08: the model reply under test
    human_verdict: bool  # => co-08: the adjudicated, human-agreed correct verdict


GROUND_TRUTH_SET: list[GroundTruthCase] = [  # => co-08: eight adjudicated cases -- the reference this scorer is measured against
    {"answer": "There are 5 open critical bugs.", "human_verdict": True},  # => co-08
    {"answer": "There are 3 open critical bugs.", "human_verdict": False},  # => co-08
    {"answer": "Around 5, roughly.", "human_verdict": False},  # => co-08
    {"answer": "Exactly 5 critical bugs.", "human_verdict": True},  # => co-08
    {"answer": "5 critical bugs remain open right now.", "human_verdict": True},  # => co-08
    {"answer": "Several critical bugs are open.", "human_verdict": False},  # => co-08: vague, no exact number at all
    {"answer": "5-6 critical bugs, depending how you count.", "human_verdict": False},  # => co-08: a range, not exact -- humans correctly reject it
    {"answer": "The count is 5.", "human_verdict": True},  # => co-08
]  # => co-08: closes GROUND_TRUTH_SET


def deterministic_count_scorer(answer: str, *, true_count: int = 5) -> bool:  # => co-17: co-06's operationalized criterion, made executable, cheaply
    """Pass iff `answer` contains the exact `true_count`, with no adjoining range and no hedge word."""  # => co-17: documents deterministic_count_scorer's contract -- no runtime output, just sets its __doc__
    pattern = rf"(?<!\d)(?<!-){true_count}(?!\d)(?!-)"  # => co-17: matches a bare number, rejecting "5-6" ranges on either side
    has_exact_number = re.search(pattern, answer) is not None  # => co-17: requirement 1, from ex-10's operationalized criterion
    is_hedged = any(w in answer.lower() for w in ("about", "roughly", "give or take", "approximately"))  # => co-17: requirement 2
    return has_exact_number and not is_hedged  # => co-17: cheap, deterministic, and never drifts between runs


def measure_agreement(cases: list[GroundTruthCase]) -> float:  # => co-10: agreement measurement -- required before trusting ANY scorer, deterministic or judge
    """Return the fraction of `cases` where the deterministic scorer's verdict matches the human verdict."""  # => co-10: documents measure_agreement's contract -- no runtime output, just sets its __doc__
    matches = sum(1 for case in cases if deterministic_count_scorer(case["answer"]) == case["human_verdict"])  # => co-10: per-case agreement
    return matches / len(cases)  # => co-10: returns this computed value to the caller


if __name__ == "__main__":  # => co-10: entry point -- runs only when this file executes directly, not on import
    for case in GROUND_TRUTH_SET:  # => co-10: prints the scorer's verdict next to the human verdict, per case
        scorer_verdict = deterministic_count_scorer(case["answer"])  # => co-17: this case's scorer verdict
        print(f"{case['answer']!r} -> scorer={scorer_verdict}, human={case['human_verdict']}")  # => co-10: one line per case

    agreement = measure_agreement(GROUND_TRUTH_SET)  # => co-10: the measured agreement statistic, BEFORE any judge is even considered
    print(f"Deterministic scorer agreement with ground truth: {agreement:.0%}")  # => co-10: the headline number
    assert agreement == 1.0, "this deterministic scorer must agree with every human label in this ground-truth set"  # => co-10: the rule this example proves
    print("MATCH: a cheap, deterministic scorer reaches 100% agreement -- no judge model is needed for THIS criterion")  # => co-10
    # => co-08,co-17: reaching for an LLM judge is unnecessary here -- ex-17 introduces a judge only for a criterion this simple approach genuinely cannot reach
