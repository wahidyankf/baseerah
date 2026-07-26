# learning/code/ex-35-evaluate-against-the-base/evaluate_against_base.py
"""Worked Example 35: Evaluate Against the Base."""  # => co-25: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-25: one immutable row per case, base and adapted results recorded SIDE BY SIDE, not separately


class PairedCase(NamedTuple):  # => co-25: a paired comparison -- the SAME case run through base and adapted, never two separate samples
    case_id: str  # => co-25: which held-out triage case this row is
    base_pass: bool  # => co-25: did the unadapted base model pass this case
    adapted_pass: bool  # => co-25: did the adapted model pass the SAME case


# => co-25: 20 held-out cases -- 11 both pass, 6 only-adapted-passes, 1 only-base-passes, 2 both fail
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
    PairedCase(case_id="case-12", base_pass=False, adapted_pass=True),  # => co-25: discordant -- adapting FIXED this case, 1 of 6
    PairedCase(case_id="case-13", base_pass=False, adapted_pass=True),  # => co-25: discordant -- adapting FIXED this case, 2 of 6
    PairedCase(case_id="case-14", base_pass=False, adapted_pass=True),  # => co-25: discordant -- adapting FIXED this case, 3 of 6
    PairedCase(case_id="case-15", base_pass=False, adapted_pass=True),  # => co-25: discordant -- adapting FIXED this case, 4 of 6
    PairedCase(case_id="case-16", base_pass=False, adapted_pass=True),  # => co-25: discordant -- adapting FIXED this case, 5 of 6
    PairedCase(case_id="case-17", base_pass=False, adapted_pass=True),  # => co-25: discordant -- adapting FIXED this case, 6 of 6
    PairedCase(case_id="case-18", base_pass=True, adapted_pass=False),  # => co-25: discordant -- adapting BROKE this case, the one regression
    PairedCase(case_id="case-19", base_pass=False, adapted_pass=False),  # => co-25: concordant fail 1 -- neither model handles this
    PairedCase(case_id="case-20", base_pass=False, adapted_pass=False),  # => co-25: concordant fail 2 -- neither model handles this
]  # => co-25: closes PAIRED_RESULTS -- 20 cases total


def discordant_counts(cases: list[PairedCase]) -> tuple[int, int]:  # => co-25: (b, c) -- only pairs where base and adapted DISAGREE carry evidence
    """Return (b, c): the count of base-fail/adapted-pass pairs and base-pass/adapted-fail pairs in `cases`."""  # => co-25: documents discordant_counts's contract -- no runtime output, just sets its __doc__
    b = sum(1 for c in cases if not c.base_pass and c.adapted_pass)  # => co-25: cases adapting FIXED -- evidence FOR the adaptation
    c = sum(1 for c in cases if c.base_pass and not c.adapted_pass)  # => co-25: cases adapting BROKE -- evidence AGAINST the adaptation
    return b, c  # => co-25: returns this computed value to the caller


if __name__ == "__main__":  # => co-25: entry point -- runs only when this file executes directly, not on import
    base_pass_rate = sum(1 for c in PAIRED_RESULTS if c.base_pass) / len(PAIRED_RESULTS)  # => co-25: naive base pass rate
    adapted_pass_rate = sum(1 for c in PAIRED_RESULTS if c.adapted_pass) / len(PAIRED_RESULTS)  # => co-25: naive adapted pass rate
    print(f"Base pass rate: {base_pass_rate:.0%} | Adapted pass rate: {adapted_pass_rate:.0%}")  # => co-25: the two printed numbers ALONE
    b, c = discordant_counts(PAIRED_RESULTS)  # => co-25: the PAIRED evidence -- what statistics-for-evaluation actually asks for
    print(f"Discordant pairs: adapting fixed {b} cases, adapting broke {c} cases")  # => co-25: this is the real comparison, not the two rates above
    assert base_pass_rate == 0.60, "base pass rate must be exactly 60% in this scenario"  # => co-25
    assert adapted_pass_rate == 0.85, "adapted pass rate must be exactly 85% in this scenario"  # => co-25
    assert (b, c) == (6, 1), "the discordant pair counts must match the planted scenario exactly"  # => co-25
    supported = b >= 5 and c <= 1  # => co-25: a stand-in for a real paired significance test -- see statistics-for-evaluation for the actual machinery
    print(f"Improvement is evidence-supported (stand-in threshold b>=5, c<=1): {supported}")  # => co-25
    assert supported, "6 fixes against 1 regression must clear this illustrative evidence threshold"  # => co-25
    print("MATCH: 'base 60%, adapted 85%' is two printed numbers -- the paired 6-fixed-vs-1-broken comparison is the actual evidence a decision needs")  # => co-25
    # => co-25: this file's b/c stand-in is illustrative -- a real capstone uses statistics-for-evaluation's paired test, not this simplified threshold
