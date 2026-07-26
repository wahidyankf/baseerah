"""Worked Example 5: Merge Open Codes Into a Small, Named Failure Taxonomy."""  # => co-03: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-03: FailureMode is a typed record, not a bare dict


class OpenCode(NamedTuple):  # => co-02: reused shape from ex-03 -- a reader-invented tag, grounded in evidence
    ticket_id: str  # => co-02: which case this tag was invented for
    tag: str  # => co-02: the reader's own fine-grained label


class FailureMode(NamedTuple):  # => co-03: a NAMED cluster, merged from several related open codes
    mode_name: str  # => co-03: a short, mutually-intelligible name for the whole cluster
    member_tags: tuple[str, ...]  # => co-03: the fine-grained open codes this mode absorbs
    example_ticket_ids: tuple[str, ...]  # => co-03: at least one real case, so the mode is never just an abstract label


# A larger open-coded sample (20 tags across 4 fine-grained variants) -- the raw material a
# taxonomy gets clustered from.
OPEN_CODES: list[OpenCode] = [  # => co-03: twenty open codes, four fine-grained variants, ready to cluster
    OpenCode(f"t-{300 + i}", tag)  # => co-03: builds one OpenCode per (id, tag) pair below
    for i, tag in enumerate(  # => co-03: the raw fine-grained tags a reader actually produced
        [  # => co-03: opens the raw fine-grained tag list, ordered by variant
            "wrong-ticket-number",  # => co-03: variant A, occurrence 1 of 3
            "wrong-ticket-number",  # => co-03: variant A, occurrence 2 of 3
            "wrong-ticket-number",  # => co-03: variant A, x3
            "archived-instead-of-closed",  # => co-03: variant B, occurrence 1 of 2
            "archived-instead-of-closed",  # => co-03: variant B, x2
            "invalid-date-generated",  # => co-03: variant C, occurrence 1 of 4
            "invalid-date-generated",  # => co-03: variant C, occurrence 2 of 4
            "invalid-date-generated",  # => co-03: variant C, occurrence 3 of 4
            "invalid-date-generated",  # => co-03: variant C, x4
            "undercounted-total",  # => co-03: variant D, occurrence 1 of 3
            "undercounted-total",  # => co-03: variant D, occurrence 2 of 3
            "undercounted-total",  # => co-03: variant D, x3
            "leaked-placeholder-text",  # => co-03: a singleton, no cluster partner
        ]  # => co-03: closes the thirteen-item raw tag list
    )  # => co-03: closes enumerate(...)
]  # => co-03: closes OPEN_CODES -- thirteen coded cases across five fine-grained variants


def cluster_into_modes(codes: list[OpenCode]) -> list[FailureMode]:  # => co-03: the actual clustering step
    """Merge related fine-grained tags into a small set of NAMED failure modes."""  # => co-03: documents cluster_into_modes's contract -- no runtime output, just sets its __doc__
    # A human reading the tags decided these two fine-grained variants describe ONE real mode:
    # acting on the wrong object (wrong ticket, or the right ticket but the wrong action).
    tag_to_mode = {  # => co-03: the human clustering decision, made explicit and checkable
        "wrong-ticket-number": "wrong-object-acted-on",  # => co-03: merges into the same mode as the next line
        "archived-instead-of-closed": "wrong-object-acted-on",  # => co-03: a DIFFERENT fine-grained tag, SAME real mode
        "invalid-date-generated": "malformed-structured-output",  # => co-03: its own mode -- a distinct failure shape
        "undercounted-total": "incorrect-aggregate-count",  # => co-03: its own mode -- a distinct failure shape
        "leaked-placeholder-text": "malformed-structured-output",  # => co-03: the singleton joins an existing mode, not a new one
    }  # => co-03: closes tag_to_mode
    modes: dict[str, list[OpenCode]] = {}  # => co-03: accumulates every code under its assigned mode name
    for code in codes:  # => co-03: walk every open code exactly once
        mode_name = tag_to_mode[code.tag]  # => co-03: look up which mode this fine-grained tag belongs to
        modes.setdefault(mode_name, []).append(code)  # => co-03: group codes by their assigned mode
    return [  # => co-03: turn each mode's group into a FailureMode record
        FailureMode(  # => co-03: one record per named mode
            mode_name=name,  # => co-03: the mode's own name
            member_tags=tuple(sorted({c.tag for c in members})),  # => co-03: every distinct fine-grained tag this mode absorbs
            example_ticket_ids=tuple(sorted(c.ticket_id for c in members)),  # => co-03: real cases backing this mode
        )  # => co-03: closes this FailureMode(...) call
        for name, members in modes.items()  # => co-03: one iteration per distinct mode name
    ]  # => co-03: closes the list comprehension


if __name__ == "__main__":  # => co-03: entry point -- runs only when this file executes directly, not on import
    modes = cluster_into_modes(OPEN_CODES)  # => co-03: run the clustering step over all twenty codes
    for mode in sorted(modes, key=lambda m: m.mode_name):  # => co-03: print modes alphabetically for a stable, readable listing
        print(f"{mode.mode_name}: absorbs {mode.member_tags} -- examples {mode.example_ticket_ids}")  # => co-03: one line per mode

    total_codes_absorbed = sum(len(m.example_ticket_ids) for m in modes)  # => co-03: every code must land in exactly one mode
    assert total_codes_absorbed == len(OPEN_CODES), "every open code must be absorbed into exactly one named mode"  # => co-03
    assert len(modes) == 3, "twenty codes across five fine-grained tags must cluster into exactly three named modes"  # => co-03
    assert all(mode.example_ticket_ids for mode in modes), "every mode must have at least one real example case"  # => co-03: no abstract, empty mode
    print(f"MATCH: {len(OPEN_CODES)} open codes clustered into {len(modes)} named failure modes")  # => co-03: reached only if all three asserts passed
    # => co-03: three named modes -- not thirteen fine-grained tags -- are what ex-06's frequency count reports on next
