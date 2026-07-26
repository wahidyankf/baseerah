# learning/code/ex-19-inconsistent-examples/inconsistent_examples.py
"""Worked Example 19: Inconsistent Examples."""  # => co-12: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import TypedDict  # => co-12: the same SFT example shape ex-17 used, reused here to plant a conflict


class SFTExample(TypedDict):  # => co-09: mirrors ex-17's schema for this file's self-containment
    instruction: str  # => co-09: what the model is asked to do
    response: str  # => co-09: the target the model is trained to produce for this instruction


# => co-12: two examples for the NEAR-IDENTICAL instruction, disagreeing about the target behaviour -- planted deliberately
DATASET_WITH_CONFLICT: list[SFTExample] = [  # => co-12: a five-example dataset, with one planted internal disagreement
    {"instruction": "Triage: customer cannot log in after a password reset.", "response": "Priority: P2. Category: access."},  # => co-12: says P2
    {"instruction": "Triage: customer cannot log in after resetting their password.", "response": "Priority: P1. Category: access."},  # => co-12: says P1 -- SAME situation, different wording, CONTRADICTS the row above
    {"instruction": "Triage: customer wants an invoice re-sent.", "response": "Priority: P3. Category: billing."},  # => co-12: unrelated, consistent
    {"instruction": "Triage: customer was double-charged this month.", "response": "Priority: P1. Category: billing."},  # => co-12: unrelated, consistent
    {"instruction": "Triage: customer wants dark mode added.", "response": "Priority: P3. Category: feature-request."},  # => co-12: unrelated, consistent
]  # => co-12: closes DATASET_WITH_CONFLICT


def mock_trained_on(dataset: list[SFTExample], instruction: str) -> str:  # => co-12: a model TRAINED on this exact dataset, queried at inference
    """Return the response the model learned for the CLOSEST matching training instruction (mocked as an exact keyword match here)."""  # => co-12: documents mock_trained_on's contract -- no runtime output, just sets its __doc__
    for example in dataset:  # => co-12: a real model would generalize across near-duplicates -- this mock finds the literal match it memorized
        if example["instruction"] == instruction:  # => co-12: exact match -- what the model actually memorized from training
            return example["response"]  # => co-12: recites exactly what it was trained on for THIS phrasing
    return "UNSEEN"  # => co-12: no training example matched this exact phrasing


if __name__ == "__main__":  # => co-12: entry point -- runs only when this file executes directly, not on import
    answer_1 = mock_trained_on(DATASET_WITH_CONFLICT, "Triage: customer cannot log in after a password reset.")  # => co-12: phrasing A
    answer_2 = mock_trained_on(DATASET_WITH_CONFLICT, "Triage: customer cannot log in after resetting their password.")  # => co-12: phrasing B
    print(f"Phrasing A -> {answer_1!r}")  # => co-12: prints the model's memorized answer for phrasing A
    print(f"Phrasing B -> {answer_2!r}")  # => co-12: prints the model's memorized answer for phrasing B -- the SAME real-world situation
    assert answer_1 != answer_2, "the two near-identical phrasings must produce DIFFERENT trained answers"  # => co-12
    print(f"Inconsistent: {answer_1 != answer_2} -- the same real situation gets two different priorities depending on exact wording")  # => co-12
    print("MATCH: the planted conflict taught the model to be genuinely inconsistent, not just imprecise")  # => co-12
    # => co-12: this failure is invisible in TRAINING loss (both examples fit perfectly) -- it only shows up when you probe near-duplicate inputs at eval time
