"""Worked Example 49: A CI Failure Feeds the NEXT Error-Analysis Pass -- the Loop Closes."""  # => co-27: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-27: CiFailure is a typed record -- a real, logged CI-gate failure


class CiFailure(NamedTuple):  # => co-27: one real failure a CI gate caught -- raw material for the NEXT error-analysis pass
    case_id: str  # => co-27: which eval case failed
    request: str  # => co-27: the request that triggered the failure
    reply: str  # => co-27: what the agent actually replied
    existing_taxonomy_mode: str | None  # => co-27: which KNOWN mode this matches, or None if it is a genuinely NEW pattern


EXISTING_TAXONOMY = frozenset({"skips-clarifying-question", "wrong-object-acted-on", "incorrect-aggregate-count"})  # => co-03: the same established taxonomy reused across ex-05, ex-06, ex-41

# A CI gate (ex-46) just blocked a merge because of these two real failures.
CI_FAILURES = (  # => co-27: real CI-gate failures, exactly as caught -- not invented after the fact
    CiFailure("case-17", "Move this to backlog.", "Moved to backlog.", existing_taxonomy_mode="skips-clarifying-question"),  # => co-27: matches a KNOWN mode
    CiFailure("case-22", "Archive tickets older than 90 days.", "Archived all tickets.", existing_taxonomy_mode=None),  # => co-27: does NOT match any known mode -- a genuinely NEW pattern (ignored a stated filter condition)
)  # => co-27: closes CI_FAILURES


def route_failures_to_analysis(failures: tuple[CiFailure, ...], known_taxonomy: frozenset[str]) -> tuple[str, ...]:  # => co-27: separates "known mode recurred" from "genuinely new mode discovered"
    """Return the case_ids of failures whose `existing_taxonomy_mode` is None -- these are NEW modes the taxonomy must grow to cover."""  # => co-27: documents route_failures_to_analysis's contract -- no runtime output, just sets its __doc__
    del known_taxonomy  # => co-27: unused directly -- each failure already carries its own match verdict from triage
    return tuple(f.case_id for f in failures if f.existing_taxonomy_mode is None)  # => co-27: returns this computed value to the caller


def grow_taxonomy(existing: frozenset[str], new_mode_name: str) -> frozenset[str]:  # => co-01: co-01's ORIGINAL error-analysis step, invoked AGAIN -- the loop closing, not a one-time pass
    """Return `existing` with `new_mode_name` added -- the taxonomy growing from a real CI-caught failure."""  # => co-01: documents grow_taxonomy's contract -- no runtime output, just sets its __doc__
    return existing | {new_mode_name}  # => co-01: returns this computed value to the caller


if __name__ == "__main__":  # => co-27: entry point -- runs only when this file executes directly, not on import
    novel_failures = route_failures_to_analysis(CI_FAILURES, EXISTING_TAXONOMY)  # => co-27: identify which CI failures are genuinely NEW patterns
    print(f"CI failures: {len(CI_FAILURES)}, of which genuinely new (unmatched) patterns: {novel_failures}")  # => co-27: prints the routing result

    new_mode_name = "ignores-stated-filter-condition"  # => co-01: the analyst's own name for the pattern behind case-22's failure -- coined by reading the failure, exactly as ex-04 did
    grown_taxonomy = grow_taxonomy(EXISTING_TAXONOMY, new_mode_name)  # => co-01: feed the new pattern back into the SAME taxonomy-growing step the course opened with
    print(f"Taxonomy before: {sorted(EXISTING_TAXONOMY)}")  # => co-01: prints the taxonomy before this pass
    print(f"Taxonomy after routing CI failures back through error analysis: {sorted(grown_taxonomy)}")  # => co-01: prints the taxonomy after this pass

    assert novel_failures == ("case-22",), "only case-22 -- the CI failure with no existing taxonomy match -- must route back as genuinely new"  # => co-27: the rule this example proves
    assert len(grown_taxonomy) == len(EXISTING_TAXONOMY) + 1, "the taxonomy must gain EXACTLY one new mode from this CI-caught failure"  # => co-01: the rule this example proves
    assert new_mode_name in grown_taxonomy, "the new mode must actually be present in the grown taxonomy, not just counted"  # => co-01
    print(f"MATCH: CI failure case-22 routes back through error analysis and grows the taxonomy from {len(EXISTING_TAXONOMY)} to {len(grown_taxonomy)} modes -- the loop this course opened with, closing on itself")  # => co-27
    # => co-27: ex-50 next assembles co-01 through co-28's full arc into one integrated capstone-style worked example
