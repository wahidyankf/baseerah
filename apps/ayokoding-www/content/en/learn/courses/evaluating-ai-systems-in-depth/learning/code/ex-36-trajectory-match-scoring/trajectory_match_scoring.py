"""Worked Example 36: Score a Trajectory Against a Reference Sequence -- Catch an Extra or Missing Tool Call."""  # => co-18: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

REFERENCE_TOOL_SEQUENCE = ("search_ticket", "get_ticket", "update_priority")  # => co-18: the expected, correct sequence of tool NAMES for this task

CANDIDATE_A_TOOL_SEQUENCE = ("search_ticket", "get_ticket", "update_priority")  # => co-18: matches the reference exactly
CANDIDATE_B_TOOL_SEQUENCE = ("search_ticket", "get_ticket", "close_ticket", "update_priority")  # => co-18: an EXTRA, unnecessary step inserted
CANDIDATE_C_TOOL_SEQUENCE = ("search_ticket", "update_priority")  # => co-18: a MISSING step -- never actually read the ticket first


def strict_trajectory_match(candidate: tuple[str, ...], reference: tuple[str, ...]) -> bool:  # => co-18: a "Strict" trajectory-match evaluator, per LangSmith's terminology
    """Pass iff `candidate` is EXACTLY equal to `reference`, in the same order, with no extra or missing steps."""  # => co-18: documents strict_trajectory_match's contract -- no runtime output, just sets its __doc__
    return candidate == reference  # => co-18: exact sequence equality -- the strictest of LangSmith's match modes


if __name__ == "__main__":  # => co-18: entry point -- runs only when this file executes directly, not on import
    verdict_a = strict_trajectory_match(CANDIDATE_A_TOOL_SEQUENCE, REFERENCE_TOOL_SEQUENCE)  # => co-18: candidate A vs. reference
    verdict_b = strict_trajectory_match(CANDIDATE_B_TOOL_SEQUENCE, REFERENCE_TOOL_SEQUENCE)  # => co-18: candidate B vs. reference
    verdict_c = strict_trajectory_match(CANDIDATE_C_TOOL_SEQUENCE, REFERENCE_TOOL_SEQUENCE)  # => co-18: candidate C vs. reference
    print(f"Candidate A (matches exactly): {verdict_a}")  # => co-18: prints A's verdict
    print(f"Candidate B (extra step): {verdict_b}")  # => co-18: prints B's verdict
    print(f"Candidate C (missing step): {verdict_c}")  # => co-18: prints C's verdict

    assert verdict_a is True, "an exactly-matching trajectory must pass strict matching"  # => co-18
    assert verdict_b is False, "a trajectory with an EXTRA, unnecessary step must fail strict matching"  # => co-18: the rule this example proves
    assert verdict_c is False, "a trajectory with a MISSING step must fail strict matching"  # => co-18: the rule this example proves
    print("MATCH: strict trajectory-match scoring catches both the extra step and the missing step, exactly as LangSmith's Strict mode is designed to")  # => co-18
    # => co-18: strict matching is deterministic, fast, and cheap -- but brittle to any legitimate variation; ex-65 relaxes this via partial credit
