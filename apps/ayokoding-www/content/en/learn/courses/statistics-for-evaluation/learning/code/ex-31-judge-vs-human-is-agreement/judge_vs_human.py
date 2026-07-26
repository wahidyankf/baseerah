"""Worked Example 31: Judge vs. Human Is Agreement."""  # => co-14: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-14: builds a simulated criterion where the true label is known but hidden from every rater

from sklearn.metrics import cohen_kappa_score  # => co-14: the SAME coefficient Theme C already used for two human raters


def generate_ratings(n: int, *, seed: int, truth_pass_rate: float, human_noise: float, judge_noise: float) -> tuple[list[str], list[str], list[str]]:  # => co-14: one hidden truth, three independent noisy readers of it
    """Return (human1, human2, judge) labels, each an independently noisy read of a hidden truth."""  # => co-14: documents the contract -- no runtime output, just sets its __doc__
    truth_rng = random.Random(seed)  # => co-14: the hidden, unobservable true pass/fail for each item
    truth = [truth_rng.random() < truth_pass_rate for _ in range(n)]  # => co-14: no rater, human or judge, ever sees this list directly

    def noisy(flip_probability: float, rater_seed: int) -> list[str]:  # => co-14: one rater's own noisy read of the hidden truth
        rater_rng = random.Random(rater_seed)  # => co-14: one fresh generator per rater, so raters' errors do not correlate by construction
        return ["pass" if (t if rater_rng.random() >= flip_probability else not t) else "fail" for t in truth]  # => co-14: flips the truth with the stated probability, per item

    human1 = noisy(human_noise, seed * 10 + 1)  # => co-14: first human rater's labels
    human2 = noisy(human_noise, seed * 10 + 2)  # => co-14: second human rater's labels -- used starting in ex-33
    judge = noisy(judge_noise, seed * 10 + 3)  # => co-14: the LLM judge's labels -- typically noisier than a trained human
    return human1, human2, judge  # => co-14: three label lists, all index-aligned to the same hidden items


if __name__ == "__main__":  # => co-14: entry point -- runs only when this file executes directly, not on import
    human1, _human2, judge = generate_ratings(  # => co-14: one criterion -- "faithfulness" -- human2 unused here, reintroduced in ex-33
        n=40,
        seed=6,
        truth_pass_rate=0.75,
        human_noise=0.06,
        judge_noise=0.20,  # => co-14: a fairly reliable human (6% flip rate) against a noisier judge (20% flip rate)
    )

    judge_human_kappa = cohen_kappa_score(judge, human1)  # => co-14: EXACTLY the same function call Theme C used for human1 vs human2
    print(f"Judge-vs-human kappa (faithfulness): {judge_human_kappa:.4f}")  # => co-14: one number, computed the identical way

    raw_agreement = sum(1 for j, h in zip(judge, human1) if j == h) / len(judge)  # => co-14: the raw percent-agreement baseline, same as ex-21's raw_agreement
    print(f"Raw agreement (judge vs. human): {raw_agreement:.4f}")  # => co-14: the number a report that skips chance-correction would show instead

    assert 0.0 < judge_human_kappa < 1.0, "judge concordance must land as an ordinary kappa value, not a special-cased metric"  # => co-14: the claim this example demonstrates
    print("MATCH: 'judge concordance' is not a new statistic -- it is cohen_kappa_score(judge_labels, human_labels), the same call as any two raters")  # => co-14
    # => co-14: once a judge produces labels, judge concordance IS inter-rater agreement -- everything Theme C already covered (chance correction, prevalence effects, intervals) applies unchanged
