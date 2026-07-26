"""Worked Example 77: A Novel CI Failure Becomes a NEW, Operationalized Criterion, Added to the Suite."""  # => co-27: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-05: Criterion is the SAME typed shape ex-08/ex-10 already established


class Criterion(NamedTuple):  # => co-05: a derived, operationalized rubric question -- the same shape as earlier criteria
    description: str  # => co-05: the human-readable rubric question
    derived_from_case_id: str  # => co-27: explicit provenance -- WHICH real CI failure this criterion traces back to


EXISTING_CRITERIA = (  # => co-05: the suite's criteria BEFORE this pass -- reused, not rebuilt from scratch
    Criterion("The reply must ask a clarifying question before acting when the request names no specific target.", derived_from_case_id="original-error-analysis"),  # => co-05
)  # => co-05: closes EXISTING_CRITERIA

# ex-49's CI failure case-22 ("Archive tickets older than 90 days." -> "Archived all tickets.")
# exposed a pattern with NO existing criterion covering it -- ignoring a stated filter condition.
NOVEL_CI_FAILURE_CASE_ID = "case-22"  # => co-27: the same real CI failure ex-49 first routed back
NEW_CRITERION = Criterion(  # => co-05: a NEW criterion, operationalized directly from the novel failure's own text
    description="The reply must apply every stated filter condition (such as an age or status threshold) exactly, never acting on the unfiltered full set.",  # => co-05
    derived_from_case_id=NOVEL_CI_FAILURE_CASE_ID,  # => co-27: explicit provenance back to the real CI failure
)  # => co-05: closes NEW_CRITERION


def add_criterion_to_suite(existing: tuple[Criterion, ...], new: Criterion) -> tuple[Criterion, ...]:  # => co-27: grows the SUITE's own criteria, not just the taxonomy label
    """Return `existing` with `new` appended."""  # => co-27: documents add_criterion_to_suite's contract -- no runtime output, just sets its __doc__
    return existing + (new,)  # => co-27: returns this computed value to the caller


if __name__ == "__main__":  # => co-27: entry point -- runs only when this file executes directly, not on import
    grown_criteria = add_criterion_to_suite(EXISTING_CRITERIA, NEW_CRITERION)  # => co-27: grow the criteria set from the real CI failure
    print(f"Criteria before: {len(EXISTING_CRITERIA)}")  # => co-05: prints the count before
    for c in grown_criteria:  # => co-05: prints every criterion, with its provenance
        print(f"  [{c.derived_from_case_id}] {c.description}")  # => co-27
    print(f"Criteria after routing {NOVEL_CI_FAILURE_CASE_ID} back through error analysis: {len(grown_criteria)}")  # => co-27

    assert len(grown_criteria) == len(EXISTING_CRITERIA) + 1, "the suite's criteria must grow by exactly one, traced to the real CI failure"  # => co-27: the rule this example proves
    assert grown_criteria[-1].derived_from_case_id == NOVEL_CI_FAILURE_CASE_ID, "the new criterion must carry explicit provenance back to the exact CI failure that triggered it"  # => co-27: the rule this example proves
    print(  # => co-27: opens the final MATCH print, reached only if both asserts above passed
        f"MATCH: {NOVEL_CI_FAILURE_CASE_ID}'s novel CI failure becomes a new, operationalized criterion with explicit provenance, growing the suite from {len(EXISTING_CRITERIA)} to {len(grown_criteria)} criteria -- the loop closes at the CRITERION level, not just the taxonomy label level"
    )  # => co-27
    # => co-27: ex-78 next names, explicitly, what even a suite grown THIS far still structurally cannot catch
