"""Capstone Step 3: Paired Comparison, Corrected for Multiple Comparisons."""  # => co-17,co-18,co-21: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-18: builds three paired baseline/candidate criteria -- one real effect, one phantom, one true negative
from dataclasses import dataclass  # => co-24: forces every criterion's comparison result through one typed shape
from typing import TypedDict  # => co-24: gives every criterion's params a precise, per-field type

from statsmodels.stats.contingency_tables import mcnemar  # => co-18: the paired significance test, per ex-37/ex-38
from statsmodels.stats.multitest import multipletests  # => co-21: the multiple-comparisons correction, per ex-44

MATERIALITY_THRESHOLD = 0.05  # => co-19: this team's own stated bar for "worth acting on"


class CriterionCompareParams(TypedDict):  # => co-24: the exact shape build_paired_dataset expects
    n: int  # => co-24: item count, always a whole number
    seed: int  # => co-24: this criterion's own fixture seed, always a whole number
    baseline_rate: float  # => co-24: baseline's true pass rate on this criterion
    candidate_rate: float  # => co-24: candidate's true pass rate on this criterion -- EQUAL to baseline_rate plants a phantom-only scenario
    correlation: float  # => co-24: how strongly baseline/candidate verdicts share per-item difficulty


# => co-17,co-21: three criteria this eval decision needs compared -- deliberately PLANTED with a known ground truth, to verify the pipeline finds what is real and rejects what is not
CRITERIA: dict[str, CriterionCompareParams] = {  # => co-24: named per criterion, per co-15 -- never one pooled comparison
    "correctness": {"n": 50, "seed": 1, "baseline_rate": 0.60, "candidate_rate": 0.88, "correlation": 0.8},  # => co-17: a PLANTED REAL improvement -- candidate's true rate is genuinely higher
    "safety": {"n": 50, "seed": 202, "baseline_rate": 0.90, "candidate_rate": 0.90, "correlation": 0.6},  # => co-21: a PLANTED PHANTOM -- IDENTICAL true rates, no real difference exists
    "tone": {"n": 50, "seed": 1, "baseline_rate": 0.65, "candidate_rate": 0.65, "correlation": 0.8},  # => co-21: a true negative -- IDENTICAL true rates, included for contrast against the phantom
}  # => co-24: closes the three-criterion table this whole file iterates over below


def build_paired_dataset(n: int, *, seed: int, baseline_rate: float, candidate_rate: float, correlation: float) -> tuple[list[bool], list[bool]]:  # => co-18: the SAME paired-fixture shape as ex-37/ex-38/ex-46
    """Build paired baseline/candidate outcomes over n SHARED items, correlated by per-item difficulty."""  # => co-18: documents build_paired_dataset's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-18: drives each item's shared difficulty draw
    baseline: list[bool] = []  # => co-18: baseline's verdict, one per item
    candidate: list[bool] = []  # => co-18: candidate's verdict on the SAME item, one per item
    for _ in range(n):  # => co-18: one shared item at a time
        difficulty_draw = rng.random()  # => co-18: this item's own shared difficulty draw
        baseline_pass = difficulty_draw < baseline_rate  # => co-18: baseline's verdict on this exact item
        if rng.random() < correlation:  # => co-18: most of the time, candidate's verdict shares baseline's own difficulty read
            candidate_pass = difficulty_draw < candidate_rate  # => co-18: correlated verdict
        else:  # => co-18: occasionally, an independent draw instead
            candidate_pass = rng.random() < candidate_rate  # => co-18: an independent verdict
        baseline.append(baseline_pass)  # => co-18: records this item's baseline verdict
        candidate.append(candidate_pass)  # => co-18: records this item's candidate verdict, SAME item, SAME index
    return baseline, candidate  # => co-18: two same-length, index-aligned lists


@dataclass(frozen=True)  # => co-24: immutable -- a comparison result cannot be silently edited
class CriterionComparison:  # => co-24: EVERY field one criterion's paired comparison needs, gathered together
    criterion: str  # => co-15: which criterion this comparison answers for
    n: int  # => co-06: how many paired items
    baseline_rate: float  # => co-02: baseline's own point estimate
    candidate_rate: float  # => co-02: candidate's own point estimate
    gap: float  # => co-17: the observed gap
    mcnemar_p: float  # => co-18: the paired test's own p-value, UNCORRECTED
    corrected_significant: bool  # => co-21: whether this criterion survives multiple-comparisons correction
    material: bool  # => co-19: whether the gap clears this team's own materiality bar, separate from significance


