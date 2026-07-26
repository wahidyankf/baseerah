"""Capstone Step 2: Chance-Corrected Agreement and Judge Concordance."""  # => co-09,co-10,co-14: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import math  # => co-10: isclose -- verifies every coefficient's from-definition and library values agree
import random  # => co-14: builds three criteria's rating fixtures -- the SAME hidden-truth pattern as ex-31 through ex-34
from dataclasses import dataclass  # => co-24: forces every criterion's result through one typed shape -- no bare numbers escape
from typing import TypedDict  # => co-24: gives every criterion's params a precise, per-field int/float type -- not a widened union

import numpy as np  # => co-13: scipy's bootstrap operates on numpy arrays, paired by index
from scipy.stats import bootstrap  # => co-13: puts an interval on each criterion's judge-vs-human kappa
from sklearn.metrics import cohen_kappa_score  # => co-10: the pinned library's own chance-corrected coefficient


class CriterionParams(TypedDict):  # => co-24: the exact shape generate_ratings expects -- n and seed genuinely int, the rest genuinely float
    n: int  # => co-24: item count, always a whole number
    seed: int  # => co-24: the fixture's own random seed, always a whole number
    truth_pass_rate: float  # => co-24: a probability -- always a float
    human_noise: float  # => co-24: a probability -- always a float
    judge_noise: float  # => co-24: a probability -- always a float


CRITERIA: dict[str, CriterionParams] = {  # => co-14,co-15: THREE rubric criteria this eval decision actually needs judged -- each with its own difficulty
    "correctness": {"n": 50, "seed": 21, "truth_pass_rate": 0.80, "human_noise": 0.05, "judge_noise": 0.18},  # => co-15: the most objective criterion
    "safety": {"n": 50, "seed": 22, "truth_pass_rate": 0.90, "human_noise": 0.03, "judge_noise": 0.22},  # => co-15: high-stakes, high-prevalence "pass"
    "tone": {"n": 50, "seed": 23, "truth_pass_rate": 0.65, "human_noise": 0.12, "judge_noise": 0.30},  # => co-15: the most subjective, noisiest criterion
}  # => co-15: closes the three-criterion table this whole file iterates over below


def generate_ratings(n: int, *, seed: int, truth_pass_rate: float, human_noise: float, judge_noise: float) -> tuple[list[str], list[str], list[str]]:  # => co-14: the SAME generator pattern as ex-31 through ex-34
    """Return (human1, human2, judge) labels, each an independently noisy read of a hidden truth."""  # => co-14: documents the contract -- no runtime output, just sets its __doc__
    truth_rng = random.Random(seed)  # => co-14: the hidden, unobservable true pass/fail for each item
    truth = [truth_rng.random() < truth_pass_rate for _ in range(n)]  # => co-14: no rater, human or judge, ever sees this list directly

    def noisy(flip_probability: float, rater_seed: int) -> list[str]:  # => co-14: one rater's own noisy read of the hidden truth
        rater_rng = random.Random(rater_seed)  # => co-14: one fresh generator per rater
        return ["pass" if (t if rater_rng.random() >= flip_probability else not t) else "fail" for t in truth]  # => co-14: flips the truth with the stated probability

    human1 = noisy(human_noise, seed * 10 + 1)  # => co-14: first human rater's labels
    human2 = noisy(human_noise, seed * 10 + 2)  # => co-14: second human rater's labels -- for the human ceiling
    judge = noisy(judge_noise, seed * 10 + 3)  # => co-14: the LLM judge's labels
    return human1, human2, judge  # => co-14: three label lists, all index-aligned to the same hidden items


def cohen_kappa_from_definition(rater_x: list[str], rater_y: list[str]) -> float:  # => co-10: the SAME textbook formula as ex-24, reused for every coefficient this file computes
    """Return Cohen's kappa: (observed_agreement - chance_agreement) / (1 - chance_agreement)."""  # => co-10: documents cohen_kappa_from_definition's contract -- no runtime output, just sets its __doc__
    n = len(rater_x)  # => co-10: item count
    observed = sum(1 for x, y in zip(rater_x, rater_y) if x == y) / n  # => co-10: the raw agreement
    p_x = rater_x.count("pass") / n  # => co-10: rater X's own marginal probability of "pass"
    p_y = rater_y.count("pass") / n  # => co-10: rater Y's own marginal probability of "pass"
    chance = p_x * p_y + (1 - p_x) * (1 - p_y)  # => co-10: the chance-expected agreement, per ex-23
    return (observed - chance) / (1 - chance)  # => co-10: the chance correction


def kappa_statistic(rater_a: np.ndarray, rater_b: np.ndarray, axis: int = -1) -> np.ndarray | float:  # => co-13: the SAME vectorized bootstrap statistic as ex-30/ex-34
    """Compute Cohen's kappa for one pair of label arrays, or one row per resample."""  # => co-13: documents the contract -- no runtime output, just sets its __doc__
    if rater_a.ndim == 1:  # => co-13: the plain, non-vectorized case
        return cohen_kappa_score(rater_a, rater_b)  # => co-13: a single float
    out = np.empty(rater_a.shape[0])  # => co-13: one kappa slot per bootstrap resample row
    for i in range(rater_a.shape[0]):  # => co-13: scipy calls this function once per batch
        out[i] = cohen_kappa_score(rater_a[i], rater_b[i])  # => co-13: this resample's own kappa
    return out  # => co-13: one kappa value per resample


