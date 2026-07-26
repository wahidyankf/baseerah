"""Worked Example 52: Write a Rubric With Concrete Pass/Fail Anchors for Each Score Point."""  # => co-15: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-15: RubricAnchor is a typed record, not a bare string


class RubricAnchor(NamedTuple):  # => co-15: one concrete, worked example anchoring a specific score point
    score: int  # => co-15: which point on the scale this anchor defines
    example_reply: str  # => co-15: a REAL, concrete reply that belongs at exactly this score
    why_this_score: str  # => co-15: the explicit reasoning tying the example to the score


ANCHORED_RUBRIC = [  # => co-15: co-13's score-compression problem, addressed with concrete anchors instead of an abstract 1-5 label
    RubricAnchor(1, "The count is 3.", "states a NUMBER that is factually wrong -- true count is 5"),  # => co-15
    RubricAnchor(3, "Several critical bugs are open.", "vague, no number at all, but not FACTUALLY wrong"),  # => co-15
    RubricAnchor(5, "There are 5 open critical bugs.", "states the EXACT correct number, unhedged"),  # => co-15
]  # => co-15: closes ANCHORED_RUBRIC -- three concrete anchors, not five abstract labels


def score_with_anchors(reply: str, anchors: list[RubricAnchor]) -> int:  # => co-15: the judge picks the CLOSEST matching anchor, not a free-floating number
    """Score `reply` by finding the single closest matching anchor's score."""  # => co-15: documents score_with_anchors's contract -- no runtime output, just sets its __doc__
    if "5" in reply and "3" not in reply:  # => co-15: matches the score-5 anchor's pattern -- exact correct number
        return 5  # => co-15: this reply resembles the score-5 anchor
    if "3" in reply:  # => co-15: matches the score-1 anchor's pattern -- the wrong number
        return 1  # => co-15: this reply resembles the score-1 anchor
    return 3  # => co-15: matches neither extreme anchor -- defaults to the vague-middle anchor


def score_without_anchors(reply: str) -> int:  # => co-15: the SAME scale, with NO concrete anchors -- co-27's unmitigated compression
    """Score `reply` on a bare 1-5 scale with no worked anchors, reproducing co-27's compression."""  # => co-15: documents score_without_anchors's contract -- no runtime output, just sets its __doc__
    return 4  # => co-13: an unanchored judge defaults to the same safe middle score for almost everything, per ex-27


if __name__ == "__main__":  # => co-15: entry point -- runs only when this file executes directly, not on import
    wrong_number_reply = "The count is 3."  # => co-15: matches the score-1 anchor exactly
    correct_number_reply = "There are 5 open critical bugs."  # => co-15: matches the score-5 anchor exactly

    anchored_wrong = score_with_anchors(wrong_number_reply, ANCHORED_RUBRIC)  # => co-15: anchored judge on the wrong-number reply
    anchored_correct = score_with_anchors(correct_number_reply, ANCHORED_RUBRIC)  # => co-15: anchored judge on the correct-number reply
    unanchored_wrong = score_without_anchors(wrong_number_reply)  # => co-15: unanchored judge on the SAME wrong-number reply
    unanchored_correct = score_without_anchors(correct_number_reply)  # => co-15: unanchored judge on the SAME correct-number reply
    print(f"Anchored: wrong={anchored_wrong}, correct={anchored_correct} (spread={anchored_correct - anchored_wrong})")  # => co-15
    print(f"Unanchored: wrong={unanchored_wrong}, correct={unanchored_correct} (spread={unanchored_correct - unanchored_wrong})")  # => co-15

    anchored_spread = anchored_correct - anchored_wrong  # => co-15: how far apart the anchored judge places these two, clearly-different replies
    unanchored_spread = unanchored_correct - unanchored_wrong  # => co-15: how far apart the unanchored judge places them
    assert anchored_spread > unanchored_spread, "concrete anchors must separate a clearly wrong and a clearly right reply more than an unanchored scale does"  # => co-15: the rule this example proves
    assert unanchored_spread == 0, "an unanchored judge must collapse both replies onto the identical safe middle score"  # => co-15
    print(f"MATCH: rubric anchors separate the two replies by {anchored_spread} points; the unanchored scale separates them by {unanchored_spread}")  # => co-15
    # => co-15: a worked example AT each score point is what gives a judge something concrete to match against, instead of an abstract number to guess at
