"""Worked Example 22: Drop a Judge From a Criterion Where Its Agreement Is Too Low."""  # => co-11: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-11: CriterionScope is a typed record, not a bare dict


class CriterionScope(NamedTuple):  # => co-11: one criterion's measured judge agreement, and the resulting scoping decision
    criterion_name: str  # => co-11: which criterion this scope decision covers
    measured_agreement: float  # => co-11: the judge's OWN measured agreement on this specific criterion (ex-21)
    minimum_usable_agreement: float = 0.70  # => co-11: the learner-justified bar below which a judge is retired, not shipped


# ex-21's two measured agreements, carried forward into an explicit scoping decision.
CRITERION_SCOPES = [  # => co-11: the judge's real, measured reach across this course's two example criteria
    CriterionScope("asks-before-acting", measured_agreement=1.00),  # => co-11: well above the bar -- keep
    CriterionScope("reassuring-tone", measured_agreement=0.40),  # => co-11: well BELOW the bar -- must retire
]  # => co-11: closes CRITERION_SCOPES


def route_for_criterion(scope: CriterionScope) -> str:  # => co-11: the actual routing decision every criterion gets
    """Return 'llm-judge' if the judge clears the bar on this criterion, else 'human-review' as the fallback."""  # => co-11: documents route_for_criterion's contract -- no runtime output, just sets its __doc__
    if scope.measured_agreement >= scope.minimum_usable_agreement:  # => co-11: the judge earned trust on THIS specific question
        return "llm-judge"  # => co-11: keep using the judge for this criterion
    return "human-review"  # => co-11: retire the judge here -- fall back to a human, never to an unvalidated score


if __name__ == "__main__":  # => co-11: entry point -- runs only when this file executes directly, not on import
    routes = {scope.criterion_name: route_for_criterion(scope) for scope in CRITERION_SCOPES}  # => co-11: one routing decision per criterion
    for name, route in routes.items():  # => co-11: prints every criterion's final routing
        print(f"{name}: routed to {route}")  # => co-11: one line per criterion

    assert routes["asks-before-acting"] == "llm-judge", "a criterion with 100% agreement must keep using the judge"  # => co-11
    assert routes["reassuring-tone"] == "human-review", "a criterion with 40% agreement must be retired to human review"  # => co-11: the rule this example proves
    print("MATCH: the SAME judge model is kept for one criterion and retired for another, based purely on its OWN measured agreement")  # => co-11
    # => co-11: retiring a judge per-criterion, not per-model, is what turns co-10's mandatory measurement into an actionable engineering decision
