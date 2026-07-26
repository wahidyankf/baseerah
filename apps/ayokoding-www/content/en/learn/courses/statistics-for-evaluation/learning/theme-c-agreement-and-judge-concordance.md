---
title: "Theme C: Agreement and Judge Concordance"
date: 2026-07-26T00:00:00+07:00
draft: false
weight: 30
---

Examples 21-34 build the case that raw percent agreement lies about skewed data (co-09), that a
chance-corrected coefficient is what tells the truth on the same data (co-10), and that choosing
the RIGHT coefficient depends on rater count, label type, and missing data (co-11) -- with the
prevalence that produced the lie in Example 22 reported alongside every coefficient from then on
(co-12) and an interval around the coefficient itself, because 60 items is not enough to pin a
kappa down exactly (co-13). The theme then reframes judge concordance as nothing more than the
SAME agreement math applied to a (judge, human) pair (co-14), computed separately per criterion
because a judge trustworthy on one question can be unreliable on another (co-15), and read against
the human-human ceiling rather than against a fantasy of perfect agreement (co-16). It closes on a
full, typed concordance report that carries every one of those fields at once (co-24). Every worked
example's real, runnable **Python 3.13** file lives under `learning/code/ex-NN-*/`, run for real
against the pinned statistical toolchain to capture genuine output.

---

### Worked Example 21: Raw Percent Agreement

_ex-21 &middot; exercises co-09_

**Context**: **co-09 -- raw percent agreement overstates** starts with the plain arithmetic every
agreement study begins with: how often do two raters assign the identical label. This example
scores two fixed 20-item rater lists and states the number that later examples will show can be
deeply misleading on its own.

```python
# learning/code/ex-21-raw-percent-agreement/raw_percent_agreement.py
"""Worked Example 21: Raw Percent Agreement."""  # => co-09: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

RATER_A = ["pass", "fail", "pass", "pass", "fail", "pass", "fail", "pass", "pass", "fail", "pass", "pass", "fail", "pass", "pass", "fail", "pass", "fail", "pass", "pass"]  # => co-09: labeler A's 20 verdicts
RATER_B = ["pass", "fail", "pass", "fail", "fail", "pass", "pass", "pass", "pass", "fail", "fail", "pass", "fail", "pass", "pass", "fail", "pass", "fail", "fail", "pass"]  # => co-09: labeler B's 20 verdicts on the SAME 20 items, in the same order


def raw_agreement(rater_x: list[str], rater_y: list[str]) -> float:  # => co-09: the plain arithmetic definition -- no library needed for this one
    """Return the fraction of items where rater_x and rater_y assigned the identical label."""  # => co-09: documents raw_agreement's contract -- no runtime output, just sets its __doc__
    assert len(rater_x) == len(rater_y), "both raters must have labeled the exact same number of items"  # => co-09: a basic shape check
    agreements = sum(1 for x, y in zip(rater_x, rater_y) if x == y)  # => co-09: count items where the two labels match, exactly
    return agreements / len(rater_x)  # => co-09: the fraction of items the two raters agreed on


if __name__ == "__main__":  # => co-09: entry point -- runs only when this file executes directly, not on import
    n = len(RATER_A)  # => co-09: how many items both raters labeled
    agreement = raw_agreement(RATER_A, RATER_B)  # => co-09: the raw percent agreement itself
    print(f"Items labeled: {n}")  # => co-09: states the sample size up front
    print(f"Raw agreement: {agreement:.4f} ({round(agreement * n)}/{n} items match)")  # => co-09: the number and the arithmetic behind it
    matches = [i for i, (x, y) in enumerate(zip(RATER_A, RATER_B)) if x == y]  # => co-09: WHICH items matched, for the arithmetic check below
    assert len(matches) == round(agreement * n), "the count of matching items must equal agreement times n"  # => co-09: verifies the arithmetic itself
    print(f"MATCH: {len(matches)} of {n} items have identical labels -- raw agreement is exactly that ratio")  # => co-09
    # => co-09: raw percent agreement is nothing more than 'how often did the two labels match' -- ex-22 shows why that number alone can be deeply misleading
```

**Run**: `python3 raw_percent_agreement.py`

**Output**:

```text
Items labeled: 20
Raw agreement: 0.8000 (16/20 items match)
MATCH: 16 of 20 items have identical labels -- raw agreement is exactly that ratio
```

**Verify**: `raw_agreement()` returns `0.8000`, and `16` of the `20` items independently counted
match exactly, satisfying co-09's rule that raw agreement is a plain count-based ratio with no
correction of any kind.

**Key takeaway**: raw percent agreement is the simplest possible number two raters' labels can
produce -- and, on its own, it says nothing about whether that agreement reflects real signal.

**Why It Matters**: this is the number most eval reports quote first, because it needs no
statistics background to compute or explain -- exactly why ex-22 through ex-25 next show how badly
it can mislead on realistic, skewed data.

---

### Worked Example 22: Skewed Labels Inflate Agreement

_ex-22 &middot; exercises co-09, co-12_

**Context**: continuing co-09, now with **co-12 -- the prevalence problem** foreshadowed: when one
label dominates, two raters can score high raw agreement while barely doing better than a rater who
never looked at the item and always guessed the majority label. This example builds a 60-item,
91.7%-"pass" fixture reused through ex-25, ex-29, and ex-30.

```python
# learning/code/ex-22-skewed-labels-inflate-agreement/skewed_labels.py
"""Worked Example 22: Skewed Labels Inflate Agreement."""  # => co-09: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-09: builds the skewed two-rater dataset this theme reuses through ex-25


def build_skewed_dataset(n: int, *, seed: int) -> tuple[list[str], list[str]]:  # => co-09: the shared fixture this theme's next several examples all regenerate identically
    """Build a two-rater dataset where 'pass' is heavily prevalent -- rater A is a fixed reference labeling, rater B labels mostly independently of the item."""  # => co-09: documents build_skewed_dataset's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-09: one fixed generator -- this exact dataset is reproduced identically wherever this function is called with the same seed
    rater_a = ["pass"] * 55 + ["fail"] * 5  # => co-12: rater A's labels -- 55/60 = 91.7% "pass", a heavily skewed distribution
    rng.shuffle(rater_a)  # => co-09: shuffles the fixed 55/5 split into item order -- the COUNT is fixed, the item assignment is randomized
    rater_b = ["pass" if rng.random() < 0.90 else "fail" for _ in range(n)]  # => co-09: rater B says "pass" 90% of the time, LARGELY WITHOUT regard to what rater A said on that item
    return rater_a, rater_b  # => co-09: returns this computed value to the caller


if __name__ == "__main__":  # => co-09: entry point -- runs only when this file executes directly, not on import
    n = 60  # => co-09: sixty labeled items -- a realistic small eval-agreement study
    rater_a, rater_b = build_skewed_dataset(n, seed=7)  # => co-09: the shared skewed fixture, seed=7, reproduced identically in ex-23 through ex-25, ex-29, ex-30
    a_pass_rate = rater_a.count("pass") / n  # => co-12: rater A's own marginal "pass" prevalence
    b_pass_rate = rater_b.count("pass") / n  # => co-12: rater B's own marginal "pass" prevalence
    print(f"Rater A 'pass' prevalence: {a_pass_rate:.4f} | Rater B 'pass' prevalence: {b_pass_rate:.4f}")  # => co-12: both heavily skewed toward "pass"

    raw_agreement = sum(1 for x, y in zip(rater_a, rater_b) if x == y) / n  # => co-09: the SAME raw-agreement arithmetic ex-21 introduced
    print(f"Raw agreement: {raw_agreement:.4f}")  # => co-09: looks like a solid, respectable agreement number

    majority_baseline = max(a_pass_rate, 1 - a_pass_rate)  # => co-12: what a rater who ALWAYS said the majority label ("pass") would achieve against rater A, by pure chance of the skew
    print(f"'Always say the majority label' baseline agreement with rater A: {majority_baseline:.4f}")  # => co-12: the trivial baseline this raw number should beat
    assert majority_baseline >= raw_agreement - 0.05, "with this skew, a trivial always-majority-label baseline must be roughly competitive with the observed raw agreement"  # => co-12
    print("MATCH: a rater who blindly guessed the majority label every time would score about as well as the observed raw agreement")  # => co-12
    # => co-09,co-12: 85% raw agreement sounds solid until you notice a rater who never looked at the item at all could nearly match it -- the skew, not real agreement, is doing most of the work
```

