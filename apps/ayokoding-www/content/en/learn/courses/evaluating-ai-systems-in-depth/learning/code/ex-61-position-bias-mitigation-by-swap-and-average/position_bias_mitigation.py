"""Worked Example 61: Average Verdicts Across Both Orderings of a Pairwise Prompt to Cancel Position Bias."""  # => co-13: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic


def mock_pairwise_judge(first_candidate: str, second_candidate: str) -> str:  # => co-14: ex-25's exact biased mock, reused unmitigated
    """A mocked pairwise judge that ALWAYS prefers whichever candidate appears first -- ex-25's exact bias."""  # => co-13: documents mock_pairwise_judge's contract -- no runtime output, just sets its __doc__
    del second_candidate  # => co-13: unused -- that IS the bug this mock deliberately reproduces
    return "first"  # => co-13: always the first-listed candidate, regardless of actual content


REPLY_A = "Deleted files stay in trash for 30 days before permanent removal."  # => co-13: candidate A -- equally correct
REPLY_B = "Trash retention is 30 days before files are permanently removed."  # => co-13: candidate B -- equally correct, different phrasing


def swap_and_average(reply_x: str, reply_y: str) -> str | None:  # => co-13: the mitigation -- run BOTH orderings, only trust a verdict that survives the swap
    """Run the judge in both orderings; return the winner only if BOTH orderings agree, else None (a tie)."""  # => co-13: documents swap_and_average's contract -- no runtime output, just sets its __doc__
    verdict_xy = mock_pairwise_judge(reply_x, reply_y)  # => co-13: ordering 1 -- x first, y second
    winner_xy = reply_x if verdict_xy == "first" else reply_y  # => co-13: which REPLY won under ordering 1
    verdict_yx = mock_pairwise_judge(reply_y, reply_x)  # => co-13: ordering 2 -- y first, x second, the SAME pair swapped
    winner_yx = reply_y if verdict_yx == "first" else reply_x  # => co-13: which REPLY won under ordering 2
    return winner_xy if winner_xy == winner_yx else None  # => co-13: only trust a verdict that agrees across BOTH orderings


if __name__ == "__main__":  # => co-13: entry point -- runs only when this file executes directly, not on import
    unmitigated_a_first = mock_pairwise_judge(REPLY_A, REPLY_B)  # => co-13: a SINGLE, unmitigated ordering, per ex-25
    print(f"Single ordering (A, B): judge prefers the {unmitigated_a_first!r}-listed candidate")  # => co-13: reproduces ex-25's flip risk

    mitigated_result = swap_and_average(REPLY_A, REPLY_B)  # => co-13: the mitigated, swap-and-average result
    print(f"Swap-and-average result: {mitigated_result}")  # => co-13: prints None -- correctly reports "no reliable winner", not a false confident pick

    assert mitigated_result is None, "swap-and-average must correctly report a TIE, since both orderings favor whichever is listed first"  # => co-13: the rule this example proves
    print("MATCH: swap-and-average correctly reports NO reliable winner between two equally-good candidates, instead of confidently reporting whichever happened to be listed first")  # => co-13
    # => co-13,co-14: this is the direct, practical fix for ex-25's demonstrated flip -- a verdict is only trusted once it survives BOTH presentation orders
