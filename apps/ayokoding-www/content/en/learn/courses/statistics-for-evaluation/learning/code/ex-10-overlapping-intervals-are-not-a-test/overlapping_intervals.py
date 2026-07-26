"""Worked Example 10: Overlapping Intervals Are Not a Test."""  # => co-18: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from statsmodels.stats.contingency_tables import mcnemar  # => co-18: the pinned library's own PAIRED significance test
from statsmodels.stats.proportion import proportion_confint  # => co-04: the pinned library's own binomial-interval function

N = 50  # => co-04: fifty paired cases -- baseline and candidate run on the IDENTICAL cases
BOTH_PASS = 29  # => co-18: cases where baseline AND candidate both passed
ONLY_BASELINE_PASSED = 3  # => co-18: cases where baseline passed but candidate REGRESSED
ONLY_CANDIDATE_PASSED = 13  # => co-18: cases where baseline failed but candidate FIXED it
BOTH_FAIL = 5  # => co-18: cases where both still fail -- BOTH_PASS + ONLY_BASELINE_PASSED + ONLY_CANDIDATE_PASSED + BOTH_FAIL must equal N


if __name__ == "__main__":  # => co-18: entry point -- runs only when this file executes directly, not on import
    assert BOTH_PASS + ONLY_BASELINE_PASSED + ONLY_CANDIDATE_PASSED + BOTH_FAIL == N, "the four paired-outcome counts must sum to N"  # => co-04: sanity check on the fixture
    baseline_passes = BOTH_PASS + ONLY_BASELINE_PASSED  # => co-04: baseline's OWN total pass count, ignoring pairing
    candidate_passes = BOTH_PASS + ONLY_CANDIDATE_PASSED  # => co-04: candidate's OWN total pass count, ignoring pairing
    print(f"Baseline: {baseline_passes}/{N} = {baseline_passes / N:.2%} | Candidate: {candidate_passes}/{N} = {candidate_passes / N:.2%}")  # => co-04

    lo_b, hi_b = proportion_confint(baseline_passes, N, method="wilson")  # => co-04: baseline's interval, treating the two runs as INDEPENDENT samples
    lo_c, hi_c = proportion_confint(candidate_passes, N, method="wilson")  # => co-04: candidate's interval, same (wrong) independence assumption
    print(f"Baseline Wilson CI:  [{lo_b:.4f}, {hi_b:.4f}]")  # => co-04: prints baseline's unpaired-style interval
    print(f"Candidate Wilson CI: [{lo_c:.4f}, {hi_c:.4f}]")  # => co-04: prints candidate's unpaired-style interval
    intervals_overlap = lo_c <= hi_b  # => co-04: the naive eyeball check a team might reach for
    print(f"Intervals overlap: {intervals_overlap}")  # => co-04: True here -- looks like "maybe no real difference"
    assert intervals_overlap, "the two independent-style intervals must overlap for this demo to make its point"  # => co-04: the setup this example needs

    table = [[BOTH_PASS, ONLY_BASELINE_PASSED], [ONLY_CANDIDATE_PASSED, BOTH_FAIL]]  # => co-18: the 2x2 PAIRED contingency table McNemar actually needs
    result = mcnemar(table, exact=True)  # => co-18: uses ONLY the discordant pairs -- where baseline and candidate actually disagreed
    print(f"Paired McNemar test: statistic={result.statistic}, p-value={result.pvalue:.4f}")  # => co-18: the test that actually respects the pairing
    assert result.pvalue < 0.05, "the paired McNemar test must find a significant difference despite the overlapping unpaired intervals"  # => co-18
    print("MATCH: overlapping unpaired intervals said 'maybe no difference' -- the paired test says 'yes, significant difference'")  # => co-18
    # => co-04,co-18: eyeballing two intervals for overlap ignores that the SAME cases were measured twice; McNemar uses that pairing directly, and is far more sensitive
