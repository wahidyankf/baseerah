"""Worked Example 78: Name, Explicitly, What Even a Mature Suite Structurally Cannot Catch."""  # => co-28: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-28: a StructuralBlindSpot is a typed record -- an EXPLICIT, named limitation, not a vague caveat


class StructuralBlindSpot(NamedTuple):  # => co-28: one thing the suite CANNOT catch, by construction -- named, not hand-waved
    name: str  # => co-28: a short label for this blind spot
    why_the_suite_misses_it: str  # => co-28: the STRUCTURAL reason -- not "we haven't gotten to it yet"
    what_would_be_needed_instead: str  # => co-28: what a DIFFERENT kind of check would need to look like to catch it


# A well-built suite (ex-01 through ex-77's arc) still cannot catch these, BY DESIGN --
# naming them explicitly is itself part of a mature eval system, not an admission of failure.
STRUCTURAL_BLIND_SPOTS = (  # => co-28: an explicit, enumerated list -- not an unstated gap
    StructuralBlindSpot(  # => co-28
        name="novel failure modes with zero observed instances",  # => co-28
        why_the_suite_misses_it="every criterion in this suite traces back to an OBSERVED failure (co-01/co-27) -- a mode that has never yet occurred in production or red-teaming has no criterion checking for it",  # => co-28
        what_would_be_needed_instead="continuous production monitoring plus periodic fresh error-analysis passes, since no FIXED suite, however large, can check for a pattern nobody has seen yet",  # => co-28
    ),  # => co-28
    StructuralBlindSpot(  # => co-28
        name="a distribution shift in real user requests",  # => co-28
        why_the_suite_misses_it="the eval dataset is a FIXED snapshot (co-14); if real traffic's request patterns shift meaningfully after that snapshot, the suite keeps testing the OLD distribution",  # => co-28
        what_would_be_needed_instead="scheduled dataset refreshes sourced from recent production traffic (co-21), not a one-time-built, never-updated dataset",  # => co-28
    ),  # => co-28
)  # => co-28: closes STRUCTURAL_BLIND_SPOTS


def blind_spot_names(spots: tuple[StructuralBlindSpot, ...]) -> tuple[str, ...]:  # => co-28: extracts just the names, for a quick inventory
    """Return the `name` of every entry in `spots`."""  # => co-28: documents blind_spot_names's contract -- no runtime output, just sets its __doc__
    return tuple(s.name for s in spots)  # => co-28: returns this computed value to the caller


if __name__ == "__main__":  # => co-28: entry point -- runs only when this file executes directly, not on import
    names = blind_spot_names(STRUCTURAL_BLIND_SPOTS)  # => co-28: list the named blind spots
    for spot in STRUCTURAL_BLIND_SPOTS:  # => co-28: prints each blind spot's full reasoning, not just its name
        print(f"Blind spot: {spot.name}")  # => co-28
        print(f"  Why the suite misses it: {spot.why_the_suite_misses_it}")  # => co-28
        print(f"  What would be needed instead: {spot.what_would_be_needed_instead}")  # => co-28

    assert len(STRUCTURAL_BLIND_SPOTS) >= 2, "a mature suite's own limitations must be named as MULTIPLE explicit, distinct blind spots, not one vague caveat"  # => co-28: the rule this example proves
    assert all(spot.why_the_suite_misses_it and spot.what_would_be_needed_instead for spot in STRUCTURAL_BLIND_SPOTS), (  # => co-28: opens this assert's multi-line message
        "every named blind spot must explain BOTH why the suite misses it AND what a different check would need to look like"
    )  # => co-28: the rule this example proves
    print(f"MATCH: {len(names)} structural blind spots -- {names} -- are named explicitly, each with its own STRUCTURAL reason and a concrete idea of what catching it WOULD require, rather than left as an unstated gap")  # => co-28
    # => co-28: ex-79 next checks a specific, RECURRING risk from co-13 -- has judge bias crept back in after this suite matured?
