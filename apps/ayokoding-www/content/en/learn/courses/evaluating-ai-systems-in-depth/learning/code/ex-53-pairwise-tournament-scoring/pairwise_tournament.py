"""Worked Example 53: Run a Round-Robin Pairwise Tournament Across N Candidates."""  # => co-14: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from itertools import combinations  # => co-14: every unique pair, exactly once -- the round-robin structure itself


CANDIDATES = {  # => co-14: four candidate replies to the SAME question, ranked by specificity via ex-28's pairwise scorer
    "vague": "Trash keeps files for a while.",  # => co-14: least specific -- expected to lose every matchup
    "hedged": "Trash keeps files for around 30 days, roughly.",  # => co-14: some specificity, but hedged
    "precise": "Trash keeps deleted files for exactly 30 days.",  # => co-14: precise and unhedged
    "precise_with_context": "Trash keeps deleted files for exactly 30 days, with a 7-day grace warning before permanent removal.",  # => co-14: most specific -- expected to win every matchup
}  # => co-14: closes CANDIDATES -- four entries, a clear specificity ordering


def pairwise_prefers_first(reply_a: str, reply_b: str) -> bool:  # => co-14: ex-28's specificity-based pairwise scorer, reused
    """Return True iff `reply_a` is at least as specific as `reply_b` (digit count + 'exactly' mentions)."""  # => co-14: documents pairwise_prefers_first's contract -- no runtime output, just sets its __doc__
    specificity_a = sum(c.isdigit() for c in reply_a) + reply_a.count("exactly")  # => co-14: a crude but genuine specificity signal
    specificity_b = sum(c.isdigit() for c in reply_b) + reply_b.count("exactly")  # => co-14: the same signal, applied to the OTHER reply
    return specificity_a >= specificity_b  # => co-14: directly resolves the comparison


def run_round_robin(candidates: dict[str, str]) -> dict[str, int]:  # => co-14: every pair compared exactly once, wins tallied per candidate
    """Compare every unique pair of candidates exactly once, tallying wins per candidate name."""  # => co-14: documents run_round_robin's contract -- no runtime output, just sets its __doc__
    wins = {name: 0 for name in candidates}  # => co-14: accumulates one win count per candidate
    for name_a, name_b in combinations(candidates, 2):  # => co-14: every unordered pair, exactly once -- true round-robin
        if pairwise_prefers_first(candidates[name_a], candidates[name_b]):  # => co-14: run the pairwise comparison for this matchup
            wins[name_a] += 1  # => co-14: name_a won this matchup
        else:  # => co-14: name_b won this matchup instead
            wins[name_b] += 1  # => co-14: tally the win for name_b
    return wins  # => co-14: returns this computed value to the caller


if __name__ == "__main__":  # => co-14: entry point -- runs only when this file executes directly, not on import
    wins = run_round_robin(CANDIDATES)  # => co-14: run the full round-robin tournament
    ranked = sorted(wins.items(), key=lambda item: item[1], reverse=True)  # => co-14: rank candidates by tournament wins, most first
    for name, win_count in ranked:  # => co-14: prints the full, ranked tournament result
        print(f"{name}: {win_count} wins")  # => co-14: one line per candidate

    total_matches = len(CANDIDATES) * (len(CANDIDATES) - 1) // 2  # => co-14: the exact number of unique pairs for 4 candidates
    assert sum(wins.values()) == total_matches, "total wins must equal the total number of matches played"  # => co-14: a sanity check on the tournament's own bookkeeping
    assert ranked[0][0] == "precise_with_context", "the most specific candidate must win the most matchups"  # => co-14: the rule this example proves
    assert ranked[-1][0] == "vague", "the least specific candidate must win the fewest matchups"  # => co-14
    print(f"MATCH: {total_matches} pairwise matchups produce a full ranking, with the most specific candidate winning outright")  # => co-14
    # => co-14: a round-robin tournament turns N-way pairwise judging into a full ranking, without ever needing an absolute 1-5 scale
