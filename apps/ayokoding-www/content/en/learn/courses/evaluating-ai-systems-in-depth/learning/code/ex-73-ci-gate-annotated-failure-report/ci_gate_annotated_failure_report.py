"""Worked Example 73: Turn a Blocked CI Run Into an Annotated, Actionable Failure Report."""  # => co-23: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-23: FailedCaseDetail and FailureReport are typed records -- a report is structured data, not a log dump


class FailedCaseDetail(NamedTuple):  # => co-23: one specific case's own failure detail, not just a count
    case_id: str  # => co-23: which case failed
    failure_mode: str  # => co-27: which taxonomy mode this failure matches, linking straight back to error analysis
    request: str  # => co-23: the request that triggered the failure


class FailureReport(NamedTuple):  # => co-23: the FULL CI-log-ready report, not a bare pass/fail exit code
    total_cases: int  # => co-23: how many cases ran
    failed_cases: tuple[FailedCaseDetail, ...]  # => co-23: the specific cases that failed, WITH their taxonomy mode
    dominant_failure_mode: str | None  # => co-27: which mode caused the most failures, pointing directly at the next error-analysis priority


THIS_RUN_FAILURES = (  # => co-23: raw failures from one CI run
    FailedCaseDetail("case-05", failure_mode="skips-clarifying-question", request="Move this to review."),  # => co-27
    FailedCaseDetail("case-11", failure_mode="skips-clarifying-question", request="Archive that ticket."),  # => co-27
    FailedCaseDetail("case-19", failure_mode="wrong-object-acted-on", request="Close #88."),  # => co-27
)  # => co-23: closes THIS_RUN_FAILURES


def build_failure_report(total_cases: int, failed_cases: tuple[FailedCaseDetail, ...]) -> FailureReport:  # => co-23: turns raw failures into an ACTIONABLE report -- which mode dominates, not just a count
    """Return a `FailureReport` summarizing `failed_cases` out of `total_cases`, naming the dominant failure mode."""  # => co-23: documents build_failure_report's contract -- no runtime output, just sets its __doc__
    if not failed_cases:  # => co-23: no failures at all -- nothing to attribute
        return FailureReport(total_cases=total_cases, failed_cases=(), dominant_failure_mode=None)  # => co-23: returns this computed value to the caller
    mode_counts: dict[str, int] = {}  # => co-27: tallies failures per taxonomy mode
    for case in failed_cases:  # => co-27: counts each failed case's mode
        mode_counts[case.failure_mode] = mode_counts.get(case.failure_mode, 0) + 1  # => co-27: increments this mode's tally
    dominant = max(mode_counts, key=lambda mode: mode_counts[mode])  # => co-27: the mode responsible for the MOST failures -- the next error-analysis priority
    return FailureReport(total_cases=total_cases, failed_cases=failed_cases, dominant_failure_mode=dominant)  # => co-23: returns this computed value to the caller


if __name__ == "__main__":  # => co-23: entry point -- runs only when this file executes directly, not on import
    report = build_failure_report(total_cases=40, failed_cases=THIS_RUN_FAILURES)  # => co-23: build the annotated report for this CI run
    print(f"CI run: {report.total_cases} total cases, {len(report.failed_cases)} failed")  # => co-23: prints the headline numbers
    for detail in report.failed_cases:  # => co-23: prints every specific failed case, annotated with its own mode
        print(f"  {detail.case_id}: {detail.failure_mode!r} -- {detail.request!r}")  # => co-23
    print(f"Dominant failure mode: {report.dominant_failure_mode!r}")  # => co-27: prints the actionable, prioritized signal

    assert len(report.failed_cases) == 3, "the report must list every individual failed case, not just a bare count"  # => co-23: the rule this example proves
    assert report.dominant_failure_mode == "skips-clarifying-question", "the report must correctly identify the mode responsible for the MOST failures as dominant"  # => co-27: the rule this example proves
    print(f"MATCH: the CI report names {len(report.failed_cases)} specific failed cases and correctly identifies {report.dominant_failure_mode!r} as the dominant mode -- an actionable report, not a bare exit code")  # => co-23
    # => co-23: ex-74 next examines a DIFFERENT CI-gate risk -- a judge call that times out or fails transiently, not a genuine eval failure
