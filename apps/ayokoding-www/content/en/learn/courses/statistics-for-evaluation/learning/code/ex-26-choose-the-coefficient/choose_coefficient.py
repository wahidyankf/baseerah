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
