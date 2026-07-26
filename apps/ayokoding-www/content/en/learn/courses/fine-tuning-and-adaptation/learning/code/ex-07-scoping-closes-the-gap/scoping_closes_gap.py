# learning/code/ex-07-scoping-closes-the-gap/scoping_closes_gap.py
"""Worked Example 7: Scoping Closes the Gap."""  # => co-05: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-05: one row per ticket keeps category and pass/fail together, one item per line


class TicketResult(NamedTuple):  # => co-05: (ticket_id, category, passed) -- a single source of truth per ticket
    ticket_id: str  # => co-05: which ticket this row reports on
    category: str  # => co-05: which of the four categories it belongs to
    passed: bool  # => co-05: did the auto-drafted reply pass review?


# => co-05: twelve tickets spanning FOUR categories -- the assistant was asked to auto-draft a reply to ALL of them
ALL_RESULTS: list[TicketResult] = [  # => co-05: one row per ticket, one comment per row -- ruff-format-stable, no magic-comma explosion
    TicketResult("t-20", "billing", True),  # => co-05: billing 1/4
    TicketResult("t-21", "billing", False),  # => co-05: billing 2/4
    TicketResult("t-22", "billing", True),  # => co-05: billing 3/4
    TicketResult("t-23", "billing", False),  # => co-05: billing 4/4 -- 2/4 pass, this category is hard for the base model
    TicketResult("t-24", "bug", False),  # => co-05: bug 1/4
    TicketResult("t-25", "bug", True),  # => co-05: bug 2/4
    TicketResult("t-26", "bug", False),  # => co-05: bug 3/4
    TicketResult("t-27", "bug", True),  # => co-05: bug 4/4 -- 2/4 pass, also hard
    TicketResult("t-28", "password-reset", True),  # => co-05: password-reset 1/4
    TicketResult("t-29", "password-reset", True),  # => co-05: password-reset 2/4
    TicketResult("t-30", "password-reset", True),  # => co-05: password-reset 3/4
    TicketResult("t-31", "password-reset", True),  # => co-05: password-reset 4/4 -- 4/4 pass, this narrow slice is easy
]  # => co-05: closes ALL_RESULTS -- 8/12 overall, a middling and misleading blended average


def pass_rate_for(category: str) -> float:  # => co-05: the SAME scorer, sliced down to one category at a time
    """Return the pass rate restricted to tickets in `category`."""  # => co-05: documents pass_rate_for's contract -- no runtime output, just sets its __doc__
    in_category = [row for row in ALL_RESULTS if row.category == category]  # => co-05: which rows belong to this category
    return sum(row.passed for row in in_category) / len(in_category)  # => co-05: pass rate within just this slice


if __name__ == "__main__":  # => co-05: entry point -- runs only when this file executes directly, not on import
    overall_rate = sum(row.passed for row in ALL_RESULTS) / len(ALL_RESULTS)  # => co-05: the BROAD task's pass rate
    print(f"All-category (broad task) pass rate: {overall_rate:.0%}")  # => co-05: prints the middling, blended number
    for category in ("billing", "bug", "password-reset"):  # => co-05: break the broad task down by category
        print(f"  {category}: {pass_rate_for(category):.0%}")  # => co-05: shows WHERE the broad task's failures concentrate
    narrowed_rate = pass_rate_for("password-reset")  # => co-05: SCOPE the task down to just the category the base model already handles
    print(f"Narrowed to password-reset only: {narrowed_rate:.0%}")  # => co-05: the measured success on the narrowed task
    assert overall_rate < 0.7, "the broad, unscoped task must show a real gap"  # => co-05
    assert narrowed_rate == 1.0, "narrowing to the category the base model already handles must succeed completely"  # => co-05
    print("MATCH: shrinking the task's scope closed the gap for the narrowed slice -- no training needed for THAT slice")  # => co-05
    # => co-05: shipping password-reset auto-drafts now, and revisiting billing/bug later, is often cheaper than adapting a model to all three at once