**Run**: `python3 skewed_labels.py`

**Output**:

```text
Rater A 'pass' prevalence: 0.9167 | Rater B 'pass' prevalence: 0.9333
Raw agreement: 0.8500
'Always say the majority label' baseline agreement with rater A: 0.9167
MATCH: a rater who blindly guessed the majority label every time would score about as well as the observed raw agreement
```

**Verify**: the raw agreement (`0.8500`) sits close to the `0.9167` a trivial always-say-"pass"
baseline achieves against rater A's own skewed marginal, satisfying co-12's rule that raw agreement
alone cannot distinguish real signal from prevalence-driven coincidence.

**Key takeaway**: an agreement number is only as informative as the label distribution it was
measured on -- the same 85% means something very different at 50% prevalence than at 92%.

**Why It Matters**: eval criteria that most items genuinely pass (a common, desirable state) are
exactly the criteria where raw agreement is least trustworthy on its own -- the majority-baseline
comparison this example runs is a five-second sanity check any report can include.

---

### Worked Example 23: Agreement by Chance

_ex-23 &middot; exercises co-10, co-09_

**Context**: **co-10 -- chance-corrected agreement** starts with the term every correction
subtracts: how much agreement would two INDEPENDENT raters produce purely from each rater's own
marginal label rate, with no real agreement at all. This example computes that chance-expected
figure on ex-22's exact fixture and compares it against the observed raw agreement.

```python
# learning/code/ex-23-agreement-by-chance/agreement_by_chance.py
"""Worked Example 23: Agreement by Chance."""  # => co-10: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-09: rebuilds the SAME skewed two-rater dataset ex-22 introduced


def build_skewed_dataset(n: int, *, seed: int) -> tuple[list[str], list[str]]:  # => co-09: the identical fixture-building function from ex-22, reproduced here so this file stays independently runnable
    """Build a two-rater dataset where 'pass' is heavily prevalent -- rater A is a fixed reference labeling, rater B labels mostly independently of the item."""  # => co-09: documents build_skewed_dataset's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-09: the SAME seed reproduces the SAME dataset ex-22 built
    rater_a = ["pass"] * 55 + ["fail"] * 5  # => co-12: rater A's labels -- 55/60 = 91.7% "pass"
    rng.shuffle(rater_a)  # => co-09: shuffles the fixed split into item order
    rater_b = ["pass" if rng.random() < 0.90 else "fail" for _ in range(n)]  # => co-09: rater B says "pass" 90% of the time
    return rater_a, rater_b  # => co-09: returns this computed value to the caller


if __name__ == "__main__":  # => co-10: entry point -- runs only when this file executes directly, not on import
    n = 60  # => co-10: the SAME sixty items ex-22 used
    rater_a, rater_b = build_skewed_dataset(n, seed=7)  # => co-09: reproduces ex-22's exact fixture
    observed_agreement = sum(1 for x, y in zip(rater_a, rater_b) if x == y) / n  # => co-10: the raw agreement ex-22 already computed -- 0.85
    print(f"Observed raw agreement: {observed_agreement:.4f}")  # => co-10: restated for direct comparison against the chance figure below

    p_a_pass = rater_a.count("pass") / n  # => co-10: rater A's marginal probability of saying "pass"
    p_b_pass = rater_b.count("pass") / n  # => co-10: rater B's marginal probability of saying "pass"
    chance_agreement = p_a_pass * p_b_pass + (1 - p_a_pass) * (1 - p_b_pass)  # => co-10: P(both say pass) + P(both say fail), if the two raters' labels were INDEPENDENT of each other
    print(f"Chance-expected agreement (from the two raters' own marginals): {chance_agreement:.4f}")  # => co-10: what TWO INDEPENDENT RANDOM LABELERS with these exact marginals would achieve, on average

    gap = abs(observed_agreement - chance_agreement)  # => co-10: how far the OBSERVED number is from what CHANCE ALONE predicts
    print(f"Gap between observed and chance-expected agreement: {gap:.4f}")  # => co-10: a small gap means the raters are barely beating pure chance
    assert gap < 0.03, "the observed raw agreement must be close to the chance-expected agreement on this skewed fixture"  # => co-10: the core claim this example demonstrates
    print("MATCH: the raters' 85% raw agreement is nearly identical to what independent random guessing would produce on this label distribution")  # => co-10
    # => co-10: this chance-expected figure IS the 'pe' term every chance-corrected coefficient subtracts -- ex-24 builds the full correction from here
```

**Run**: `python3 agreement_by_chance.py`

**Output**:

```text
Observed raw agreement: 0.8500
Chance-expected agreement (from the two raters' own marginals): 0.8611
Gap between observed and chance-expected agreement: 0.0111
MATCH: the raters' 85% raw agreement is nearly identical to what independent random guessing would produce on this label distribution
```

**Verify**: the observed raw agreement (`0.8500`) sits only `0.0111` away from the chance-expected
agreement (`0.8611`) computed purely from the two raters' own marginal "pass" rates, satisfying
co-10's rule that this dataset's raw agreement is barely better than what independence alone
predicts.

**Key takeaway**: "chance agreement" is not a vague statistical caveat -- it is a specific,
computable number, derived entirely from each rater's own marginal label rate.

**Why It Matters**: this chance-expected figure is exactly the term every named agreement
coefficient in this theme subtracts before dividing by the room left above chance -- understanding
it here makes every coefficient's formula legible rather than a black box.

---

### Worked Example 24: Chance-Corrected Agreement, From Definition

_ex-24 &middot; exercises co-10, co-09_

**Context**: continuing co-10, this example assembles ex-21's raw agreement and ex-23's
chance-expected agreement into the full Cohen's kappa formula, computed once from the textbook
definition and once from the pinned library, confirming the two agree exactly.

