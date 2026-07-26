# learning/code/ex-74-a-licence-blocked-retirement/licence_blocked_retirement.py
"""Worked Example 74: A Licence-Blocked Retirement."""  # => co-32: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-31: a small, self-documenting record for a licence check applied to a retirement CANDIDATE, not just training


@dataclass(frozen=True)  # => co-31: frozen -- a licence check's result is a fact once verified, not a mutable running total
class RetirementCandidate:  # => co-32: a proposed replacement for an existing adapter, evaluated on BOTH quality and licence
    name: str  # => co-32: which candidate replacement this is
    pass_rate: float  # => co-32: the candidate's own measured pass rate on the target task
    licence: str  # => co-31: the candidate's licence terms
    permits_commercial_use: bool  # => co-31: whether the licence actually permits Vantage's commercial, for-profit use


CURRENT_ADAPTER_PASS_RATE = 0.81  # => co-32: matches ex-49's own retiring adapter's pass rate

# => co-32,co-31: a NEW, better-performing base model release is proposed as the retirement replacement -- but its licence is the wrong kind
PROPOSED_REPLACEMENT = RetirementCandidate(  # => co-32: the candidate Vantage's team wants to switch to
    name="newer-base-model-r2",  # => co-32: a newer release, tempting on quality alone
    pass_rate=0.96,  # => co-32: a genuinely better measured pass rate than the current adapter
    licence="custom non-commercial research licence",  # => co-31: NOT an open-commercial licence -- this is the blocker
    permits_commercial_use=False,  # => co-31: explicitly forbids the commercial use Vantage's product requires
)  # => co-32: closes PROPOSED_REPLACEMENT


def retirement_is_permitted(candidate: RetirementCandidate, current_pass_rate: float) -> tuple[bool, str]:  # => co-31,co-32: (permitted, reason) -- both quality AND licence must clear
    """Return whether `candidate` may replace the current adapter, and the reason, checking quality first then licence."""  # => co-31: documents retirement_is_permitted's contract -- no runtime output, just sets its __doc__
    if candidate.pass_rate <= current_pass_rate:  # => co-32: a replacement that is not even better on quality is not worth the switch
        return False, "candidate does not beat the current adapter on quality"  # => co-32: returns this computed value to the caller
    if not candidate.permits_commercial_use:  # => co-31: the licence check happens BEFORE the retirement is allowed to proceed, per co-31's own discipline
        return False, "candidate's licence forbids Vantage's commercial use"  # => co-31: returns this computed value to the caller
    return True, "candidate beats the current adapter and its licence permits commercial use"  # => co-31,co-32: returns this computed value to the caller


if __name__ == "__main__":  # => co-32: entry point -- runs only when this file executes directly, not on import
    print(f"Current adapter pass rate: {CURRENT_ADAPTER_PASS_RATE:.0%}")  # => co-32
    print(f"Proposed replacement: {PROPOSED_REPLACEMENT.name} | pass rate {PROPOSED_REPLACEMENT.pass_rate:.0%} | licence {PROPOSED_REPLACEMENT.licence!r}")  # => co-32,co-31
    permitted, reason = retirement_is_permitted(PROPOSED_REPLACEMENT, CURRENT_ADAPTER_PASS_RATE)  # => co-31,co-32: run the check
    print(f"Retirement permitted: {permitted} ({reason})")  # => co-31,co-32
    assert PROPOSED_REPLACEMENT.pass_rate > CURRENT_ADAPTER_PASS_RATE, "the candidate must genuinely beat the current adapter on quality in this scenario"  # => co-32
    assert not permitted, "a quality-winning candidate with a licence that forbids commercial use must still be blocked from retirement"  # => co-31,co-32
    assert reason == "candidate's licence forbids Vantage's commercial use", "the blocking reason must be attributed to the licence, since quality alone would have passed"  # => co-31
    print("MATCH: a 15-point quality win is not enough -- the licence check alone blocks this retirement, exactly as it would have blocked training in the first place")  # => co-31,co-32
    # => co-31,co-32: co-31's licence discipline applies to a retirement's REPLACEMENT candidate too, not only to the model being trained in the first place
