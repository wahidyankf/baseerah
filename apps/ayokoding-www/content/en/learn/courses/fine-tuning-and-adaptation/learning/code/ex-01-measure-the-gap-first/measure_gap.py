# learning/code/ex-01-measure-the-gap-first/measure_gap.py
"""Worked Example 1: Measure the Gap First."""  # => co-06: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

# => co-06: Vantage is a fictional B2B analytics SaaS -- every example in this course reuses it
BASELINE_REPLIES: dict[str, str] = {  # => co-25: ten real support tickets, each answered by the CURRENT (unadapted) assistant
    "t-01": "Summary: export disabled.\nRoot Cause: role lacks Export permission.\nResolution: ask an admin to grant it.\nNext Steps: retry after the grant.",  # => co-06: on-format
    "t-02": "Your dashboard is probably cached, try a hard refresh.",  # => co-06: OFF-format -- no required sections at all
    "t-03": "Summary: chart colors look wrong.\nRoot Cause: a custom palette override.\nResolution: reset to default theme.\nNext Steps: reapply custom colors after reset.",  # => co-06: on-format
    "t-04": "Summary: API key rejected.\nRoot Cause: key was rotated last week.\nResolution: generate a new key.\nNext Steps: update the key in your integration.",  # => co-06: on-format
    "t-05": "That's a known limitation, we're working on it.",  # => co-06: OFF-format
    "t-06": "Summary: slow report load.\nRoot Cause: report spans 18 months of data.\nResolution: narrow the date range.\nNext Steps: consider a scheduled export instead.",  # => co-06: on-format
    "t-07": "Summary: seat count mismatch.\nRoot Cause: a deactivated user still counted.\nResolution: purge deactivated seats.\nNext Steps: verify the new count in Billing.",  # => co-06: on-format
    "t-08": "Try logging out and back in.",  # => co-06: OFF-format
    "t-09": "Summary: webhook not firing.\nRoot Cause: endpoint returned a 500 once and was auto-disabled.\nResolution: fix the endpoint, re-enable in settings.\nNext Steps: send a test event to confirm.",  # => co-06: on-format
    "t-10": "It sounds like a browser extension is interfering.",  # => co-06: OFF-format -- 4th off-format reply
}  # => co-25: closes BASELINE_REPLIES -- ten tickets is this course's own fixed eval floor, mirroring co-03 from the eval course

REQUIRED_SECTIONS = ("Summary:", "Root Cause:", "Resolution:", "Next Steps:")  # => co-06: the internal style guide's four mandatory sections


def follows_format(reply: str) -> bool:  # => co-06: the deterministic scorer for THIS gap -- structure, not content correctness
    """Pass iff every required section header appears in `reply`."""  # => co-06: documents follows_format's contract -- no runtime output, just sets its __doc__
    return all(section in reply for section in REQUIRED_SECTIONS)  # => co-06: all four, or the reply does not comply


if __name__ == "__main__":  # => co-06: entry point -- runs only when this file executes directly, not on import
    verdicts = {tid: follows_format(reply) for tid, reply in BASELINE_REPLIES.items()}  # => co-25: one pass/fail per ticket
    pass_rate = sum(verdicts.values()) / len(verdicts)  # => co-25: the headline number -- the SIZE of the gap, not a guess
    for tid, passed in verdicts.items():  # => co-06: prints one line per ticket for a quick visual audit
        print(f"  {tid}: {'PASS' if passed else 'FAIL'}")  # => co-06: shows exactly which tickets miss the format
    print(f"Format-compliance pass rate: {pass_rate:.0%} ({sum(verdicts.values())}/{len(verdicts)})")  # => co-25: the measured gap
    assert 0.0 < pass_rate < 1.0, "the gap must be real -- neither 0% (nothing works) nor 100% (nothing to fix)"  # => co-06
    print("MATCH: a real, SIZED gap exists -- this is the number every later remedy in this course must beat")  # => co-06
    # => co-06,co-25: measuring the gap FIRST is what turns "the assistant feels inconsistent" into a number a remedy can be judged against
