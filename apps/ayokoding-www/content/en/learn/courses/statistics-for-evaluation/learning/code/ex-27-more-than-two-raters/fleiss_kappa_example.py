"""Worked Example 27: More Than Two Raters."""  # => co-11: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-11: builds the three-rater dataset this example computes agreement over

import numpy as np  # => co-11: statsmodels' inter-rater tools operate on numpy arrays
from sklearn.metrics import cohen_kappa_score  # => co-11: the naive "average the pairwise kappas" comparison
from statsmodels.stats.inter_rater import aggregate_raters, fleiss_kappa  # => co-11: the pinned library's own multi-rater coefficient

N = 30  # => co-11: thirty items, three raters each
TRUE_PASS_RATE = 0.75  # => co-11: the latent quality signal all three raters are independently noisy estimates of


def noisy_label(true_pass: bool, flip_probability: float, *, seed: int) -> str:  # => co-11: one rater's own noisy read of one item's true quality
    """Return 'pass' or 'fail', flipping the true label with probability flip_probability."""  # => co-11: documents noisy_label's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-11: one fresh generator per (rater, item) draw
    observed = true_pass if rng.random() >= flip_probability else not true_pass  # => co-11: flips the true label with the stated probability
    return "pass" if observed else "fail"  # => co-11: the rater's own printed verdict


if __name__ == "__main__":  # => co-11: entry point -- runs only when this file executes directly, not on import
    truth_rng = random.Random(12)  # => co-11: builds the fixed latent quality signal every rater independently estimates
    true_quality = [truth_rng.random() < TRUE_PASS_RATE for _ in range(N)]  # => co-11: the (unobserved) true pass/fail for each item

    rater_1 = [noisy_label(t, 0.10, seed=100 + i) for i, t in enumerate(true_quality)]  # => co-11: rater 1 -- fairly reliable, 10% flip rate
    rater_2 = [noisy_label(t, 0.15, seed=200 + i) for i, t in enumerate(true_quality)]  # => co-11: rater 2 -- somewhat noisier, 15% flip rate
    rater_3 = [noisy_label(t, 0.20, seed=300 + i) for i, t in enumerate(true_quality)]  # => co-11: rater 3 -- noisiest of the three, 20% flip rate

    stacked = np.array(list(zip(rater_1, rater_2, rater_3)))  # => co-11: one row per item, one column per rater -- the shape statsmodels expects
    table, categories = aggregate_raters(stacked)  # => co-11: converts to a per-item category-count table, the format fleiss_kappa needs
    fk = fleiss_kappa(table)  # => co-11: the pinned library's genuine multi-rater coefficient
    print(f"Fleiss' kappa (all 3 raters at once): {fk:.4f}")  # => co-11: ONE coefficient, using all three raters' information jointly

    k12 = cohen_kappa_score(rater_1, rater_2)  # => co-11: pairwise Cohen's kappa, raters 1 vs 2
    k13 = cohen_kappa_score(rater_1, rater_3)  # => co-11: pairwise Cohen's kappa, raters 1 vs 3
    k23 = cohen_kappa_score(rater_2, rater_3)  # => co-11: pairwise Cohen's kappa, raters 2 vs 3
    average_pairwise = (k12 + k13 + k23) / 3  # => co-11: the NAIVE shortcut -- averaging three separate two-rater numbers
    print(f"Pairwise kappas: 1v2={k12:.4f} 1v3={k13:.4f} 2v3={k23:.4f} | average: {average_pairwise:.4f}")  # => co-11

    assert fk != average_pairwise, "Fleiss' kappa must genuinely differ from the naive average of pairwise Cohen's kappas"  # => co-11: the claim this example demonstrates
    print(f"MATCH: Fleiss' kappa ({fk:.4f}) is NOT the same number as averaging three pairwise kappas ({average_pairwise:.4f})")  # => co-11
    # => co-11: Fleiss' kappa corrects for chance using the POOLED marginal distribution across all three raters at once -- averaging pairwise numbers throws that joint structure away