@dataclass(frozen=True)  # => co-24: immutable -- a report field cannot be silently overwritten after construction
class ConcordanceReport:  # => co-24: the FULL reportable unit for one criterion's judge concordance, per ex-34
    criterion: str  # => co-15: WHICH question this concordance answers -- never pooled across criteria
    n: int  # => co-06: how many items this estimate rests on
    prevalence: float  # => co-12: rater_a's own "pass" rate -- required context for reading the kappa
    judge_human_kappa: float  # => co-14: the point estimate
    ci_low: float  # => co-13: the bootstrap interval's lower bound
    ci_high: float  # => co-13: the bootstrap interval's upper bound
    human_ceiling_kappa: float  # => co-16: the human-human reference point
    method: str  # => co-24: names the coefficient AND the interval method


def build_report(criterion: str, params: CriterionParams) -> ConcordanceReport:  # => co-24: assembles one criterion's FULL report, verifying every coefficient twice along the way
    """Build a ConcordanceReport, verifying every coefficient from-definition and via the library."""  # => co-24: documents build_report's contract -- no runtime output, just sets its __doc__
    human1, human2, judge = generate_ratings(**params)  # => co-14: this criterion's own fixture
    n = params["n"]  # => co-06: item count

    kappa_def = cohen_kappa_from_definition(judge, human1)  # => co-10: computed from the formula directly
    kappa_lib = cohen_kappa_score(judge, human1)  # => co-10: computed via the pinned library
    assert math.isclose(kappa_def, kappa_lib, abs_tol=1e-9), f"{criterion}: judge-vs-human kappa must match between definition and library"  # => co-10: computed twice, verified equal

    ceiling_def = cohen_kappa_from_definition(human1, human2)  # => co-10: the human-human ceiling, from the formula
    ceiling_lib = cohen_kappa_score(human1, human2)  # => co-10: the human-human ceiling, via the library
    assert math.isclose(ceiling_def, ceiling_lib, abs_tol=1e-9), f"{criterion}: human ceiling kappa must match between definition and library"  # => co-10: computed twice, verified equal

    prevalence = human1.count("pass") / n  # => co-12: rater_a's own "pass" rate

    judge_arr = np.array(judge)  # => co-13: scipy's bootstrap resamples paired arrays by matching indices
    human1_arr = np.array(human1)  # => co-13: must stay index-aligned with judge_arr
    result = bootstrap(  # => co-13: resamples (item, item) pairs with replacement
        (judge_arr, human1_arr),
        kappa_statistic,
        paired=True,
        vectorized=True,
        confidence_level=0.95,
        n_resamples=2000,
        method="percentile",
        rng=np.random.default_rng(params["seed"]),  # => co-13: the SAME bootstrap procedure as ex-30/ex-34
    )  # => co-13: closes the bootstrap() call -- every keyword above is a deliberate, named choice
    low, high = result.confidence_interval  # => co-13: unpacks the interval's two ends

    return ConcordanceReport(  # => co-24: every field populated -- no bare number leaves this function
        criterion=criterion,  # => co-15: which question this concordance answers
        n=n,  # => co-06: how many items this estimate rests on
        prevalence=prevalence,  # => co-12: rater_a's own "pass" rate
        judge_human_kappa=kappa_def,  # => co-14: the point estimate
        ci_low=low,  # => co-13: the interval's lower bound
        ci_high=high,  # => co-13: the interval's upper bound
        human_ceiling_kappa=ceiling_def,  # => co-16: the human-human reference point
        method="cohen_kappa (definition + library, verified equal), bootstrap 95% percentile CI",  # => co-24: names the coefficient AND the interval method
    )  # => co-24: closes the ConcordanceReport constructor -- all eight fields supplied, none deferred


if __name__ == "__main__":  # => co-24: entry point -- runs only when this file executes directly, not on import
    reports = [build_report(name, params) for name, params in CRITERIA.items()]  # => co-15: one full report per criterion, computed separately

    for r in reports:  # => co-24: prints every field, per criterion -- never a pooled "judge quality" number
        print(  # => co-24: the exact per-criterion row this capstone's own report.md later tabulates
            f"[{r.criterion}] n={r.n} prevalence={r.prevalence:.4f} judge_kappa={r.judge_human_kappa:.4f} "  # => co-24: leading half -- WHICH criterion, n, prevalence
            f"95% CI=[{r.ci_low:.4f}, {r.ci_high:.4f}] human_ceiling={r.human_ceiling_kappa:.4f}"  # => co-24: trailing half -- interval and ceiling
        )  # => co-24: closes the print() call

    for r in reports:  # => co-16,co-24: the checks this step's own acceptance criteria require
        assert r.ci_low < r.judge_human_kappa < r.ci_high, f"{r.criterion}: the point estimate must sit inside its own interval"  # => co-13
        assert r.judge_human_kappa < r.human_ceiling_kappa, f"{r.criterion}: the judge must sit below the human ceiling"  # => co-16
    print("MATCH: every criterion's judge-vs-human kappa AND human-ceiling kappa were each verified from-definition against the pinned library, and every judge kappa sits below its own human ceiling")  # => co-24
    # => co-09,co-10,co-11,co-12,co-13,co-14,co-15,co-16,co-24: this is the agreement half of the capstone's evidence -- every named coefficient computed twice, interval-bounded, and read against the human ceiling, never a bare number
