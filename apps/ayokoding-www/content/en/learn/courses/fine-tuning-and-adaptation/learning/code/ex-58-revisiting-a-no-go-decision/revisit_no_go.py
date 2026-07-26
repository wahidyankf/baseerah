# learning/code/ex-58-revisiting-a-no-go-decision/revisit_no_go.py
"""Worked Example 58: Revisiting a No-Go Decision."""  # => co-06: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-06,co-30: a re-run gate needs the same typed record ex-08 used, dated this time


@dataclass(frozen=True)  # => co-06: frozen -- each gate run is its own immutable snapshot in time
class GateRun:  # => co-06,co-30: one dated run of the gate against the SAME case, so re-runs are comparable
    run_date: str  # => co-30: WHEN this gate was run -- a decision can go stale, just like an adapter (co-30)
    base_model_tool_use_accuracy: float  # => co-07: the one input that actually changed between runs
    required_accuracy: float  # => co-07: the bar, unchanged across runs


REQUIRED = 0.90  # => co-07: the same bar ex-51 used

FIRST_RUN = GateRun(run_date="2026-01-15", base_model_tool_use_accuracy=0.60, required_accuracy=REQUIRED)  # => co-30: the original ex-51 measurement
SECOND_RUN = GateRun(run_date="2026-07-20", base_model_tool_use_accuracy=0.93, required_accuracy=REQUIRED)  # => co-30: SIX MONTHS later, a new base-model release


def gate_passes(run: GateRun) -> bool:  # => co-06: the identical single-check gate from ex-51, reusable across dated runs
    """Return True iff this run's measured accuracy falls below the required bar (a legitimate gap exists)."""  # => co-06: documents gate_passes's contract -- no runtime output, just sets its __doc__
    return run.base_model_tool_use_accuracy < run.required_accuracy  # => co-06: a gap this large still justifies a candidate fine-tune


if __name__ == "__main__":  # => co-06: entry point -- runs only when this file executes directly, not on import
    first_decision = gate_passes(FIRST_RUN)  # => co-06: the original decision, made in January
    print(f"{FIRST_RUN.run_date}: base accuracy {FIRST_RUN.base_model_tool_use_accuracy:.0%}, gate passes (legitimate case): {first_decision}")  # => co-06
    assert first_decision, "the January run must find a legitimate gap -- this is ex-51's original finding"  # => co-06
    second_decision = gate_passes(SECOND_RUN)  # => co-06,co-30: RE-RUN the identical gate function, nothing else changed
    print(f"{SECOND_RUN.run_date}: base accuracy {SECOND_RUN.base_model_tool_use_accuracy:.0%}, gate passes (legitimate case): {second_decision}")  # => co-06
    assert not second_decision, "a newer base model release closing the gap on its own must flip this gate to NO-GO"  # => co-06,co-30
    print("MATCH: the SAME gate, re-run six months later against a newer base release, now correctly says NO-GO")  # => co-06,co-30
    # => co-06,co-30: a decision gate is not a one-time verdict -- co-30's drift applies to the DECISION itself, not just a shipped adapter
