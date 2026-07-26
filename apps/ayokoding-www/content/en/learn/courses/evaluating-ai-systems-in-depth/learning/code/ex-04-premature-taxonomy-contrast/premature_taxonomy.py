"""Worked Example 4: A Borrowed Taxonomy Applied to the Same Sample Leaves Cases Unclassified."""  # => co-02: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

# A taxonomy borrowed from a GENERIC customer-support glossary -- written before anyone read
# Tasklight's own failures. This is the premature taxonomy this example contrasts against open
# coding (ex-03).
BORROWED_TAXONOMY = {"rude-tone", "slow-response", "wrong-language", "billing-error"}  # => co-02: none of these fit an AI agent's failures

FAILING_REPLIES = {  # => co-02: the SAME five cases ex-03 open-coded, reused here for a direct contrast
    "t-201": "I've closed ticket #4821 for you.",  # => co-02: wrong ticket acted on
    "t-202": "I've set the due date to 2026-13-40.",  # => co-02: an impossible calendar date
    "t-203": "Here's a summary: [no summary was generated]",  # => co-02: leaked placeholder text
    "t-204": "I've archived ticket #77 instead of closing it.",  # => co-02: wrong action on the right ticket
    "t-205": "Your team has 3 open critical bugs.",  # => co-02: undercounts the real total
}  # => co-02: closes FAILING_REPLIES


def classify_with_borrowed_taxonomy(reply: str) -> str | None:  # => co-02: forces each case into ONE of the pre-existing buckets
    """Return the first BORROWED_TAXONOMY bucket whose keyword appears in `reply`, or None if none fit."""  # => co-02: documents classify_with_borrowed_taxonomy's contract -- no runtime output, just sets its __doc__
    lowered = reply.lower()  # => co-02: case-insensitive keyword search against the borrowed labels
    keyword_by_bucket = {  # => co-02: a crude keyword stand-in for each borrowed bucket's real-world trigger
        "rude-tone": "stupid",  # => co-02: none of these keywords are remotely relevant to an AI agent's failures
        "slow-response": "timeout",  # => co-02
        "wrong-language": "translate",  # => co-02
        "billing-error": "invoice",  # => co-02
    }  # => co-02: closes keyword_by_bucket
    for bucket, keyword in keyword_by_bucket.items():  # => co-02: try each borrowed bucket in turn
        if keyword in lowered:  # => co-02: none of the five real cases below will ever match
            return bucket  # => co-02: unreachable for this sample, by construction
    return None  # => co-02: the case fits NO bucket in the borrowed taxonomy at all


if __name__ == "__main__":  # => co-02: entry point -- runs only when this file executes directly, not on import
    unclassified: list[str] = []  # => co-02: accumulates every case the borrowed taxonomy could not place
    for ticket_id, reply in FAILING_REPLIES.items():  # => co-02: apply the borrowed taxonomy to every real case
        bucket = classify_with_borrowed_taxonomy(reply)  # => co-02: this case's forced classification, if any
        print(f"{ticket_id}: {reply!r} -> {bucket}")  # => co-02: prints the (usually None) classification
        if bucket is None:  # => co-02: tracks cases the borrowed taxonomy has no bucket for
            unclassified.append(ticket_id)  # => co-02: this case does not fit ANY pre-existing label

    print(f"Unclassified: {len(unclassified)}/{len(FAILING_REPLIES)} -- {unclassified}")  # => co-02: the honest tally
    assert len(unclassified) == len(FAILING_REPLIES), "a borrowed taxonomy must fail to classify EVERY one of these real cases"  # => co-02
    print("MATCH: the borrowed taxonomy classifies zero of the five real failures -- it was written for a different problem")  # => co-02
    # => co-02: co-03's open coding (ex-03) produced FOUR usable tags from these SAME five cases -- a taxonomy built from the data beats one imported from elsewhere
