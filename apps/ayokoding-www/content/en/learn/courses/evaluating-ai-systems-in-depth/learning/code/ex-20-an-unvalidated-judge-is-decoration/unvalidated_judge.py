"""Worked Example 20: Annotate a Judge Deployed Without Agreement Measurement and the Decision It Silently Corrupted."""  # => co-10: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import TypedDict  # => co-10: DeploymentCase types every field this narrative reads


class DeploymentCase(TypedDict):  # => co-10: one real decision a team made, trusting an UNVALIDATED judge
    candidate_name: str  # => co-10: which candidate prompt this judge scored
    judge_reported_pass_rate: float  # => co-10: the number the team actually looked at and trusted
    judge_actual_agreement_with_humans: float  # => co-10: the number NOBODY measured before shipping -- revealed only in hindsight


# A team shipped "candidate-v4" because their UNVALIDATED judge reported an 88% pass rate --
# nobody had ever measured whether that judge's verdicts agreed with real human review.
DEPLOYMENT_HISTORY: list[DeploymentCase] = [  # => co-10: what the team saw vs. what was actually true, discovered after the fact
    {"candidate_name": "candidate-v3", "judge_reported_pass_rate": 0.80, "judge_actual_agreement_with_humans": 0.55},  # => co-10
    {"candidate_name": "candidate-v4", "judge_reported_pass_rate": 0.88, "judge_actual_agreement_with_humans": 0.52},  # => co-10: the one that shipped
]  # => co-10: closes DEPLOYMENT_HISTORY


def decision_the_team_made(cases: list[DeploymentCase]) -> str:  # => co-10: what the team ACTUALLY decided, using only the reported number
    """Return the candidate name the team shipped, using only judge_reported_pass_rate to decide."""  # => co-10: documents decision_the_team_made's contract -- no runtime output, just sets its __doc__
    best = max(cases, key=lambda c: c["judge_reported_pass_rate"])  # => co-10: exactly what happened -- the reported number alone drove the decision
    return best["candidate_name"]  # => co-10: returns this computed value to the caller


def decision_a_measured_judge_would_have_supported(cases: list[DeploymentCase]) -> str | None:  # => co-10: what SHOULD have driven the decision
    """Return the candidate name that would have been justified had agreement been measured first -- None if neither clears a usable bar."""  # => co-10: documents decision_a_measured_judge_would_have_supported's contract -- no runtime output, just sets its __doc__
    usable = [c for c in cases if c["judge_actual_agreement_with_humans"] >= 0.70]  # => co-10: a judge below 70% agreement is not usable evidence at all
    if not usable:  # => co-10: NEITHER candidate's judge score was ever trustworthy evidence
        return None  # => co-10: no candidate is justifiably better, on the judge's word alone
    return max(usable, key=lambda c: c["judge_reported_pass_rate"])["candidate_name"]  # => co-10: returns this computed value to the caller


if __name__ == "__main__":  # => co-10: entry point -- runs only when this file executes directly, not on import
    shipped = decision_the_team_made(DEPLOYMENT_HISTORY)  # => co-10: what actually happened
    justified = decision_a_measured_judge_would_have_supported(DEPLOYMENT_HISTORY)  # => co-10: what a MEASURED judge would have supported
    print(f"Team shipped: {shipped!r} (judge reported {DEPLOYMENT_HISTORY[1]['judge_reported_pass_rate']:.0%})")  # => co-10
    print(f"A measured judge would have justified: {justified!r}")  # => co-10: prints the honest answer -- likely None

    assert shipped == "candidate-v4", "the team must have shipped the candidate with the HIGHER reported (but unmeasured) pass rate"  # => co-10
    assert justified is None, "neither candidate's judge score ever cleared a usable agreement bar -- the whole decision was decoration"  # => co-10
    print("MATCH: the team's real decision rested on a judge whose agreement with humans was never measured -- and was, in fact, near-random (52-55%)")  # => co-10
    # => co-10: an unmeasured judge is not conservative or neutral -- it is a confidently-worded coin flip a team mistook for evidence
