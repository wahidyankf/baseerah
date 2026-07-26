# learning/code/ex-63-stratified-splitting/stratified_split.py
"""Worked Example 63: Stratified Splitting."""  # => co-15: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import TypedDict  # => co-15: one case per row, each tagged with its category, for a per-category split


class Case(TypedDict):  # => co-15: the minimal shape a stratified split needs
    case_id: str  # => co-15: unique id
    category: str  # => co-15: which category this case belongs to -- the axis this split stratifies on


CASES: list[Case] = [  # => co-15: 40 cases across four categories, DELIBERATELY imbalanced (30/6/2/2), like real traffic
    *[Case(case_id=f"pr-{i:02d}", category="password-reset") for i in range(30)],  # => co-15: 30 password-reset cases
    *[Case(case_id=f"bi-{i:02d}", category="billing") for i in range(6)],  # => co-15: 6 billing cases
    *[Case(case_id=f"bu-{i:02d}", category="bug") for i in range(2)],  # => co-15: 2 bug cases
    *[Case(case_id=f"fr-{i:02d}", category="feature-request") for i in range(2)],  # => co-15: 2 feature-request cases
]  # => co-15: closes CASES -- 40 total, badly imbalanced across categories


def stratified_split(cases: list[Case], test_fraction: float) -> tuple[list[Case], list[Case]]:  # => co-15: (train, test), proportional PER category
    """Split `cases` into (train, test) such that EACH category is split at roughly `test_fraction`, not the whole dataset at once."""  # => co-15: documents stratified_split's contract -- no runtime output, just sets its __doc__
    train: list[Case] = []  # => co-15: accumulates the train split, built up category by category
    test: list[Case] = []  # => co-15: accumulates the test split, built up category by category
    categories = sorted({c["category"] for c in cases})  # => co-15: process each category independently, in a stable order
    for category in categories:  # => co-15: split THIS category's own cases at test_fraction, not the whole dataset
        in_category = [c for c in cases if c["category"] == category]  # => co-15: only this category's rows
        test_count = max(1, round(len(in_category) * test_fraction))  # => co-15: at least one test case per category, even a rare one
        test.extend(in_category[:test_count])  # => co-15: this category's test slice
        train.extend(in_category[test_count:])  # => co-15: this category's train slice
    return train, test  # => co-15: returns this computed value to the caller


if __name__ == "__main__":  # => co-15: entry point -- runs only when this file executes directly, not on import
    train, test = stratified_split(CASES, test_fraction=0.2)  # => co-15: a 20% stratified test split
    print(f"Train: {len(train)} | Test: {len(test)}")  # => co-15: the overall split sizes
    test_categories = {c["category"] for c in test}  # => co-15: which categories actually made it into the test split
    print(f"Categories represented in test split: {sorted(test_categories)}")  # => co-15
    assert test_categories == {"password-reset", "billing", "bug", "feature-request"}, "EVERY category must appear in the test split, even the rarest ones"  # => co-15
    bug_test_count = sum(1 for c in test if c["category"] == "bug")  # => co-15: how many bug cases landed in test
    print(f"bug cases in test split: {bug_test_count} of {sum(1 for c in CASES if c['category'] == 'bug')} total bug cases")  # => co-15
    assert bug_test_count >= 1, "a purely random 20% split could easily place ZERO of the two bug cases into test -- stratification guarantees at least one"  # => co-15
    print("MATCH: every category is represented in the test split, even the ones with only two cases total")  # => co-15
    # => co-15: a naive random split on this imbalanced dataset would very plausibly leave rare categories with ZERO test coverage -- stratification fixes that structurally
