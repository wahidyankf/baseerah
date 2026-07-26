"""Worked Example 76: Run a Cheap Dry-Run Smoke Check Before Paying for the Full Suite."""  # => co-26: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-26: DryRunResult is a typed record -- the cheap smoke check's own verdict


class DryRunResult(NamedTuple):  # => co-26: a cheap, tiny-sample smoke check, run BEFORE the full tiered suite
    sample_size: int  # => co-26: how many cases the dry run actually checked -- a small fraction of the full suite
    sample_pass_rate: float  # => co-26: the pass rate on just this small sample
    proceed_to_full_suite: bool  # => co-25: whether it is worth spending the full suite's cost at all


DRY_RUN_SAMPLE_SIZE = 5  # => co-26: a tiny, cheap sample -- a fraction of the fast tier's own 40 cases
OBVIOUSLY_BROKEN_SAMPLE_RESULTS = (False, False, False, True, False)  # => co-26: a candidate that is clearly, badly broken -- 4 of 5 dry-run cases fail
PLAUSIBLE_SAMPLE_RESULTS = (True, True, False, True, True)  # => co-26: a candidate that looks reasonable on the cheap sample


def run_dry_run(sample_results: tuple[bool, ...], *, proceed_threshold: float = 0.5) -> DryRunResult:  # => co-26: decides whether the FULL suite is worth running at all, based on a cheap sample first
    """Return a `DryRunResult` from `sample_results`, recommending against the full suite if the sample pass rate falls below `proceed_threshold`."""  # => co-26: documents run_dry_run's contract -- no runtime output, just sets its __doc__
    pass_rate = sum(sample_results) / len(sample_results)  # => co-26: the cheap sample's own pass rate
    return DryRunResult(sample_size=len(sample_results), sample_pass_rate=pass_rate, proceed_to_full_suite=pass_rate >= proceed_threshold)  # => co-25: returns this computed value to the caller


if __name__ == "__main__":  # => co-26: entry point -- runs only when this file executes directly, not on import
    broken_dry_run = run_dry_run(OBVIOUSLY_BROKEN_SAMPLE_RESULTS)  # => co-26: dry-run check on the obviously-broken candidate
    plausible_dry_run = run_dry_run(PLAUSIBLE_SAMPLE_RESULTS)  # => co-26: dry-run check on the plausible candidate
    print(f"Obviously-broken candidate: {broken_dry_run.sample_pass_rate:.0%} on {broken_dry_run.sample_size} cases, proceed to full suite: {broken_dry_run.proceed_to_full_suite}")  # => co-26
    print(f"Plausible candidate: {plausible_dry_run.sample_pass_rate:.0%} on {plausible_dry_run.sample_size} cases, proceed to full suite: {plausible_dry_run.proceed_to_full_suite}")  # => co-26

    assert broken_dry_run.proceed_to_full_suite is False, "an obviously-broken candidate must be caught by the cheap dry run, saving the cost of the full tiered suite"  # => co-25: the rule this example proves
    assert plausible_dry_run.proceed_to_full_suite is True, (  # => co-26: opens the second assert's multi-line message
        "a plausible candidate must proceed to the full suite -- the dry run only screens out the OBVIOUSLY broken, it does not replace the real gate"  # => co-26: the assertion message itself
    )  # => co-26: the rule this example proves
    print(  # => co-26: opens the final MATCH print, reached only if both asserts above passed
        f"MATCH: the {DRY_RUN_SAMPLE_SIZE}-case dry run stops the obviously-broken candidate before it reaches the full suite, while letting the plausible candidate through -- a cheap filter, not a substitute for the real gate"
    )  # => co-26
    # => co-26: ex-77 next routes a REAL full-suite failure back not just to a new taxonomy mode, but to a specific NEW criterion