if __name__ == "__main__":  # => co-24: entry point -- runs only when this file executes directly, not on import
    raw_results: list[tuple[str, int, float, float, float, float]] = []  # => co-18: one row per criterion, before correction
    for criterion, params in CRITERIA.items():  # => co-17: compares EVERY criterion the SAME way
        baseline, candidate = build_paired_dataset(**params)  # => co-18: this criterion's own paired data
        n = params["n"]  # => co-06: item count
        baseline_rate = sum(baseline) / n  # => co-02: baseline's own point estimate
        candidate_rate = sum(candidate) / n  # => co-02: candidate's own point estimate
        gap = candidate_rate - baseline_rate  # => co-17: the observed gap

        both_pass = sum(1 for b, c in zip(baseline, candidate) if b and c)  # => co-18: concordant pairs
        both_fail = sum(1 for b, c in zip(baseline, candidate) if not b and not c)  # => co-18: concordant pairs
        baseline_only = sum(1 for b, c in zip(baseline, candidate) if b and not c)  # => co-18: candidate regressed
        candidate_only = sum(1 for b, c in zip(baseline, candidate) if not b and c)  # => co-18: candidate improved
        table = [[both_pass, baseline_only], [candidate_only, both_fail]]  # => co-18: the 2x2 table McNemar's test uses
        mcnemar_p = mcnemar(table, exact=False, correction=True).pvalue  # => co-18: the UNCORRECTED paired p-value

        raw_results.append((criterion, n, baseline_rate, candidate_rate, gap, mcnemar_p))  # => co-18: stored for correction below
        print(f"[{criterion}] baseline={baseline_rate:.4f} candidate={candidate_rate:.4f} gap={gap:+.4f} uncorrected_p={mcnemar_p:.4f}")  # => co-17,co-18

    pvalues = [r[5] for r in raw_results]  # => co-21: every criterion's own uncorrected p-value, in order
    reject_bonferroni, _, _, _ = multipletests(pvalues, alpha=0.05, method="bonferroni")  # => co-21: the multiple-comparisons correction, per ex-44

    comparisons = [  # => co-24: assembles the FULL typed comparison for every criterion
        CriterionComparison(  # => co-24: no bare gap or bare p-value leaves this constructor
            criterion=criterion,  # => co-15: which criterion this comparison answers for
            n=n,  # => co-06: how many paired items
            baseline_rate=baseline_rate,  # => co-02: baseline's own point estimate
            candidate_rate=candidate_rate,  # => co-02: candidate's own point estimate
            gap=gap,  # => co-17: the observed gap
            mcnemar_p=mcnemar_p,  # => co-18: the paired test's own p-value, uncorrected
            corrected_significant=bool(rejected),  # => co-21: whether this criterion survives correction
            material=abs(gap) > MATERIALITY_THRESHOLD,  # => co-19: whether the gap clears materiality, separate from significance
        )  # => co-24: closes the CriterionComparison constructor -- all eight fields supplied
        for (criterion, n, baseline_rate, candidate_rate, gap, mcnemar_p), rejected in zip(raw_results, reject_bonferroni)  # => co-24: pairs each raw result with its own corrected verdict, in order
    ]  # => co-24: closes the list comprehension -- one CriterionComparison per criterion tested

    for c in comparisons:  # => co-19,co-21: prints the FULL, corrected, materiality-aware verdict per criterion
        print(f"[{c.criterion}] corrected_significant={c.corrected_significant} material={c.material}")  # => co-19,co-21

    correctness = next(c for c in comparisons if c.criterion == "correctness")  # => co-17: the PLANTED real effect
    safety = next(c for c in comparisons if c.criterion == "safety")  # => co-21: the PLANTED phantom
    tone = next(c for c in comparisons if c.criterion == "tone")  # => co-21: the true negative, for contrast

    assert correctness.corrected_significant and correctness.material, "the planted REAL effect (correctness) must survive correction AND clear materiality"  # => co-17,co-19,co-21: the pipeline must find what is real
    assert not safety.corrected_significant, "the planted PHANTOM (safety, identical true rates) must NOT survive correction"  # => co-21: the pipeline must reject what is not real
    assert not tone.corrected_significant, "the true negative (tone, identical true rates) must stay non-significant"  # => co-21: the boring control case
    print("MATCH: the corrected pipeline finds the planted real effect (correctness) significant and material, and rejects the planted phantom (safety) despite its uncorrected p < 0.05")  # => co-17,co-18,co-19,co-21
    # => co-17,co-18,co-19,co-21,co-24: this is the comparison half of the capstone's evidence -- paired, corrected for testing three criteria at once, and read against materiality, never a bare uncorrected p-value
