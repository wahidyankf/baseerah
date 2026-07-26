# learning/code/ex-06-structured-output-closes-the-gap/structured_output.py
"""Worked Example 6: Structured Output Closes the Gap."""  # => co-03: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import TypedDict  # => co-03: the schema this example enforces instead of training for it


class TriageResult(TypedDict):  # => co-03: the exact shape Vantage's triage pipeline needs downstream
    ticket_id: str  # => co-03: which ticket this triage result belongs to
    category: str  # => co-03: one of a fixed label set
    priority: str  # => co-03: one of "P1", "P2", "P3"


VALID_CATEGORIES = {"billing", "access", "bug", "feature-request"}  # => co-03: the fixed category vocabulary
VALID_PRIORITIES = {"P1", "P2", "P3"}  # => co-03: the fixed priority vocabulary

# => co-03: free-text triage notes from the CURRENT prompt -- the model was only asked to "triage this ticket"
FREETEXT_TRIAGE_NOTES = [  # => co-03: five raw notes -- inconsistent shape, hard to parse reliably downstream
    "This looks like a billing issue, probably medium priority.",  # => co-03: no fixed vocabulary used at all
    "Access problem, seems urgent, P1 I'd say.",  # => co-03: close, but "urgent" is not a valid label
    "category=bug priority=P2",  # => co-03: accidentally matches the target shape
    "Feature request, low priority (P3).",  # => co-03: close, but not the exact required shape
    "billing / P1",  # => co-03: partially matches
]  # => co-03: closes FREETEXT_TRIAGE_NOTES -- only ONE of these five would parse cleanly downstream

# => co-03: the SAME five tickets, now the prompt REQUIRES a fixed-shape structured response and each is parsed + validated
STRUCTURED_TRIAGE_RESULTS: list[TriageResult] = [  # => co-03: zero training happened -- only the OUTPUT CONTRACT changed
    {"ticket_id": "t-11", "category": "billing", "priority": "P2"},  # => co-03: schema-valid
    {"ticket_id": "t-12", "category": "access", "priority": "P1"},  # => co-03: schema-valid
    {"ticket_id": "t-13", "category": "bug", "priority": "P2"},  # => co-03: schema-valid
    {"ticket_id": "t-14", "category": "feature-request", "priority": "P3"},  # => co-03: schema-valid
    {"ticket_id": "t-15", "category": "billing", "priority": "P1"},  # => co-03: schema-valid -- 5/5
]  # => co-03: closes STRUCTURED_TRIAGE_RESULTS


def is_schema_valid(result: TriageResult) -> bool:  # => co-03: the deterministic check -- structure, not judgment
    """Pass iff category and priority are both drawn from their fixed vocabularies."""  # => co-03: documents is_schema_valid's contract -- no runtime output, just sets its __doc__
    return result["category"] in VALID_CATEGORIES and result["priority"] in VALID_PRIORITIES  # => co-03: both must hold


if __name__ == "__main__":  # => co-03: entry point -- runs only when this file executes directly, not on import
    freetext_parseable = sum(  # => co-03: count how many free-text notes would parse cleanly downstream
        1
        for note in FREETEXT_TRIAGE_NOTES
        if any(cat in note for cat in VALID_CATEGORIES) and any(pri in note for pri in VALID_PRIORITIES)  # => co-03
    )  # => co-03: closes the count
    freetext_rate = freetext_parseable / len(FREETEXT_TRIAGE_NOTES)  # => co-03: the format-reliability gap BEFORE enforcing a schema
    print(f"Free-text notes reliably parseable: {freetext_rate:.0%} ({freetext_parseable}/{len(FREETEXT_TRIAGE_NOTES)})")  # => co-03
    structured_valid = sum(is_schema_valid(r) for r in STRUCTURED_TRIAGE_RESULTS)  # => co-03: count schema-valid structured results
    structured_rate = structured_valid / len(STRUCTURED_TRIAGE_RESULTS)  # => co-03: the SAME question, AFTER enforcing structured output
    print(f"Schema-enforced results valid: {structured_rate:.0%} ({structured_valid}/{len(STRUCTURED_TRIAGE_RESULTS)})")  # => co-03
    assert freetext_rate < 0.5, "free-text triage notes must show a real format-reliability gap"  # => co-03
    assert structured_rate == 1.0, "enforcing a schema must make the format problem disappear entirely"  # => co-03
    print("MATCH: the format problem disappeared once the OUTPUT CONTRACT was enforced -- no fine-tuning needed")  # => co-03
    # => co-03: this is co-03's second alternative -- often a schema, not a weight update, is what "consistent format" actually needs
