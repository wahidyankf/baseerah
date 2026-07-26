"""Worked Example 9: Annotate a Speculative Criterion and Delete It."""  # => co-05: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-05: Criterion is a typed record, not a bare string


class Criterion(NamedTuple):  # => co-05: the same shape ex-08 derived a criterion into
    criterion_text: str  # => co-05: the written pass/fail rule itself
    traces_to_mode: str | None  # => co-05: the observed mode this criterion traces to -- None means speculation
    grounding_ticket_id: str | None  # => co-05: the real case motivating it -- None means no such case exists


# The taxonomy's three real, observed modes (from ex-05/ex-06) -- the only legitimate source a
# criterion is allowed to trace back to.
OBSERVED_MODES = {"wrong-object-acted-on", "malformed-structured-output", "incorrect-aggregate-count"}  # => co-05

CANDIDATE_CRITERIA: list[Criterion] = [  # => co-05: two candidates, one grounded, one speculative
    Criterion(  # => co-05: candidate 1 -- grounded, same as ex-08's derived criterion
        "Every count-type answer must match the true underlying count exactly.",  # => co-05: positional criterion_text field
        traces_to_mode="incorrect-aggregate-count",  # => co-05: a REAL, observed mode
        grounding_ticket_id="t-205",  # => co-05: a REAL case that exhibited it
    ),  # => co-05: closes candidate 1's Criterion(...) call
    Criterion(  # => co-05: candidate 2 -- an engineer's hunch, no observed failure behind it
        "Replies should always include a friendly emoji to feel more personable.",  # => co-05: positional criterion_text field
        traces_to_mode=None,  # => co-05: no mode in OBSERVED_MODES supports this -- it is pure speculation
        grounding_ticket_id=None,  # => co-05: no real failing case ever exhibited "missing emoji"
    ),  # => co-05: closes candidate 2's Criterion(...) call
]  # => co-05: closes CANDIDATE_CRITERIA


def keep_or_delete(criterion: Criterion) -> tuple[bool, str]:  # => co-05: the actual gate every candidate criterion passes through
    """Keep a criterion only if it traces to a real, observed mode; delete it otherwise, with a reason."""  # => co-05: documents keep_or_delete's contract -- no runtime output, just sets its __doc__
    if criterion.traces_to_mode in OBSERVED_MODES and criterion.grounding_ticket_id is not None:  # => co-05: BOTH must be present
        return True, f"kept -- traces to observed mode {criterion.traces_to_mode!r}"  # => co-05: a real failure justifies it
    return False, "deleted -- no observed failure mode or real case backs this criterion; it is speculation"  # => co-05: the rejection reason


if __name__ == "__main__":  # => co-05: entry point -- runs only when this file executes directly, not on import
    decisions = [(c, *keep_or_delete(c)) for c in CANDIDATE_CRITERIA]  # => co-05: run the gate over both candidates
    for criterion, keep, reason in decisions:  # => co-05: prints each candidate's fate
        print(f"{criterion.criterion_text!r} -> keep={keep} ({reason})")  # => co-05: one line per candidate

    assert decisions[0][1] is True, "the count-accuracy criterion must be kept -- it traces to a real observed mode"  # => co-05
    assert decisions[1][1] is False, "the emoji criterion must be deleted -- no observed failure backs it"  # => co-05
    print("MATCH: the grounded criterion is kept, and the speculative one is deleted with a stated rationale")  # => co-05: reached only if both asserts passed
    # => co-05: a criterion nobody can trace to a real failure is a guess wearing the costume of rigor -- deleting it is not a loss
