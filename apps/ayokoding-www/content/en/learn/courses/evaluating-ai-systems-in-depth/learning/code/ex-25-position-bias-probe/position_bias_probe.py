"""Worked Example 25: Swap the Order of Two Candidates in a Pairwise Prompt to Probe for Position Bias."""  # => co-13: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic


def mock_pairwise_judge(first_candidate: str, second_candidate: str) -> str:  # => co-14: a pairwise judge -- "which is better," not "score this alone"
    """A mocked pairwise judge with a DELIBERATE positional lean toward whichever candidate appears first."""  # => co-13: documents mock_pairwise_judge's contract -- no runtime output, just sets its __doc__
    del second_candidate  # => co-13: unused -- that IS the bug this mock deliberately reproduces
    return "first"  # => co-13: this mock ALWAYS prefers the first-listed candidate, regardless of actual content


REPLY_A = "Deleted files stay in trash for 30 days before permanent removal."  # => co-13: candidate A -- equally correct
REPLY_B = "Trash retention is 30 days before files are permanently removed."  # => co-13: candidate B -- equally correct, different phrasing


if __name__ == "__main__":  # => co-13: entry point -- runs only when this file executes directly, not on import
    verdict_a_first = mock_pairwise_judge(REPLY_A, REPLY_B)  # => co-14: A listed first, B listed second
    verdict_b_first = mock_pairwise_judge(REPLY_B, REPLY_A)  # => co-14: SAME two candidates, order SWAPPED -- B now listed first
    print(f"Order (A, B): judge prefers the {verdict_a_first!r}-listed candidate")  # => co-13: prints the first ordering's verdict
    print(f"Order (B, A): judge prefers the {verdict_b_first!r}-listed candidate")  # => co-13: prints the swapped ordering's verdict

    winner_when_a_first = REPLY_A if verdict_a_first == "first" else REPLY_B  # => co-13: which REPLY actually won, order (A, B)
    winner_when_b_first = REPLY_B if verdict_b_first == "first" else REPLY_A  # => co-13: which REPLY actually won, order (B, A)
    print(f"Actual winner when A is first: {winner_when_a_first == REPLY_A}")  # => co-13: True -- A wins when listed first
    print(f"Actual winner when B is first: {winner_when_b_first == REPLY_B}")  # => co-13: True -- B wins when listed first (the SAME reply that lost before)

    flips = winner_when_a_first != winner_when_b_first  # => co-13: the verdict flips purely from reordering equally-good content
    assert flips, "the winner must flip purely from swapping presentation order, on two equally correct candidates"  # => co-13: the rule this example proves
    print("MATCH: swapping presentation order alone flips the verdict, on two equally correct candidates -- position bias, demonstrated")  # => co-13
    # => co-13,co-14: a pairwise judge's verdict must never be trusted from a SINGLE ordering -- ex-62 fixes this by swapping and averaging both orders
