# learning/code/ex-17-first-sft-dataset/first_sft_dataset.py
"""Worked Example 17: First SFT Dataset."""  # => co-09: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import TypedDict  # => co-09: the minimal shape every supervised fine-tuning example must have

MINIMUM_DATASET_FLOOR = 10  # => co-10: an illustrative floor for this worked example -- a real project targets "a few hundred" per co-11


class SFTExample(TypedDict):  # => co-09: input/output pairs -- the model learns to produce `response` given `instruction`
    instruction: str  # => co-09: what the model is asked to do
    response: str  # => co-09: the target the model is trained to produce for this instruction


# => co-09,co-10: twelve illustrative instruction/response pairs teaching the ticket-vocabulary behaviour from ex-08 -- authored fresh for this course
DATASET: list[SFTExample] = [  # => co-10: this dataset, not any hyperparameter, is the actual deliverable of this band
    {"instruction": "Triage: customer cannot log in after a password reset.", "response": "Priority: P2. Category: access."},  # => co-09: 1
    {"instruction": "Triage: customer reports the API is returning 500 errors.", "response": "Priority: P1. Category: bug."},  # => co-09: 2
    {"instruction": "Triage: customer wants an invoice re-sent.", "response": "Priority: P3. Category: billing."},  # => co-09: 3
    {"instruction": "Triage: a scheduled export silently failed overnight.", "response": "Priority: P1. Category: bug."},  # => co-09: 4
    {"instruction": "Triage: customer asks how to add a teammate.", "response": "Priority: P3. Category: feature-request."},  # => co-09: 5
    {"instruction": "Triage: customer's dashboard is loading slowly.", "response": "Priority: P2. Category: bug."},  # => co-09: 6
    {"instruction": "Triage: customer was double-charged this month.", "response": "Priority: P1. Category: billing."},  # => co-09: 7
    {"instruction": "Triage: customer wants dark mode added.", "response": "Priority: P3. Category: feature-request."},  # => co-09: 8
    {"instruction": "Triage: customer's SSO login is broken company-wide.", "response": "Priority: P1. Category: access."},  # => co-09: 9
    {"instruction": "Triage: customer asks about the free trial length.", "response": "Priority: P3. Category: billing."},  # => co-09: 10
    {"instruction": "Triage: a webhook stopped firing after a deploy.", "response": "Priority: P2. Category: bug."},  # => co-09: 11
    {"instruction": "Triage: customer wants to downgrade their plan.", "response": "Priority: P3. Category: billing."},  # => co-09: 12
]  # => co-10: closes DATASET


def is_valid(example: SFTExample) -> bool:  # => co-09: a minimal schema check -- both fields present and non-empty
    """Pass iff both `instruction` and `response` are non-empty strings."""  # => co-09: documents is_valid's contract -- no runtime output, just sets its __doc__
    return bool(example["instruction"].strip()) and bool(example["response"].strip())  # => co-09: neither field may be blank


if __name__ == "__main__":  # => co-09: entry point -- runs only when this file executes directly, not on import
    valid_count = sum(is_valid(ex) for ex in DATASET)  # => co-09: how many examples pass the minimal schema check
    print(f"Dataset size: {len(DATASET)} | Schema-valid: {valid_count}")  # => co-09: prints the actual, committed count
    print(f"Sample: {DATASET[0]['instruction']!r} -> {DATASET[0]['response']!r}")  # => co-09: shows one real pair, for a quick sanity check
    assert valid_count == len(DATASET), "every committed example must satisfy the minimal instruction/response schema"  # => co-09
    assert len(DATASET) >= MINIMUM_DATASET_FLOOR, f"the dataset must clear the {MINIMUM_DATASET_FLOOR}-example floor for this worked example"  # => co-10
    print(f"MATCH: {len(DATASET)} schema-valid instruction/response pairs, clearing the {MINIMUM_DATASET_FLOOR}-example floor")  # => co-09,co-10
    # => co-09,co-10: this dataset is exactly what "supervised fine-tuning" trains on -- and per co-10, THIS is the real work, not the training loop