```python
# learning/code/ex-24-chance-corrected-from-definition/chance_corrected_kappa.py
"""Worked Example 24: Chance-Corrected Agreement, From Definition."""  # => co-10: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import math  # => co-10: isclose -- verifies the from-definition and library kappas agree
import random  # => co-09: rebuilds the SAME skewed two-rater dataset ex-22 introduced

from sklearn.metrics import cohen_kappa_score  # => co-10: the pinned library's own chance-corrected two-rater coefficient


def build_skewed_dataset(n: int, *, seed: int) -> tuple[list[str], list[str]]:  # => co-09: the identical fixture-building function from ex-22/ex-23
    """Build a two-rater dataset where 'pass' is heavily prevalent -- rater A is a fixed reference labeling, rater B labels mostly independently of the item."""  # => co-09: documents build_skewed_dataset's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-09: the SAME seed reproduces the SAME dataset ex-22/ex-23 built
    rater_a = ["pass"] * 55 + ["fail"] * 5  # => co-12: rater A's labels -- 55/60 = 91.7% "pass"
    rng.shuffle(rater_a)  # => co-09: shuffles the fixed split into item order
    rater_b = ["pass" if rng.random() < 0.90 else "fail" for _ in range(n)]  # => co-09: rater B says "pass" 90% of the time
    return rater_a, rater_b  # => co-09: returns this computed value to the caller


def cohen_kappa_from_definition(rater_x: list[str], rater_y: list[str]) -> float:  # => co-10: the textbook formula, in code
    """Return Cohen's kappa: (observed_agreement - chance_agreement) / (1 - chance_agreement)."""  # => co-10: documents cohen_kappa_from_definition's contract -- no runtime output, just sets its __doc__
    n = len(rater_x)  # => co-10: item count
    observed = sum(1 for x, y in zip(rater_x, rater_y) if x == y) / n  # => co-10: the raw agreement, from ex-21's own arithmetic
    p_x_pass = rater_x.count("pass") / n  # => co-10: rater X's own marginal probability of "pass"
    p_y_pass = rater_y.count("pass") / n  # => co-10: rater Y's own marginal probability of "pass"
    chance = p_x_pass * p_y_pass + (1 - p_x_pass) * (1 - p_y_pass)  # => co-10: the pe term ex-23 computed
    return (observed - chance) / (1 - chance)  # => co-10: the chance CORRECTION -- how much better than chance, as a fraction of the room available above chance


if __name__ == "__main__":  # => co-10: entry point -- runs only when this file executes directly, not on import
    rater_a, rater_b = build_skewed_dataset(60, seed=7)  # => co-09: reproduces ex-22/ex-23's exact fixture
    kappa_def = cohen_kappa_from_definition(rater_a, rater_b)  # => co-10: computed from the formula directly
    print(f"Cohen's kappa, from definition: {kappa_def:.4f}")  # => co-10: the chance-corrected coefficient this file derives itself

    kappa_lib = cohen_kappa_score(rater_a, rater_b)  # => co-10: the SAME computation, called from the pinned library
    print(f"Cohen's kappa, from scikit-learn: {kappa_lib:.4f}")  # => co-10: the library's answer to the identical question
    assert math.isclose(kappa_def, kappa_lib, abs_tol=1e-9), "the from-definition and library kappa must agree"  # => co-10
    print("MATCH: the hand-derived kappa and the library's kappa agree to within floating-point precision")  # => co-10
    # => co-10: kappa is negative here -- these raters do WORSE than chance-level agreement, despite 85% raw agreement; ex-25 puts both numbers side by side
```

**Run**: `python3 chance_corrected_kappa.py`

**Output**:

```text
Cohen's kappa, from definition: -0.0800
Cohen's kappa, from scikit-learn: -0.0800
MATCH: the hand-derived kappa and the library's kappa agree to within floating-point precision
```

**Verify**: `cohen_kappa_from_definition()` and `cohen_kappa_score()` both return `-0.0800` on the
identical fixture, satisfying co-10's rule that the textbook formula and the pinned library compute
the same chance-corrected coefficient.

**Key takeaway**: Cohen's kappa is not a mysterious library call -- it is `(observed - chance) / (1

- chance)`, computable by hand from two numbers this theme already derived.

**Why It Matters**: a negative kappa on 85%-raw-agreement data is the single clearest demonstration
that raw agreement and chance-corrected agreement can point in opposite directions -- ex-25 makes
that collapse visible in one place.

---

### Worked Example 25: The Coefficient Collapses

_ex-25 &middot; exercises co-01, co-09, co-10_

**Context**: this example closes the raw-agreement arc by printing all three numbers -- raw
agreement, chance-expected agreement, and Cohen's kappa -- side by side on the same 60-item
dataset, making co-01's opening claim ("a number without context invites a wrong decision")
concrete for the first time with real, chance-corrected data.

```python
# learning/code/ex-25-the-coefficient-collapses/coefficient_collapses.py
"""Worked Example 25: The Coefficient Collapses."""  # => co-10: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-09: rebuilds the SAME skewed two-rater dataset ex-22 through ex-24 used

from sklearn.metrics import cohen_kappa_score  # => co-10: the pinned library's own chance-corrected two-rater coefficient


def build_skewed_dataset(n: int, *, seed: int) -> tuple[list[str], list[str]]:  # => co-09: the identical fixture-building function from ex-22 through ex-24
    """Build a two-rater dataset where 'pass' is heavily prevalent -- rater A is a fixed reference labeling, rater B labels mostly independently of the item."""  # => co-09: documents build_skewed_dataset's contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-09: the SAME seed reproduces the SAME dataset ex-22 through ex-24 built
    rater_a = ["pass"] * 55 + ["fail"] * 5  # => co-12: rater A's labels -- 55/60 = 91.7% "pass"
    rng.shuffle(rater_a)  # => co-09: shuffles the fixed split into item order
    rater_b = ["pass" if rng.random() < 0.90 else "fail" for _ in range(n)]  # => co-09: rater B says "pass" 90% of the time
    return rater_a, rater_b  # => co-09: returns this computed value to the caller


if __name__ == "__main__":  # => co-10: entry point -- runs only when this file executes directly, not on import
    rater_a, rater_b = build_skewed_dataset(60, seed=7)  # => co-09: the SAME fixture threaded through this whole run of examples
    n = len(rater_a)  # => co-09: item count
    raw_agreement = sum(1 for x, y in zip(rater_a, rater_b) if x == y) / n  # => co-09: ex-21/ex-22's own raw-agreement number
    p_a_pass = rater_a.count("pass") / n  # => co-10: rater A's marginal "pass" probability
    p_b_pass = rater_b.count("pass") / n  # => co-10: rater B's marginal "pass" probability
    chance_agreement = p_a_pass * p_b_pass + (1 - p_a_pass) * (1 - p_b_pass)  # => co-10: ex-23's chance-expected agreement
    kappa = cohen_kappa_score(rater_a, rater_b)  # => co-10: ex-24's chance-corrected coefficient

    print("Same 60-item dataset, four numbers:")  # => co-01: the "collapse" is only visible when every number is printed side by side
    print(f"  Raw agreement:              {raw_agreement:.4f}")  # => co-09: the number that LOOKS reassuring
    print(f"  Chance-expected agreement:  {chance_agreement:.4f}")  # => co-10: what pure chance alone predicts, given the skew
    print(f"  Cohen's kappa (corrected):  {kappa:.4f}")  # => co-10: the number that tells the truth

    assert raw_agreement >= 0.80, "raw agreement must look superficially solid (>= 0.80) for this example's own point to land"  # => co-09
    assert abs(kappa) < 0.10, "the chance-corrected coefficient must collapse to near zero on this same data"  # => co-10: the collapse itself
    print(f"MATCH: {raw_agreement:.0%} raw agreement collapses to a kappa of {kappa:.4f} once corrected for the label skew's own chance agreement")  # => co-10
    # => co-01,co-09,co-10: a number without its chance-corrected companion is not just incomplete, it can be actively misleading -- 85% sounds like a strong result, and it is not one
