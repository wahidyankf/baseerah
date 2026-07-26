# learning/code/ex-41-the-fine-tune-that-did-not-help/fine_tune_that_did_not_help.py
"""Worked Example 41: The Fine-Tune That Did Not Help."""  # => co-25: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-25: one immutable row per paired case, the SAME discipline ex-35 used, applied to a run that fails it


class PairedCase(NamedTuple):  # => co-25: a paired comparison -- base and adapted run through the SAME held-out case
    case_id: str  # => co-25: which held-out case this row is
    base_pass: bool  # => co-25: did the unadapted base model pass this case
    adapted_pass: bool  # => co-25: did the FULLY TRAINED adapted model pass the same case, after weeks of data work


# => co-25: a completed adaptation -- the data was curated, the adapter trained, and it STILL does not clear the bar
PAIRED_RESULTS: list[PairedCase] = [  # => co-25: one row per case, in evaluation order
    PairedCase(case_id="case-01", base_pass=True, adapted_pass=True),  # => co-25: concordant pass 1
    PairedCase(case_id="case-02", base_pass=True, adapted_pass=True),  # => co-25: concordant pass 2
    PairedCase(case_id="case-03", base_pass=True, adapted_pass=True),  # => co-25: concordant pass 3
    PairedCase(case_id="case-04", base_pass=True, adapted_pass=True),  # => co-25: concordant pass 4
    PairedCase(case_id="case-05", base_pass=True, adapted_pass=True),  # => co-25: concordant pass 5
    PairedCase(case_id="case-06", base_pass=True, adapted_pass=True),  # => co-25: concordant pass 6
    PairedCase(case_id="case-07", base_pass=True, adapted_pass=True),  # => co-25: concordant pass 7
    PairedCase(case_id="case-08", base_pass=True, adapted_pass=True),  # => co-25: concordant pass 8
    PairedCase(case_id="case-09", base_pass=True, adapted_pass=True),  # => co-25: concordant pass 9
    PairedCase(case_id="case-10", base_pass=True, adapted_pass=True),  # => co-25: concordant pass 10
    PairedCase(case_id="case-11", base_pass=True, adapted_pass=True),  # => co-25: concordant pass 11
    PairedCase(case_id="case-12", base_pass=True, adapted_pass=True),  # => co-25: concordant pass 12
    PairedCase(case_id="case-13", base_pass=False, adapted_pass=True),  # => co-25: discordant -- adapting fixed this case, 1 of 2
    PairedCase(case_id="case-14", base_pass=False, adapted_pass=True),  # => co-25: discordant -- adapting fixed this case, 2 of 2
    PairedCase(case_id="case-15", base_pass=True, adapted_pass=False),  # => co-25: discordant -- adapting broke this case, 1 of 2
    PairedCase(case_id="case-16", base_pass=True, adapted_pass=False),  # => co-25: discordant -- adapting broke this case, 2 of 2
    PairedCase(case_id="case-17", base_pass=False, adapted_pass=False),  # => co-25: concordant fail 1
    PairedCase(case_id="case-18", base_pass=False, adapted_pass=False),  # => co-25: concordant fail 2
    PairedCase(case_id="case-19", base_pass=False, adapted_pass=False),  # => co-25: concordant fail 3
    PairedCase(case_id="case-20", base_pass=False, adapted_pass=False),  # => co-25: concordant fail 4
]  # => co-25: closes PAIRED_RESULTS -- 20 cases, 2 fixed and 2 broken, net zero


def discordant_counts(cases: list[PairedCase]) -> tuple[int, int]:  # => co-25: (b, c) -- fixed count and broken count
    """Return (b, c): the count of base-fail/adapted-pass pairs and base-pass/adapted-fail pairs in `cases`."""  # => co-25: documents discordant_counts's contract -- no runtime output, just sets its __doc__
    b = sum(1 for c in cases if not c.base_pass and c.adapted_pass)  # => co-25: cases adapting fixed
    c = sum(1 for c in cases if c.base_pass and not c.adapted_pass)  # => co-25: cases adapting broke
    return b, c  # => co-25: returns this computed value to the caller


if __name__ == "__main__":  # => co-25: entry point -- runs only when this file executes directly, not on import
    base_pass_rate = sum(1 for c in PAIRED_RESULTS if c.base_pass) / len(PAIRED_RESULTS)  # => co-25: naive base pass rate
    adapted_pass_rate = sum(1 for c in PAIRED_RESULTS if c.adapted_pass) / len(PAIRED_RESULTS)  # => co-25: naive adapted pass rate
    print(f"Base pass rate: {base_pass_rate:.0%} | Adapted pass rate: {adapted_pass_rate:.0%}")  # => co-25: the two headline numbers LOOK identical
    assert base_pass_rate == adapted_pass_rate, "this scenario's whole point is that the naive pass rates land EXACTLY equal"  # => co-25
    b, c = discordant_counts(PAIRED_RESULTS)  # => co-25: the paired evidence behind that equal-looking headline number
    print(f"Discordant pairs: adapting fixed {b} cases, adapting broke {c} cases")  # => co-25
    assert (b, c) == (2, 2), "two fixed and two broken must exactly offset in this scenario"  # => co-25
    weeks_of_data_work_spent = 3  # => co-08: the total-cost-of-a-fine-tune line item this run actually consumed
    print(f"Weeks of data work spent: {weeks_of_data_work_spent} | Net paired improvement: {b - c} cases")  # => co-08,co-25
    beats_base = b > c  # => co-25: the honest verdict -- did adapting actually help, net, on the paired evidence
    print(f"Adapted model beats the base (paired evidence): {beats_base}")  # => co-25
    assert not beats_base, "a fine-tune with equal fixed and broken counts must NOT be judged to beat the base"  # => co-25
    print("MATCH: three weeks of data work produced a model that neither beats nor clearly loses to the base -- the correct decision is to discard it, not ship it")  # => co-25,co-32
    # => co-25,co-32: co-32's discipline starts here -- discarding a completed, working, but non-beating adaptation is a normal and correct outcome
