# learning/code/ex-60-balancing-task-coverage/balancing_coverage.py
"""Worked Example 60: Balancing Task Coverage."""  # => co-10: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

MINIMUM_EXAMPLES_PER_CATEGORY = 20  # => co-10: below this floor, a category is too thin to teach the model anything reliable

RAW_CATEGORY_COUNTS: dict[str, int] = {  # => co-12: how many examples this dataset ACTUALLY has per category, before any fix
    "password-reset": 180,  # => co-10: well over the floor
    "billing": 95,  # => co-10: well over the floor
    "bug": 40,  # => co-10: over the floor, but thinner
    "feature-request": 6,  # => co-10,co-12: badly under the floor -- the model will barely learn this category at all
}  # => co-10: closes RAW_CATEGORY_COUNTS


def under_covered_categories(counts: dict[str, int], floor: int) -> list[str]:  # => co-10: the categories a real audit must flag
    """Return the categories in `counts` whose count falls below `floor`."""  # => co-10: documents under_covered_categories's contract -- no runtime output, just sets its __doc__
    return [category for category, count in counts.items() if count < floor]  # => co-10: names exactly which categories are thin


if __name__ == "__main__":  # => co-10: entry point -- runs only when this file executes directly, not on import
    for category, count in RAW_CATEGORY_COUNTS.items():  # => co-10: show every category's raw count against the floor
        status = "OK" if count >= MINIMUM_EXAMPLES_PER_CATEGORY else "UNDER-COVERED"  # => co-10
        print(f"  {category}: {count} examples ({status})")  # => co-10
    thin_categories = under_covered_categories(RAW_CATEGORY_COUNTS, MINIMUM_EXAMPLES_PER_CATEGORY)  # => co-10: the audit's actual finding
    print(f"Under-covered categories: {thin_categories}")  # => co-10
    assert thin_categories == ["feature-request"], "exactly one category must fall below the coverage floor in this scenario"  # => co-10
    additional_examples_needed = MINIMUM_EXAMPLES_PER_CATEGORY - RAW_CATEGORY_COUNTS["feature-request"]  # => co-10: how many MORE examples this category needs
    print(f"feature-request needs {additional_examples_needed} more examples before training, or the whole run should be scoped without it")  # => co-10
    assert additional_examples_needed == 14, "the specific gap must be computed exactly, not estimated"  # => co-10
    print("MATCH: the audit flags feature-request as too thin to train on reliably -- a coverage decision made BEFORE training, not discovered after")  # => co-10,co-12
    # => co-10,co-12: a dataset can pass ex-20's consistency audit and STILL fail here -- consistency and coverage are two separate checks, both part of "dataset is the work"
