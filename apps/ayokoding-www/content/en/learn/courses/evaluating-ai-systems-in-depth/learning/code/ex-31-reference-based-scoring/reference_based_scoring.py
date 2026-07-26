"""Worked Example 31: Score Against Gold Answers -- and See Where It Breaks on Valid Alternative Phrasings."""  # => co-17: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

GOLD_ANSWER = "Files in trash are permanently removed after 30 days."  # => co-17: the single reference answer this scorer compares against


def reference_based_scorer(candidate: str, *, gold: str = GOLD_ANSWER) -> bool:  # => co-17: scores AGAINST the fixed gold answer -- never sees the source
    """Pass iff `candidate` shares at least 4 of the gold answer's 5 key content words."""  # => co-17: documents reference_based_scorer's contract -- no runtime output, just sets its __doc__
    gold_key_words = {"files", "trash", "permanently", "removed", "30"}  # => co-17: the gold answer's own key content words
    candidate_words = set(candidate.lower().replace(".", "").split())  # => co-17: the candidate's own words, normalized
    overlap = gold_key_words & candidate_words  # => co-17: how many of the gold's key words the candidate shares
    return len(overlap) >= 4  # => co-17: an arbitrary-but-fixed overlap bar, applied identically to every candidate


if __name__ == "__main__":  # => co-17: entry point -- runs only when this file executes directly, not on import
    close_paraphrase = "Trash files get permanently removed after 30 days."  # => co-17: a valid paraphrase, close to the gold's own wording
    valid_but_different_phrasing = "After a month, deleted items are gone from trash for good."  # => co-17: a VALID answer, phrased almost entirely differently

    close_verdict = reference_based_scorer(close_paraphrase)  # => co-17: scores the close paraphrase
    different_verdict = reference_based_scorer(valid_but_different_phrasing)  # => co-17: scores the differently-phrased valid answer
    print(f"Close paraphrase: {close_verdict} ({close_paraphrase!r})")  # => co-17: prints the close paraphrase's verdict
    print(f"Valid, differently-phrased answer: {different_verdict} ({valid_but_different_phrasing!r})")  # => co-17: prints the differently-phrased answer's verdict

    assert close_verdict is True, "a paraphrase sharing the gold's own key words must pass reference-based scoring"  # => co-17
    assert different_verdict is False, "a VALID answer phrased almost entirely differently must FAIL, despite being correct"  # => co-17: the failure mode this example demonstrates
    print("MATCH: reference-based scoring passes a close paraphrase but wrongly fails an equally valid, differently-worded answer")  # => co-17
    # => co-17: ex-32 scores the SAME differently-phrased answer reference-FREE instead, against the source fact -- and gets it right
