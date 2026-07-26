"""Worked Example 8: Write a Criterion That Exists Because of an Observed Failure Mode."""  # => co-05: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-05: Criterion is a typed record, not a bare string


class ObservedMode(NamedTuple):  # => co-05: the failure mode a criterion must trace back to
    mode_name: str  # => co-05: ex-07's top-ranked mode, by frequency-times-cost
    example_ticket_id: str  # => co-05: a real case this mode was observed in
    example_reply: str  # => co-05: the actual, quoted reply that failed this way


class Criterion(NamedTuple):  # => co-05: a criterion, with an explicit trace back to the mode that motivated it
    criterion_text: str  # => co-05: the written pass/fail rule itself
    traces_to_mode: str  # => co-05: WHICH observed mode this criterion exists to catch -- never left implicit
    grounding_ticket_id: str  # => co-05: the specific real case that motivated writing this criterion


# ex-07's top-ranked mode by frequency-times-cost: incorrect-aggregate-count.
TOP_MODE = ObservedMode(  # => co-05: the highest-priority mode this criterion is derived FROM
    mode_name="incorrect-aggregate-count",  # => co-05: the mode's name, from ex-05/ex-07
    example_ticket_id="t-205",  # => co-05: the real case from ex-03/ex-04 that exhibited this exact mode
    example_reply="Your team has 3 open critical bugs.",  # => co-05: the actual wrong output -- true count was 5
)  # => co-05: closes TOP_MODE


def derive_criterion(mode: ObservedMode) -> Criterion:  # => co-05: the derivation step -- mode IN, criterion OUT
    """Turn an observed failure mode into a criterion that would have caught it, with an explicit trace."""  # => co-05: documents derive_criterion's contract -- no runtime output, just sets its __doc__
    text = (  # => co-05: the criterion is written to name the EXACT failure this mode exhibited -- undercounting
        "Every count-type answer (open tickets, bugs, overdue items) must match the true underlying count exactly -- no undercounting or overcounting a real, checkable total."
    )  # => co-05: closes text
    return Criterion(  # => co-05: bundles the criterion with its required trace back to TOP_MODE
        criterion_text=text,  # => co-05: the rule itself
        traces_to_mode=mode.mode_name,  # => co-05: the explicit link -- never a criterion floating free of any observed failure
        grounding_ticket_id=mode.example_ticket_id,  # => co-05: the specific case that motivated this exact wording
    )  # => co-05: closes the Criterion construction


if __name__ == "__main__":  # => co-05: entry point -- runs only when this file executes directly, not on import
    criterion = derive_criterion(TOP_MODE)  # => co-05: run the derivation over the top-ranked mode
    print(f"Criterion: {criterion.criterion_text}")  # => co-05: prints the written rule
    print(f"Traces to mode: {criterion.traces_to_mode} (grounded in {criterion.grounding_ticket_id})")  # => co-05: prints the trace

    assert criterion.traces_to_mode == TOP_MODE.mode_name, "the criterion must trace to the exact mode it was derived from"  # => co-05: the rule this example proves
    assert criterion.grounding_ticket_id == TOP_MODE.example_ticket_id, "the criterion must cite the real case that motivated it"  # => co-05
    would_have_caught_it = "5" not in TOP_MODE.example_reply and "3" in TOP_MODE.example_reply  # => co-05: confirms the ORIGINAL failing reply violates this new criterion
    print(f"The original failing reply violates this criterion: {would_have_caught_it}")  # => co-05
    assert would_have_caught_it, "the derived criterion must actually be violated by the case that motivated it"  # => co-05: a criterion that its own motivating case would PASS is useless
    print("MATCH: the criterion traces to an observed mode, and the original failing case violates it")  # => co-05: reached only if all three asserts passed
    # => co-05: co-06 next turns THIS prose criterion into something two independent labelers apply identically