```

**Run**: `python3 coefficient_collapses.py`

**Output**:

```text
Same 60-item dataset, four numbers:
  Raw agreement:              0.8500
  Chance-expected agreement:  0.8611
  Cohen's kappa (corrected):  -0.0800
MATCH: 85% raw agreement collapses to a kappa of -0.0800 once corrected for the label skew's own chance agreement
```

**Verify**: the same 60-item dataset produces `0.8500` raw agreement but `-0.0800` chance-corrected
kappa, satisfying co-10's rule that the two numbers can diverge dramatically on identical data.

**Key takeaway**: "85% agreement" and "kappa of -0.08" are both true statements about the identical
dataset -- only one of them corrects for the label skew, and it is the one that should drive a
decision.

**Why It Matters**: this is the single result to remember from this theme's opening arc: never
report raw agreement without its chance-corrected companion on skewed criteria, because the gap
between them is exactly where a false sense of reliability hides.

---

### Worked Example 26: Choose the Coefficient

_ex-26 &middot; exercises co-11_

**Context**: **co-11 -- choosing an agreement coefficient** turns rater count, label type, and
missing data into a routing decision, not a guess: two nominal raters use Cohen's kappa, two
ordinal raters use weighted Cohen's kappa, three-plus raters use Fleiss' kappa, and any case with
missing data uses Krippendorff's alpha. This example encodes that decision as a function and
verifies the missing-data branch against a real computation.

```python
# learning/code/ex-26-choose-the-coefficient/choose_coefficient.py
"""Worked Example 26: Choose the Coefficient."""  # => co-11: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import numpy as np  # => co-11: builds the missing-data branch's own worked case
import krippendorff  # => co-11: the pinned library's own coefficient for the missing-data branch


def choose_coefficient(*, rater_count: int, label_type: str, has_missing_data: bool) -> str:  # => co-11: the decision itself -- one function, four branches
    """Return the name of the agreement coefficient appropriate for these study characteristics."""  # => co-11: documents choose_coefficient's contract -- no runtime output, just sets its __doc__
    if has_missing_data:  # => co-11: missing data dominates the decision -- Krippendorff's alpha handles it regardless of rater count or label type
        return "krippendorffs_alpha"  # => co-11: the ONLY coefficient among these four built to tolerate missing ratings natively
    if rater_count == 2 and label_type == "nominal":  # => co-11: the two-rater, unordered-category case
        return "cohens_kappa"  # => co-11: ex-24's own coefficient
    if rater_count == 2 and label_type == "ordinal":  # => co-11: the two-rater, ORDERED-category case
        return "weighted_cohens_kappa"  # => co-11: ex-28's own coefficient -- distance-aware
    if rater_count > 2:  # => co-11: more than two raters, regardless of label type in this simplified table
        return "fleiss_kappa"  # => co-11: ex-27's own coefficient
    raise ValueError(f"no rule covers rater_count={rater_count}, label_type={label_type}")  # => co-11: an unhandled combination is a bug, not a silent guess


if __name__ == "__main__":  # => co-11: entry point -- runs only when this file executes directly, not on import
    case_1 = choose_coefficient(rater_count=2, label_type="nominal", has_missing_data=False)  # => co-11: mirrors ex-24's exact study design
    print(f"2 raters, nominal, no missing data -> {case_1}")  # => co-11
    assert case_1 == "cohens_kappa", "two raters with nominal labels and no missing data must choose Cohen's kappa"  # => co-11

    case_2 = choose_coefficient(rater_count=2, label_type="ordinal", has_missing_data=False)  # => co-11: mirrors ex-28's exact study design
    print(f"2 raters, ordinal, no missing data -> {case_2}")  # => co-11
    assert case_2 == "weighted_cohens_kappa", "two raters with ordinal labels must choose the WEIGHTED kappa, not the plain one"  # => co-11

    case_3 = choose_coefficient(rater_count=3, label_type="nominal", has_missing_data=False)  # => co-11: mirrors ex-27's exact study design
    print(f"3 raters, nominal, no missing data -> {case_3}")  # => co-11
    assert case_3 == "fleiss_kappa", "more than two raters must choose Fleiss' kappa, not Cohen's kappa averaged pairwise"  # => co-11

    case_4 = choose_coefficient(rater_count=2, label_type="nominal", has_missing_data=True)  # => co-11: a study where some items went unlabeled by one rater
    print(f"2 raters, nominal, WITH missing data -> {case_4}")  # => co-11
    assert case_4 == "krippendorffs_alpha", "missing data must route to Krippendorff's alpha regardless of rater count or label type"  # => co-11

    rater_x = [1, 1, 0, 1, np.nan, 0, 1, 1, 0, np.nan]  # => co-11: rater X left two items unlabeled -- a real missing-data pattern
    rater_y = [1, np.nan, 0, 1, 1, 0, 0, 1, 0, 1]  # => co-11: rater Y independently left a DIFFERENT item unlabeled
    alpha = krippendorff.alpha(reliability_data=np.array([rater_x, rater_y]), level_of_measurement="nominal")  # => co-11: the branch-4 coefficient, actually computed
    print(f"Krippendorff's alpha on this missing-data case: {alpha:.4f}")  # => co-11: proves branch 4 is a real, callable coefficient, not just a label
    assert -1.0 <= alpha <= 1.0, "Krippendorff's alpha must fall within its valid [-1, 1] range"  # => co-11
    print("MATCH: all four branches route to the correct coefficient, and the missing-data branch is independently verified as a real, computable statistic")  # => co-11
    # => co-11: rater count, label type, and missing data are three independent axes -- picking the wrong coefficient on any one axis produces a number that answers a different question
