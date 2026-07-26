"""Worked Example 34: Concordance Report."""  # => co-24: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-24: reuses the two-criterion fixture from ex-32/ex-33
from dataclasses import dataclass  # => co-24: forces every criterion's result through one typed shape -- no bare numbers escape
from typing import TypedDict  # => co-24: gives every criterion's params a precise, per-field int/float type -- not a widened union

import numpy as np  # => co-24: scipy's bootstrap operates on numpy arrays, paired by index
from scipy.stats import bootstrap  # => co-24: puts an interval on each criterion's judge-vs-human kappa, per ex-30
from sklearn.metrics import cohen_kappa_score  # => co-24: the coefficient computed for every (criterion, rater-pair)


class CriterionParams(TypedDict):  # => co-24: the exact shape generate_ratings expects -- n and seed genuinely int, the rest genuinely float
    n: int  # => co-24: item count, always a whole number
    seed: int  # => co-24: the fixture's own random seed, always a whole number
    truth_pass_rate: float  # => co-24: a probability -- always a float
    human_noise: float  # => co-24: a probability -- always a float
    judge_noise: float  # => co-24: a probability -- always a float


def generate_ratings(n: int, *, seed: int, truth_pass_rate: float, human_noise: float, judge_noise: float) -> tuple[list[str], list[str], list[str]]:  # => co-24: the SAME generator as ex-31 through ex-33, unchanged
    """Return (human1, human2, judge) labels, each an independently noisy read of a hidden truth."""  # => co-24: documents the contract -- no runtime output, just sets its __doc__
    truth_rng = random.Random(seed)  # => co-24: the hidden, unobservable true pass/fail for each item
    truth = [truth_rng.random() < truth_pass_rate for _ in range(n)]  # => co-24: no rater, human or judge, ever sees this list directly

    def noisy(flip_probability: float, rater_seed: int) -> list[str]:  # => co-24: one rater's own noisy read of the hidden truth
        rater_rng = random.Random(rater_seed)  # => co-24: one fresh generator per rater, so raters' errors do not correlate by construction
        return ["pass" if (t if rater_rng.random() >= flip_probability else not t) else "fail" for t in truth]  # => co-24: flips the truth with the stated probability, per item

    human1 = noisy(human_noise, seed * 10 + 1)  # => co-24: first human rater's labels
    human2 = noisy(human_noise, seed * 10 + 2)  # => co-24: second human rater's labels -- feeds the ceiling field
    judge = noisy(judge_noise, seed * 10 + 3)  # => co-24: the LLM judge's labels
    return human1, human2, judge  # => co-24: three label lists, all index-aligned to the same hidden items


def kappa_statistic(rater_a: np.ndarray, rater_b: np.ndarray, axis: int = -1) -> np.ndarray | float:  # => co-24: the SAME vectorized bootstrap statistic as ex-30
    """Compute Cohen's kappa for one pair of label arrays, or one row per resample."""  # => co-24: documents the contract -- no runtime output, just sets its __doc__
    if rater_a.ndim == 1:  # => co-24: the plain, non-vectorized case -- one dataset, one kappa
        return cohen_kappa_score(rater_a, rater_b)  # => co-24: a single float
    out = np.empty(rater_a.shape[0])  # => co-24: one kappa slot per bootstrap resample row
    for i in range(rater_a.shape[0]):  # => co-24: scipy calls this function once per batch, so loop over the batch's rows
        out[i] = cohen_kappa_score(rater_a[i], rater_b[i])  # => co-24: this resample's own kappa
    return out  # => co-24: one kappa value per resample, feeding the percentile interval below


@dataclass(frozen=True)  # => co-24: immutable -- a report field cannot be silently overwritten after construction
class ConcordanceReport:  # => co-24: the FULL reportable unit for one criterion's judge concordance -- no bare kappa allowed
    criterion: str  # => co-24: WHICH question this concordance answers -- per co-15, never pooled across criteria
    n: int  # => co-24: how many items this estimate rests on
    prevalence: float  # => co-24: rater_a's own "pass" rate, per co-12 -- required context for reading the kappa
    judge_human_kappa: float  # => co-24: the point estimate
    ci_low: float  # => co-24: the bootstrap interval's lower bound, per co-13
    ci_high: float  # => co-24: the bootstrap interval's upper bound, per co-13
    human_ceiling_kappa: float  # => co-24: the human-human reference point, per co-16
    method: str  # => co-24: names the coefficient AND the interval method -- a report is not portable without this


