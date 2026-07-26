"""Worked Example 2: Sample and Read Failing Outputs Into a Review Sheet."""  # => co-01: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-01: ReviewRow is a typed record, not a bare tuple or dict


class FailingCase(NamedTuple):  # => co-01: one raw, real ticket/reply pair flagged as a failure
    ticket_id: str  # => co-01: a stable identifier for this failing case
    reply: str  # => co-01: the agent's actual reply, verbatim -- read, not summarized
    reported_problem: str  # => co-01: the ORIGINAL, one-line user complaint that flagged this case


class ReviewRow(NamedTuple):  # => co-01: what "reading" a case produces -- an observation, not a score
    ticket_id: str  # => co-01: ties this observation back to its source case
    observation: str  # => co-01: a short, written note a human made while actually reading the reply


# Ten sampled real failures from Tasklight's support agent -- the input to reading, not to scoring.
FAILING_SAMPLE: list[FailingCase] = [  # => co-01: a small SAMPLE of real failures, not a synthetic list
    FailingCase("t-101", "You can adjust priority from the ticket detail view.", "asked HOW, got a vague pointer"),  # => co-01
    FailingCase("t-102", "Sure, I've closed ticket #4821 for you.", "closed the WRONG ticket number"),  # => co-01
    FailingCase("t-103", "Tasklight supports many workflows for many teams.", "never answered the question"),  # => co-01
    FailingCase("t-104", "I've set the due date to 2026-13-40.", "produced an invalid calendar date"),  # => co-01
    FailingCase("t-105", "You do not have permission to do that.", "user DOES have permission -- wrong denial"),  # => co-01
    FailingCase("t-106", "Escalating to a human agent now.", "should have been answerable directly"),  # => co-01
    FailingCase("t-107", "Here's a summary: [no summary was generated]", "placeholder text leaked into the reply"),  # => co-01
    FailingCase("t-108", "The sprint ends on Friday.", "the sprint actually ends on Thursday this cycle"),  # => co-01
    FailingCase("t-109", "I'll archive all 40 tickets right away.", "user asked to CLOSE, not archive"),  # => co-01
    FailingCase("t-110", "Your team has 3 open critical bugs.", "the real count was 5, two were missed"),  # => co-01
]  # => co-01: closes FAILING_SAMPLE -- ten real cases, sampled for a hand-reading pass


def read_case(case: FailingCase) -> ReviewRow:  # => co-01: reading ONE case -- turns a raw failure into a written note
    """Produce a written observation for one failing case -- the human act of actually reading it."""  # => co-01: documents read_case's contract -- no runtime output, just sets its __doc__
    observation = f"reply {case.reply!r} -- reported: {case.reported_problem}"  # => co-01: a note quoting the actual output, not paraphrasing from memory
    return ReviewRow(case.ticket_id, observation)  # => co-01: returns this computed value to the caller


if __name__ == "__main__":  # => co-01: entry point -- runs only when this file executes directly, not on import
    review_sheet: list[ReviewRow] = [read_case(case) for case in FAILING_SAMPLE]  # => co-01: one written observation PER sampled case
    for row in review_sheet:  # => co-01: prints the whole review sheet for a quick visual check
        print(f"{row.ticket_id}: {row.observation}")  # => co-01: one line per reviewed case
    covered_ids = {row.ticket_id for row in review_sheet}  # => co-01: which cases actually got a written observation
    source_ids = {case.ticket_id for case in FAILING_SAMPLE}  # => co-01: which cases were sampled in the first place
    assert covered_ids == source_ids, "every sampled case must have a written observation"  # => co-01: the floor this example demonstrates
    assert all(row.observation for row in review_sheet), "no observation may be empty"  # => co-01: a blank note is not reading
    print(f"MATCH: {len(review_sheet)}/{len(FAILING_SAMPLE)} sampled cases each have a written observation")  # => co-01: reached only if both asserts passed
    # => co-01: this review sheet -- not a metric -- is the raw material ex-03's open coding reads next
