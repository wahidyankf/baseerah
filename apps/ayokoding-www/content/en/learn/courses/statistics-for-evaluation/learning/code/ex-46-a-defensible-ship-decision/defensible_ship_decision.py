"""Worked Example 46: A Defensible Ship Decision."""  # => co-24: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-24: builds one paired baseline/candidate comparison, tying this theme's techniques together
from dataclasses import dataclass  # => co-24: forces the final ship/hold call through one typed, checkable record -- no bare verdict allowed

from statsmodels.stats.contingency_tables import mcnemar  # => co-18: the paired significance test, per ex-37/ex-38
from statsmodels.stats.proportion import proportion_confint  # => co-04,co-05: the small-n-appropriate interval, per Theme A

MEASURED_NOISE_FLOOR = 0.03  # => co-22: this suite's own measured generation-noise floor -- from a decomposition like ex-45's, run previously on this exact eval set
MATERIALITY_THRESHOLD = 0.05  # => co-19: this team's own stated bar for "worth acting on" -- a 5-point gap or larger


def build_paired_dataset(n: int, *, seed: int, baseline_rate: float, candidate_rate: float, correlation: float) -> tuple[list[bool], list[bool]]:  # => co-18: the SAME paired-fixture shape as ex-37/ex-38
    """Build paired baseline/candidate outcomes over n SHARED items, correlated by per-item difficulty."""  # => co-18: documents build_paired_dataset's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-18: drives each item's shared difficulty draw
    baseline: list[bool] = []  # => co-18: baseline's verdict, one per item
    candidate: list[bool] = []  # => co-18: candidate's verdict on the SAME item, one per item
    for _ in range(n):  # => co-18: one shared item at a time
        difficulty_draw = rng.random()  # => co-18: this item's own shared difficulty draw -- read by BOTH systems below
        baseline_pass = difficulty_draw < baseline_rate  # => co-18: baseline's verdict on this exact item
        if rng.random() < correlation:  # => co-18: most of the time, candidate's verdict is driven by the SAME difficulty draw
            candidate_pass = difficulty_draw < candidate_rate  # => co-18: correlated verdict
        else:  # => co-18: occasionally, candidate's verdict is an independent draw instead
            candidate_pass = rng.random() < candidate_rate  # => co-18: an independent verdict
        baseline.append(baseline_pass)  # => co-18: records this item's baseline verdict
        candidate.append(candidate_pass)  # => co-18: records this item's candidate verdict, SAME item, SAME index
    return baseline, candidate  # => co-18: two same-length, index-aligned lists


@dataclass(frozen=True)  # => co-24: immutable -- a ship decision cannot be silently edited after being made
class ShipDecision:  # => co-24: EVERY field this theme built, gathered into the ONE record a real ship/hold call rests on
    n: int  # => co-06: how many paired items this decision rests on
    candidate_rate: float  # => co-02: candidate's own point estimate
    ci_low: float  # => co-04,co-05: candidate's own interval -- small-n-appropriate, per Theme A
    ci_high: float  # => co-04,co-05: candidate's own interval upper bound
    gap: float  # => co-17: the observed gap over baseline
    mcnemar_p: float  # => co-18: the paired significance test's own p-value
    noise_floor: float  # => co-22: this suite's own measured generation-noise floor
    materiality_threshold: float  # => co-19: this team's own stated bar for "worth acting on"
    verdict: str  # => co-24: SHIP or HOLD -- derived from every field above, never chosen by feel


