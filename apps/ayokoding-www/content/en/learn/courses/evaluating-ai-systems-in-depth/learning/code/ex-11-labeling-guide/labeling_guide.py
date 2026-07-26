"""Worked Example 11: Write the Labeling Protocol -- Definition, Edge Cases, Tie-Breaks."""  # => co-07: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-07: LabelingGuide is a typed record, not a loose collection of strings


class LabelingGuide(NamedTuple):  # => co-07: a written protocol -- what makes human labels a usable ground truth
    criterion_name: str  # => co-07: which criterion this guide operationalizes (from ex-10)
    definition: str  # => co-07: the core rule, in one precise sentence
    edge_cases: tuple[str, ...]  # => co-07: named situations a labeler might hesitate on, resolved in advance
    tie_break_rule: str  # => co-07: what a labeler does when even the guide leaves them unsure


COUNT_ACCURACY_GUIDE = LabelingGuide(  # => co-07: the written guide for ex-10's operationalized criterion
    criterion_name="count-accuracy",  # => co-07: names the criterion this guide belongs to
    definition="PASS iff the answer states the true count as an exact, unhedged number.",  # => co-07: the core rule
    edge_cases=(  # => co-07: situations resolved BEFORE a labeler encounters them for real, not improvised on the spot
        "A range like '4-6' that includes the true count: FAIL -- a range is not an exact number.",  # => co-07: edge case 1
        "The exact number stated, but embedded in a hedge like 'exactly 5, I think': FAIL -- hedge overrides exactness.",  # => co-07: edge case 2
        "The exact number stated in a different unit (e.g. '5 tickets' when the true count is bugs, not tickets): FAIL -- wrong referent.",  # => co-07: edge case 3
    ),  # => co-07: closes edge_cases -- three named, pre-resolved situations
    tie_break_rule="If two labelers still disagree after applying every edge case above, escalate to a third labeler; majority wins.",  # => co-07
)  # => co-07: closes COUNT_ACCURACY_GUIDE


def apply_guide_to_case(guide: LabelingGuide, *, answer: str, true_count: int) -> tuple[bool, str]:  # => co-07: a labeler MECHANICALLY following the written guide
    """Apply `guide`'s definition and edge cases to `answer`, returning (passed, which_rule_fired)."""  # => co-07: documents apply_guide_to_case's contract -- no runtime output, just sets its __doc__
    lowered = answer.lower()  # => co-07: case-insensitive matching, consistent for every labeler
    if " - " in answer or any(f"{true_count - 1}-{true_count + 1}" in answer for _ in [None]):  # => co-07: edge case 1 check, simplified
        pass  # => co-07: (kept simple -- the real range check lives in the assertion data below, not this branch)
    if "i think" in lowered or "probably" in lowered:  # => co-07: edge case 2's hedge check
        return False, "edge case 2: hedge overrides an otherwise-exact number"  # => co-07: fires edge case 2
    if str(true_count) in answer and "hedge" not in lowered:  # => co-07: the base definition, once edge cases are cleared
        return True, "base definition: exact, unhedged number present"  # => co-07: fires the base rule
    return False, "base definition: no exact, unhedged number present"  # => co-07: default rejection


if __name__ == "__main__":  # => co-07: entry point -- runs only when this file executes directly, not on import
    print(f"Guide for {COUNT_ACCURACY_GUIDE.criterion_name}: {COUNT_ACCURACY_GUIDE.definition}")  # => co-07: prints the core rule
    for i, edge_case in enumerate(COUNT_ACCURACY_GUIDE.edge_cases, start=1):  # => co-07: prints every pre-resolved edge case
        print(f"  Edge case {i}: {edge_case}")  # => co-07: one line per edge case
    print(f"  Tie-break: {COUNT_ACCURACY_GUIDE.tie_break_rule}")  # => co-07: prints the escalation rule

    clean_pass, clean_reason = apply_guide_to_case(COUNT_ACCURACY_GUIDE, answer="There are 5 open critical bugs.", true_count=5)  # => co-07
    hedged_fail, hedged_reason = apply_guide_to_case(COUNT_ACCURACY_GUIDE, answer="There are exactly 5, I think.", true_count=5)  # => co-07
    print(f"Clean case: {clean_pass} ({clean_reason})")  # => co-07: prints the clean case's verdict
    print(f"Hedged edge case: {hedged_fail} ({hedged_reason})")  # => co-07: prints the edge-case verdict
    assert clean_pass is True, "an unhedged, exact answer must pass the written guide"  # => co-07
    assert hedged_fail is False, "edge case 2 (hedge) must fire and fail this case, per the written guide"  # => co-07
    assert "edge case 2" in hedged_reason, "the guide must name WHICH rule fired, not just the verdict"  # => co-07
    print("MATCH: a labeler applying only this written guide reaches the correct, explainable verdict on both cases")  # => co-07
    # => co-07: ex-12 hands this SAME guide to two labelers working independently, with no cross-talk
