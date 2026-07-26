"""Worked Example 80: A Miniature End-to-End Dry Run -- Trajectory, Judge, and Noise-Aware CI Gate Together."""  # => co-18: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import statistics  # => co-24: stdlib mean/stdev for the noise-floor step, reused from ex-44's pattern
from typing import NamedTuple  # => co-18: every stage below uses a typed record, not a bare dict


class Trajectory(NamedTuple):  # => co-18: the same shape ex-35 first captured
    tool_sequence: tuple[str, ...]  # => co-18: which tools were called, in order
    final_answer: str  # => co-18: what the agent ultimately told the user


REFERENCE_TOOL_SEQUENCE = ("search_ticket", "get_ticket", "update_priority")  # => co-18: the sanctioned path for this task
CANDIDATE_TRAJECTORY = Trajectory(tool_sequence=("search_ticket", "get_ticket", "update_priority"), final_answer="I found ticket #4821 and raised its priority to high.")  # => co-18: a candidate run to evaluate end to end


def process_score(trajectory: Trajectory, *, reference: tuple[str, ...] = REFERENCE_TOOL_SEQUENCE) -> bool:  # => co-19: the SAME process scorer pattern as ex-37/ex-38
    """Pass iff `trajectory.tool_sequence` matches `reference` exactly."""  # => co-19: documents process_score's contract -- no runtime output, just sets its __doc__
    return trajectory.tool_sequence == reference  # => co-19: returns this computed value to the caller


def mock_judge_on_final_answer(final_answer: str) -> bool:  # => co-09: a mocked, DIFFERENT judge model scoring just the final answer's content
    """Return True iff `final_answer` mentions the specific ticket ID and the priority action taken."""  # => co-09: documents mock_judge_on_final_answer's contract -- no runtime output, just sets its __doc__
    return "4821" in final_answer and "priority" in final_answer.lower()  # => co-09: returns this computed value to the caller


def build_case_verdict(trajectory: Trajectory) -> bool:  # => co-18: an OVERALL verdict requiring BOTH the process AND the outcome to be correct
    """Return True iff the trajectory passes BOTH process scoring and judge-scored outcome checking."""  # => co-19: documents build_case_verdict's contract -- no runtime output, just sets its __doc__
    return process_score(trajectory) and mock_judge_on_final_answer(trajectory.final_answer)  # => co-19: returns this computed value to the caller -- a trajectory only counts as a genuine pass if BOTH checks agree


# Five repeated dry-run passes of this same candidate trajectory through the pipeline above,
# standing in for five repeated CI runs -- establishes the noise floor for THIS combined check.
REPEATED_DRY_RUN_PASS_RATES = (0.92, 0.88, 0.90, 0.94, 0.86)  # => co-24: illustrative repeated pass rates for this combined trajectory+judge pipeline


if __name__ == "__main__":  # => co-18: entry point -- runs only when this file executes directly, not on import
    case_verdict = build_case_verdict(CANDIDATE_TRAJECTORY)  # => co-18: run the candidate through BOTH trajectory scoring and judge-based outcome scoring
    print(f"Trajectory: {CANDIDATE_TRAJECTORY.tool_sequence}")  # => co-18: prints the candidate trajectory
    print(f"Process score: {process_score(CANDIDATE_TRAJECTORY)}")  # => co-19: prints the process verdict
    print(f"Judge verdict on final answer: {mock_judge_on_final_answer(CANDIDATE_TRAJECTORY.final_answer)}")  # => co-09: prints the judge verdict
    print(f"Combined case verdict: {case_verdict}")  # => co-18: prints the overall verdict

    baseline = statistics.mean(REPEATED_DRY_RUN_PASS_RATES)  # => co-24: the measured baseline from repeated dry runs
    noise_floor = statistics.stdev(REPEATED_DRY_RUN_PASS_RATES)  # => co-24: the measured noise floor
    regression_bar = baseline - 2 * noise_floor  # => co-23: the SAME derivation formula as ex-45
    this_run_pass_rate = 1.0 if case_verdict else 0.0  # => co-23: this single candidate case, expressed as a pass rate for gate comparison
    merge_allowed = this_run_pass_rate >= regression_bar or case_verdict  # => co-23: for a single-case dry run, an outright pass always clears a bar derived from a multi-case baseline
    print(f"Noise-aware regression bar: {regression_bar:.1%} (baseline {baseline:.1%}, noise {noise_floor:.1%})")  # => co-23
    print(f"Merge allowed: {merge_allowed}")  # => co-23

    assert process_score(CANDIDATE_TRAJECTORY) is True, "the candidate's tool sequence must match the sanctioned reference path"  # => co-19: the rule this example proves
    assert mock_judge_on_final_answer(CANDIDATE_TRAJECTORY.final_answer) is True, "the judge must confirm the final answer names the right ticket and the right action"  # => co-09: the rule this example proves
    assert case_verdict is True, "the COMBINED verdict must require both the process check and the judge check to pass -- neither alone is sufficient"  # => co-18: the rule this example proves
    assert merge_allowed is True, "a fully-passing candidate must be allowed to merge under the noise-aware gate"  # => co-23: the rule this example proves
    print(  # => co-18: opens the final MATCH print, reached only if all three asserts above passed
        "MATCH: trajectory scoring (co-18/co-19), a validated judge (co-09), and a noise-aware CI gate (co-23/co-24) run together end to end on one candidate -- exactly the integrated system the course's own capstone builds in full"
    )  # => co-18
    # => co-18: this closes the Advanced tier's worked-example arc; the course's own five-step capstone assembles the same pieces at full scale next
