# learning/code/ex-08-the-decision-procedure/decision_procedure.py
"""Worked Example 8: The Decision Procedure."""  # => co-06: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-06: a typed record beats five loose booleans passed positionally


@dataclass(frozen=True)  # => co-06: frozen -- a decision-gate record should not mutate after it is built
class DecisionGateInputs:  # => co-06: co-06's five ordered gate checks, made an actual checkable type
    measured_gap: bool  # => co-06: check 1 -- is there a real, SIZED gap (ex-01)?
    prompting_exhausted: bool  # => co-06: check 2a -- was prompting genuinely tried and insufficient (ex-05)?
    retrieval_exhausted: bool  # => co-06: check 2b -- was retrieval genuinely tried and insufficient (ex-04)?
    scoping_exhausted: bool  # => co-06: check 2c -- was scoping genuinely tried and insufficient (ex-07)?
    is_behaviour_shaped: bool  # => co-06: check 3 -- behaviour-shaped, not knowledge-shaped (ex-02)?
    data_obtainable: bool  # => co-06: check 4 -- can a few hundred consistent examples actually be assembled?
    eval_possible: bool  # => co-06: check 5 -- can the result be measured against the base afterward?


def decide(inputs: DecisionGateInputs) -> tuple[bool, str]:  # => co-06: the gate itself -- (go/no-go, the reason)
    """Return (True, reason) only if every one of the five ordered checks passes; otherwise (False, reason) naming the first failure."""  # => co-06: documents decide's contract -- no runtime output, just sets its __doc__
    if not inputs.measured_gap:  # => co-06: check 1 fails first, if it fails at all -- order matters
        return False, "NO-GO: no measured gap -- there is nothing yet to fix"  # => co-06
    if not (inputs.prompting_exhausted and inputs.retrieval_exhausted and inputs.scoping_exhausted):  # => co-06: check 2, all three
        return False, "NO-GO: at least one cheaper alternative was not genuinely exhausted"  # => co-06
    if not inputs.is_behaviour_shaped:  # => co-06: check 3
        return False, "NO-GO: this gap is knowledge-shaped -- retrieval is the correct tool, not adaptation"  # => co-06
    if not inputs.data_obtainable:  # => co-06: check 4
        return False, "NO-GO: no obtainable dataset -- there is nothing to train on"  # => co-06
    if not inputs.eval_possible:  # => co-06: check 5
        return False, "NO-GO: no way to measure the result -- an unfalsifiable fine-tune is not worth running"  # => co-06
    return True, "GO: every ordered check passed -- adaptation is a defensible candidate"  # => co-06: only reached if ALL five hold


if __name__ == "__main__":  # => co-06: entry point -- runs only when this file executes directly, not on import
    real_case = DecisionGateInputs(  # => co-06: the internal-vocabulary case (c-05 from ex-02) -- behaviour-shaped, genuinely resistant
        measured_gap=True,  # => co-06: measured: the assistant invents its own priority words instead of P1/P2/P3
        prompting_exhausted=True,  # => co-06: tried explicit instructions across many tickets -- drifts back within a few turns
        retrieval_exhausted=True,  # => co-06: tried injecting the vocabulary as retrieved context -- same drift
        scoping_exhausted=True,  # => co-06: tried narrowing to one ticket type -- the vocabulary drift persists regardless
        is_behaviour_shaped=True,  # => co-06: this is a VOCABULARY/style requirement, not a fact
        data_obtainable=True,  # => co-06: hundreds of correctly-labelled historical tickets already exist
        eval_possible=True,  # => co-06: a vocabulary-compliance scorer is trivial to build and run against the base
    )  # => co-06: closes real_case
    decision, reason = decide(real_case)  # => co-06: run the gate
    print(f"Decision: {'GO' if decision else 'NO-GO'} -- {reason}")  # => co-06: prints the gate's own verdict and its reason
    assert decision is True, "this case must pass every ordered check"  # => co-06
    assert reason.startswith("GO"), "a passing case's reason must start with GO"  # => co-06
    print("MATCH: the written gate produced a defensible GO, with a reason traceable to each of the five checks")  # => co-06
    # => co-06: this SAME decide() function is reused, unchanged, by ex-09 on a case that should fail it
