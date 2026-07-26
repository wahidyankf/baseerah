"""Worked Example 3: Open-Code a Failure Sample Without a Prior Taxonomy."""  # => co-02: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-02: OpenCode is a typed record pairing a tag to its quoted evidence


class FailingReply(NamedTuple):  # => co-02: one raw failing reply, read fresh -- no taxonomy applied yet
    ticket_id: str  # => co-02: a stable identifier for this failing case
    reply: str  # => co-02: the agent's actual reply, verbatim


class OpenCode(NamedTuple):  # => co-02: a tag INVENTED while reading, grounded in a quoted fragment
    ticket_id: str  # => co-02: which case this tag was invented for
    tag: str  # => co-02: a short, descriptive label the reader made up on the spot
    quoted_evidence: str  # => co-02: the exact substring of the reply that justifies the tag


FAILING_SAMPLE: list[FailingReply] = [  # => co-02: the same kind of raw sample ex-02 read, coded fresh here
    FailingReply("t-201", "I've closed ticket #4821 for you."),  # => co-02: wrong ticket number acted on
    FailingReply("t-202", "I've set the due date to 2026-13-40."),  # => co-02: an impossible calendar date
    FailingReply("t-203", "Here's a summary: [no summary was generated]"),  # => co-02: leaked placeholder text
    FailingReply("t-204", "I've archived ticket #77 instead of closing it."),  # => co-02: wrong action taken on the right ticket
    FailingReply("t-205", "Your team has 3 open critical bugs."),  # => co-02: a number that undercounts the real total
]  # => co-02: closes FAILING_SAMPLE -- five reader-fresh cases


def open_code(case: FailingReply, *, tag: str, evidence: str) -> OpenCode:  # => co-02: one reader's invented tag for one case
    """Attach a reader-invented `tag`, grounded in a `evidence` substring actually present in the reply."""  # => co-02: documents open_code's contract -- no runtime output, just sets its __doc__
    assert evidence in case.reply, "the quoted evidence must be a real substring of the reply being coded"  # => co-02: grounding check, not a demo assertion
    return OpenCode(case.ticket_id, tag, evidence)  # => co-02: returns this computed value to the caller


if __name__ == "__main__":  # => co-02: entry point -- runs only when this file executes directly, not on import
    coded = [  # => co-02: NO pre-existing taxonomy consulted -- every tag below was invented while reading
        open_code(FAILING_SAMPLE[0], tag="wrong-object-acted-on", evidence="#4821"),  # => co-02: tag 1, invented fresh
        open_code(FAILING_SAMPLE[1], tag="invalid-date-generated", evidence="2026-13-40"),  # => co-02: tag 2, invented fresh
        open_code(FAILING_SAMPLE[2], tag="placeholder-text-leaked", evidence="[no summary was generated]"),  # => co-02: tag 3
        open_code(FAILING_SAMPLE[3], tag="wrong-object-acted-on", evidence="archived ticket #77"),  # => co-02: tag 1 recurs -- a real pattern, not forced
        open_code(FAILING_SAMPLE[4], tag="undercounted-total", evidence="3 open critical"),  # => co-02: tag 4, invented fresh
    ]  # => co-02: closes coded -- five open codes, none borrowed from any existing list
    for code in coded:  # => co-02: prints every open code alongside its grounding evidence
        print(f"{code.ticket_id}: [{code.tag}] <- {code.quoted_evidence!r}")  # => co-02: one line per coded case

    all_grounded = all(  # => co-02: re-verify every tag's evidence is a real quoted substring, not paraphrase
        code.quoted_evidence in case.reply  # => co-02: checks the quote actually appears in the original reply
        for code, case in zip(coded, FAILING_SAMPLE)  # => co-02: pairs each code back to its source case, in order
    )  # => co-02: closes the all(...) check
    distinct_tags = {code.tag for code in coded}  # => co-02: how many genuinely distinct tags emerged
    print(f"Every tag grounded in a real quote: {all_grounded} | distinct tags: {sorted(distinct_tags)}")  # => co-02
    assert all_grounded, "every open code must be grounded in a quoted fragment of its own reply"  # => co-02: the rule this example proves
    assert len(distinct_tags) == 4, "five cases must produce four distinct tags, with one genuine repeat"  # => co-02: a real pattern, not five unique one-offs
    print("MATCH: every open code is grounded in quoted evidence, with one tag recurring naturally")  # => co-02: reached only if both asserts passed
    # => co-02: these tags -- not a borrowed taxonomy -- are the raw material ex-05 clusters into named failure modes
