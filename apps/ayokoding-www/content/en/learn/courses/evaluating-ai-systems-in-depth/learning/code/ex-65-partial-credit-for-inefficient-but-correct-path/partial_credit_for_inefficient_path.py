"""Worked Example 65: Give Partial Credit to a Correct-but-Inefficient Trajectory, Not a Binary Verdict."""  # => co-19: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

REQUIRED_TOOL_SEQUENCE = ("search_ticket", "get_ticket", "update_priority")  # => co-18: the minimal, sanctioned path for this task

# A trajectory that contains every REQUIRED step, in the right relative order, but pads them
# with two harmless, non-required detours -- correct, but inefficient.
INEFFICIENT_TRAJECTORY = ("search_ticket", "list_recent_tickets", "get_ticket", "add_comment", "update_priority")  # => co-19: superset of the required path, in the same relative order


def contains_required_steps_in_order(candidate: tuple[str, ...], required: tuple[str, ...]) -> bool:  # => co-18: a "Superset" trajectory-match evaluator, per LangSmith's terminology -- extra steps allowed, order preserved
    """Pass iff every tool in `required` appears in `candidate`, in the SAME relative order, with extra steps allowed between them."""  # => co-18: documents contains_required_steps_in_order's contract -- no runtime output, just sets its __doc__
    required_iter = iter(required)  # => co-18: an iterator that advances only when the current required step is found
    current = next(required_iter, None)  # => co-18: the required step we are currently looking for
    for tool in candidate:  # => co-18: scans the candidate once, left to right
        if tool == current:  # => co-18: found the required step we were looking for
            current = next(required_iter, None)  # => co-18: advance to the next required step
    return current is None  # => co-18: True iff every required step was found, in order


def partial_credit_score(candidate: tuple[str, ...], required: tuple[str, ...]) -> float:  # => co-19: a graded score instead of a binary pass/fail
    """Return 1.0 if all required steps appear in order; otherwise the fraction of required steps found in order."""  # => co-19: documents partial_credit_score's contract -- no runtime output, just sets its __doc__
    if contains_required_steps_in_order(candidate, required):  # => co-19: the correctness gate -- efficiency does not matter for THIS check
        efficiency_penalty = len(required) / len(candidate)  # => co-19: rewards efficiency without punishing correctness -- 1.0 only if no extra steps at all
        return 0.7 + 0.3 * efficiency_penalty  # => co-19: returns this computed value to the caller -- a correctness floor of 0.7, plus an efficiency bonus up to 0.3
    return 0.0  # => co-19: an incorrect trajectory (missing or misordered required steps) gets zero, regardless of efficiency


if __name__ == "__main__":  # => co-19: entry point -- runs only when this file executes directly, not on import
    strict_match = INEFFICIENT_TRAJECTORY == REQUIRED_TOOL_SEQUENCE  # => co-19: what ex-36's STRICT matcher would say -- fails, since extra steps are present
    superset_match = contains_required_steps_in_order(INEFFICIENT_TRAJECTORY, REQUIRED_TOOL_SEQUENCE)  # => co-18: what a SUPERSET matcher says -- passes, extra steps allowed
    graded_score = partial_credit_score(INEFFICIENT_TRAJECTORY, REQUIRED_TOOL_SEQUENCE)  # => co-19: the graded, partial-credit score
    print(f"Strict match: {strict_match}")  # => co-19: prints the strict verdict
    print(f"Superset match (required steps present, in order): {superset_match}")  # => co-18: prints the superset verdict
    print(f"Partial-credit score: {graded_score:.2f}")  # => co-19: prints the graded score

    assert strict_match is False, "strict matching must fail this trajectory -- it is not byte-identical to the required sequence"  # => co-19
    assert superset_match is True, "superset matching must pass -- every required step is present, in the correct relative order"  # => co-18: the rule this example proves
    assert 0.0 < graded_score < 1.0, "the graded score must land strictly between 0 and 1 -- correct, but not maximally efficient"  # => co-19: the rule this example proves
    print(f"MATCH: strict matching wrongly zeroes out a correct trajectory, while partial credit ({graded_score:.2f}) correctly rewards correctness while still penalizing the two unnecessary detour steps")  # => co-19
    # => co-19: ex-66 next examines a DIFFERENT trajectory defect -- not inefficiency, but a schema mismatch across a subagent handoff
