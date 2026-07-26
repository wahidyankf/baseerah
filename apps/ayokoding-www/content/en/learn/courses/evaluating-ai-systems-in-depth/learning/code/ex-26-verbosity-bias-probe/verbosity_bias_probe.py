"""Worked Example 26: Pad a Worse Answer With Length and Watch the Judge's Score Rise."""  # => co-13: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

SHORT_CORRECT_REPLY = "Deleted files stay in trash for 30 days."  # => co-13: short, correct, and complete on its own
PADDED_SAME_CONTENT_REPLY = (  # => co-13: the SAME single fact, wrapped in extra words that add zero new information
    "Great question! When it comes to file management, it's worth noting that our platform "  # => co-13: pure filler, zero facts
    "takes data retention seriously. Specifically, deleted files stay in trash for 30 days, "  # => co-13: the one real fact, buried mid-padding
    "which we believe strikes a thoughtful balance for most users and their workflows."  # => co-13: closing filler, zero new facts
)  # => co-13: closes PADDED_SAME_CONTENT_REPLY -- pure padding around the identical fact


def mock_length_swayed_judge(reply: str) -> float:  # => co-13: a judge whose score is DELIBERATELY influenced by length, not just content
    """A mocked judge that rewards length as a proxy for 'thoroughness' -- verbosity bias, reproduced deliberately."""  # => co-13: documents mock_length_swayed_judge's contract -- no runtime output, just sets its __doc__
    base_score = 0.6  # => co-13: a baseline score every factually-correct reply starts from
    length_bonus = min(0.35, len(reply) / 1000)  # => co-13: MORE length adds MORE score, capped -- exactly the bias this example probes
    return base_score + length_bonus  # => co-13: returns this computed value to the caller


if __name__ == "__main__":  # => co-13: entry point -- runs only when this file executes directly, not on import
    short_score = mock_length_swayed_judge(SHORT_CORRECT_REPLY)  # => co-13: the concise reply's score
    padded_score = mock_length_swayed_judge(PADDED_SAME_CONTENT_REPLY)  # => co-13: the padded reply's score -- SAME underlying fact
    print(f"Short reply ({len(SHORT_CORRECT_REPLY)} chars): score={short_score:.2f}")  # => co-13: prints the concise reply's score
    print(f"Padded reply ({len(PADDED_SAME_CONTENT_REPLY)} chars, SAME fact): score={padded_score:.2f}")  # => co-13: prints the padded reply's score

    assert padded_score > short_score, "the padded reply must score HIGHER despite adding zero new information"  # => co-13: the rule this example proves
    score_gap = padded_score - short_score  # => co-13: how much score was bought purely with padding
    print(f"MATCH: padding with zero new information raised the score by {score_gap:.2f} -- verbosity bias, demonstrated")  # => co-13
    # => co-13: a judge that rewards length is trivially gamed by padding -- co-15's binary rubric design is the direct countermeasure
