"""Worked Example 10: Rewrite a Vague Criterion Until Two Labelers Agree."""  # => co-06: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

ANSWER = "Your team has about 5 open critical bugs, give or take."  # => co-06: the candidate reply both labelers score


def labeler_a_vague(answer: str) -> bool:  # => co-06: labeler A applying the VAGUE version of the criterion
    """Labeler A's private reading of 'the count should be accurate'."""  # => co-06: documents labeler_a_vague's contract -- no runtime output, just sets its __doc__
    return "5" in answer  # => co-06: labeler A: "it says 5, the true count is 5 -- passes"


def labeler_b_vague(answer: str) -> bool:  # => co-06: labeler B applying the SAME vague criterion, differently
    """Labeler B's private reading of 'the count should be accurate'."""  # => co-06: documents labeler_b_vague's contract -- no runtime output, just sets its __doc__
    return "give or take" not in answer  # => co-06: labeler B: "an ACCURATE count is stated with confidence, not hedged -- fails"


def labeler_a_operationalized(answer: str, *, true_count: int) -> tuple[bool, str]:  # => co-06: labeler A applying the REWRITTEN criterion
    """Pass iff `answer` states `true_count` as an unhedged, exact figure (no 'about'/'give or take')."""  # => co-06: documents labeler_a_operationalized's contract -- no runtime output, just sets its __doc__
    has_exact_number = str(true_count) in answer  # => co-06: requirement 1, made explicit and checkable
    is_hedged = any(word in answer.lower() for word in ("about", "give or take", "roughly", "approximately"))  # => co-06: requirement 2
    passed = has_exact_number and not is_hedged  # => co-06: BOTH conditions, spelled out, no room for private interpretation
    reason = f"exact number present: {has_exact_number}, hedged: {is_hedged}"  # => co-06: a reason anyone can re-check by eye
    return passed, reason  # => co-06: returns this computed value to the caller


def labeler_b_operationalized(answer: str, *, true_count: int) -> tuple[bool, str]:  # => co-06: labeler B's OWN implementation of the SAME rewritten criterion
    """Labeler B's independent implementation of the identical operationalized rule."""  # => co-06: documents labeler_b_operationalized's contract -- no runtime output, just sets its __doc__
    checks = {str(true_count) in answer, not any(w in answer.lower() for w in ("about", "give or take", "roughly", "approximately"))}  # => co-06
    passed = checks == {True}  # => co-06: passes only when BOTH independently-coded checks are True
    return passed, f"both requirements met: {passed}"  # => co-06: returns this computed value to the caller


if __name__ == "__main__":  # => co-06: entry point -- runs only when this file executes directly, not on import
    vague_a = labeler_a_vague(ANSWER)  # => co-06: labeler A's vague-criterion verdict
    vague_b = labeler_b_vague(ANSWER)  # => co-06: labeler B's vague-criterion verdict
    print(f"Vague criterion -- Labeler A: {vague_a} | Labeler B: {vague_b}")  # => co-06: prints the disagreement
    assert vague_a != vague_b, "the vague criterion must produce genuine labeler disagreement for this demo"  # => co-06

    op_a_passed, op_a_reason = labeler_a_operationalized(ANSWER, true_count=5)  # => co-06: labeler A applies the rewritten rule
    op_b_passed, op_b_reason = labeler_b_operationalized(ANSWER, true_count=5)  # => co-06: labeler B applies the SAME rewritten rule
    print(f"Operationalized -- Labeler A: {op_a_passed} ({op_a_reason})")  # => co-06: prints labeler A's reproducible verdict
    print(f"Operationalized -- Labeler B: {op_b_passed} ({op_b_reason})")  # => co-06: prints labeler B's reproducible verdict
    assert op_a_passed == op_b_passed, "the operationalized criterion must produce the SAME verdict for both labelers"  # => co-06
    assert op_a_passed is False, "a hedged answer must fail the operationalized precision requirement"  # => co-06: confirms the specific verdict
    print("MATCH: operationalizing the criterion turned genuine disagreement into identical, reproducible verdicts")  # => co-06
    # => co-06: agreement improved from a coin-flip disagreement to a guaranteed match -- ex-11 writes this rule down as a labeling guide
