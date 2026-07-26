"""Worked Example 28: Compare Pairwise and Pointwise Agreement on the Same Items."""  # => co-14: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-14: RankingPair is a typed record, not a bare tuple


class RankingPair(NamedTuple):  # => co-14: one pair of replies with a HUMAN-agreed relative preference
    reply_x: str  # => co-14: the first candidate reply
    reply_y: str  # => co-14: the second candidate reply
    human_prefers_x: bool  # => co-14: the human-adjudicated relative preference between x and y


PAIRS: list[RankingPair] = [  # => co-14: five pairs, each with a real human preference between the two replies
    RankingPair("States the exact 30-day retention period.", "Vaguely says files are kept 'for a while'.", human_prefers_x=True),  # => co-14
    RankingPair("Confirms the export completed successfully.", "Says the export is 'probably' done.", human_prefers_x=True),  # => co-14
    RankingPair("Cites the exact AES-256 encryption standard.", "Says data is 'securely encrypted'.", human_prefers_x=True),  # => co-14
    RankingPair("Correctly names the Pro plan for offline sync.", "Incorrectly names the Free plan.", human_prefers_x=True),  # => co-14
    RankingPair("Directly answers the question asked.", "Answers a DIFFERENT, related question instead.", human_prefers_x=True),  # => co-14
]  # => co-14: closes PAIRS -- x is the genuinely better reply in every pair


def pointwise_scorer(reply: str) -> float:  # => co-14: scores each reply IN ISOLATION, on a 1-5-like scale
    """A mocked pointwise judge -- scores each reply alone, prone to co-13's score compression."""  # => co-14: documents pointwise_scorer's contract -- no runtime output, just sets its __doc__
    return 4.0 if len(reply) > 40 else 3.5  # => co-13: length-swayed and compressed -- most replies land near the same score


def pairwise_scorer(reply_x: str, reply_y: str) -> bool:  # => co-14: scores the PAIR directly -- "which is better," not two isolated scores
    """A mocked pairwise judge -- directly compares which of two replies is better."""  # => co-14: documents pairwise_scorer's contract -- no runtime output, just sets its __doc__
    specificity_x = sum(c.isdigit() for c in reply_x) + reply_x.count("exact")  # => co-14: a crude but genuine specificity signal
    specificity_y = sum(c.isdigit() for c in reply_y) + reply_y.count("exact")  # => co-14: the same signal, applied to the OTHER reply
    return specificity_x >= specificity_y  # => co-14: directly resolves the comparison, without needing an absolute scale at all


if __name__ == "__main__":  # => co-14: entry point -- runs only when this file executes directly, not on import
    pointwise_correct = 0  # => co-14: accumulates how many pairs the POINTWISE approach ranks correctly
    pairwise_correct = 0  # => co-14: accumulates how many pairs the PAIRWISE approach ranks correctly
    for pair in PAIRS:  # => co-14: run both approaches over every pair
        pointwise_ranks_x_higher = pointwise_scorer(pair.reply_x) > pointwise_scorer(pair.reply_y)  # => co-14: pointwise's INFERRED preference
        pairwise_prefers_x = pairwise_scorer(pair.reply_x, pair.reply_y)  # => co-14: pairwise's DIRECT preference
        pointwise_correct += int(pointwise_ranks_x_higher == pair.human_prefers_x)  # => co-14: did pointwise's inferred ranking match the human?
        pairwise_correct += int(pairwise_prefers_x == pair.human_prefers_x)  # => co-14: did pairwise's direct verdict match the human?

    pointwise_agreement = pointwise_correct / len(PAIRS)  # => co-14: pointwise's overall agreement with human preference
    pairwise_agreement = pairwise_correct / len(PAIRS)  # => co-14: pairwise's overall agreement with human preference
    print(f"Pointwise agreement with human preference: {pointwise_agreement:.0%} ({pointwise_correct}/{len(PAIRS)})")  # => co-14
    print(f"Pairwise agreement with human preference: {pairwise_agreement:.0%} ({pairwise_correct}/{len(PAIRS)})")  # => co-14

    assert pairwise_agreement > pointwise_agreement, "pairwise comparison must track human relative preference better than inferring it from two isolated scores"  # => co-14: the rule this example proves
    print("MATCH: comparing the pair directly tracks human preference better than inferring a ranking from two separately-scored, compression-prone numbers")  # => co-14
    # => co-14: this tracks Liusie et al. (2023) and Liu et al. (2024) -- pairwise tends to align better with human PREFERENCE RANKING specifically, not every reliability dimension (see ex-54's contrasting robustness finding)
