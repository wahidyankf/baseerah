"""Worked Example 58: Quantify a Reference-Based Scorer's False-Negative Rate Across Many Valid Paraphrases."""  # => co-17: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import TypedDict  # => co-17: PhrasingCase types every field this rate calculation reads


class PhrasingCase(TypedDict):  # => co-17: one VALID answer, phrased differently from the gold, all human-confirmed correct
    candidate: str  # => co-17: the candidate reply, factually correct but phrased differently from GOLD_ANSWER
    is_actually_correct: bool  # => co-08: human-adjudicated -- every entry here is True, by construction of this fixture


GOLD_ANSWER_KEY_WORDS = {"files", "trash", "permanently", "removed", "30"}  # => co-17: ex-31's exact gold-answer key words, reused
VALID_PARAPHRASES: list[PhrasingCase] = [  # => co-17: eight VALID, human-confirmed-correct answers, phrased in eight different ways
    {"candidate": "Files in trash are permanently removed after 30 days.", "is_actually_correct": True},  # => co-17: matches gold closely
    {"candidate": "After a month, deleted items are gone from trash for good.", "is_actually_correct": True},  # => co-17: differently phrased
    {"candidate": "Trash empties itself automatically once 30 days have passed.", "is_actually_correct": True},  # => co-17: differently phrased
    {"candidate": "Your deleted items vanish permanently 30 days after deletion.", "is_actually_correct": True},  # => co-17: differently phrased
    {"candidate": "The trash folder purges content older than a month.", "is_actually_correct": True},  # => co-17: differently phrased
    {"candidate": "Anything sitting in trash for 30+ days gets wiped for good.", "is_actually_correct": True},  # => co-17: differently phrased
    {"candidate": "It's a 30-day window before trash contents are gone permanently.", "is_actually_correct": True},  # => co-17: differently phrased
    {"candidate": "Deleted files are permanently gone from trash after 30 days.", "is_actually_correct": True},  # => co-17: close to gold
]  # => co-17: closes VALID_PARAPHRASES


def reference_based_scorer(candidate: str, *, gold_key_words: set[str] = GOLD_ANSWER_KEY_WORDS) -> bool:  # => co-17: ex-31's exact scorer, reused for a RATE measurement here
    """Pass iff `candidate` shares at least 4 of the gold answer's 5 key content words."""  # => co-17: documents reference_based_scorer's contract -- no runtime output, just sets its __doc__
    candidate_words = set(candidate.lower().replace(".", "").split())  # => co-17: the candidate's own words, normalized
    return len(gold_key_words & candidate_words) >= 4  # => co-17: an arbitrary-but-fixed overlap bar


if __name__ == "__main__":  # => co-17: entry point -- runs only when this file executes directly, not on import
    false_negatives = [c for c in VALID_PARAPHRASES if not reference_based_scorer(c["candidate"])]  # => co-17: every VALID answer this scorer wrongly rejects
    for case in false_negatives:  # => co-17: prints every false negative found
        print(f"FALSE NEGATIVE: {case['candidate']!r}")  # => co-17: one line per wrongly-rejected valid answer

    false_negative_rate = len(false_negatives) / len(VALID_PARAPHRASES)  # => co-17: the headline rate -- how often a VALID answer gets wrongly rejected
    print(f"False-negative rate on {len(VALID_PARAPHRASES)} valid paraphrases: {false_negative_rate:.0%}")  # => co-17

    assert false_negative_rate >= 0.5, "reference-based scoring must wrongly reject at least half of these genuinely valid paraphrases"  # => co-17: the rule this example proves
    print("MATCH: reference-based scoring's false-negative rate on valid paraphrase is high enough to make it unusable as this criterion's sole scorer")  # => co-17
    # => co-17: this is a RATE across eight real cases, not a single anecdote -- the anecdote in ex-31 generalizes into a genuine, measurable failure rate
