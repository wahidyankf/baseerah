"""Worked Example 7: Rank Failure Modes by Frequency Times User Cost, Not Frequency Alone."""  # => co-04: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-04: RankedMode is a typed record, not a bare tuple


class ModeStats(NamedTuple):  # => co-04: one failure mode's raw frequency PLUS its estimated per-incident cost
    mode_name: str  # => co-04: the named mode, from ex-05/ex-06's taxonomy
    frequency: int  # => co-04: how many sampled cases fell under this mode (ex-06's count)
    cost_per_incident: float  # => co-04: an ESTIMATED cost per incident -- support-hours saved by fixing it, on a fixed scale


class RankedMode(NamedTuple):  # => co-04: a mode's final priority score, ready for ranking
    mode_name: str  # => co-04: the named mode this score belongs to
    priority_score: float  # => co-04: frequency times cost -- what actually decides ranking, not either alone


# The SAME four modes ex-06 counted, now annotated with an estimated cost per incident -- costs
# come from support-hours-to-resolve, not from how "interesting" a mode looks to fix.
MODE_STATS: list[ModeStats] = [  # => co-04: four modes, each with a real frequency and an estimated cost
    ModeStats("malformed-structured-output", frequency=14, cost_per_incident=1.0),  # => co-04: MOST frequent, but LOW cost -- usually self-evident and quickly retried
    ModeStats("tone-mismatch-for-audience", frequency=11, cost_per_incident=0.5),  # => co-04: frequent, but LOWEST cost -- rarely blocks the user's actual task
    ModeStats("wrong-object-acted-on", frequency=9, cost_per_incident=6.0),  # => co-04: fewer incidents, but HIGH cost -- a wrongly closed/archived ticket needs manual recovery
    ModeStats("incorrect-aggregate-count", frequency=6, cost_per_incident=12.0),  # => co-04: FEWEST incidents, HIGHEST cost -- a wrong bug count can misdirect a whole team's priorities
]  # => co-04: closes MODE_STATS


def rank_by_frequency_times_cost(stats: list[ModeStats]) -> list[RankedMode]:  # => co-04: the actual prioritization step
    """Rank modes by frequency * cost_per_incident, descending -- highest total impact first."""  # => co-04: documents rank_by_frequency_times_cost's contract -- no runtime output, just sets its __doc__
    scored = [RankedMode(s.mode_name, s.frequency * s.cost_per_incident) for s in stats]  # => co-04: one score per mode -- total estimated user impact
    return sorted(scored, key=lambda r: r.priority_score, reverse=True)  # => co-04: highest total impact ranked first


if __name__ == "__main__":  # => co-04: entry point -- runs only when this file executes directly, not on import
    by_frequency_alone = sorted(MODE_STATS, key=lambda s: s.frequency, reverse=True)  # => co-04: what ex-06's raw count alone would rank first
    print(f"Most frequent mode alone: {by_frequency_alone[0].mode_name} ({by_frequency_alone[0].frequency} cases)")  # => co-04

    ranked = rank_by_frequency_times_cost(MODE_STATS)  # => co-04: the REAL prioritization -- frequency times cost
    for rank in ranked:  # => co-04: prints the full ranked list, most impactful first
        print(f"{rank.mode_name}: priority score {rank.priority_score:.1f}")  # => co-04: one ranked line per mode

    assert by_frequency_alone[0].mode_name == "malformed-structured-output", "frequency alone must rank the most-common mode first"  # => co-04: confirms the naive ranking
    assert ranked[0].mode_name == "incorrect-aggregate-count", "frequency-times-cost must rank the highest-IMPACT mode first"  # => co-04: confirms the real ranking
    assert ranked[0].mode_name != by_frequency_alone[0].mode_name, "the top mode must differ between the two rankings"  # => co-04: proves they genuinely disagree
    print("MATCH: the most FREQUENT mode is not the highest-PRIORITY mode once real user cost is weighed in")  # => co-04: reached only if all three asserts passed
    # => co-04: fixing the fourteen malformed-output cases first would have been the "interesting" choice -- fixing the six mis-counted-bugs cases first is the right one
