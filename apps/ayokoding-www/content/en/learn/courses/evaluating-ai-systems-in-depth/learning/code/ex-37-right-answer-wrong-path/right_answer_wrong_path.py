"""Worked Example 37: An Agent Reaching the Correct Output Through an Invalid Path."""  # => co-19: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-19: Trajectory is a typed record pairing steps and a final answer


class Trajectory(NamedTuple):  # => co-18: the same shape ex-35 captured -- steps plus a final answer
    tool_sequence: tuple[str, ...]  # => co-18: which tools were called, in order
    final_answer: str  # => co-18: what the agent ultimately told the user


REFERENCE_TOOL_SEQUENCE = ("search_ticket", "get_ticket", "update_priority")  # => co-18: the CORRECT, sanctioned path for this task

# The agent reached the right FINAL ANSWER, but by directly guessing the ticket ID and skipping
# get_ticket's verification step -- it got lucky, it did not verify.
ACTUAL_TRAJECTORY = Trajectory(  # => co-19: a trajectory that reaches the RIGHT outcome via a WRONG path
    tool_sequence=("search_ticket", "update_priority"),  # => co-19: skipped get_ticket -- never verified the ticket's current state before acting
    final_answer="I found ticket #4821 and raised its priority to high.",  # => co-19: happens to be the CORRECT final answer anyway
)  # => co-19: closes ACTUAL_TRAJECTORY

EXPECTED_FINAL_ANSWER = "I found ticket #4821 and raised its priority to high."  # => co-19: the reference, correct final answer


def outcome_score(trajectory: Trajectory, *, expected: str = EXPECTED_FINAL_ANSWER) -> bool:  # => co-19: scores ONLY the final answer, ignoring the path
    """Pass iff `trajectory.final_answer` equals `expected`, regardless of how it was reached."""  # => co-19: documents outcome_score's contract -- no runtime output, just sets its __doc__
    return trajectory.final_answer == expected  # => co-19: outcome scoring is BLIND to the path taken


def process_score(trajectory: Trajectory, *, reference: tuple[str, ...] = REFERENCE_TOOL_SEQUENCE) -> bool:  # => co-19: scores the PATH itself, independent of the outcome
    """Pass iff `trajectory.tool_sequence` matches the sanctioned `reference` path exactly."""  # => co-19: documents process_score's contract -- no runtime output, just sets its __doc__
    return trajectory.tool_sequence == reference  # => co-19: process scoring is BLIND to whether the outcome happened to be correct


if __name__ == "__main__":  # => co-19: entry point -- runs only when this file executes directly, not on import
    outcome_verdict = outcome_score(ACTUAL_TRAJECTORY)  # => co-19: does the final answer match, ignoring the path?
    process_verdict = process_score(ACTUAL_TRAJECTORY)  # => co-19: does the path match the sanctioned sequence?
    print(f"Actual tool sequence: {ACTUAL_TRAJECTORY.tool_sequence}")  # => co-19: prints the actual (skipped-verification) path
    print(f"Outcome score (final answer correct?): {outcome_verdict}")  # => co-19: prints the outcome verdict
    print(f"Process score (path matches reference?): {process_verdict}")  # => co-19: prints the process verdict

    assert outcome_verdict is True, "the final answer must be correct -- the agent got lucky"  # => co-19: the rule this example proves
    assert process_verdict is False, "the path must NOT match the reference -- get_ticket's verification step was skipped"  # => co-19: the rule this example proves
    assert outcome_verdict != process_verdict, "outcome and process scoring must genuinely disagree on this trajectory"  # => co-19
    print("MATCH: outcome scoring passes (the answer is right) while process scoring fails (the path skipped a required verification step) -- a right answer via a wrong path")  # => co-19
    # => co-19: ex-38 shows this exact skipped-verification path failing for real, on a neighbouring input where guessing does NOT get lucky
