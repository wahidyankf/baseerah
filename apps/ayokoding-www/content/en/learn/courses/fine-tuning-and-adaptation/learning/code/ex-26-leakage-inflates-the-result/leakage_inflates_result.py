# learning/code/ex-26-leakage-inflates-the-result/leakage_inflates_result.py
"""Worked Example 26: Leakage Inflates the Result."""  # => co-16: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

TRAIN_IDS = {f"case-{i:03d}" for i in range(1, 71)}  # => co-15: the clean 70-case training split, per ex-25's discipline
CLEAN_TEST_IDS = {f"case-{i:03d}" for i in range(86, 101)}  # => co-15: the honest, disjoint 15-case test split

LEAKED_TEST_IDS = CLEAN_TEST_IDS | {"case-005", "case-012", "case-033"}  # => co-16: THREE training cases accidentally copied into "test" too

# => co-16: a mocked per-case eval result -- True on every case the (over-fit, memorizing) model gets right
GENUINELY_UNSEEN_CORRECT = {"case-088", "case-091", "case-095"}  # => co-16: it ALSO happens to get a few genuinely-unseen test cases right, honestly
MEMORIZED_CORRECTLY: set[str] = TRAIN_IDS | GENUINELY_UNSEEN_CORRECT  # => co-16: the model is perfect on anything it has SEEN in training, including the leaked copies


def eval_pass_rate(test_ids: set[str], correct: set[str]) -> float:  # => co-16: the SAME scoring function, run against two different "test" sets
    """Return the fraction of `test_ids` present in `correct`."""  # => co-16: documents eval_pass_rate's contract -- no runtime output, just sets its __doc__
    return len(test_ids & correct) / len(test_ids)  # => co-16: fraction of the given test set the model got right


if __name__ == "__main__":  # => co-16: entry point -- runs only when this file executes directly, not on import
    leaked_rate = eval_pass_rate(LEAKED_TEST_IDS, MEMORIZED_CORRECTLY)  # => co-16: the reported result USING the leaked test set
    clean_rate = eval_pass_rate(CLEAN_TEST_IDS, MEMORIZED_CORRECTLY)  # => co-16: the TRUE result, using the honest, disjoint test set
    print(f"Leaked test set ({len(LEAKED_TEST_IDS)} cases, includes 3 training copies): {leaked_rate:.0%} pass rate")  # => co-16
    print(f"Clean test set ({len(CLEAN_TEST_IDS)} cases, zero training overlap): {clean_rate:.0%} pass rate")  # => co-16
    overlap = LEAKED_TEST_IDS & TRAIN_IDS  # => co-16: exactly which test cases were ALSO in training -- the leak itself
    print(f"Leaked cases (present in both train AND test): {sorted(overlap)}")  # => co-16
    assert len(overlap) == 3, "exactly three training cases must have leaked into the leaked test set"  # => co-16
    assert leaked_rate > clean_rate, "the leaked test set must report a HIGHER, inflated pass rate than the clean one"  # => co-15,co-16
    print(f"MATCH: leakage inflated the reported result from {clean_rate:.0%} (true) to {leaked_rate:.0%} (leaked) -- a result that will NOT transfer to production")  # => co-16
    # => co-15,co-16: this is co-16 made concrete -- the leaked number looks better and is simply wrong; production sees the clean number's reality
