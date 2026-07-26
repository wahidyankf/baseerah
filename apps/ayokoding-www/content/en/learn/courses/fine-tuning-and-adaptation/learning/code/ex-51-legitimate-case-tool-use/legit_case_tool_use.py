# learning/code/ex-51-legitimate-case-tool-use/legit_case_tool_use.py
"""Worked Example 51: Legitimate Case -- Tool Use."""  # => co-07: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

REQUIRED_TOOL_CALL_ACCURACY = 0.90  # => co-07: the bar Vantage's platform team set before letting this call fire unsupervised

# => co-07: fifteen trials asking the base model to call `create_refund(ticket_id, amount_cents, reason_code)` -- Vantage's internal API
TOOL_CALL_TRIALS = [  # => co-07: True = correct tool name, correct argument names, correct types; False = any mistake
    {"tool_name_correct": True, "args_correct": True},  # => co-07: trial 1 -- fully correct
    {"tool_name_correct": True, "args_correct": False},  # => co-07: trial 2 -- used "amount" instead of "amount_cents"
    {"tool_name_correct": True, "args_correct": True},  # => co-07: trial 3
    {"tool_name_correct": True, "args_correct": False},  # => co-07: trial 4 -- passed a dollar float instead of an integer cent count
    {"tool_name_correct": True, "args_correct": True},  # => co-07: trial 5
    {"tool_name_correct": False, "args_correct": False},  # => co-07: trial 6 -- called "issue_refund" instead of "create_refund"
    {"tool_name_correct": True, "args_correct": True},  # => co-07: trial 7
    {"tool_name_correct": True, "args_correct": True},  # => co-07: trial 8
    {"tool_name_correct": True, "args_correct": False},  # => co-07: trial 9 -- reason_code used a free-text string, not the fixed enum
    {"tool_name_correct": True, "args_correct": True},  # => co-07: trial 10 -- 6/10 fully correct on this first block
]  # => co-07: closes TOOL_CALL_TRIALS -- the base model handles this specific API poorly and consistently


def is_fully_correct(trial: dict[str, bool]) -> bool:  # => co-07: a tool call counts only if EVERY part of it is right
    """Pass iff both the tool name and its arguments are correct."""  # => co-07: documents is_fully_correct's contract -- no runtime output, just sets its __doc__
    return trial["tool_name_correct"] and trial["args_correct"]  # => co-07: a half-right tool call is still a wrong tool call


if __name__ == "__main__":  # => co-07: entry point -- runs only when this file executes directly, not on import
    correct_count = sum(is_fully_correct(t) for t in TOOL_CALL_TRIALS)  # => co-07: how many trials were fully correct
    accuracy = correct_count / len(TOOL_CALL_TRIALS)  # => co-07: the base model's measured tool-use accuracy
    print(f"Base model tool-call accuracy: {accuracy:.0%} ({correct_count}/{len(TOOL_CALL_TRIALS)})")  # => co-07
    print(f"Required before unsupervised use: {REQUIRED_TOOL_CALL_ACCURACY:.0%}")  # => co-07: the actual bar
    gate_passes = accuracy < REQUIRED_TOOL_CALL_ACCURACY  # => co-07: co-06's "alternatives exhausted" check for this specific case
    assert accuracy < 0.7, "the base model must show a real, persistent tool-use gap"  # => co-07
    assert gate_passes, "this case must pass the gate -- the base model handles this specific tool poorly and consistently"  # => co-07
    print(f"Gate passes (legitimate fine-tuning case): {gate_passes}")  # => co-07
    print("MATCH: a tool-use pattern the base model handles poorly, verified across repeated trials -- co-07's fifth legitimate case")  # => co-07
    # => co-07: unlike a prose format or a voice, a tool call has an exact right answer -- which makes this gap unusually easy to measure and fix
