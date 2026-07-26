# learning/code/ex-61-a-mixed-sourcing-strategy/mixed_sourcing.py
"""Worked Example 61: A Mixed Sourcing Strategy."""  # => co-13: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import Literal, TypedDict  # => co-13: a fixed, named vocabulary of sources -- every example must be tagged with exactly one

Source = Literal["production-traffic", "expert-authored", "synthetic"]  # => co-13: the three sourcing strategies ex-21/ex-22/ex-23 taught in isolation


class TaggedExample(TypedDict):  # => co-13: an SFT example, now WITH its own provenance recorded, not lost after collection
    instruction: str  # => co-09: what the model is asked to do
    response: str  # => co-09: the target the model is trained to produce for this instruction
    source: Source  # => co-13: exactly which strategy produced this example -- traceable, not anonymous


# => co-13: a blended dataset -- volume from traffic, deliberate rare-category coverage from experts, speed from synthetic
BLENDED_DATASET: list[TaggedExample] = [  # => co-13: one entry per row, provenance tag visible on every single one
    {"instruction": "Triage: customer cannot log in.", "response": "Priority: P2. Category: access.", "source": "production-traffic"},  # => co-13: 1
    {"instruction": "Triage: customer's dashboard is slow.", "response": "Priority: P2. Category: bug.", "source": "production-traffic"},  # => co-13: 2
    {"instruction": "Triage: customer wants dark mode.", "response": "Priority: P3. Category: feature-request.", "source": "expert-authored"},  # => co-13: 3 -- rare category, deliberately authored
    {"instruction": "Triage: customer wants SSO support.", "response": "Priority: P3. Category: feature-request.", "source": "expert-authored"},  # => co-13: 4 -- rare category, deliberately authored
    {"instruction": "Triage: customer asks about export limits.", "response": "Priority: P3. Category: billing.", "source": "synthetic"},  # => co-13: 5
    {"instruction": "Triage: customer asks about seat pricing.", "response": "Priority: P3. Category: billing.", "source": "synthetic"},  # => co-13: 6
]  # => co-13: closes BLENDED_DATASET


if __name__ == "__main__":  # => co-13: entry point -- runs only when this file executes directly, not on import
    source_counts: dict[str, int] = {}  # => co-13: tally examples per source
    for example in BLENDED_DATASET:  # => co-13: count every row's provenance tag
        source_counts[example["source"]] = source_counts.get(example["source"], 0) + 1  # => co-13: increment this source's count
    for source, count in source_counts.items():  # => co-13: prints the actual blend achieved
        print(f"  {source}: {count} examples")  # => co-13
    feature_request_examples = [ex for ex in BLENDED_DATASET if ex["response"].endswith("feature-request.")]  # => co-13: this course's rare category
    feature_request_sources = {ex["source"] for ex in feature_request_examples}  # => co-13: WHICH sourcing strategy actually covers it
    print(f"feature-request examples came from: {feature_request_sources}")  # => co-13
    assert feature_request_sources == {"expert-authored"}, "the rare category must be covered ONLY by the deliberate, expert-authored source"  # => co-13
    assert len(source_counts) == 3, "the blend must genuinely draw from all three sourcing strategies, not just one"  # => co-13
    print("MATCH: the blend combines traffic's volume, expert authoring's deliberate rare-category coverage, and synthetic's speed -- each doing what it is best at")  # => co-13
    # => co-13: no single source from ex-21/ex-22/ex-23 alone would have produced this dataset's coverage profile at this cost
