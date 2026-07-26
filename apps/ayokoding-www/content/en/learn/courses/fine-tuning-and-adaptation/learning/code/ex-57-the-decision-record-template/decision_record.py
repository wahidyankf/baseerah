# learning/code/ex-57-the-decision-record-template/decision_record.py
"""Worked Example 57: The Decision Record Template."""  # => co-06: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass, fields  # => co-06: a written record beats a verbal decision nobody can audit later


@dataclass(frozen=True)  # => co-06: frozen -- a decision record is a historical artefact, not a living document
class DecisionRecord:  # => co-06,co-08: the reusable written-record shape co-06's gate should always produce
    case_name: str  # => co-06: which gap this record is about
    measured_gap_pct: float  # => co-06: the actual measured gap size, not a description
    alternatives_tried: tuple[str, ...]  # => co-06: exactly which alternatives were genuinely attempted
    decision: str  # => co-06: "GO" or "NO-GO", never left implicit
    reason: str  # => co-06: the specific, traceable reason for the decision
    estimated_cost_usd: float  # => co-08: the honest cost estimate this decision is weighed against


def is_complete(record: DecisionRecord) -> tuple[bool, list[str]]:  # => co-06: a record is only useful if every field is actually filled in
    """Return (True, []) iff every field on `record` is non-empty/non-zero; otherwise (False, missing_field_names)."""  # => co-06: documents is_complete's contract -- no runtime output, just sets its __doc__
    missing: list[str] = []  # => co-06: accumulates any field that was left blank
    for f in fields(record):  # => co-06: check every field generically, so this stays correct if the record grows more fields later
        value = getattr(record, f.name)  # => co-06: this record's actual value for field f
        if value in ("", 0, 0.0, ()):  # => co-06: a blank string, a zero, or an empty tuple all count as "not actually filled in"
            missing.append(f.name)  # => co-06: name exactly which field is missing
    return len(missing) == 0, missing  # => co-06: returns this computed value to the caller


RECORD = DecisionRecord(  # => co-06: the completed record for the tool-use case from ex-51
    case_name="tool-use case (ex-51)",  # => co-06
    measured_gap_pct=0.40,  # => co-06: 60% accuracy against a 100% target, from ex-51
    alternatives_tried=("prompting", "structured-output-schema"),  # => co-06: both tried and found insufficient for tool calls specifically
    decision="GO",  # => co-06
    reason="tool-use pattern the base model handles poorly and consistently, per co-07",  # => co-06
    estimated_cost_usd=4_200.0,  # => co-08: the honest total, per ex-13's discipline
)  # => co-06: closes RECORD

if __name__ == "__main__":  # => co-06: entry point -- runs only when this file executes directly, not on import
    complete, missing_fields = is_complete(RECORD)  # => co-06: run the completeness check
    print(f"Decision record for {RECORD.case_name!r}: complete={complete}")  # => co-06
    print(f"  gap={RECORD.measured_gap_pct:.0%}, alternatives={list(RECORD.alternatives_tried)}, decision={RECORD.decision}, cost=${RECORD.estimated_cost_usd:,.0f}")  # => co-06
    assert complete, f"a real decision record must have no missing fields, found: {missing_fields}"  # => co-06
    incomplete_record = DecisionRecord(  # => co-06: a SECOND record, deliberately left half-finished, to prove the check actually catches gaps
        case_name="unnamed case",  # => co-06: filled in
        measured_gap_pct=0.0,  # => co-06: left blank
        alternatives_tried=(),  # => co-06: left blank
        decision="",  # => co-06: left blank
        reason="",  # => co-06: left blank
        estimated_cost_usd=0.0,  # => co-06: left blank -- 4 of 6 fields blank
    )  # => co-06: closes incomplete_record
    incomplete_ok, incomplete_missing = is_complete(incomplete_record)  # => co-06: run the SAME check on the blank record
    print(f"Blank record complete: {incomplete_ok}, missing fields: {incomplete_missing}")  # => co-06
    assert not incomplete_ok and len(incomplete_missing) >= 4, "a blank record must be flagged incomplete with several named missing fields"  # => co-06
    print("MATCH: the template both accepts a genuinely filled-out record and rejects a blank one, naming exactly what's missing")  # => co-06
    # => co-06,co-08: this exact template is what the capstone's decision phase writes for the one real gap it carries end to end
