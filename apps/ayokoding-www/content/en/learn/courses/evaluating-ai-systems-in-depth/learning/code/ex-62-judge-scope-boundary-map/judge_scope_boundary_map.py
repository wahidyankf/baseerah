"""Worked Example 62: Build a Table Mapping Criteria to Per-Criterion Judge Agreement, Retiring Judges Below Threshold."""  # => co-11: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-11: ScopeMapEntry is a typed record, not a bare dict


class ScopeMapEntry(NamedTuple):  # => co-11: one criterion's measured agreement and the resulting routing decision
    criterion_name: str  # => co-11: which criterion this row covers
    measured_agreement: float  # => co-11: this judge's OWN measured agreement on this specific criterion
    routed_to: str  # => co-11: the resulting decision -- "llm-judge" or "human-review"


MINIMUM_USABLE_AGREEMENT = 0.70  # => co-11: the SAME bar ex-22 used, applied here across MANY criteria at once

# A judge's measured agreement across FIVE different criteria -- ex-21/ex-22 showed two; this
# example generalizes the same map to a realistic, wider set.
RAW_MEASUREMENTS = {  # => co-11: five criteria, each independently measured against ground truth
    "asks-before-acting": 1.00,  # => co-11: structural -- well above the bar
    "count-accuracy": 1.00,  # => co-11: structural -- well above the bar
    "reassuring-tone": 0.40,  # => co-11: subjective -- well below the bar
    "cites-ticket-id": 0.90,  # => co-11: structural -- well above the bar
    "matches-brand-voice": 0.55,  # => co-11: subjective -- below the bar
}  # => co-11: closes RAW_MEASUREMENTS


def build_scope_map(measurements: dict[str, float], *, bar: float) -> list[ScopeMapEntry]:  # => co-11: the actual map-building step
    """Build one ScopeMapEntry per criterion, routing each based on its own measured agreement against `bar`."""  # => co-11: documents build_scope_map's contract -- no runtime output, just sets its __doc__
    return [  # => co-11: one entry per criterion, in the order they were measured
        ScopeMapEntry(name, agreement, "llm-judge" if agreement >= bar else "human-review")  # => co-11: per-criterion routing
        for name, agreement in measurements.items()  # => co-11: iterate every measured criterion
    ]  # => co-11: closes the list comprehension


if __name__ == "__main__":  # => co-11: entry point -- runs only when this file executes directly, not on import
    scope_map = build_scope_map(RAW_MEASUREMENTS, bar=MINIMUM_USABLE_AGREEMENT)  # => co-11: build the full map across all five criteria
    for entry in scope_map:  # => co-11: prints the whole map, one row per criterion
        print(f"{entry.criterion_name}: {entry.measured_agreement:.0%} -> {entry.routed_to}")  # => co-11: one line per criterion

    judge_routed = [e for e in scope_map if e.routed_to == "llm-judge"]  # => co-11: every criterion this judge is trusted on
    human_routed = [e for e in scope_map if e.routed_to == "human-review"]  # => co-11: every criterion this judge is retired from
    assert len(judge_routed) == 3, "exactly three of the five criteria must clear the agreement bar"  # => co-11: the rule this example proves
    assert len(human_routed) == 2, "exactly two of the five criteria must fall below the agreement bar"  # => co-11
    assert {e.criterion_name for e in human_routed} == {"reassuring-tone", "matches-brand-voice"}, "the two SUBJECTIVE criteria must be exactly the ones retired"  # => co-11
    print(f"MATCH: {len(judge_routed)}/{len(scope_map)} criteria keep the judge; {len(human_routed)}/{len(scope_map)} are retired to human review -- a real map, not a global yes/no")  # => co-11
    # => co-11: this map -- not a single "is this judge good" verdict -- is what the capstone's own judge.py commits and reports per criterion