```

**Run**: `python3 choose_coefficient.py`

**Output**:

```text
2 raters, nominal, no missing data -> cohens_kappa
2 raters, ordinal, no missing data -> weighted_cohens_kappa
3 raters, nominal, no missing data -> fleiss_kappa
2 raters, nominal, WITH missing data -> krippendorffs_alpha
Krippendorff's alpha on this missing-data case: 0.7347
MATCH: all four branches route to the correct coefficient, and the missing-data branch is independently verified as a real, computable statistic
```

**Verify**: `choose_coefficient()` routes all four representative cases to their correct
coefficient name, and `krippendorff.alpha()` returns a genuine `0.7347` on a real missing-data
matrix, satisfying co-11's rule that the choice of coefficient is a deterministic lookup, not a
default.

**Key takeaway**: rater count, label type, and missing data are three separate yes/no facts about
a dataset, and together they fully determine which agreement coefficient is defensible to use.

**Why It Matters**: reflexively reaching for Cohen's kappa on every agreement study is how teams
end up applying a two-rater, nominal-only coefficient to three-rater or ordinal data it was never
built for -- ex-27 and ex-28 next show exactly what goes wrong when the wrong coefficient is used.

---

### Worked Example 27: More Than Two Raters

_ex-27 &middot; exercises co-11_

**Context**: continuing co-11's multi-rater branch, this example builds a three-rater dataset and
verifies Fleiss' kappa -- which pools all three raters' marginals jointly -- genuinely differs from
the naive shortcut of averaging three separate pairwise Cohen's kappas.

```python
# learning/code/ex-27-more-than-two-raters/fleiss_kappa_example.py
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
```

**Run**: `python3 fleiss_kappa_example.py`

**Output**:

```text
Fleiss' kappa (all 3 raters at once): 0.1802
Pairwise kappas: 1v2=0.2823 1v3=0.1485 2v3=0.1485 | average: 0.1931
MATCH: Fleiss' kappa (0.1802) is NOT the same number as averaging three pairwise kappas (0.1931)
```

**Verify**: `fleiss_kappa()` returns `0.1802` while averaging the three pairwise `cohen_kappa_score`
calls gives `0.1931`, satisfying co-11's rule that a genuine multi-rater coefficient is a
mathematically different quantity from averaged pairwise ones, even on the same three-rater data.

**Key takeaway**: Fleiss' kappa uses the pooled marginal label distribution across all raters at
once, which averaging pairwise numbers structurally cannot reconstruct.

**Why It Matters**: an eval study with more than two raters (a common design for subjective
criteria) needs a genuine multi-rater coefficient -- reporting "the average pairwise kappa" instead
is a common shortcut that quietly discards the joint structure a real study needs.

---

### Worked Example 28: Ordinal Labels

_ex-28 &middot; exercises co-11_

**Context**: continuing co-11's ordinal branch, this example fixes a 4-point ordinal scale and one
rater's sequence, then compares two disagreement scenarios that share the identical raw agreement
and unweighted kappa but differ in HOW FAR apart the disagreements are -- showing weighted kappa is
the only one of the three that can see the difference.

```python
# learning/code/ex-28-ordinal-labels/ordinal_weighted_kappa.py
"""Worked Example 28: Ordinal Labels."""  # => co-11: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from sklearn.metrics import cohen_kappa_score  # => co-11: the pinned library's own weighted-kappa support (weights='linear')

RATER_X = [1, 2, 3, 4, 2, 3, 1, 4, 2, 3, 1, 4, 2, 3, 4]  # => co-11: a 1-4 ordinal quality rating -- the SAME sequence used in both scenarios below
RATER_Y_NEAR = [1, 2, 3, 4, 3, 3, 1, 4, 1, 3, 1, 4, 3, 3, 4]  # => co-11: every disagreement with RATER_X is exactly ONE step away
RATER_Y_FAR = [1, 2, 3, 4, 4, 3, 1, 4, 4, 3, 1, 4, 4, 3, 4]  # => co-11: the SAME items disagree, but now THREE steps away (1 vs 4, the scale's extremes)


if __name__ == "__main__":  # => co-11: entry point -- runs only when this file executes directly, not on import
    raw_near = sum(1 for x, y in zip(RATER_X, RATER_Y_NEAR) if x == y) / len(RATER_X)  # => co-11: exact-match agreement, near-disagreement scenario
    raw_far = sum(1 for x, y in zip(RATER_X, RATER_Y_FAR) if x == y) / len(RATER_X)  # => co-11: exact-match agreement, far-disagreement scenario
    print(f"Raw agreement: near-disagreements={raw_near:.4f} | far-disagreements={raw_far:.4f}")  # => co-11: IDENTICAL -- raw agreement cannot see the distance at all
    assert raw_near == raw_far, "raw agreement must be identical in both scenarios -- it does not know disagreements even have a distance"  # => co-11

    unweighted_near = cohen_kappa_score(RATER_X, RATER_Y_NEAR)  # => co-11: plain Cohen's kappa treats every mismatch the same, regardless of distance
    unweighted_far = cohen_kappa_score(RATER_X, RATER_Y_FAR)  # => co-11: same formula, far-disagreement scenario
    print(f"Unweighted kappa: near={unweighted_near:.4f} | far={unweighted_far:.4f}")  # => co-11: nearly identical -- unweighted kappa ALSO cannot see the distance

    weighted_near = cohen_kappa_score(RATER_X, RATER_Y_NEAR, weights="linear")  # => co-11: linear-weighted kappa -- a k-step disagreement costs k/(scale_span) of full disagreement
    weighted_far = cohen_kappa_score(RATER_X, RATER_Y_FAR, weights="linear")  # => co-11: same formula, far-disagreement scenario
    print(f"Linear-weighted kappa: near={weighted_near:.4f} | far={weighted_far:.4f}")  # => co-11: now CLEARLY different
    assert weighted_near > weighted_far, "the linear-weighted kappa must score the near-disagreement scenario HIGHER than the far-disagreement one"  # => co-11: the distance-cost claim itself
    assert abs(unweighted_near - unweighted_far) < 0.01, "unweighted kappa must stay nearly the SAME across both scenarios, for contrast"  # => co-11
    print(f"MATCH: unweighted kappa is blind to distance ({unweighted_near:.4f} vs {unweighted_far:.4f}), weighted kappa is not ({weighted_near:.4f} vs {weighted_far:.4f})")  # => co-11
    # => co-11: on an ordinal scale, a rater who is off by one step is making a smaller mistake than one who is off by three -- weighted kappa is the coefficient that actually says so
```

**Run**: `python3 ordinal_weighted_kappa.py`

**Output**:

```text
Raw agreement: near-disagreements=0.8000 | far-disagreements=0.8000
Unweighted kappa: near=0.7337 | far=0.7321
Linear-weighted kappa: near=0.8387 | far=0.6897
MATCH: unweighted kappa is blind to distance (0.7337 vs 0.7321), weighted kappa is not (0.8387 vs 0.6897)
```

**Verify**: raw agreement (`0.8000`) and unweighted kappa (`0.7337` vs `0.7321`) stay essentially
identical across both scenarios, while linear-weighted kappa clearly separates them (`0.8387` vs
`0.6897`), satisfying co-11's rule that only a weighted coefficient reflects an ordinal scale's own
distance structure.

**Key takeaway**: on an ordinal scale, "how far off" carries real information that raw agreement
and unweighted kappa both discard -- weighted kappa is the coefficient built to use it.

**Why It Matters**: a 4-point quality rubric where a judge is off by one step is a materially
different failure from a judge who confuses the best and worst ratings -- weighted kappa is what
lets a report distinguish "close disagreement" from "opposite disagreement" instead of collapsing
both into one exact-match number.

---

### Worked Example 29: Prevalence Alongside the Coefficient

_ex-29 &middot; exercises co-12_

**Context**: continuing co-12, this example builds a SECOND dataset -- balanced 50/50 between
"pass" and "fail" -- engineered to share the identical 0.85 raw agreement as ex-22's skewed
dataset, then shows the two datasets' Cohen's kappa diverge dramatically, proving the coefficient
alone cannot be read without also knowing the prevalence it was computed at.

```python
# learning/code/ex-29-prevalence-alongside-the-coefficient/prevalence_alongside_kappa.py
"""Worked Example 29: Prevalence Alongside the Coefficient."""  # => co-12: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-12: builds two datasets that share a raw agreement number but differ in prevalence

