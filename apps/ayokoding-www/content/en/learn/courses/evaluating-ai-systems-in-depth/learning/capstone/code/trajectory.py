"""Capstone Step 4: Score the Agent's Trajectory Alongside Its Final Answer, With Step Attribution."""  # => co-18/co-19/co-20: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-18: every record below is typed, not a bare dict


class ToolCall(NamedTuple):  # => co-18: one step in a trajectory -- the same shape ex-35 first established
    tool_name: str  # => co-18: which tool was invoked
    result: str  # => co-18: what the tool returned
    step_is_correct: bool  # => co-20: ground truth -- was THIS step, in isolation, correct?


class Trajectory(NamedTuple):  # => co-18: the full sequence of steps plus a final answer
    case_id: str  # => co-18: which capstone case this trajectory belongs to
    steps: tuple[ToolCall, ...]  # => co-18: every tool call, in order
    final_answer: str  # => co-18: what the agent ultimately told the user


class CaseVerdict(NamedTuple):  # => co-19: the full, combined verdict for one trajectory -- process, outcome, and attribution together
    case_id: str  # => co-19
    process_passed: bool  # => co-19: did the trajectory follow the sanctioned tool sequence?
    outcome_passed: bool  # => co-19: is the final answer correct?
    first_failing_step_index: int | None  # => co-20: WHERE, if anywhere, the trajectory first went wrong


REFERENCE_TOOL_SEQUENCE = ("search_ticket", "get_ticket", "update_priority")  # => co-18: the sanctioned path for this task, reused from ex-36/ex-37

# Two capstone-scale trajectories: one a genuine right-answer-wrong-path case (per the syllabus's
# OWN acceptance criterion for this step), and one a fully correct trajectory for contrast.
RIGHT_ANSWER_WRONG_PATH = Trajectory(  # => co-19: the syllabus's own required scenario -- process fails, outcome passes
    case_id="traj-01",  # => co-18
    steps=(  # => co-18: SKIPPED get_ticket -- guessed the ticket's state instead of verifying it
        ToolCall("search_ticket", "found ticket #4821", step_is_correct=True),  # => co-20: step 0 -- correct
        ToolCall("update_priority", "priority updated to high", step_is_correct=False),  # => co-20: step 1 -- WRONG: acted without verifying via get_ticket first
    ),  # => co-18: closes steps
    final_answer="I found ticket #4821 and raised its priority to high.",  # => co-19: happens to be the CORRECT final answer anyway
)  # => co-18: closes RIGHT_ANSWER_WRONG_PATH

FULLY_CORRECT_TRAJECTORY = Trajectory(  # => co-19: both process and outcome correct, for contrast
    case_id="traj-02",  # => co-18
    steps=(  # => co-18: follows the full sanctioned sequence
        ToolCall("search_ticket", "found ticket #501", step_is_correct=True),  # => co-20: step 0 -- correct
        ToolCall("get_ticket", "status=open, priority=low", step_is_correct=True),  # => co-20: step 1 -- correct
        ToolCall("update_priority", "priority updated to high", step_is_correct=True),  # => co-20: step 2 -- correct
    ),  # => co-18: closes steps
    final_answer="I found ticket #501 and raised its priority to high.",  # => co-19: correct final answer
)  # => co-18: closes FULLY_CORRECT_TRAJECTORY


def process_score(trajectory: Trajectory, *, reference: tuple[str, ...] = REFERENCE_TOOL_SEQUENCE) -> bool:  # => co-19: the SAME process scorer as ex-37/ex-38
    """Pass iff `trajectory`'s tool names match `reference` exactly, in order."""  # => co-19: documents process_score's contract -- no runtime output, just sets its __doc__
    tool_names = tuple(step.tool_name for step in trajectory.steps)  # => co-19: extracts just the tool-name sequence
    return tool_names == reference  # => co-19: returns this computed value to the caller


def outcome_score(trajectory: Trajectory, *, expected_ticket_id: str = "4821") -> bool:  # => co-19: an outcome scorer keyed to THIS trajectory's own expected ticket
    """Pass iff `trajectory.final_answer` mentions the ticket ID actually found in its first step."""  # => co-19: documents outcome_score's contract -- no runtime output, just sets its __doc__
    found_id = next((step.result.split("#")[1].split()[0] for step in trajectory.steps if step.tool_name == "search_ticket" and "#" in step.result), None)  # => co-19: the ticket ID this trajectory's own search_ticket step actually found
    return found_id is not None and found_id in trajectory.final_answer  # => co-19: returns this computed value to the caller -- consistency between what was found and what was reported


def find_first_failing_step(trajectory: Trajectory) -> int | None:  # => co-20: the SAME step-attribution pattern as ex-39
    """Return the 0-based index of the first step where `step_is_correct` is False, or None."""  # => co-20: documents find_first_failing_step's contract -- no runtime output, just sets its __doc__
    for index, step in enumerate(trajectory.steps):  # => co-20: scans steps in order
        if not step.step_is_correct:  # => co-20: found the causing step
            return index  # => co-20: returns this computed value to the caller
    return None  # => co-20: every step was individually correct


def score_case(trajectory: Trajectory) -> CaseVerdict:  # => co-19: combines process, outcome, and attribution into ONE reported verdict per case
    """Return a `CaseVerdict` combining process scoring, outcome scoring, and step attribution for `trajectory`."""  # => co-19: documents score_case's contract -- no runtime output, just sets its __doc__
    return CaseVerdict(  # => co-19: returns this computed value to the caller
        case_id=trajectory.case_id,  # => co-18
        process_passed=process_score(trajectory),  # => co-19
        outcome_passed=outcome_score(trajectory),  # => co-19
        first_failing_step_index=find_first_failing_step(trajectory),  # => co-20
    )  # => co-19: closes the CaseVerdict(...) call


if __name__ == "__main__":  # => co-19: entry point -- runs only when this file executes directly, not on import
    verdict_1 = score_case(RIGHT_ANSWER_WRONG_PATH)  # => co-19: score the right-answer-wrong-path case
    verdict_2 = score_case(FULLY_CORRECT_TRAJECTORY)  # => co-19: score the fully correct case, for contrast
    print(f"{verdict_1.case_id}: process={verdict_1.process_passed}, outcome={verdict_1.outcome_passed}, first_failing_step={verdict_1.first_failing_step_index}")  # => co-19
    print(f"{verdict_2.case_id}: process={verdict_2.process_passed}, outcome={verdict_2.outcome_passed}, first_failing_step={verdict_2.first_failing_step_index}")  # => co-19

    assert verdict_1.process_passed is False, "traj-01 must FAIL process scoring -- it skipped get_ticket's verification step"  # => co-19: the syllabus's own required acceptance check
    assert verdict_1.outcome_passed is True, "traj-01 must PASS outcome scoring -- the final answer happens to be correct"  # => co-19: the syllabus's own required acceptance check
    assert verdict_1.first_failing_step_index == 1, "traj-01's failure must be attributed to step 1 (update_priority), the step that skipped verification"  # => co-20
    assert verdict_2.process_passed is True and verdict_2.outcome_passed is True, "traj-02, following the full sanctioned sequence, must pass BOTH process and outcome scoring"  # => co-19
    assert verdict_2.first_failing_step_index is None, "traj-02 has no failing step to attribute -- every step was individually correct"  # => co-20
    print("MATCH: traj-01 demonstrates the syllabus's own required right-answer-wrong-path case -- process fails while outcome passes -- with the failure correctly attributed to step 1")  # => co-19
    # => co-20: Step 5 next wires ALL of this (criteria, judge, trajectory scoring) into a noise-aware CI gate
