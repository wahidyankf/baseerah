"""Worked Example 13: Adjudicate Disagreements by the Written Rule."""  # => co-07: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-07: Adjudication is a typed record, not a bare tuple


class LabelPair(NamedTuple):  # => co-07: two labelers' verdicts on the SAME case, genuinely disagreeing
    ticket_id: str  # => co-07: which case is under dispute
    labeler_a_verdict: bool  # => co-07: labeler A's own verdict
    labeler_b_verdict: bool  # => co-07: labeler B's own, DIFFERENT verdict


class Adjudication(NamedTuple):  # => co-07: the recorded resolution -- every disagreement must end here
    ticket_id: str  # => co-07: ties the resolution back to its disputed case
    final_verdict: bool  # => co-07: the adjudicated, final answer
    resolution_rule: str  # => co-07: WHICH rule resolved it -- never left unrecorded


# Two cases where the two labelers from ex-12's guide genuinely disagree -- a deliberately harder
# pair than ex-12's five clean cases.
DISPUTED_PAIRS: list[LabelPair] = [  # => co-07: genuine disagreements needing adjudication
    LabelPair("t-501", labeler_a_verdict=True, labeler_b_verdict=False),  # => co-07: A says pass, B says fail
    LabelPair("t-502", labeler_a_verdict=False, labeler_b_verdict=True),  # => co-07: A says fail, B says pass
]  # => co-07: closes DISPUTED_PAIRS

THIRD_LABELER_VERDICTS = {"t-501": False, "t-502": True}  # => co-07: the tie-breaking third labeler's own, independent verdicts


def adjudicate(pair: LabelPair, *, third_verdict: bool) -> Adjudication:  # => co-07: applies the WRITTEN tie-break rule from ex-11's guide
    """Resolve a disagreement by majority vote among labeler A, labeler B, and a third labeler."""  # => co-07: documents adjudicate's contract -- no runtime output, just sets its __doc__
    votes = [pair.labeler_a_verdict, pair.labeler_b_verdict, third_verdict]  # => co-07: three votes, per the written tie-break rule
    final = sum(votes) >= 2  # => co-07: majority wins -- at least two of three votes must agree
    return Adjudication(pair.ticket_id, final, resolution_rule="majority-of-three (ex-11's written tie-break rule)")  # => co-07


if __name__ == "__main__":  # => co-07: entry point -- runs only when this file executes directly, not on import
    resolutions = [  # => co-07: run adjudication over EVERY disputed pair -- none may go unresolved
        adjudicate(pair, third_verdict=THIRD_LABELER_VERDICTS[pair.ticket_id])  # => co-07: pulls the matching third-labeler verdict
        for pair in DISPUTED_PAIRS  # => co-07: one adjudication per disputed case
    ]  # => co-07: closes resolutions
    for r in resolutions:  # => co-07: prints every resolution, including the rule that produced it
        print(f"{r.ticket_id}: final={r.final_verdict} (via {r.resolution_rule})")  # => co-07: one line per resolved case

    assert len(resolutions) == len(DISPUTED_PAIRS), "every disagreement must resolve to a recorded decision"  # => co-07: the floor this example demonstrates
    assert all(r.resolution_rule for r in resolutions), "every resolution must name the rule that produced it"  # => co-07: no silent, unrecorded decision
    by_id = {r.ticket_id: r.final_verdict for r in resolutions}  # => co-07: lookup, for the two targeted checks below
    assert by_id["t-501"] is False, "t-501's majority (A=True, B=False, third=False) must resolve to False"  # => co-07: 1 vote True, 2 votes False
    assert by_id["t-502"] is True, "t-502's majority (A=False, B=True, third=True) must resolve to True"  # => co-07: 2 votes True, 1 vote False
    print("MATCH: every disputed case resolves to a recorded verdict via the written majority-of-three rule")  # => co-07: reached only if all four asserts passed
    # => co-07: no case in this course's labeling pipeline ever ships with an unresolved disagreement -- ex-14 assembles the resolved labels into the ground-truth set
