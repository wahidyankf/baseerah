# learning/code/ex-56-a-request-that-should-be-scoped-not-trained/scope_not_train.py
"""Worked Example 56: A Request That Should Be Scoped, Not Trained."""  # => co-05: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

# => co-05: Support wants the assistant to auto-draft replies for EVERY ticket category, and asked for a fine-tune to get there
CATEGORY_VOLUME_SHARE: dict[str, float] = {  # => co-05: what fraction of MONTHLY ticket volume each category represents
    "password-reset": 0.42,  # => co-05: by far the largest single category
    "billing": 0.23,  # => co-05
    "bug": 0.19,  # => co-05
    "feature-request": 0.16,  # => co-05
}  # => co-05: closes CATEGORY_VOLUME_SHARE -- sums to 1.00

CATEGORY_BASE_MODEL_PASS_RATE: dict[str, float] = {  # => co-05: the base model's measured pass rate, per category, from ex-07's eval
    "password-reset": 1.00,  # => co-05: the base model already handles this category perfectly
    "billing": 0.50,  # => co-05: weak
    "bug": 0.50,  # => co-05: weak
    "feature-request": 0.75,  # => co-05: moderate
}  # => co-05: closes CATEGORY_BASE_MODEL_PASS_RATE


if __name__ == "__main__":  # => co-05: entry point -- runs only when this file executes directly, not on import
    volume_covered_by_scoping = CATEGORY_VOLUME_SHARE["password-reset"]  # => co-05: ship auto-draft for JUST this one category today
    print(f"Scoping to password-reset alone covers {volume_covered_by_scoping:.0%} of monthly ticket volume")  # => co-05
    print(f"...at {CATEGORY_BASE_MODEL_PASS_RATE['password-reset']:.0%} pass rate, using the base model, with zero training")  # => co-05
    remaining_categories = [c for c in CATEGORY_VOLUME_SHARE if c != "password-reset"]  # => co-05: everything still un-served
    remaining_volume = sum(CATEGORY_VOLUME_SHARE[c] for c in remaining_categories)  # => co-05: how much volume is left over
    blended_remaining_rate = (  # => co-05: volume-weighted average pass rate across the un-served categories
        sum(  # => co-05: the volume-weighted pass rate across just the un-served categories
            CATEGORY_VOLUME_SHARE[c] * CATEGORY_BASE_MODEL_PASS_RATE[c]
            for c in remaining_categories  # => co-05
        )
        / remaining_volume  # => co-05: divide by total remaining volume share to normalize to a blended rate
    )  # => co-05: normalize by the remaining volume share
    print(f"Remaining {remaining_volume:.0%} of volume (billing, bug, feature-request) stays at a blended {blended_remaining_rate:.0%}")  # => co-05
    assert volume_covered_by_scoping > 0.4, "scoping to the single easiest category must cover a substantial share of volume"  # => co-05
    assert CATEGORY_BASE_MODEL_PASS_RATE["password-reset"] == 1.0, "the scoped category must already be fully solved by the base model"  # => co-05
    decision = "SHIP scoped auto-draft NOW; revisit remaining categories with prompting/scoping BEFORE proposing a fine-tune"  # => co-05
    print(f"Decision: {decision}")  # => co-05
    print("MATCH: scoping alone ships real value TODAY on the largest single category -- the fine-tune request is premature")  # => co-05
    # => co-05,co-06: co-05's rule made concrete -- narrowing the task, not training a model, was the highest-leverage next move here
