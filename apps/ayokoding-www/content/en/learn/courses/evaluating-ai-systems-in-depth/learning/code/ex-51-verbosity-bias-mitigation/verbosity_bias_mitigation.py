"""Worked Example 51: Cap Response Length Before Judging to Shrink Verbosity Bias."""  # => co-13: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

SHORT_CORRECT_REPLY = "Deleted files stay in trash for 30 days."  # => co-13: the same short, correct reply as ex-26
PADDED_SAME_CONTENT_REPLY = (  # => co-13: the same padded reply as ex-26, carrying zero new information
    "Great question! When it comes to file management, it's worth noting that our platform "  # => co-13: pure filler, zero facts
    "takes data retention seriously. Specifically, deleted files stay in trash for 30 days, "  # => co-13: the one real fact, buried mid-padding
    "which we believe strikes a thoughtful balance for most users and their workflows."  # => co-13: closing filler, zero new facts
)  # => co-13: closes PADDED_SAME_CONTENT_REPLY


def biased_judge(reply: str) -> float:  # => co-13: ex-26's length-swayed judge, reused unmitigated
    """A judge that rewards length as a proxy for thoroughness -- verbosity bias, unmitigated."""  # => co-13: documents biased_judge's contract -- no runtime output, just sets its __doc__
    return 0.6 + min(0.35, len(reply) / 1000)  # => co-13: identical formula to ex-26 -- length directly inflates the score


def length_capped_judge(reply: str, *, cap_chars: int = 60) -> float:  # => co-13: the mitigation -- truncate BEFORE scoring, removing length as a signal
    """Truncate `reply` to `cap_chars` before scoring, so extra padding cannot inflate the result."""  # => co-13: documents length_capped_judge's contract -- no runtime output, just sets its __doc__
    truncated = reply[:cap_chars]  # => co-13: the judge never sees content past the cap, padding or otherwise
    return 0.6 + min(0.35, len(truncated) / 1000)  # => co-13: the SAME scoring formula, but now bounded by the cap, not the reply's real length


if __name__ == "__main__":  # => co-13: entry point -- runs only when this file executes directly, not on import
    unmitigated_short = biased_judge(SHORT_CORRECT_REPLY)  # => co-13: unmitigated judge, short reply
    unmitigated_padded = biased_judge(PADDED_SAME_CONTENT_REPLY)  # => co-13: unmitigated judge, padded reply
    capped_short = length_capped_judge(SHORT_CORRECT_REPLY)  # => co-13: length-capped judge, short reply
    capped_padded = length_capped_judge(PADDED_SAME_CONTENT_REPLY)  # => co-13: length-capped judge, SAME padded reply

    unmitigated_gap = unmitigated_padded - unmitigated_short  # => co-13: the score gap BEFORE mitigation
    capped_gap = capped_padded - capped_short  # => co-13: the score gap AFTER mitigation
    print(f"Unmitigated gap (padded - short): {unmitigated_gap:.3f}")  # => co-13: prints the raw bias magnitude
    print(f"Length-capped gap (padded - short): {capped_gap:.3f}")  # => co-13: prints the mitigated bias magnitude

    assert unmitigated_gap > 0.1, "the unmitigated judge must show a substantial verbosity-driven gap"  # => co-13: sanity check against ex-26
    assert capped_gap < unmitigated_gap, "capping response length before scoring must shrink the verbosity-driven gap"  # => co-13: the rule this example proves
    assert capped_gap < 0.02, "the capped gap must be nearly eliminated once both replies are truncated to the same length"  # => co-13
    print(f"MATCH: length-capping shrank the verbosity bias from {unmitigated_gap:.3f} to {capped_gap:.3f}")  # => co-13
    # => co-13: capping is a cheap partial mitigation -- it does not fix a judge that ALSO rewards padding within the cap, only bias from length past it