from sklearn.metrics import cohen_kappa_score  # => co-12: the pinned library's chance-corrected coefficient


def build_skewed_dataset(n: int, *, seed: int) -> tuple[list[str], list[str]]:  # => co-12: the SAME fixture-builder used in ex-22 through ex-25 -- extreme prevalence, 91.7% "pass"
    """Return two raters' labels over n items, 55/60 skewed toward 'pass'."""  # => co-12: documents the contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-12: one generator seeds both the shuffle and rater_b's noise
    rater_a = ["pass"] * 55 + ["fail"] * 5  # => co-12: fixed skew -- 55 pass, 5 fail, out of 60
    rng.shuffle(rater_a)  # => co-12: randomizes the order so rater_b's noise does not correlate with position
    rater_b = ["pass" if rng.random() < 0.90 else "fail" for _ in range(n)]  # => co-12: rater_b independently leans "pass" 90% of the time
    return rater_a, rater_b  # => co-12: two label lists, same length, ready for agreement scoring


def build_balanced_dataset(n: int, *, seed: int) -> tuple[list[str], list[str]]:  # => co-12: a SECOND fixture -- same raw agreement, but 50/50 prevalence
    """Return two raters' labels over n items, balanced 50/50 between 'pass' and 'fail'."""  # => co-12: documents the contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-12: shuffles rater_a's balanced labels
    rater_a = ["pass"] * (n // 2) + ["fail"] * (n // 2)  # => co-12: exactly half pass, half fail
    rng.shuffle(rater_a)  # => co-12: randomizes order before rater_b reads it
    noise_rng = random.Random(seed + 1)  # => co-12: a second, independent generator for rater_b's per-item noise
    rater_b: list[str] = []  # => co-12: built item by item below
    for label in rater_a:  # => co-12: rater_b agrees with rater_a 85% of the time, disagrees the other 15%
        if noise_rng.random() < 0.85:  # => co-12: the 85% "agree" branch
            rater_b.append(label)  # => co-12: copies rater_a's label exactly
        else:  # => co-12: the 15% "disagree" branch
            rater_b.append("fail" if label == "pass" else "pass")  # => co-12: flips to the opposite label
    return rater_a, rater_b  # => co-12: two label lists, same length, ready for agreement scoring


if __name__ == "__main__":  # => co-12: entry point -- runs only when this file executes directly, not on import
    skewed_a, skewed_b = build_skewed_dataset(60, seed=7)  # => co-12: the extreme-prevalence dataset -- 91.7% "pass"
    balanced_a, balanced_b = build_balanced_dataset(60, seed=3)  # => co-12: the balanced-prevalence dataset -- 50% "pass"

    skewed_raw = sum(1 for x, y in zip(skewed_a, skewed_b) if x == y) / len(skewed_a)  # => co-12: raw percent agreement, skewed dataset
    balanced_raw = sum(1 for x, y in zip(balanced_a, balanced_b) if x == y) / len(balanced_a)  # => co-12: raw percent agreement, balanced dataset
    print(f"Raw agreement: skewed={skewed_raw:.4f} | balanced={balanced_raw:.4f}")  # => co-12: IDENTICAL -- both datasets show 85% raw agreement
    assert skewed_raw == balanced_raw, "both datasets must share the same raw agreement, for contrast"  # => co-12: the setup this example depends on

    skewed_kappa = cohen_kappa_score(skewed_a, skewed_b)  # => co-12: chance-corrected agreement, skewed dataset
    balanced_kappa = cohen_kappa_score(balanced_a, balanced_b)  # => co-12: chance-corrected agreement, balanced dataset
    skewed_prevalence = skewed_a.count("pass") / len(skewed_a)  # => co-12: rater_a's own "pass" rate -- the number that explains the gap below
    balanced_prevalence = balanced_a.count("pass") / len(balanced_a)  # => co-12: rater_a's own "pass" rate, balanced dataset
    print(f"Cohen's kappa: skewed={skewed_kappa:.4f} (prevalence={skewed_prevalence:.4f}) | balanced={balanced_kappa:.4f} (prevalence={balanced_prevalence:.4f})")  # => co-12: WILDLY different, despite identical raw agreement

    assert skewed_kappa < 0 < balanced_kappa, "the skewed dataset's kappa must be negative while the balanced dataset's kappa stays clearly positive"  # => co-12: the claim this example demonstrates
    print(f"MATCH: same raw agreement (0.85) produces kappa={skewed_kappa:.4f} at 91.7% prevalence but kappa={balanced_kappa:.4f} at 50% prevalence")  # => co-12
    # => co-12: a kappa number alone does not tell a reader whether the dataset was skewed or balanced -- report the prevalence next to it, or the coefficient is unreadable on its own
```

**Run**: `python3 prevalence_alongside_kappa.py`

**Output**:

```text
Raw agreement: skewed=0.8500 | balanced=0.8500
Cohen's kappa: skewed=-0.0800 (prevalence=0.9167) | balanced=0.7000 (prevalence=0.5000)
MATCH: same raw agreement (0.85) produces kappa=-0.0800 at 91.7% prevalence but kappa=0.7000 at 50% prevalence
```

**Verify**: both datasets share `0.8500` raw agreement, yet the skewed dataset's kappa is `-0.0800`
while the balanced dataset's kappa is `0.7000`, satisfying co-12's rule that the same raw agreement
number can correspond to wildly different chance-corrected coefficients depending on prevalence.

**Key takeaway**: report prevalence alongside every chance-corrected coefficient, or a reader
cannot tell whether a given kappa reflects real agreement or a skew artifact.

**Why It Matters**: two teams could each report "kappa=0.70" or "kappa=-0.08" on their own criteria
and neither number is interpretable without knowing the underlying prevalence -- ex-34's report
format later makes prevalence one of the fields no report can omit.

---

### Worked Example 30: Interval on a Coefficient

_ex-30 &middot; exercises co-13_

**Context**: **co-13 -- agreement has an interval too** treats a kappa value the same way Theme A
treated a pass rate: a point estimate from a finite sample, with its own sampling uncertainty. No
closed-form standard error exists for kappa, so this example uses `scipy.stats.bootstrap` to
resample the item pairs and build a percentile confidence interval around ex-29's balanced-dataset
kappa.

```python
# learning/code/ex-30-interval-on-a-coefficient/bootstrap_kappa_ci.py
"""Worked Example 30: Interval on a Coefficient."""  # => co-13: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-13: builds the same balanced two-rater fixture used in ex-29

import numpy as np  # => co-13: scipy's bootstrap operates on numpy arrays, paired by index
from scipy.stats import bootstrap  # => co-13: no closed-form standard error exists for kappa -- bootstrap resampling instead
from sklearn.metrics import cohen_kappa_score  # => co-13: the coefficient this example puts an interval around


def build_balanced_dataset(n: int, *, seed: int) -> tuple[list[str], list[str]]:  # => co-13: reuses ex-29's balanced-prevalence fixture -- kappa=0.70 on 60 items
    """Return two raters' labels over n items, balanced 50/50 between 'pass' and 'fail'."""  # => co-13: documents the contract -- no runtime output, just sets its __doc__
    rng = random.Random(seed)  # => co-13: shuffles rater_a's balanced labels
    rater_a = ["pass"] * (n // 2) + ["fail"] * (n // 2)  # => co-13: exactly half pass, half fail
    rng.shuffle(rater_a)  # => co-13: randomizes order before rater_b reads it
    noise_rng = random.Random(seed + 1)  # => co-13: a second, independent generator for rater_b's per-item noise
    rater_b: list[str] = []  # => co-13: built item by item below
    for label in rater_a:  # => co-13: rater_b agrees with rater_a 85% of the time, disagrees the other 15%
        if noise_rng.random() < 0.85:  # => co-13: the 85% "agree" branch
            rater_b.append(label)  # => co-13: copies rater_a's label exactly
        else:  # => co-13: the 15% "disagree" branch
            rater_b.append("fail" if label == "pass" else "pass")  # => co-13: flips to the opposite label
    return rater_a, rater_b  # => co-13: two label lists, same length, ready for agreement scoring


def kappa_statistic(rater_a: np.ndarray, rater_b: np.ndarray, axis: int = -1) -> np.ndarray | float:  # => co-13: the statistic scipy's bootstrap resamples -- vectorized over resample rows
    """Compute Cohen's kappa for one pair of label arrays, or one row per resample."""  # => co-13: documents the contract -- no runtime output, just sets its __doc__
    if rater_a.ndim == 1:  # => co-13: the plain, non-vectorized case -- one dataset, one kappa
        return cohen_kappa_score(rater_a, rater_b)  # => co-13: a single float
    out = np.empty(rater_a.shape[0])  # => co-13: one kappa slot per bootstrap resample row
    for i in range(rater_a.shape[0]):  # => co-13: scipy calls this function once per batch, so loop over the batch's rows
        out[i] = cohen_kappa_score(rater_a[i], rater_b[i])  # => co-13: this resample's own kappa
    return out  # => co-13: one kappa value per resample, feeding the percentile interval below


if __name__ == "__main__":  # => co-13: entry point -- runs only when this file executes directly, not on import
    rater_a, rater_b = build_balanced_dataset(60, seed=3)  # => co-13: the SAME fixture as ex-29's balanced dataset
    observed_kappa = cohen_kappa_score(rater_a, rater_b)  # => co-13: the point estimate -- one number from the observed 60 items
    print(f"Observed kappa (point estimate): {observed_kappa:.4f}")  # => co-13: this is the ONLY number a bare kappa report would show

    a_arr = np.array(rater_a)  # => co-13: scipy's bootstrap resamples paired arrays by matching indices
    b_arr = np.array(rater_b)  # => co-13: must stay index-aligned with a_arr through every resample
    result = bootstrap(  # => co-13: resamples (item, item) pairs with replacement, recomputes kappa each time
        (a_arr, b_arr),  # => co-13: the paired data -- resampling shuffles WHICH items are drawn, never which rater said what about a given item
        kappa_statistic,  # => co-13: the statistic recomputed on every resample
        paired=True,  # => co-13: keeps rater_a[i] and rater_b[i] together -- this is agreement data, not two independent samples
        vectorized=True,  # => co-13: lets kappa_statistic receive a whole batch of resamples at once, for speed
        confidence_level=0.95,  # => co-13: the standard 95% interval
        n_resamples=2000,  # => co-13: 2000 resamples -- enough for a stable percentile estimate at this sample size
        method="percentile",  # => co-13: the simplest bootstrap interval -- the 2.5th and 97.5th percentiles of the resampled kappas
        rng=np.random.default_rng(3),  # => co-13: fixes the resampling draw, so this script reproduces the same interval every run
    )
    low, high = result.confidence_interval  # => co-13: unpacks the interval's two ends
    print(f"Bootstrap 95% CI on kappa: [{low:.4f}, {high:.4f}]")  # => co-13: THIS is the honest report -- a range, not a bare point

    assert low < observed_kappa < high, "the observed kappa must sit inside its own bootstrap interval"  # => co-13: a basic sanity check on the interval itself
    assert (high - low) > 0.1, "an interval this wide is the point -- 60 items is not enough for a tight kappa estimate"  # => co-13: the claim this example demonstrates
    print(f"MATCH: a single kappa={observed_kappa:.4f} hides a {high - low:.4f}-wide range of plausible values at n=60")  # => co-13
    # => co-13: reporting "kappa=0.70" with no interval implies more precision than 60 items can support -- the bootstrap interval is what makes that uncertainty visible
```

**Run**: `python3 bootstrap_kappa_ci.py`

**Output**:

```text
Observed kappa (point estimate): 0.7000
Bootstrap 95% CI on kappa: [0.5129, 0.8661]
MATCH: a single kappa=0.7000 hides a 0.3532-wide range of plausible values at n=60
```

**Verify**: the observed kappa (`0.7000`) sits inside its own bootstrap 95% interval
(`[0.5129, 0.8661]`), an interval `0.3532` wide, satisfying co-13's rule that a kappa reported
without an interval hides substantial sampling uncertainty at this sample size.

**Key takeaway**: an agreement coefficient is a point estimate like any other -- it needs an
interval, and bootstrap resampling is the standard way to build one when no closed form exists.

**Why It Matters**: a report that reads "our agreement coefficient is 0.70" invites exactly the
same overconfidence co-01 opened this course with -- ex-34's assembled report makes the interval a
required field, not an optional footnote.

---

### Worked Example 31: Judge vs. Human Is Agreement

_ex-31 &middot; exercises co-14_

**Context**: **co-14 -- judge concordance** reframes "how good is our LLM judge" as nothing more
than an inter-rater agreement question with the judge standing in as one of the raters. This
example builds a hidden-truth simulation where a human and a judge each independently label the
same items, and computes their concordance with the identical `cohen_kappa_score` call Theme C has
used throughout.

```python
# learning/code/ex-31-judge-vs-human-is-agreement/judge_vs_human.py
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
        n=40, seed=6, truth_pass_rate=0.75, human_noise=0.06, judge_noise=0.20  # => co-14: a fairly reliable human (6% flip rate) against a noisier judge (20% flip rate)
    )

    judge_human_kappa = cohen_kappa_score(judge, human1)  # => co-14: EXACTLY the same function call Theme C used for human1 vs human2
    print(f"Judge-vs-human kappa (faithfulness): {judge_human_kappa:.4f}")  # => co-14: one number, computed the identical way

    raw_agreement = sum(1 for j, h in zip(judge, human1) if j == h) / len(judge)  # => co-14: the raw percent-agreement baseline, same as ex-21's raw_agreement
    print(f"Raw agreement (judge vs. human): {raw_agreement:.4f}")  # => co-14: the number a report that skips chance-correction would show instead

    assert 0.0 < judge_human_kappa < 1.0, "judge concordance must land as an ordinary kappa value, not a special-cased metric"  # => co-14: the claim this example demonstrates
    print("MATCH: 'judge concordance' is not a new statistic -- it is cohen_kappa_score(judge_labels, human_labels), the same call as any two raters")  # => co-14
    # => co-14: once a judge produces labels, judge concordance IS inter-rater agreement -- everything Theme C already covered (chance correction, prevalence effects, intervals) applies unchanged
```

**Run**: `python3 judge_vs_human.py`

**Output**:

```text
Judge-vs-human kappa (faithfulness): 0.6373
Raw agreement (judge vs. human): 0.8250
MATCH: 'judge concordance' is not a new statistic -- it is cohen_kappa_score(judge_labels, human_labels), the same call as any two raters
```

**Verify**: `cohen_kappa_score(judge, human1)` returns `0.6373` -- a well-formed kappa strictly
between `0` and `1` -- computed with the identical function call Theme C used for two human raters,
satisfying co-14's rule that judge concordance is inter-rater agreement with no special-cased
formula.

**Key takeaway**: measuring an LLM judge against human labels is the same computation as measuring
two humans against each other -- the judge is simply the second rater.

**Why It Matters**: this reframing is what lets every technique this theme already built --
chance correction, prevalence reporting, bootstrap intervals -- transfer directly to judge
evaluation without inventing a parallel set of "judge-specific" statistics.

---

### Worked Example 32: Concordance Is Per Criterion

_ex-32 &middot; exercises co-15_

**Context**: **co-15 -- concordance is per question** extends ex-31 to a second, harder criterion
and shows the judge's kappa differs materially between them -- a single "judge accuracy" number
would hide exactly which criterion the judge is weak on.

```python
# learning/code/ex-32-concordance-per-criterion/concordance_per_criterion.py
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
```

**Run**: `python3 concordance_per_criterion.py`

**Output**:

```text
Judge-vs-human kappa (faithfulness): 0.6373 (n=40)
Judge-vs-human kappa (tone): 0.3401 (n=40)
(For contrast only) naive average across criteria: 0.4887
MATCH: faithfulness kappa (0.6373) and tone kappa (0.3401) differ by 0.2972 -- a single averaged number would erase this
```

**Verify**: the faithfulness kappa (`0.6373`) and tone kappa (`0.3401`) differ by `0.2972`, more
than double the naive pooled average's own distance from either, satisfying co-15's rule that
concordance is estimated separately per criterion, never pooled into one "judge quality" figure.

**Key takeaway**: a judge's concordance is a property of a (criterion, judge) pair, not a property
of the judge alone -- report it separately for every criterion the judge scores.

**Why It Matters**: a judge deployed across several rubric criteria (faithfulness, tone,
completeness, safety) will not be equally reliable on all of them -- per-criterion concordance is
what tells a team exactly where to trust the judge and where to keep a human in the loop.

---

### Worked Example 33: Human Ceiling

_ex-33 &middot; exercises co-16_

**Context**: **co-16 -- human ceiling** reads a judge's concordance against the reference point that
actually matters: how well two trained humans agree with EACH OTHER on the same criterion, not a
fantasy of perfect agreement. This example computes both numbers for both of ex-32's criteria.

```python
# learning/code/ex-33-human-ceiling/human_ceiling.py
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
```

**Run**: `python3 human_ceiling.py`

**Output**:

```text
[faithfulness] human-human (ceiling): 0.8256 | judge-human: 0.6373 | gap: 0.1883
[tone] human-human (ceiling): 0.7487 | judge-human: 0.3401 | gap: 0.4086
MATCH: on both criteria, the judge's kappa against a human sits below that same human's kappa against another human -- 'agrees with a human' is meaningless without also knowing how well humans agree with EACH OTHER
```

**Verify**: on faithfulness, the human-human ceiling (`0.8256`) exceeds the judge-human kappa
(`0.6373`) by `0.1883`; on tone, the ceiling (`0.7487`) exceeds the judge (`0.3401`) by `0.4086`,
satisfying co-16's rule that the judge sits below the human ceiling on every criterion, with the
gap itself varying by criterion.

**Key takeaway**: "the judge agrees with humans 64% of the time (kappa)" is unreadable without also
knowing that humans only agree with EACH OTHER 83% of the time on that same criterion.

**Why It Matters**: the human-human ceiling reframes "how good is the judge" into "how close is the
judge to the best achievable agreement on this criterion" -- a materially more honest, and often
more forgiving, question than comparing the judge against an unreachable standard of perfection.

---

### Worked Example 34: Concordance Report

_ex-34 &middot; exercises co-24_

**Context**: this theme closes on **co-24 -- reporting honestly**, assembling every field this
theme built -- criterion, sample size, prevalence, judge-human kappa, its bootstrap interval, and
the human ceiling -- into one typed `ConcordanceReport`, so a bare kappa number can never again
leave this course's own examples without its required context.

```python
# learning/code/ex-34-concordance-report/concordance_report.py
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
```

**Run**: `python3 concordance_report.py`

**Output**:

```text
[faithfulness] n=40 prevalence=0.6750 kappa=0.6373 95% CI=[0.3962, 0.8500] human-ceiling=0.8256 method='cohen_kappa_score, bootstrap 95% percentile CI'
[tone] n=40 prevalence=0.5500 kappa=0.3401 95% CI=[0.0355, 0.6114] human-ceiling=0.7487 method='cohen_kappa_score, bootstrap 95% percentile CI'
MATCH: every criterion's concordance carries its own n, prevalence, interval, and human-ceiling reference -- none of those fields is optional
```

**Verify**: `build_report()` returns a `ConcordanceReport` for each criterion with every field
populated -- including a bootstrap interval whose bounds bracket the point estimate and a human
ceiling strictly above it -- satisfying co-24's rule that a reportable concordance number carries
its criterion, n, prevalence, interval, and human-ceiling reference together, never alone.

**Key takeaway**: the reportable unit for judge concordance is the full `ConcordanceReport`, not
the kappa field in isolation -- every other field this theme built exists to make that one number
interpretable.

**Why It Matters**: this is the exact shape a real eval report's "judge reliability" section should
take: one row per criterion, each with n, prevalence, kappa, an interval, and the human ceiling --
closing the arc this theme opened with raw agreement's own false reassurance in ex-21.

---

← Previous: [Theme B: Sampling](./theme-b-sampling.md) &middot; Next: [Theme D: Comparing Runs and
Gating on Them](./theme-d-comparing-runs-and-gating-on-them.md) →
