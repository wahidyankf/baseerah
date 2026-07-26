"""Worked Example 32: Score Groundedness Against the Source, Not a Gold Answer -- and Accept Valid Paraphrase."""  # => co-17: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

SOURCE_FACT: dict[str, object] = {"retention_days": 30, "is_permanent": True}  # => co-17: the underlying FACT itself, not any one phrasing of it


def reference_free_scorer(candidate: str, *, source: dict[str, object] = SOURCE_FACT) -> bool:  # => co-17: scores AGAINST the source fact, not a fixed phrasing
    """Pass iff `candidate` states the correct retention days AND correctly implies permanence, in ANY phrasing."""  # => co-17: documents reference_free_scorer's contract -- no runtime output, just sets its __doc__
    retention_days = source["retention_days"]  # => co-17: the ground-truth number, however this candidate chooses to express it
    states_correct_number = str(retention_days) in candidate or "a month" in candidate.lower()  # => co-17: accepts EITHER the exact number OR an equivalent common phrasing
    implies_permanence = any(w in candidate.lower() for w in ("permanently", "for good", "gone", "removed"))  # => co-17: accepts ANY phrasing that implies permanence
    return states_correct_number and implies_permanence  # => co-17: checks the FACT, not the wording


if __name__ == "__main__":  # => co-17: entry point -- runs only when this file executes directly, not on import
    close_paraphrase = "Trash files get permanently removed after 30 days."  # => co-17: same close paraphrase as ex-31
    valid_but_different_phrasing = "After a month, deleted items are gone from trash for good."  # => co-17: same differently-phrased valid answer as ex-31

    close_verdict = reference_free_scorer(close_paraphrase)  # => co-17: scores the close paraphrase, reference-free
    different_verdict = reference_free_scorer(valid_but_different_phrasing)  # => co-17: scores the differently-phrased answer, reference-free
    print(f"Close paraphrase: {close_verdict}")  # => co-17: prints the close paraphrase's verdict
    print(f"Valid, differently-phrased answer: {different_verdict}")  # => co-17: prints the differently-phrased answer's verdict

    assert close_verdict is True, "the close paraphrase must still pass reference-free scoring"  # => co-17
    assert different_verdict is True, "the differently-phrased but factually correct answer must NOW pass -- this is the fix over ex-31"  # => co-17: the rule this example proves
    print("MATCH: scoring against the SOURCE FACT, not a fixed gold phrasing, correctly accepts valid paraphrase that reference-based scoring wrongly rejected")  # => co-17
    # => co-17: reference-free scoring trades one failure mode for another -- ex-17's judge-based approach is what checks GROUNDEDNESS without either fixed-phrasing brittleness
