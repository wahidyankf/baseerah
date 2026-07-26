"""Worked Example 43: Detect a Case That Leaked Into a Prompt Cache, Inflating Its Score."""  # => co-22: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-22: CaseResult is a typed record with a timing field, used to detect leakage


class CaseResult(NamedTuple):  # => co-22: one eval case's result, PLUS its response latency -- the leakage signal
    case_id: str  # => co-22: which eval case this result belongs to
    passed: bool  # => co-22: the scorer's verdict
    response_latency_ms: float  # => co-22: how long the model took to respond -- a cache hit is suspiciously fast


NORMAL_LATENCIES_MS = (820.0, 910.0, 875.0, 940.0, 860.0)  # => co-22: latencies for cases that were genuinely generated, not cached

SUITE_RESULTS = (  # => co-22: a suite run where ONE case's result looks suspiciously different from the rest
    CaseResult("case-01", passed=True, response_latency_ms=820.0),  # => co-22: normal latency
    CaseResult("case-02", passed=True, response_latency_ms=910.0),  # => co-22: normal latency
    CaseResult("case-03", passed=True, response_latency_ms=12.0),  # => co-22: SUSPICIOUS -- far too fast for genuine generation; this exact case likely leaked into a prompt cache
    CaseResult("case-04", passed=True, response_latency_ms=940.0),  # => co-22: normal latency
    CaseResult("case-05", passed=False, response_latency_ms=860.0),  # => co-22: normal latency, and it failed genuinely
)  # => co-22: closes SUITE_RESULTS


def median(values: tuple[float, ...]) -> float:  # => co-22: a small stdlib-free median helper -- no import needed for five values
    """Return the median of `values`."""  # => co-22: documents median's contract -- no runtime output, just sets its __doc__
    ordered = sorted(values)  # => co-22: sorts ascending
    mid = len(ordered) // 2  # => co-22: the middle index for an odd-length sequence
    return ordered[mid]  # => co-22: returns this computed value to the caller


def flag_suspiciously_fast_cases(results: tuple[CaseResult, ...], *, baseline_ms: float, threshold_ratio: float = 0.05) -> tuple[str, ...]:  # => co-22: flags cases whose latency is implausibly below the baseline
    """Return the `case_id`s whose `response_latency_ms` is under `threshold_ratio` of `baseline_ms` -- a likely cache-hit / leakage signal."""  # => co-22: documents flag_suspiciously_fast_cases's contract -- no runtime output, just sets its __doc__
    return tuple(r.case_id for r in results if r.response_latency_ms < baseline_ms * threshold_ratio)  # => co-22: returns this computed value to the caller


if __name__ == "__main__":  # => co-22: entry point -- runs only when this file executes directly, not on import
    baseline = median(NORMAL_LATENCIES_MS)  # => co-22: establish a normal-latency baseline from genuinely-generated cases
    flagged = flag_suspiciously_fast_cases(SUITE_RESULTS, baseline_ms=baseline)  # => co-22: flag any case that ran implausibly fast
    print(f"Baseline (median) latency: {baseline:.1f}ms")  # => co-22: prints the baseline
    print(f"Flagged as possibly leaked / cached: {flagged}")  # => co-22: prints the flagged case IDs

    naive_pass_rate = sum(r.passed for r in SUITE_RESULTS) / len(SUITE_RESULTS)  # => co-22: the pass rate INCLUDING the suspicious case -- inflated
    clean_results = tuple(r for r in SUITE_RESULTS if r.case_id not in flagged)  # => co-22: exclude the flagged case before recomputing
    clean_pass_rate = sum(r.passed for r in clean_results) / len(clean_results)  # => co-22: the pass rate with the leaked case removed
    print(f"Pass rate including flagged case: {naive_pass_rate:.0%}")  # => co-22: prints the inflated rate
    print(f"Pass rate with flagged case excluded: {clean_pass_rate:.0%}")  # => co-22: prints the corrected rate

    assert flagged == ("case-03",), "only case-03's implausibly low latency must be flagged as likely leaked into a cache"  # => co-22: the rule this example proves
    assert clean_pass_rate < naive_pass_rate, "removing the suspicious case must LOWER the reported pass rate -- it was inflating the score"  # => co-22
    print(f"MATCH: case-03's suspiciously low {SUITE_RESULTS[2].response_latency_ms}ms latency flags it as likely leaked, and excluding it drops the pass rate from {naive_pass_rate:.0%} to {clean_pass_rate:.0%}")  # => co-22
    # => co-22: ex-44 next measures the noise floor of an UNCHANGED, leakage-free suite -- how much a score wobbles with no code change at all
