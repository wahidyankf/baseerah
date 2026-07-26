# learning/code/ex-59-deduplicating-the-dataset/dedupe_dataset.py
"""Worked Example 59: Deduplicating the Dataset."""  # => co-11: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import TypedDict  # => co-09: the same SFT example shape reused across this band


class SFTExample(TypedDict):  # => co-09: mirrors ex-17's schema for this file's self-containment
    instruction: str  # => co-09: what the model is asked to do
    response: str  # => co-09: the target the model is trained to produce for this instruction


# => co-11: a dataset where one pattern was copy-pasted with trivial edits five separate times, skewing training toward it
RAW_DATASET: list[SFTExample] = [  # => co-10: near-duplicates are silent -- they look like five DIFFERENT examples in a file listing
    {"instruction": "Triage: customer cannot log in after a password reset.", "response": "Priority: P2. Category: access."},  # => co-11: original
    {"instruction": "Triage: customer cannot log in after a password reset!", "response": "Priority: P2. Category: access."},  # => co-11: near-dup 1 (punctuation)
    {"instruction": "Triage: customer cannot log in after a password reset.  ", "response": "Priority: P2. Category: access."},  # => co-11: near-dup 2 (trailing whitespace)
    {"instruction": "triage: customer cannot log in after a password reset.", "response": "Priority: P2. Category: access."},  # => co-11: near-dup 3 (casing)
    {"instruction": "Triage: customer wants an invoice re-sent.", "response": "Priority: P3. Category: billing."},  # => co-11: a genuinely distinct example
    {"instruction": "Triage: customer was double-charged this month.", "response": "Priority: P1. Category: billing."},  # => co-11: a genuinely distinct example
]  # => co-11: closes RAW_DATASET -- 4 of 6 rows are the SAME underlying example, restated


def normalize(instruction: str) -> str:  # => co-11: the near-duplicate signal -- case-fold and collapse whitespace/punctuation noise
    """Return `instruction`, lowercased, stripped, with trailing punctuation removed, for near-duplicate comparison."""  # => co-11: documents normalize's contract -- no runtime output, just sets its __doc__
    return instruction.strip().lower().rstrip("!.")  # => co-11: collapses exactly the three cosmetic variants planted above to one key


def deduplicate(dataset: list[SFTExample]) -> list[SFTExample]:  # => co-11: keep the FIRST occurrence of each normalized instruction
    """Return `dataset` with near-duplicate instructions (by `normalize`) collapsed to their first occurrence."""  # => co-11: documents deduplicate's contract -- no runtime output, just sets its __doc__
    seen: set[str] = set()  # => co-11: tracks normalized keys already kept
    result: list[SFTExample] = []  # => co-11: accumulates the deduplicated dataset
    for example in dataset:  # => co-11: process in original order, keeping the first of each near-duplicate group
        key = normalize(example["instruction"])  # => co-11: this example's normalized identity
        if key not in seen:  # => co-11: only the first occurrence of each identity survives
            seen.add(key)  # => co-11: mark this identity as kept
            result.append(example)  # => co-11: keep this example
    return result  # => co-11: returns this computed value to the caller


if __name__ == "__main__":  # => co-11: entry point -- runs only when this file executes directly, not on import
    deduped = deduplicate(RAW_DATASET)  # => co-11: run the dedup pass BEFORE training
    print(f"Raw dataset: {len(RAW_DATASET)} rows | Deduplicated: {len(deduped)} rows")  # => co-11: prints the before/after counts
    for example in deduped:  # => co-11: shows what actually survived
        print(f"  {example['instruction']!r}")  # => co-11
    assert len(deduped) == 3, "the four near-duplicate rows must collapse to exactly one, leaving three distinct examples"  # => co-11
    access_rows = [ex for ex in deduped if ex["response"] == "Priority: P2. Category: access."]  # => co-11: how many access-category rows remain
    assert len(access_rows) == 1, "only ONE copy of the near-duplicated access example must survive deduplication"  # => co-10,co-11
    print("MATCH: four cosmetic restatements of the SAME example collapsed to one -- the dataset's true diversity was 3, not 6")  # => co-10,co-11
    # => co-10,co-11: an un-deduplicated dataset silently over-weights whatever pattern got copy-pasted, which is a quality problem, not a size one
