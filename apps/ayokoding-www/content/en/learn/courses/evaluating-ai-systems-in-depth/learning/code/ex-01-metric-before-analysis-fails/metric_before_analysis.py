"""Worked Example 1: A Metric Chosen Before Reading Failures Misses the Dominant Failure Mode."""  # => co-01: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

# Two candidate replies from "Tasklight" (a fictional project-management support agent) to the
# SAME real ticket: "How do I bulk-close 40 stale tickets in one go?"
REPLY_LONG = (  # => co-01: the reply an engineer picked BEFORE reading any real failures
    "Great question! Tasklight is built to help teams stay organized and keep their ticket "  # => co-01: opens warmly, still no concrete instruction
    "queues healthy over time, which matters a lot as a team grows. Managing a large backlog "  # => co-01: still generic, still no answer
    "well is something many teams struggle with, and we're always improving our tools here."  # => co-01: closes without ever answering
)  # => co-01: long, generic, warmly worded -- and never once answers the actual question asked
REPLY_SHORT = "Select the 40 tickets, then Actions > Bulk Close."  # => co-01: short, and answers exactly what was asked


def naive_length_metric(reply: str) -> int:  # => co-01: the METRIC picked before any failures were read
    """Score a reply by character count -- a plausible-sounding proxy for "thoroughness"."""  # => co-01: documents naive_length_metric's contract -- no runtime output, just sets its __doc__
    return len(reply)  # => co-01: longer text scores "better" under this metric, with no regard for correctness


def actually_answers_the_ask(reply: str) -> bool:  # => co-01: what reading the failure by hand actually checks
    """Return True iff `reply` contains the concrete bulk-close instruction the ticket asked for."""  # => co-01: documents actually_answers_the_ask's contract -- no runtime output, just sets its __doc__
    return "bulk close" in reply.lower() or "bulk-close" in reply.lower()  # => co-01: the dominant failure mode this metric can't see


if __name__ == "__main__":  # => co-01: entry point -- runs only when this file executes directly, not on import
    long_score = naive_length_metric(REPLY_LONG)  # => co-01: the naive metric's verdict on the long reply
    short_score = naive_length_metric(REPLY_SHORT)  # => co-01: the naive metric's verdict on the short reply
    metric_picks_long = long_score > short_score  # => co-01: which reply the METRIC alone would call "better"
    print(f"REPLY_LONG length={long_score} | REPLY_SHORT length={short_score}")  # => co-01: prints both raw scores
    print(f"Metric alone picks the long reply as better: {metric_picks_long}")  # => co-01: True -- length rewards the wrong one

    long_answers = actually_answers_the_ask(REPLY_LONG)  # => co-01: what a human reading the failure by hand would find
    short_answers = actually_answers_the_ask(REPLY_SHORT)  # => co-01: the short reply's real correctness
    print(f"REPLY_LONG actually answers the ask: {long_answers}")  # => co-01: False -- the dominant failure mode
    print(f"REPLY_SHORT actually answers the ask: {short_answers}")  # => co-01: True

    assert metric_picks_long is True, "the naive length metric must prefer the long, non-answering reply"  # => co-01: proves the metric's blind spot
    assert long_answers is False and short_answers is True, "reading the replies must reverse the metric's verdict"  # => co-01: proves the reversal
    print("MATCH: the metric picked before reading failures rewards exactly the reply that fails the real ask")  # => co-01: reached only if both asserts passed
    # => co-01: a metric chosen before reading failures optimizes for whatever it happens to measure, not for what users need
