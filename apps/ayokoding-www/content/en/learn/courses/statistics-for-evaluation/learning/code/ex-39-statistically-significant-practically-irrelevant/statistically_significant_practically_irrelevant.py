"""Worked Example 39: Statistically Significant, Practically Irrelevant."""  # => co-19: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-19: draws two large independent samples of two near-identical systems

from statsmodels.stats.proportion import proportions_ztest  # => co-19: the SAME two-proportion test ex-36 introduced

TRUE_A_RATE = 0.850  # => co-19: system A's real pass rate
TRUE_B_RATE = 0.862  # => co-19: system B's real pass rate -- only 1.2 points higher, genuinely
N = 8000  # => co-19: a LARGE eval run -- big enough to detect even a small true gap
MATERIALITY_THRESHOLD = 0.03  # => co-19: this team's own stated bar for "worth acting on" -- a 3-point gap or larger


def sample_outcomes(true_rate: float, n: int, *, seed: int) -> list[bool]:  # => co-19: one independent sample of one system's outcomes
    """Draw n independent Bernoulli outcomes from true_rate."""  # => co-19: documents sample_outcomes's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-19: one fixed generator per sample
    return [rng.random() < true_rate for _ in range(n)]  # => co-19: one Bernoulli draw per case


if __name__ == "__main__":  # => co-19: entry point -- runs only when this file executes directly, not on import
    a = sample_outcomes(TRUE_A_RATE, N, seed=31)  # => co-19: system A's own large sample
    b = sample_outcomes(TRUE_B_RATE, N, seed=32)  # => co-19: system B's own large sample
    a_rate = sum(a) / N  # => co-19: A's point estimate
    b_rate = sum(b) / N  # => co-19: B's point estimate
    gap = b_rate - a_rate  # => co-19: the observed gap
    print(f"N={N}: A={a_rate:.4f} B={b_rate:.4f} gap={gap:+.4f}")  # => co-19: a genuinely small gap

    count = [sum(b), sum(a)]  # => co-19: successes for each group
    nobs = [N, N]  # => co-19: each group's sample size
    z_stat, p_value = proportions_ztest(count, nobs)  # => co-19: the SAME formal test ex-36 built
    print(f"Two-proportion z-test: z={z_stat:.4f} p={p_value:.4f}")  # => co-19: the test's own verdict

    assert p_value < 0.05, "at this large n, the test must reach statistical significance despite the small true gap"  # => co-19: the "statistically significant" half of the claim
    assert abs(gap) < MATERIALITY_THRESHOLD, "the observed gap must fall BELOW this team's own materiality threshold, despite being statistically significant"  # => co-19: the "practically irrelevant" half of the claim
    print(f"MATCH: p={p_value:.4f} < 0.05 (statistically significant) AND gap={gap:+.4f} is below the {MATERIALITY_THRESHOLD:.2f} materiality threshold (practically irrelevant)")  # => co-19
    # => co-19: a large enough sample can make even a genuinely trivial difference statistically detectable -- 'significant' answers 'is there a real difference,' never 'is this difference worth acting on'
