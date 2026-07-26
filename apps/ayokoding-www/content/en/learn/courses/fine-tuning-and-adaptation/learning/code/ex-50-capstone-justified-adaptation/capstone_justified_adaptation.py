# learning/code/ex-50-capstone-justified-adaptation/capstone_justified_adaptation.py
"""Worked Example 50: Capstone-Justified Adaptation."""  # => co-06: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-06: one immutable row per arc phase, each citing the earlier example that actually proved it


class ArcPhase(NamedTuple):  # => co-06: a single phase of the full justified-adaptation arc, with its own pass/fail evidence
    phase: str  # => co-06: which phase of the arc this row is
    evidenced_by_example: str  # => co-06: which earlier worked example in THIS course actually produced this phase's evidence
    passed: bool  # => co-06: did this phase clear its own bar


# => co-01–co-32: the full arc, phase by phase, each row citing the example that actually measured it -- nothing here is asserted without a source
FULL_ARC: list[ArcPhase] = [  # => co-06: one row per phase, in the order the decision gate and capstone spec both require
    ArcPhase(phase="measured gap", evidenced_by_example="ex-01", passed=True),  # => co-06,co-25: the gap was real and sized, not assumed
    ArcPhase(phase="prompting exhausted", evidenced_by_example="ex-05", passed=True),  # => co-03: tried and measured insufficient for THIS gap's remainder
    ArcPhase(phase="retrieval exhausted", evidenced_by_example="ex-04", passed=True),  # => co-04: tried and measured insufficient for THIS gap's remainder
    ArcPhase(phase="scoping exhausted", evidenced_by_example="ex-07", passed=True),  # => co-05: tried and measured insufficient for THIS gap's remainder
    ArcPhase(phase="decision gate", evidenced_by_example="ex-08", passed=True),  # => co-06: the ordered gate, applied and documented
    ArcPhase(phase="total cost budgeted", evidenced_by_example="ex-13", passed=True),  # => co-08: data, compute, eval, and maintenance all costed up front
    ArcPhase(phase="licence and rights checked", evidenced_by_example="ex-15", passed=True),  # => co-31: verified BEFORE training, not after
    ArcPhase(phase="dataset consistency audited", evidenced_by_example="ex-20", passed=True),  # => co-12,co-10: no planted or accidental conflicts survived
    ArcPhase(phase="splits disjoint, no leakage", evidenced_by_example="ex-26", passed=True),  # => co-15,co-16: verified clean, not assumed clean
    ArcPhase(phase="adapter rank justified", evidenced_by_example="ex-31", passed=True),  # => co-20: chosen from a sweep, not defaulted
    ArcPhase(phase="early stopping on validation", evidenced_by_example="ex-40", passed=True),  # => co-23,co-24: stopped on the held-out signal, not the epoch count
    ArcPhase(phase="paired evaluation against base", evidenced_by_example="ex-35", passed=True),  # => co-25: the target-task improvement is evidence-supported
    ArcPhase(phase="forgetting-regression suite run", evidenced_by_example="ex-37", passed=True),  # => co-22,co-26: untouched capability was checked, not assumed intact
    ArcPhase(phase="served as swappable artefact", evidenced_by_example="ex-46", passed=True),  # => co-21,co-29: hot-swappable against the shared base
    ArcPhase(phase="version-pinned to base", evidenced_by_example="ex-48", passed=True),  # => co-30: the base-version dependency made explicit
    ArcPhase(phase="maintenance and retirement plan written", evidenced_by_example="ex-49", passed=True),  # => co-32: a planned, healthy end state, not an afterthought
]  # => co-06: closes FULL_ARC -- 16 phases, each with a named source


def arc_is_justified(arc: list[ArcPhase]) -> bool:  # => co-06: the whole adaptation is justified only if EVERY phase passed, not most of them
    """Return whether every `ArcPhase` in `arc` passed."""  # => co-06: documents arc_is_justified's contract -- no runtime output, just sets its __doc__
    return all(phase.passed for phase in arc)  # => co-06: returns this computed value to the caller


if __name__ == "__main__":  # => co-06: entry point -- runs only when this file executes directly, not on import
    for phase in FULL_ARC:  # => co-06: print the whole arc, phase by phase, with its evidence source
        status = "PASS" if phase.passed else "FAIL"  # => co-06
        print(f"  [{status}] {phase.phase} (evidenced by {phase.evidenced_by_example})")  # => co-06
    unique_examples_cited = {phase.evidenced_by_example for phase in FULL_ARC}  # => co-06: how many DISTINCT earlier examples actually back this arc
    print(f"Distinct examples cited as evidence: {len(unique_examples_cited)}")  # => co-06
    assert len(FULL_ARC) == 16, "the full arc must cover exactly the 16 phases the capstone spec and decision gate require"  # => co-06
    assert len(unique_examples_cited) == 16, "every phase must be backed by its OWN distinct example, not one example doing double duty"  # => co-06
    justified = arc_is_justified(FULL_ARC)  # => co-06: the final verdict
    print(f"Adaptation is justified end to end: {justified}")  # => co-06
    assert justified, "every phase in this scenario's arc must pass for the overall adaptation to be justified"  # => co-06
    print("MATCH: 16 phases, 16 distinct pieces of measured evidence, all passing -- a justified adaptation is a CITED arc, not a single retrospective claim")  # => co-06
    # => co-06: this file's own structure is the point -- 'co-01-co-32' in the syllabus means every concept touched this arc, evidenced, not merely mentioned
