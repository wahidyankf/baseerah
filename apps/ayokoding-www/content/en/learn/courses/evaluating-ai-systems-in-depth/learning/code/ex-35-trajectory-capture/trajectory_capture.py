"""Worked Example 35: Capture an Agent Run's Tool-Call Sequence as an Evaluable Object."""  # => co-18: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-18: ToolCall and Trajectory are typed records, not bare dicts


class ToolCall(NamedTuple):  # => co-18: one step in an agent's trajectory -- a single tool invocation
    tool_name: str  # => co-18: which tool the agent invoked
    arguments: dict[str, str]  # => co-18: the exact arguments passed to that tool
    result: str  # => co-18: the tool's own return value, as the agent observed it


class Trajectory(NamedTuple):  # => co-18: the FULL sequence of tool calls plus the agent's final answer -- an object in its own right
    steps: tuple[ToolCall, ...]  # => co-18: every tool call made, in order
    final_answer: str  # => co-18: what the agent ultimately told the user


def run_agent_and_capture_trajectory(user_request: str) -> Trajectory:  # => co-18: a mocked agent run -- captures EVERY step, not just the final answer
    """Run a mocked Tasklight agent against `user_request`, capturing its full tool-call trajectory."""  # => co-18: documents run_agent_and_capture_trajectory's contract -- no runtime output, just sets its __doc__
    del user_request  # => co-18: unused in this mock -- the trajectory below is a fixed, illustrative script
    steps = (  # => co-18: three ordered tool calls, exactly as the agent actually made them
        ToolCall("search_ticket", {"query": "offline sync bug"}, result="found ticket #4821"),  # => co-18: step 1
        ToolCall("get_ticket", {"ticket_id": "4821"}, result="status=open, priority=medium"),  # => co-18: step 2
        ToolCall("update_priority", {"ticket_id": "4821", "priority": "high"}, result="priority updated to high"),  # => co-18: step 3
    )  # => co-18: closes steps
    return Trajectory(steps=steps, final_answer="I found ticket #4821 and raised its priority to high.")  # => co-18: returns this computed value to the caller


if __name__ == "__main__":  # => co-18: entry point -- runs only when this file executes directly, not on import
    trajectory = run_agent_and_capture_trajectory("Please raise the priority on the offline sync bug.")  # => co-18: run the mocked agent, capture the whole trajectory
    for i, step in enumerate(trajectory.steps, start=1):  # => co-18: prints every captured step, in order
        print(f"Step {i}: {step.tool_name}({step.arguments}) -> {step.result}")  # => co-18: one line per tool call
    print(f"Final answer: {trajectory.final_answer}")  # => co-18: prints the agent's final answer

    assert len(trajectory.steps) == 3, "the captured trajectory must contain exactly the three tool calls the agent actually made"  # => co-18: the rule this example proves
    assert trajectory.steps[0].tool_name == "search_ticket", "the FIRST captured step must be the agent's first real action"  # => co-18: order is preserved, not just presence
    assert "4821" in trajectory.final_answer, "the final answer must be captured alongside the steps, not lost"  # => co-18
    print(f"MATCH: the trajectory is complete -- {len(trajectory.steps)} ordered steps plus a final answer, all captured as one evaluable object")  # => co-18
    # => co-18: this Trajectory object -- not just the final answer -- is what ex-36's match scorer and ex-37's process scoring evaluate next
