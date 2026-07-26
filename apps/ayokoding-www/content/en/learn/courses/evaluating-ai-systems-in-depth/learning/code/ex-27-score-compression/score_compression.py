"""Worked Example 27: Annotate a 1-5 Judge Clustering on 3-4 -- The Lost Resolution."""  # => co-13: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from collections import Counter  # => co-13: a Counter is the right tool to show the clustering directly


def mock_five_point_judge(reply_quality: float) -> int:  # => co-14: a 1-5 pointwise judge -- the scale this example probes
    """A mocked 1-5 judge that AVOIDS the extremes, clustering almost everything onto 3 or 4."""  # => co-13: documents mock_five_point_judge's contract -- no runtime output, just sets its __doc__
    if reply_quality < 0.15:  # => co-13: only a genuinely terrible reply ever earns a 1 or 2
        return 2  # => co-13: rare -- the judge is reluctant to commit to the low end
    if reply_quality > 0.95:  # => co-13: only a near-perfect reply ever earns a 5
        return 5  # => co-13: rare -- the judge is equally reluctant to commit to the high end
    return 3 if reply_quality < 0.55 else 4  # => co-13: almost EVERYTHING else lands on 3 or 4 -- the compressed middle


# Ten replies spanning a WIDE range of true quality (0.05 to 0.98), fed through the 1-5 judge.
TRUE_QUALITY_SCORES = [0.05, 0.22, 0.30, 0.38, 0.45, 0.52, 0.60, 0.70, 0.85, 0.98]  # => co-13: a genuinely wide spread of true quality


if __name__ == "__main__":  # => co-13: entry point -- runs only when this file executes directly, not on import
    judge_scores = [mock_five_point_judge(q) for q in TRUE_QUALITY_SCORES]  # => co-14: run every true-quality value through the 1-5 judge
    for true_q, judge_s in zip(TRUE_QUALITY_SCORES, judge_scores):  # => co-13: prints each true value next to its judge score
        print(f"true quality={true_q:.2f} -> judge score={judge_s}")  # => co-13: one line per reply

    distribution = Counter(judge_scores)  # => co-13: how the ten scores actually distributed across the 1-5 scale
    print(f"Score distribution: {dict(sorted(distribution.items()))}")  # => co-13: prints the full distribution
    middle_share = (distribution[3] + distribution[4]) / len(judge_scores)  # => co-13: what fraction landed in the compressed middle
    print(f"Share landing on 3 or 4: {middle_share:.0%}")  # => co-13: the headline number for this example

    assert middle_share >= 0.7, "at least 70% of a wide true-quality spread must compress onto just two of the five scale points"  # => co-13: the rule this example proves
    assert distribution[1] == 0, "this judge must never actually use the scale's lowest point"  # => co-13: an unused scale point is lost resolution
    print("MATCH: ten replies spanning a wide TRUE quality range collapse onto just 2 of 5 scale points -- most of the scale's resolution is never actually used")  # => co-13
    # => co-13,co-14: a compressed scale cannot distinguish a merely-okay reply from a genuinely good one -- ex-29 shows a binary rubric avoids this entirely
