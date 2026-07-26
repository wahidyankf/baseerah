# learning/code/ex-02-behaviour-vs-knowledge-triage/triage.py
"""Worked Example 2: Behaviour-vs-Knowledge Triage."""  # => co-01: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from enum import Enum, auto  # => co-01: an explicit, exhaustive two-value classification -- not a loose string


class GapKind(Enum):  # => co-01: co-01's central distinction, made an actual checkable type
    BEHAVIOUR_SHAPED = auto()  # => co-01: "the model does not FORMAT/STYLE/ACT the way we want"
    KNOWLEDGE_SHAPED = auto()  # => co-01: "the model does not KNOW a fact"


# => co-01: five real complaints filed against Vantage's support assistant this quarter
COMPLAINTS: dict[str, str] = {  # => co-01: complaint id -> the raw stakeholder-reported text
    "c-01": "Replies never follow our four-section reply template, every agent reformats them by hand.",  # => co-01
    "c-02": "The assistant told a customer our Enterprise plan includes SSO, but that shipped last month and it doesn't know yet.",  # => co-01
    "c-03": "It answers in a casual tone even for security-incident tickets, which reads as dismissive.",  # => co-01
    "c-04": "It quoted last year's storage limit, which changed in the latest pricing update.",  # => co-01
    "c-05": "It refuses to use our internal ticket-priority vocabulary (P1/P2/P3), inventing its own words instead.",  # => co-01
}  # => co-01: closes COMPLAINTS -- a realistic, mixed inbox, deliberately not sorted by kind

CLASSIFICATION: dict[str, GapKind] = {  # => co-01: the analyst's own triage, checked against by-hand reasoning below
    "c-01": GapKind.BEHAVIOUR_SHAPED,  # => co-01: a FORMAT complaint -- shaping how it writes, not what it knows
    "c-02": GapKind.KNOWLEDGE_SHAPED,  # => co-01: a FACT complaint -- a plan feature that changed after training
    "c-03": GapKind.BEHAVIOUR_SHAPED,  # => co-01: a TONE/REGISTER complaint -- shaping style, not facts
    "c-04": GapKind.KNOWLEDGE_SHAPED,  # => co-01: a FACT complaint -- a price that changed after training
    "c-05": GapKind.BEHAVIOUR_SHAPED,  # => co-01: a VOCABULARY complaint -- shaping task behaviour, not facts
}  # => co-01: closes CLASSIFICATION


def justify(complaint_id: str, kind: GapKind) -> str:  # => co-01: makes each classification checkable against the raw text
    """Return the one word in the raw complaint text that justifies the classification."""  # => co-01: documents justify's contract -- no runtime output, just sets its __doc__
    text = COMPLAINTS[complaint_id]  # => co-01: the raw text this classification must be traceable to
    behaviour_markers = ("template", "tone", "vocabulary", "reads as")  # => co-01: words signalling FORM, not facts
    knowledge_markers = ("plan includes", "changed", "storage limit", "doesn't know")  # => co-01: words signalling FACTS
    markers = behaviour_markers if kind is GapKind.BEHAVIOUR_SHAPED else knowledge_markers  # => co-01: pick the right marker set
    found = [marker for marker in markers if marker in text]  # => co-01: which markers actually appear in this complaint
    return found[0] if found else "no marker found"  # => co-01: the evidence, or an honest admission of none


if __name__ == "__main__":  # => co-01: entry point -- runs only when this file executes directly, not on import
    for cid, kind in CLASSIFICATION.items():  # => co-01: one line per complaint, showing the classification AND its evidence
        evidence = justify(cid, kind)  # => co-01: the specific text fragment backing this call
        print(f"{cid}: {kind.name} (evidence: {evidence!r})")  # => co-01: prints the classification with its justification
        assert evidence != "no marker found", f"{cid}'s classification must be traceable to its own text"  # => co-01
    behaviour_count = sum(1 for k in CLASSIFICATION.values() if k is GapKind.BEHAVIOUR_SHAPED)  # => co-01: tally
    knowledge_count = sum(1 for k in CLASSIFICATION.values() if k is GapKind.KNOWLEDGE_SHAPED)  # => co-01: tally
    print(f"Behaviour-shaped: {behaviour_count} | Knowledge-shaped: {knowledge_count}")  # => co-01: the split this inbox actually contains
    assert behaviour_count == 3 and knowledge_count == 2, "this inbox must split 3 behaviour / 2 knowledge"  # => co-01
    print("MATCH: every complaint classified with traceable evidence -- fine-tuning is a candidate only for the 3 behaviour-shaped ones")  # => co-01
    # => co-01: this triage is the FIRST gate -- a knowledge-shaped complaint never reaches the rest of this course's decision procedure
