# learning/code/ex-09-a-correct-no-go/correct_no_go.py
"""Worked Example 9: A Correct No-Go."""  # => co-06: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-06: the same typed-record shape ex-08 introduced, reused here on a DIFFERENT case


@dataclass(frozen=True)  # => co-06: frozen -- a decision-gate record should not mutate after it is built
class DecisionGateInputs:  # => co-06: the identical five ordered gate checks from ex-08, redefined for this file's self-containment
    measured_gap: bool  # => co-06: check 1
    prompting_exhausted: bool  # => co-06: check 2a
    retrieval_exhausted: bool  # => co-06: check 2b
    scoping_exhausted: bool  # => co-06: check 2c
    is_behaviour_shaped: bool  # => co-06: check 3
    data_obtainable: bool  # => co-06: check 4
    eval_possible: bool  # => co-06: check 5


def decide(inputs: DecisionGateInputs) -> tuple[bool, str]:  # => co-06: the identical gate logic from ex-08
    """Return (True, reason) only if every one of the five ordered checks passes; otherwise (False, reason) naming the first failure."""  # => co-06: documents decide's contract -- no runtime output, just sets its __doc__
    if not inputs.measured_gap:  # => co-06: check 1
        return False, "NO-GO: no measured gap -- there is nothing yet to fix"  # => co-06
    if not (inputs.prompting_exhausted and inputs.retrieval_exhausted and inputs.scoping_exhausted):  # => co-06: check 2
        return False, "NO-GO: at least one cheaper alternative was not genuinely exhausted"  # => co-06
    if not inputs.is_behaviour_shaped:  # => co-06: check 3 -- this is the check the pricing case will fail
        return False, "NO-GO: this gap is knowledge-shaped -- retrieval is the correct tool, not adaptation"  # => co-06
    if not inputs.data_obtainable:  # => co-06: check 4
        return False, "NO-GO: no obtainable dataset -- there is nothing to train on"  # => co-06
    if not inputs.eval_possible:  # => co-06: check 5
        return False, "NO-GO: no way to measure the result -- an unfalsifiable fine-tune is not worth running"  # => co-06
    return True, "GO: every ordered check passed -- adaptation is a defensible candidate"  # => co-06


if __name__ == "__main__":  # => co-06: entry point -- runs only when this file executes directly, not on import
    pricing_case = DecisionGateInputs(  # => co-04,co-06: the storage-limit pricing case from ex-03/ex-04
        measured_gap=True,  # => co-06: measured: the assistant states a stale storage limit
        prompting_exhausted=True,  # => co-06: tried instructing it to "always state the current limit" -- it still recites the stale figure
        retrieval_exhausted=False,  # => co-04,co-06: "exhausted" means TRIED AND INSUFFICIENT -- ex-04 showed retrieval fully SOLVES this, so it is not exhausted
        scoping_exhausted=True,  # => co-06: tried narrowing to just pricing questions -- the staleness persists regardless of scope
        is_behaviour_shaped=False,  # => co-06: this is a FACT, not a behaviour -- co-01's triage already flagged it knowledge-shaped
        data_obtainable=True,  # => co-06: hundreds of pricing Q&A pairs could technically be assembled
        eval_possible=True,  # => co-06: an eval could technically be built
    )  # => co-06: closes pricing_case
    decision, reason = decide(pricing_case)  # => co-06: run the gate
    print(f"Decision: {'GO' if decision else 'NO-GO'} -- {reason}")  # => co-06: prints the gate's verdict and its reason
    assert decision is False, "the pricing case must fail this gate"  # => co-06
    assert "alternative" in reason, "the failure must be attributed to a cheaper alternative left unexhausted, not a data or eval problem"  # => co-06
    print("MATCH: the gate correctly REJECTS this case at check 2 -- ex-04's already-working retrieval fix means the alternative is not yet exhausted")  # => co-06
    # => co-04,co-06: a documented NO-GO, backed by ex-04's already-working retrieval fix, is this course's definition of a passing outcome
