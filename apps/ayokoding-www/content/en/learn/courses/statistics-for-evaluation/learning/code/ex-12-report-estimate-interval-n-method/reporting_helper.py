"""Worked Example 12: A Reporting Helper -- Estimate, Interval, n, Method."""  # => co-24: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-24: a typed record -- every field below is REQUIRED, none optional

from statsmodels.stats.proportion import proportion_confint  # => co-04: the pinned library's own binomial-interval function


@dataclass(frozen=True)  # => co-24: frozen -- a reported figure is a fact about a completed run, not something later code can quietly mutate
class ReportedRate:  # => co-24: the reportable unit this whole course insists on -- never just a percentage
    estimate: float  # => co-01: the point estimate itself
    ci_low: float  # => co-04: the interval's lower bound
    ci_high: float  # => co-04: the interval's upper bound
    n: int  # => co-24: the sample size this estimate rests on
    method: str  # => co-24: the exact method used to compute the interval -- "wilson", "beta", never left implicit

    def __str__(self) -> str:  # => co-24: how this record prints -- every field visible, every time
        return f"{self.estimate:.2%} CI=[{self.ci_low:.2%}, {self.ci_high:.2%}] n={self.n} method={self.method}"  # => co-24: a bare number CANNOT escape this format


def report_rate(passes: int, n: int, *, method: str = "wilson") -> ReportedRate:  # => co-24: the ONE function every pass-rate report in this course routes through
    """Compute a pass rate and package it with its interval, n, and method -- never returns a bare float."""  # => co-24: documents report_rate's contract -- no runtime output, just sets its __doc__
    estimate = passes / n  # => co-01: the point estimate
    ci_low, ci_high = proportion_confint(passes, n, method=method)  # => co-04: the interval, computed with the STATED method
    return ReportedRate(estimate=estimate, ci_low=ci_low, ci_high=ci_high, n=n, method=method)  # => co-24: returns this computed value to the caller -- ALWAYS all four fields


if __name__ == "__main__":  # => co-24: entry point -- runs only when this file executes directly, not on import
    report = report_rate(34, 40)  # => co-24: a normal-sized eval run
    print(f"Report: {report}")  # => co-24: prints every required field, via ReportedRate.__str__
    assert isinstance(report, ReportedRate), "report_rate must always return a ReportedRate, never a bare float"  # => co-24: the type-level guarantee

    tiny_report = report_rate(9, 10)  # => co-05: a near-ceiling, very small n -- exactly where Wilson matters most
    print(f"Tiny-n report: {tiny_report}")  # => co-24: the same four fields, at a much smaller n
    assert tiny_report.n == 10 and tiny_report.method == "wilson", "n and method must be exactly what was passed in, never inferred silently"  # => co-24
    assert 0.0 <= tiny_report.ci_low <= tiny_report.estimate <= tiny_report.ci_high <= 1.0, "the interval must bracket the estimate and stay in [0, 1]"  # => co-04
    print("MATCH: both reports carry estimate, interval, n, and method -- neither can be quoted as a bare percentage")  # => co-24
    # => co-01,co-04,co-24: this is the function every LATER worked example in this course calls to print a pass rate -- never print(f'{rate:.0%}') alone again
