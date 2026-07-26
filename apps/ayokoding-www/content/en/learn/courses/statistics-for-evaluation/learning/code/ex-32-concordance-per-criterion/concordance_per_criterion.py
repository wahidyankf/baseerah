"""Worked Example 32: Concordance Per Criterion."""  # => co-15: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-15: builds two DIFFERENT criteria's rating fixtures
from typing import TypedDict  # => co-15: gives every criterion's params a precise, per-field int/float type -- not a widened union

from sklearn.metrics import cohen_kappa_score  # => co-15: the same coefficient, applied separately per criterion


class CriterionParams(TypedDict):  # => co-15: the exact shape generate_ratings expects -- n and seed genuinely int, the rest genuinely float
    n: int  # => co-15: item count, always a whole number
    seed: int  # => co-15: the fixture's own random seed, always a whole number
    truth_pass_rate: float  # => co-15: a probability -- always a float
    human_noise: float  # => co-15: a probability -- always a float
    judge_noise: float  # => co-15: a probability -- always a float


def generate_ratings(n: int, *, seed: int, truth_pass_rate: float, human_noise: float, judge_noise: float) -> tuple[list[str], list[str], list[str]]:  # => co-15: the SAME generator as ex-31, unchanged
    """Return (human1, human2, judge) labels, each an independently noisy read of a hidden truth."""  # => co-15: documents the contract -- no runtime output, just sets its __doc__
    truth_rng = random.Random(seed)  # => co-15: the hidden, unobservable true pass/fail for each item
    truth = [truth_rng.random() < truth_pass_rate for _ in range(n)]  # => co-15: no rater, human or judge, ever sees this list directly

    def noisy(flip_probability: float, rater_seed: int) -> list[str]:  # => co-15: one rater's own noisy read of the hidden truth
        rater_rng = random.Random(rater_seed)  # => co-15: one fresh generator per rater, so raters' errors do not correlate by construction
        return ["pass" if (t if rater_rng.random() >= flip_probability else not t) else "fail" for t in truth]  # => co-15: flips the truth with the stated probability, per item

    human1 = noisy(human_noise, seed * 10 + 1)  # => co-15: first human rater's labels
    human2 = noisy(human_noise, seed * 10 + 2)  # => co-15: second human rater's labels -- used starting in ex-33
    judge = noisy(judge_noise, seed * 10 + 3)  # => co-15: the LLM judge's labels
    return human1, human2, judge  # => co-15: three label lists, all index-aligned to the same hidden items


CRITERIA: dict[str, CriterionParams] = {  # => co-15: two DIFFERENT evaluation criteria, each with its own difficulty and its own judge reliability
    "faithfulness": {"n": 40, "seed": 6, "truth_pass_rate": 0.75, "human_noise": 0.06, "judge_noise": 0.20},  # => co-15: an easier, more objective criterion -- lower noise on both sides
    "tone": {"n": 40, "seed": 11, "truth_pass_rate": 0.60, "human_noise": 0.14, "judge_noise": 0.38},  # => co-15: a harder, more subjective criterion -- higher noise on both sides
}

if __name__ == "__main__":  # => co-15: entry point -- runs only when this file executes directly, not on import
    results: dict[str, float] = {}  # => co-15: collects one kappa per criterion, for the side-by-side comparison below
    for criterion_name, params in CRITERIA.items():  # => co-15: iterates the two criteria, computing concordance separately for each
        human1, _human2, judge = generate_ratings(**params)  # => co-15: this criterion's own fixture -- human2 unused here
        kappa = cohen_kappa_score(judge, human1)  # => co-15: this criterion's OWN judge-vs-human kappa -- not pooled with the other criterion
        results[criterion_name] = kappa  # => co-15: stored under this criterion's own name
        print(f"Judge-vs-human kappa ({criterion_name}): {kappa:.4f} (n={params['n']})")  # => co-15: printed per criterion, never averaged together

    pooled_average = sum(results.values()) / len(results)  # => co-15: the tempting-but-wrong shortcut -- one "overall judge quality" number
    print(f"(For contrast only) naive average across criteria: {pooled_average:.4f}")  # => co-15: this single number would hide which criterion the judge is actually weak on

    assert abs(results["faithfulness"] - results["tone"]) > 0.2, "the two criteria's kappas must differ substantially -- that is the point of reporting per-criterion"  # => co-15: the claim this example demonstrates
    print(f"MATCH: faithfulness kappa ({results['faithfulness']:.4f}) and tone kappa ({results['tone']:.4f}) differ by {abs(results['faithfulness'] - results['tone']):.4f} -- a single averaged number would erase this")  # => co-15
    # => co-15: a judge that is trustworthy on one criterion can be unreliable on another -- concordance belongs to a (criterion, judge) pair, never to "the judge" in general
