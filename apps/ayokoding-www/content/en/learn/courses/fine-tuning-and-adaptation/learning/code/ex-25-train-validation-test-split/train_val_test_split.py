# learning/code/ex-25-train-validation-test-split/train_val_test_split.py
"""Worked Example 25: Train/Validation/Test Split."""  # => co-15: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

ALL_CASE_IDS = [f"case-{i:03d}" for i in range(1, 101)]  # => co-15: one hundred illustrative case ids, standing in for a real dataset

TRAIN_FRACTION = 0.70  # => co-15: the majority -- what the model actually trains on
VALIDATION_FRACTION = 0.15  # => co-15: held out for early stopping (ex-40) and hyperparameter choices
TEST_FRACTION = 0.15  # => co-15: held out for the FINAL, one-time reported result -- never looked at during development


def split_dataset(case_ids: list[str]) -> tuple[list[str], list[str], list[str]]:  # => co-15: (train, validation, test), always in this order
    """Split `case_ids` into disjoint train/validation/test slices using the fixed module-level fractions."""  # => co-15: documents split_dataset's contract -- no runtime output, just sets its __doc__
    n = len(case_ids)  # => co-15: total case count
    train_end = int(n * TRAIN_FRACTION)  # => co-15: index where the train slice ends
    validation_end = train_end + int(n * VALIDATION_FRACTION)  # => co-15: index where the validation slice ends
    return case_ids[:train_end], case_ids[train_end:validation_end], case_ids[validation_end:]  # => co-15: three disjoint slices, by construction


if __name__ == "__main__":  # => co-15: entry point -- runs only when this file executes directly, not on import
    train, validation, test = split_dataset(ALL_CASE_IDS)  # => co-15: the three disjoint splits
    print(f"Train: {len(train)} | Validation: {len(validation)} | Test: {len(test)} | Total: {len(train) + len(validation) + len(test)}")  # => co-15
    train_set, validation_set, test_set = set(train), set(validation), set(test)  # => co-15: sets make overlap checking trivial
    overlap_train_val = train_set & validation_set  # => co-15: must be empty
    overlap_train_test = train_set & test_set  # => co-15: must be empty
    overlap_val_test = validation_set & test_set  # => co-15: must be empty
    print(f"Train/validation overlap: {len(overlap_train_val)} | Train/test overlap: {len(overlap_train_test)} | Validation/test overlap: {len(overlap_val_test)}")  # => co-15
    assert not overlap_train_val and not overlap_train_test and not overlap_val_test, "all three splits must be pairwise disjoint -- zero overlap"  # => co-15
    assert train_set | validation_set | test_set == set(ALL_CASE_IDS), "every original case must land in EXACTLY one split -- none dropped, none duplicated"  # => co-15
    print("MATCH: three disjoint splits, covering every original case exactly once -- the discipline everything downstream in this band relies on")  # => co-15
    # => co-15,co-16: a leak between any two of these three sets is exactly what ex-26 shows inflating a reported result