CRITERIA: dict[str, CriterionParams] = {  # => co-24: the SAME two criteria as ex-32 and ex-33
    "faithfulness": {"n": 40, "seed": 6, "truth_pass_rate": 0.75, "human_noise": 0.06, "judge_noise": 0.20},  # => co-24: the easier criterion
    "tone": {"n": 40, "seed": 11, "truth_pass_rate": 0.60, "human_noise": 0.14, "judge_noise": 0.38},  # => co-24: the harder criterion
}  # => co-24: closes the two-criterion table this whole file iterates over below


def build_report(criterion: str, params: CriterionParams) -> ConcordanceReport:  # => co-24: assembles one criterion's FULL report -- every field above, in one call
    """Build a ConcordanceReport for one criterion from its rating fixture."""  # => co-24: documents the contract -- no runtime output, just sets its __doc__
    human1, human2, judge = generate_ratings(**params)  # => co-24: this criterion's own fixture
    n = int(params["n"])  # => co-24: narrows the dict value back to int, for the dataclass field
    kappa = cohen_kappa_score(judge, human1)  # => co-24: the point estimate
    ceiling = cohen_kappa_score(human1, human2)  # => co-24: the reference ceiling
    prevalence = human1.count("pass") / n  # => co-24: rater_a's own "pass" rate

    judge_arr = np.array(judge)  # => co-24: scipy's bootstrap resamples paired arrays by matching indices
    human1_arr = np.array(human1)  # => co-24: must stay index-aligned with judge_arr through every resample
    result = bootstrap(  # => co-24: resamples (item, item) pairs with replacement, recomputes kappa each time
        (judge_arr, human1_arr),  # => co-24: the paired data
        kappa_statistic,  # => co-24: the statistic recomputed on every resample
        paired=True,  # => co-24: keeps judge[i] and human1[i] together
        vectorized=True,  # => co-24: lets kappa_statistic receive a whole batch of resamples at once
        confidence_level=0.95,  # => co-24: the standard 95% interval
        n_resamples=2000,  # => co-24: 2000 resamples -- enough for a stable percentile estimate at this sample size
        method="percentile",  # => co-24: the simplest bootstrap interval
        rng=np.random.default_rng(int(params["seed"])),  # => co-24: fixes the resampling draw, per criterion, for reproducibility
    )  # => co-24: closes the bootstrap() call -- every keyword above is a deliberate, named choice, not a default
    low, high = result.confidence_interval  # => co-24: unpacks the interval's two ends
    return ConcordanceReport(  # => co-24: every field populated -- no bare number leaves this function
        criterion=criterion,  # => co-24: which question this concordance answers
        n=n,  # => co-24: how many items this estimate rests on
        prevalence=prevalence,  # => co-24: rater_a's own "pass" rate, required context for the kappa
        judge_human_kappa=kappa,  # => co-24: the point estimate
        ci_low=low,  # => co-24: the bootstrap interval's lower bound
        ci_high=high,  # => co-24: the bootstrap interval's upper bound
        human_ceiling_kappa=ceiling,  # => co-24: the human-human reference point
        method="cohen_kappa_score, bootstrap 95% percentile CI",  # => co-24: names both the coefficient and the interval method
    )  # => co-24: closes the ConcordanceReport constructor -- all eight fields supplied, none deferred


if __name__ == "__main__":  # => co-24: entry point -- runs only when this file executes directly, not on import
    reports = [build_report(name, params) for name, params in CRITERIA.items()]  # => co-24: one full report per criterion
    for r in reports:  # => co-24: prints every field -- never just the kappa
        print(  # => co-24: the format any of this course's own worked examples could paste into a PR description
            f"[{r.criterion}] n={r.n} prevalence={r.prevalence:.4f} kappa={r.judge_human_kappa:.4f} "  # => co-24: leading half -- WHICH criterion, on how many items, at what prevalence
            f"95% CI=[{r.ci_low:.4f}, {r.ci_high:.4f}] human-ceiling={r.human_ceiling_kappa:.4f} method='{r.method}'"  # => co-24: trailing half -- interval, ceiling, and named method
        )  # => co-24: closes the print() call -- every one of the eight fields appears in the printed line

    for r in reports:  # => co-24: sanity checks on the assembled report, not just the printed text
        assert r.ci_low < r.judge_human_kappa < r.ci_high, f"{r.criterion}: the point estimate must sit inside its own interval"  # => co-24
        assert r.judge_human_kappa < r.human_ceiling_kappa, f"{r.criterion}: the judge must sit below the human ceiling"  # => co-24
    print("MATCH: every criterion's concordance carries its own n, prevalence, interval, and human-ceiling reference -- none of those fields is optional")  # => co-24
    # => co-24: a defensible judge-concordance report never reduces to a single kappa number -- it is the full ConcordanceReport, criterion by criterion, or it is not reportable
