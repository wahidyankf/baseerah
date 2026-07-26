"""Worked Example 64: Diff a Candidate Trajectory Against a Known-Good Baseline, Step by Step."""  # => co-18: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-18: StepDiff is a typed record -- one aligned pair of baseline/candidate steps


class StepDiff(NamedTuple):  # => co-18: one position's comparison between baseline and candidate trajectories
    position: int  # => co-18: which step index this diff covers
    baseline_tool: str | None  # => co-18: the baseline's tool at this position, or None if the baseline ended earlier
    candidate_tool: str | None  # => co-18: the candidate's tool at this position, or None if the candidate ended earlier
    matches: bool  # => co-18: whether baseline and candidate agree at this exact position


BASELINE_TRAJECTORY = ("search_ticket", "get_ticket", "update_priority")  # => co-18: a known-good, previously-approved trajectory for this task
CANDIDATE_TRAJECTORY = ("search_ticket", "get_ticket", "close_ticket", "update_priority")  # => co-18: a NEW candidate run, one step longer


def diff_trajectories(baseline: tuple[str, ...], candidate: tuple[str, ...]) -> tuple[StepDiff, ...]:  # => co-18: a positional diff, not just a pass/fail verdict -- shows WHERE they diverge
    """Return a `StepDiff` for every position across the longer of `baseline` and `candidate`."""  # => co-18: documents diff_trajectories's contract -- no runtime output, just sets its __doc__
    length = max(len(baseline), len(candidate))  # => co-18: covers every position, even past the shorter sequence's end
    diffs: list[StepDiff] = []  # => co-18: accumulates one StepDiff per position
    for i in range(length):  # => co-18: walks both sequences in lockstep
        b = baseline[i] if i < len(baseline) else None  # => co-18: None once the baseline has run out of steps
        c = candidate[i] if i < len(candidate) else None  # => co-18: None once the candidate has run out of steps
        diffs.append(StepDiff(position=i, baseline_tool=b, candidate_tool=c, matches=(b == c)))  # => co-18: records this position's comparison
    return tuple(diffs)  # => co-18: returns this computed value to the caller


if __name__ == "__main__":  # => co-18: entry point -- runs only when this file executes directly, not on import
    diffs = diff_trajectories(BASELINE_TRAJECTORY, CANDIDATE_TRAJECTORY)  # => co-18: compute the full positional diff
    for d in diffs:  # => co-18: prints every position's comparison
        print(f"Position {d.position}: baseline={d.baseline_tool!r}, candidate={d.candidate_tool!r}, matches={d.matches}")  # => co-18
    first_divergence = next(d.position for d in diffs if not d.matches)  # => co-18: the FIRST position where the two trajectories disagree

    assert len(diffs) == 4, "the diff must cover every position across the LONGER trajectory, here 4 positions"  # => co-18: the rule this example proves
    assert first_divergence == 2, "the trajectories must first diverge at position 2, where the candidate inserted an unplanned close_ticket step"  # => co-18: the rule this example proves
    assert diffs[0].matches and diffs[1].matches, "the two trajectories must still agree on their shared first two steps"  # => co-18
    print(f"MATCH: the diff pinpoints divergence at position {first_divergence}, where the candidate's extra close_ticket step departs from the known-good baseline")  # => co-18
    # => co-18: ex-65 next asks whether an inserted step like this one should cost the candidate FULL credit, or only PARTIAL credit