def decide(baseline: list[bool], candidate: list[bool]) -> ShipDecision:  # => co-24: the ONE function that turns raw paired data into a defensible decision
    """Compute every field a ship/hold decision needs, and the resulting verdict."""  # => co-24: documents decide's contract -- no runtime output, just sets its __doc__
    n = len(baseline)  # => co-06: item count
    candidate_rate = sum(candidate) / n  # => co-02: candidate's own point estimate
    ci_low, ci_high = proportion_confint(sum(candidate), n, alpha=0.05, method="wilson")  # => co-04,co-05: candidate's own Wilson interval, per Theme A
    gap = candidate_rate - (sum(baseline) / n)  # => co-17: the observed gap over baseline

    both_pass = sum(1 for b, c in zip(baseline, candidate) if b and c)  # => co-18: concordant pairs -- uninformative for the test
    both_fail = sum(1 for b, c in zip(baseline, candidate) if not b and not c)  # => co-18: concordant pairs -- uninformative for the test
    baseline_only = sum(1 for b, c in zip(baseline, candidate) if b and not c)  # => co-18: discordant -- candidate regressed
    candidate_only = sum(1 for b, c in zip(baseline, candidate) if not b and c)  # => co-18: discordant -- candidate improved
    table = [[both_pass, baseline_only], [candidate_only, both_fail]]  # => co-18: the 2x2 table McNemar's test uses
    mcnemar_p = mcnemar(table, exact=False, correction=True).pvalue  # => co-18: the paired significance test's own p-value

    is_significant = mcnemar_p < 0.05  # => co-18: passes the formal test, per ex-37/ex-38
    beats_noise_floor = gap > MEASURED_NOISE_FLOOR  # => co-22: the gap must exceed what an UNCHANGED system's own re-runs would show, per ex-45
    beats_materiality = gap > MATERIALITY_THRESHOLD  # => co-19: the gap must clear this team's own stated bar for action, per ex-39/ex-40
    verdict = "SHIP" if (is_significant and beats_noise_floor and beats_materiality) else "HOLD"  # => co-24: ALL THREE conditions, never any one number alone

    return ShipDecision(  # => co-24: every field populated -- no bare verdict leaves this function
        n=n,  # => co-06: how many paired items this decision rests on
        candidate_rate=candidate_rate,  # => co-02: candidate's own point estimate
        ci_low=ci_low,  # => co-04,co-05: the interval's lower bound
        ci_high=ci_high,  # => co-04,co-05: the interval's upper bound
        gap=gap,  # => co-17: the observed gap over baseline
        mcnemar_p=mcnemar_p,  # => co-18: the paired test's own p-value
        noise_floor=MEASURED_NOISE_FLOOR,  # => co-22: this suite's own measured noise floor
        materiality_threshold=MATERIALITY_THRESHOLD,  # => co-19: this team's own stated bar for action
        verdict=verdict,  # => co-24: SHIP or HOLD, derived above from all three conditions together
    )  # => co-24: closes the ShipDecision constructor -- all nine fields supplied, none deferred


if __name__ == "__main__":  # => co-24: entry point -- runs only when this file executes directly, not on import
    baseline, candidate = build_paired_dataset(60, seed=15, baseline_rate=0.72, candidate_rate=0.88, correlation=0.85)  # => co-18: one paired eval run over 60 shared items
    decision = decide(baseline, candidate)  # => co-24: the full, typed decision

    print(f"n={decision.n} candidate_rate={decision.candidate_rate:.4f} 95% CI=[{decision.ci_low:.4f}, {decision.ci_high:.4f}]")  # => co-02,co-04,co-05: the estimate and its interval, never the estimate alone
    print(f"gap={decision.gap:.4f} mcnemar_p={decision.mcnemar_p:.4f}")  # => co-17,co-18: the comparison and its actual test
    print(f"noise_floor={decision.noise_floor:.4f} materiality_threshold={decision.materiality_threshold:.4f}")  # => co-19,co-22: the two bars the gap must clear, beyond bare significance
    print(f"VERDICT: {decision.verdict}")  # => co-24: the final, defensible call

    assert decision.verdict == "SHIP", "with a significant, above-noise-floor, above-materiality gap, this decision must resolve to SHIP"  # => co-24: the claim this example demonstrates
    assert decision.ci_low > 0.0 and decision.ci_high < 1.0, "the reported interval must be a genuine range, not a degenerate point"  # => co-04,co-05
    print("MATCH: every field this theme built -- estimate, interval, gap, paired test, noise floor, materiality -- points the SAME direction, which is what makes this verdict defensible")  # => co-24
    # => co-24: a ship decision built on any ONE of these numbers alone -- the bare rate, the bare p-value, the bare gap -- is not yet defensible; this course's own discipline is reporting every field together, or not shipping the claim
