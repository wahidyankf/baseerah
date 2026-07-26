"""Worked Example 63: Weigh a Cheap Deterministic Scorer Against an LLM Judge on Cost vs. Coverage."""  # => co-25: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-25: ScorerProfile is a typed record comparing two scoring strategies


class ScorerProfile(NamedTuple):  # => co-25: one scorer's own cost/coverage tradeoff profile
    name: str  # => co-25: which scorer this profile describes
    cost_per_case_usd: float  # => co-25: what running this scorer on ONE case costs
    catches_semantic_criteria: bool  # => co-17: whether it can judge criteria that need semantic understanding, not just string matching
    measured_agreement_with_human: float  # => co-17: this scorer's OWN measured agreement rate with human labels


DETERMINISTIC_SCORER = ScorerProfile(  # => co-16: a cheap, deterministic reference-based scorer
    name="deterministic-keyword-scorer",  # => co-25
    cost_per_case_usd=0.0001,  # => co-25: essentially free -- no model call at all
    catches_semantic_criteria=False,  # => co-17: cannot tell "asks a clarifying question" from a superficially similar sentence that does not
    measured_agreement_with_human=0.71,  # => co-17: measured, not assumed -- decent but limited, per co-16's own limits
)  # => co-25: closes DETERMINISTIC_SCORER

LLM_JUDGE_SCORER = ScorerProfile(  # => co-09: an LLM judge, validated per co-17
    name="validated-llm-judge",  # => co-25
    cost_per_case_usd=0.018,  # => co-25: 180x the deterministic scorer's cost, per case
    catches_semantic_criteria=True,  # => co-17: can assess the underlying semantic behavior, not just surface text
    measured_agreement_with_human=0.91,  # => co-17: measured, not assumed -- meaningfully higher agreement on semantic criteria
)  # => co-25: closes LLM_JUDGE_SCORER


def choose_scorer_for_criterion(*, criterion_is_semantic: bool, budget_per_case_usd: float) -> ScorerProfile:  # => co-25: the actual cost-benefit decision, made explicitly rather than by default
    """Return the scorer that satisfies `criterion_is_semantic` and stays within `budget_per_case_usd`, preferring the cheaper option when both qualify."""  # => co-25: documents choose_scorer_for_criterion's contract -- no runtime output, just sets its __doc__
    candidates = [s for s in (DETERMINISTIC_SCORER, LLM_JUDGE_SCORER) if (not criterion_is_semantic or s.catches_semantic_criteria) and s.cost_per_case_usd <= budget_per_case_usd]  # => co-25: filters to scorers that qualify at all
    if not candidates:  # => co-25: no scorer satisfies both constraints
        raise ValueError("no scorer satisfies both the semantic requirement and the budget")  # => co-25: fails loudly rather than silently picking a wrong scorer
    return min(candidates, key=lambda s: s.cost_per_case_usd)  # => co-25: returns this computed value to the caller -- cheapest QUALIFYING scorer


if __name__ == "__main__":  # => co-25: entry point -- runs only when this file executes directly, not on import
    exact_match_choice = choose_scorer_for_criterion(criterion_is_semantic=False, budget_per_case_usd=0.001)  # => co-25: a non-semantic criterion, tight budget
    semantic_choice = choose_scorer_for_criterion(criterion_is_semantic=True, budget_per_case_usd=0.02)  # => co-25: a semantic criterion, generous budget
    print(f"Non-semantic criterion, tight budget -> chosen: {exact_match_choice.name} (${exact_match_choice.cost_per_case_usd:.4f}/case)")  # => co-25: prints the decision
    print(f"Semantic criterion, generous budget -> chosen: {semantic_choice.name} (${semantic_choice.cost_per_case_usd:.4f}/case)")  # => co-25: prints the decision

    assert exact_match_choice is DETERMINISTIC_SCORER, "a non-semantic criterion under a tight budget must choose the cheap deterministic scorer"  # => co-25: the rule this example proves
    assert semantic_choice is LLM_JUDGE_SCORER, "a semantic criterion must choose the judge, even though it costs 180x more per case"  # => co-25: the rule this example proves
    print("MATCH: the cost-benefit choice depends on WHETHER the criterion is semantic, not on always preferring the cheaper or always preferring the fancier scorer")  # => co-25
    # => co-25: ex-64 next moves from scorer selection to comparing two FULL trajectories against each other, baseline vs. candidate
