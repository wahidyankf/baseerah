"""Worked Example 12: Label the Same Items Independently, Without Cross-Contamination."""  # => co-07: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-07: LabelRecord is a typed record, not a bare tuple


class LabelRecord(NamedTuple):  # => co-07: one labeler's verdict on one case -- kept SEPARATE per labeler until both finish
    ticket_id: str  # => co-07: which case this label belongs to
    labeler_name: str  # => co-07: which labeler produced it -- never merged mid-process
    passed: bool  # => co-07: this labeler's own pass/fail verdict


CASES = {  # => co-07: five cases, unseen by either labeler before their own independent pass
    "t-401": ("There are 5 open critical bugs.", 5),  # => co-07: (answer, true_count)
    "t-402": ("Roughly 5 or so critical bugs remain.", 5),  # => co-07
    "t-403": ("There are 3 open critical bugs.", 5),  # => co-07: WRONG count
    "t-404": ("5 critical bugs are currently open.", 5),  # => co-07
    "t-405": ("About 6 critical bugs, give or take.", 5),  # => co-07: WRONG count, also hedged
}  # => co-07: closes CASES


def label_as_labeler_a(answer: str, true_count: int) -> bool:  # => co-07: labeler A -- works from CASES alone, never sees labeler B's output
    """Labeler A's independent application of the count-accuracy guide (ex-11)."""  # => co-07: documents label_as_labeler_a's contract -- no runtime output, just sets its __doc__
    exact = str(true_count) in answer  # => co-07: requirement 1
    hedged = any(w in answer.lower() for w in ("about", "roughly", "give or take"))  # => co-07: requirement 2
    return exact and not hedged  # => co-07: labeler A's own, unaided verdict


def label_as_labeler_b(answer: str, true_count: int) -> bool:  # => co-07: labeler B -- ALSO works from CASES alone, never sees labeler A's output
    """Labeler B's independent application of the identical count-accuracy guide (ex-11)."""  # => co-07: documents label_as_labeler_b's contract -- no runtime output, just sets its __doc__
    words = answer.lower().split()  # => co-07: a differently-coded (but equivalent) check -- proves it's independent, not copy-pasted
    has_number = str(true_count) in words or f"{true_count}" in answer  # => co-07: requirement 1, coded differently from labeler A
    hedge_words = {"about", "roughly", "give"}  # => co-07: requirement 2, coded differently from labeler A
    return has_number and not (hedge_words & set(words))  # => co-07: labeler B's own, unaided verdict


if __name__ == "__main__":  # => co-07: entry point -- runs only when this file executes directly, not on import
    labels_a: list[LabelRecord] = []  # => co-07: labeler A's own, private label set -- collected without seeing labeler B's
    labels_b: list[LabelRecord] = []  # => co-07: labeler B's own, private label set -- collected without seeing labeler A's
    for ticket_id, (answer, true_count) in CASES.items():  # => co-07: iterate all five cases once for each labeler
        labels_a.append(LabelRecord(ticket_id, "labeler-a", label_as_labeler_a(answer, true_count)))  # => co-07: labeler A labels independently
        labels_b.append(LabelRecord(ticket_id, "labeler-b", label_as_labeler_b(answer, true_count)))  # => co-07: labeler B labels independently

    for a, b in zip(labels_a, labels_b):  # => co-07: only NOW -- after both are fully collected -- do we compare them
        agree = a.passed == b.passed  # => co-07: per-case agreement, checked post hoc
        print(f"{a.ticket_id}: labeler-a={a.passed}, labeler-b={b.passed}, agree={agree}")  # => co-07: one line per case

    agreements = sum(1 for a, b in zip(labels_a, labels_b) if a.passed == b.passed)  # => co-07: total agreement count
    print(f"Agreement: {agreements}/{len(CASES)} cases")  # => co-07: the raw agreement tally
    assert agreements == 5, "an operationalized, written guide must produce full agreement across all five cases"  # => co-07
    disagreement_ids = [a.ticket_id for a, b in zip(labels_a, labels_b) if a.passed != b.passed]  # => co-07: which cases (if any) disagreed
    assert disagreement_ids == [], "no case may show a labeler disagreement once the guide is applied correctly"  # => co-07
    print("MATCH: two independently-labeling readers, never sharing intermediate verdicts, agree on all five cases")  # => co-07
    # => co-07: ex-13 next handles the case where independent labelers STILL disagree, via the guide's tie-break rule
