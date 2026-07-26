# learning/code/ex-05-prompting-closes-the-gap/prompting_closes_gap.py
"""Worked Example 5: Prompting Closes the Gap."""  # => co-03: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

DISCLAIMER = "Refunds are processed within 5-7 business days."  # => co-03: the sentence Legal requires on every refund-ticket reply

# => co-03: ten refund-ticket replies from the CURRENT prompt -- no explicit instruction to include the disclaimer
BASELINE_REPLIES = [  # => co-03: a fixed, ten-case eval -- same discipline as ex-01
    "Your refund has been approved.",  # => co-03: missing
    "We've refunded $42.00 to your card. " + DISCLAIMER,  # => co-03: present
    "Refund approved, you'll see it soon.",  # => co-03: missing
    "Approved -- refund is on its way.",  # => co-03: missing
    "Your $19.99 refund is confirmed. " + DISCLAIMER,  # => co-03: present
    "Refund processed.",  # => co-03: missing
    "We've issued your refund.",  # => co-03: missing
    "Refund confirmed for your last invoice.",  # => co-03: missing
    "Approved. " + DISCLAIMER,  # => co-03: present
    "Your refund request has been approved.",  # => co-03: missing
]  # => co-03: closes BASELINE_REPLIES -- 3/10 include the required sentence

# => co-03: the SAME ten cases, now with one explicit instruction + a single few-shot example prepended to the prompt
INSTRUCTED_REPLIES = [  # => co-03: zero training happened between these two lists -- only the prompt changed
    "Your refund has been approved. " + DISCLAIMER,  # => co-03: present
    "We've refunded $42.00 to your card. " + DISCLAIMER,  # => co-03: present
    "Refund approved. " + DISCLAIMER,  # => co-03: present
    "Approved -- refund is on its way. " + DISCLAIMER,  # => co-03: present
    "Your $19.99 refund is confirmed. " + DISCLAIMER,  # => co-03: present
    "Refund processed. " + DISCLAIMER,  # => co-03: present
    "We've issued your refund. " + DISCLAIMER,  # => co-03: present
    "Refund confirmed for your last invoice. " + DISCLAIMER,  # => co-03: present
    "Approved. " + DISCLAIMER,  # => co-03: present
    "Your refund request has been approved. " + DISCLAIMER,  # => co-03: present -- 10/10, the instruction alone fixed it
]  # => co-03: closes INSTRUCTED_REPLIES


def pass_rate(replies: list[str]) -> float:  # => co-03: the deterministic scorer this whole example measures against
    """Return the fraction of `replies` that contain DISCLAIMER, verbatim."""  # => co-03: documents pass_rate's contract -- no runtime output, just sets its __doc__
    return sum(DISCLAIMER in reply for reply in replies) / len(replies)  # => co-03: exact substring check -- no ambiguity


if __name__ == "__main__":  # => co-03: entry point -- runs only when this file executes directly, not on import
    baseline_rate = pass_rate(BASELINE_REPLIES)  # => co-03: the gap BEFORE any prompt change
    instructed_rate = pass_rate(INSTRUCTED_REPLIES)  # => co-03: the SAME eval, AFTER an instruction + one few-shot example
    print(f"Baseline (no instruction): {baseline_rate:.0%}")  # => co-03: prints the starting gap
    print(f"With explicit instruction + one example: {instructed_rate:.0%}")  # => co-03: prints the closed gap
    assert baseline_rate < 0.5, "the baseline must show a real gap for this demo to make its point"  # => co-03
    assert instructed_rate == 1.0, "an explicit instruction plus one example must close this gap completely"  # => co-03
    print(f"MATCH: pass rate improved from {baseline_rate:.0%} to {instructed_rate:.0%} with ZERO training runs")  # => co-03
    # => co-03: co-06's decision gate requires exhausting exactly this move BEFORE any fine-tuning candidate is considered
