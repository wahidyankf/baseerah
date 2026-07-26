"""Worked Example 66: Catch a Schema Mismatch at a Subagent Handoff Boundary."""  # => co-20: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import TypedDict  # => co-20: the handoff payload has an explicit, checkable shape


class TriageHandoffPayload(TypedDict):  # => co-20: the CONTRACT the orchestrator and the triage-subagent must both honor
    ticket_id: str  # => co-20: required field
    priority: str  # => co-20: required field
    requested_by: str  # => co-20: required field


REQUIRED_FIELDS = frozenset(TriageHandoffPayload.__required_keys__)  # => co-20: derives the required-field set directly from the TypedDict, not a hand-maintained duplicate list

ORCHESTRATOR_SENT_PAYLOAD: dict[str, str] = {"ticket_id": "4821", "priority": "high", "requested_by": "orchestrator"}  # => co-20: what the orchestrator actually sent -- complete, matches the contract
SUBAGENT_RECEIVED_PAYLOAD: dict[str, str] = {"ticket_id": "4821", "priority": "high"}  # => co-20: what the subagent actually received -- MISSING "requested_by", a real transport-layer bug


def validate_handoff_payload(payload: dict[str, str], required_fields: frozenset[str]) -> tuple[bool, frozenset[str]]:  # => co-20: checks a received payload against the CONTRACT, not against what was merely sent
    """Return `(is_valid, missing_fields)` for `payload` against `required_fields`."""  # => co-20: documents validate_handoff_payload's contract -- no runtime output, just sets its __doc__
    missing = required_fields - payload.keys()  # => co-20: the fields the contract requires but this payload lacks
    return (len(missing) == 0, missing)  # => co-20: returns this computed value to the caller


if __name__ == "__main__":  # => co-20: entry point -- runs only when this file executes directly, not on import
    sent_valid, sent_missing = validate_handoff_payload(ORCHESTRATOR_SENT_PAYLOAD, REQUIRED_FIELDS)  # => co-20: validate what was SENT
    received_valid, received_missing = validate_handoff_payload(SUBAGENT_RECEIVED_PAYLOAD, REQUIRED_FIELDS)  # => co-20: validate what was RECEIVED
    print(f"Sent payload valid: {sent_valid}, missing: {sent_missing}")  # => co-20: prints the sent-side validation
    print(f"Received payload valid: {received_valid}, missing: {received_missing}")  # => co-20: prints the received-side validation

    assert sent_valid is True, "the orchestrator's own sent payload must satisfy the handoff contract -- the bug is not on the sending side"  # => co-20: the rule this example proves
    assert received_valid is False, "the subagent's RECEIVED payload must fail validation -- 'requested_by' was lost somewhere in transport"  # => co-20: the rule this example proves
    assert received_missing == frozenset({"requested_by"}), "the missing-field set must precisely name 'requested_by', pinpointing exactly what the handoff boundary dropped"  # => co-20
    print(f"MATCH: the orchestrator's sent payload satisfies the contract, but the subagent's received payload is missing {sorted(received_missing)} -- the failure is localized to the HANDOFF, not either agent's own logic")  # => co-20
    # => co-20: this handoff-boundary check is what a trajectory evaluator applies at every subagent transition, not just at the final answer
