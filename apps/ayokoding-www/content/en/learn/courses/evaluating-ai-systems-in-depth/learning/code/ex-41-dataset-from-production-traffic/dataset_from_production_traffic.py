"""Worked Example 41: Construct Eval Cases From Real Traffic, Checked Against Taxonomy Coverage."""  # => co-21: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-21: ProductionLogEntry and EvalCase are typed records


class ProductionLogEntry(NamedTuple):  # => co-21: one real, logged production interaction -- not invented
    request: str  # => co-21: the real user request, as logged
    failure_mode: str  # => co-03: which of the ERROR-ANALYSIS taxonomy's modes this real interaction exhibited
    was_flagged: bool  # => co-21: whether this interaction was flagged (by a user or a monitor) as problematic


# The same failure-mode taxonomy ex-05/ex-06 already derived from manual error analysis --
# reused here, not re-invented, so coverage is checked against the SAME modes.
KNOWN_TAXONOMY_MODES = frozenset({"skips-clarifying-question", "wrong-object-acted-on", "incorrect-aggregate-count"})  # => co-03: the established taxonomy this dataset must be checked against

PRODUCTION_LOG = (  # => co-21: a small slice of real, logged production traffic -- genuine interactions, not fabricated for the eval
    ProductionLogEntry("Move this to done.", failure_mode="skips-clarifying-question", was_flagged=True),  # => co-21: a real flagged interaction
    ProductionLogEntry("Close ticket #12.", failure_mode="wrong-object-acted-on", was_flagged=True),  # => co-21: a real flagged interaction
    ProductionLogEntry("How many bugs are open?", failure_mode="incorrect-aggregate-count", was_flagged=True),  # => co-21: a real flagged interaction
    ProductionLogEntry("What is the sprint deadline?", failure_mode="", was_flagged=False),  # => co-21: a real, UNflagged interaction -- correctly excluded from the eval set
)  # => co-21: closes PRODUCTION_LOG


def build_eval_cases_from_traffic(log: tuple[ProductionLogEntry, ...]) -> tuple[ProductionLogEntry, ...]:  # => co-21: filters real traffic down to genuine eval cases
    """Return only the FLAGGED log entries -- the ones that actually exhibited a real failure mode."""  # => co-21: documents build_eval_cases_from_traffic's contract -- no runtime output, just sets its __doc__
    return tuple(entry for entry in log if entry.was_flagged)  # => co-21: only real, observed failures become eval cases -- not every logged request


def taxonomy_coverage(eval_cases: tuple[ProductionLogEntry, ...], known_modes: frozenset[str]) -> set[str]:  # => co-21: which of the KNOWN taxonomy modes does this dataset actually cover?
    """Return the subset of `known_modes` that at least one case in `eval_cases` exhibits."""  # => co-21: documents taxonomy_coverage's contract -- no runtime output, just sets its __doc__
    covered = {case.failure_mode for case in eval_cases}  # => co-21: the modes this dataset actually represents
    return covered & known_modes  # => co-21: returns this computed value to the caller -- the overlap with the KNOWN taxonomy


if __name__ == "__main__":  # => co-21: entry point -- runs only when this file executes directly, not on import
    eval_cases = build_eval_cases_from_traffic(PRODUCTION_LOG)  # => co-21: build the eval set from real traffic
    coverage = taxonomy_coverage(eval_cases, KNOWN_TAXONOMY_MODES)  # => co-21: check it against the known taxonomy
    print(f"Production log has {len(PRODUCTION_LOG)} entries; {len(eval_cases)} were flagged and became eval cases")  # => co-21
    print(f"Taxonomy modes covered by this real-traffic dataset: {sorted(coverage)}")  # => co-21: prints the covered modes

    assert len(eval_cases) == 3, "only the three FLAGGED, real interactions become eval cases -- the unflagged one is excluded"  # => co-21: the rule this example proves
    assert coverage == KNOWN_TAXONOMY_MODES, "a well-sourced production dataset should cover ALL known taxonomy modes, not just some"  # => co-21
    print(f"MATCH: {len(eval_cases)} real production interactions become eval cases, covering all {len(coverage)} known taxonomy modes")  # => co-21
    # => co-21: ex-42 next folds in cases that DON'T occur in production yet -- deliberately constructed red-team probes
