# learning/code/ex-20-consistency-audit/consistency_audit.py
"""Worked Example 20: Consistency Audit."""  # => co-12: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import TypedDict  # => co-12: the same SFT example shape ex-19 used


class SFTExample(TypedDict):  # => co-09: mirrors ex-17/ex-19's schema for this file's self-containment
    instruction: str  # => co-09: what the model is asked to do
    response: str  # => co-09: the target the model is trained to produce for this instruction


# => co-12: the SAME planted-conflict dataset from ex-19, now run through an audit BEFORE training, not discovered after
DATASET_WITH_CONFLICT: list[SFTExample] = [  # => co-10: the audit belongs in the dataset pipeline, upstream of any training run
    {"instruction": "Triage: customer cannot log in after a password reset.", "response": "Priority: P2. Category: access."},  # => co-12: conflict half 1
    {"instruction": "Triage: customer cannot log in after resetting their password.", "response": "Priority: P1. Category: access."},  # => co-12: conflict half 2
    {"instruction": "Triage: customer wants an invoice re-sent.", "response": "Priority: P3. Category: billing."},  # => co-12: consistent
    {"instruction": "Triage: customer was double-charged this month.", "response": "Priority: P1. Category: billing."},  # => co-12: consistent
    {"instruction": "Triage: customer wants dark mode added.", "response": "Priority: P3. Category: feature-request."},  # => co-12: consistent
]  # => co-12: closes DATASET_WITH_CONFLICT


DOMAIN_STOPWORDS = {"triage:", "customer"}  # => co-12: words every single instruction in this dataset shares -- excluded so they cannot fake a topic match


def shared_words(a: str, b: str) -> set[str]:  # => co-12: a crude but effective near-duplicate signal -- shared significant words
    """Return the set of words length >= 5, excluding DOMAIN_STOPWORDS, shared between `a` and `b`, lowercased."""  # => co-12: documents shared_words's contract -- no runtime output, just sets its __doc__
    words_a = {w.lower().strip(".,") for w in a.split() if len(w) >= 5} - DOMAIN_STOPWORDS  # => co-12: "significant" words, minus this dataset's own boilerplate
    words_b = {w.lower().strip(".,") for w in b.split() if len(w) >= 5} - DOMAIN_STOPWORDS  # => co-12: same filter on the second instruction
    return words_a & words_b  # => co-12: the overlap -- a proxy for "these two instructions describe the same situation"


def find_conflicts(dataset: list[SFTExample]) -> list[tuple[int, int]]:  # => co-12: the actual audit -- every near-duplicate pair with disagreeing targets
    """Return index pairs (i, j) whose instructions share >= 3 significant words but whose responses differ."""  # => co-12: documents find_conflicts's contract -- no runtime output, just sets its __doc__
    conflicts: list[tuple[int, int]] = []  # => co-12: accumulates every conflicting pair found
    for i in range(len(dataset)):  # => co-12: compare every pair exactly once
        for j in range(i + 1, len(dataset)):  # => co-12: j always ahead of i -- no duplicate (i, j)/(j, i) pairs
            overlap = shared_words(dataset[i]["instruction"], dataset[j]["instruction"])  # => co-12: how similar are these two instructions?
            same_topic = len(overlap) >= 3  # => co-12: three or more shared significant words -- likely describing the same real situation
            different_target = dataset[i]["response"] != dataset[j]["response"]  # => co-12: do they disagree about what the model should say?
            if same_topic and different_target:  # => co-12: same topic AND disagreeing targets -- a genuine planted conflict
                conflicts.append((i, j))  # => co-12: record it
    return conflicts  # => co-12: returns this computed value to the caller


if __name__ == "__main__":  # => co-12: entry point -- runs only when this file executes directly, not on import
    conflicts = find_conflicts(DATASET_WITH_CONFLICT)  # => co-12: run the audit BEFORE any training begins
    print(f"Conflicts found: {conflicts}")  # => co-12: prints the exact index pairs flagged
    for i, j in conflicts:  # => co-12: show WHAT was flagged, in plain terms
        print(f"  [{i}] {DATASET_WITH_CONFLICT[i]['response']!r} vs. [{j}] {DATASET_WITH_CONFLICT[j]['response']!r}")  # => co-12
    assert conflicts == [(0, 1)], "the audit must catch EXACTLY the planted conflict at indices 0 and 1, and nothing else"  # => co-12
    print("MATCH: the audit caught the planted conflict BEFORE training -- ex-19's inconsistency never has to reach eval to be found")  # => co-10,co-12
    # => co-10,co-12: this audit is what ex-19's failure needed all along -- catching it at dataset-review time is far cheaper than catching it at eval time
