"""Worked Example 33: Human Ceiling."""  # => co-16: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-16: reuses the two-criterion fixture from ex-32, this time reading BOTH human raters
from typing import TypedDict  # => co-16: gives every criterion's params a precise, per-field int/float type -- not a widened union

from sklearn.metrics import cohen_kappa_score  # => co-16: computes both the ceiling (human-human) and the judge's own kappa


class CriterionParams(TypedDict):  # => co-16: the exact shape generate_ratings expects -- n and seed genuinely int, the rest genuinely float
    n: int  # => co-16: item count, always a whole number
    seed: int  # => co-16: the fixture's own random seed, always a whole number
    truth_pass_rate: float  # => co-16: a probability -- always a float
    human_noise: float  # => co-16: a probability -- always a float
    judge_noise: float  # => co-16: a probability -- always a float


def generate_ratings(n: int, *, seed: int, truth_pass_rate: float, human_noise: float, judge_noise: float) -> tuple[list[str], list[str], list[str]]:  # => co-16: the SAME generator as ex-31 and ex-32, unchanged
    """Return (human1, human2, judge) labels, each an independently noisy read of a hidden truth."""  # => co-16: documents the contract -- no runtime output, just sets its __doc__
    truth_rng = random.Random(seed)  # => co-16: the hidden, unobservable true pass/fail for each item
    truth = [truth_rng.random() < truth_pass_rate for _ in range(n)]  # => co-16: no rater, human or judge, ever sees this list directly

    def noisy(flip_probability: float, rater_seed: int) -> list[str]:  # => co-16: one rater's own noisy read of the hidden truth
        rater_rng = random.Random(rater_seed)  # => co-16: one fresh generator per rater, so raters' errors do not correlate by construction
        return ["pass" if (t if rater_rng.random() >= flip_probability else not t) else "fail" for t in truth]  # => co-16: flips the truth with the stated probability, per item

    human1 = noisy(human_noise, seed * 10 + 1)  # => co-16: first human rater's labels -- used for the judge comparison, same as before
    human2 = noisy(human_noise, seed * 10 + 2)  # => co-16: second human rater's labels -- NOW used, for the human-human ceiling
    judge = noisy(judge_noise, seed * 10 + 3)  # => co-16: the LLM judge's labels
    return human1, human2, judge  # => co-16: three label lists, all index-aligned to the same hidden items


CRITERIA: dict[str, CriterionParams] = {  # => co-16: the SAME two criteria as ex-32, unchanged
    "faithfulness": {"n": 40, "seed": 6, "truth_pass_rate": 0.75, "human_noise": 0.06, "judge_noise": 0.20},  # => co-16: the easier criterion
    "tone": {"n": 40, "seed": 11, "truth_pass_rate": 0.60, "human_noise": 0.14, "judge_noise": 0.38},  # => co-16: the harder criterion
}

if __name__ == "__main__":  # => co-16: entry point -- runs only when this file executes directly, not on import
    for criterion_name, params in CRITERIA.items():  # => co-16: computes both numbers for each criterion in turn
        human1, human2, judge = generate_ratings(**params)  # => co-16: this criterion's own fixture -- all three raters used this time

        human_ceiling_kappa = cohen_kappa_score(human1, human2)  # => co-16: how well two TRAINED HUMANS agree with each other -- the practical upper bound
        judge_human_kappa = cohen_kappa_score(judge, human1)  # => co-16: how well the judge agrees with one of those same humans
        gap = human_ceiling_kappa - judge_human_kappa  # => co-16: how far below the human ceiling the judge falls
        print(f"[{criterion_name}] human-human (ceiling): {human_ceiling_kappa:.4f} | judge-human: {judge_human_kappa:.4f} | gap: {gap:.4f}")  # => co-16: both numbers side by side, never the judge number alone

        assert judge_human_kappa < human_ceiling_kappa, f"the judge must sit below the human ceiling on {criterion_name}, not above or at it"  # => co-16: the claim this example demonstrates
        assert human_ceiling_kappa < 1.0, "even two trained humans do not reach perfect agreement -- that ceiling itself is not 1.0"  # => co-16: humans are not a perfect oracle either

    print("MATCH: on both criteria, the judge's kappa against a human sits below that same human's kappa against another human -- 'agrees with a human' is meaningless without also knowing how well humans agree with EACH OTHER")  # => co-16
    # => co-16: a judge cannot exceed the reliability of the labels it is trained or prompted against -- the human-human ceiling is the honest reference point, not perfect agreement
