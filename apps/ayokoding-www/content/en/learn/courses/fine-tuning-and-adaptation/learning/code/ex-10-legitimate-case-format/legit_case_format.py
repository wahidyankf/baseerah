# learning/code/ex-10-legitimate-case-format/legit_case_format.py
"""Worked Example 10: Legitimate Case -- Format."""  # => co-07: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

REQUIRED_COMPLIANCE_FOR_AUDIT_SIGNOFF = 0.95  # => co-07: Vantage Legal's own bar for unsupervised auto-send -- not negotiable

# => co-07: twenty independent trials of the SAME best-effort instructed prompt, drafting a full incident post-mortem report
# => co-07: (five mandatory narrative sections, in order -- unlike ex-06's flat key/value schema, prose narrative resists a schema check)
INSTRUCTED_TRIAL_COMPLIANCE = [  # => co-07: True = all five sections present, in order; False = at least one drifted
    True,  # => co-07: trial 1 -- complies
    True,  # => co-07: trial 2 -- complies
    False,  # => co-07: trial 3 -- drifted
    True,  # => co-07: trial 4 -- complies
    True,  # => co-07: trial 5 -- complies
    True,  # => co-07: trial 6 -- complies
    False,  # => co-07: trial 7 -- drifted
    True,  # => co-07: trial 8 -- complies
    True,  # => co-07: trial 9 -- complies
    True,  # => co-07: trial 10 -- complies -- 8/10 so far
    True,  # => co-07: trial 11 -- complies
    False,  # => co-07: trial 12 -- drifted
    True,  # => co-07: trial 13 -- complies
    True,  # => co-07: trial 14 -- complies
    True,  # => co-07: trial 15 -- complies
    False,  # => co-07: trial 16 -- drifted
    True,  # => co-07: trial 17 -- complies
    True,  # => co-07: trial 18 -- complies
    True,  # => co-07: trial 19 -- complies
    True,  # => co-07: trial 20 -- complies -- 16/20 overall
]  # => co-07: closes INSTRUCTED_TRIAL_COMPLIANCE -- the best this instruction has achieved across many repeated tries


def compliance_rate(trials: list[bool]) -> float:  # => co-07: the same simple reduction ex-05 used, applied to a repeated-trial series
    """Return the fraction of `trials` that comply."""  # => co-07: documents compliance_rate's contract -- no runtime output, just sets its __doc__
    return sum(trials) / len(trials)  # => co-07: fraction of True values


if __name__ == "__main__":  # => co-07: entry point -- runs only when this file executes directly, not on import
    observed_rate = compliance_rate(INSTRUCTED_TRIAL_COMPLIANCE)  # => co-07: the best instruction's ACTUAL, repeated-trial reliability
    print(f"Best instructed-prompt compliance across {len(INSTRUCTED_TRIAL_COMPLIANCE)} trials: {observed_rate:.0%}")  # => co-07
    print(f"Required for unsupervised audit sign-off: {REQUIRED_COMPLIANCE_FOR_AUDIT_SIGNOFF:.0%}")  # => co-07: the actual bar
    gate_passes = observed_rate < REQUIRED_COMPLIANCE_FOR_AUDIT_SIGNOFF  # => co-07: co-06's "alternatives exhausted" check, for THIS case
    print(f"Instructions alone reach the bar: {not gate_passes}")  # => co-07
    assert observed_rate < 0.90, "the best instruction must still fall meaningfully short of the audit bar"  # => co-07
    assert gate_passes, "this case's decision gate must PASS -- instructions cannot reliably enforce this narrative format"  # => co-07
    print("MATCH: unlike ex-06's flat schema, a five-section PROSE narrative resists reliable prompt-only enforcement -- a legitimate case")  # => co-07
    # => co-07: co-07's first legitimate case -- a consistent output format instructions cannot reliably reach, verified across real repeated trials, not one lucky run
