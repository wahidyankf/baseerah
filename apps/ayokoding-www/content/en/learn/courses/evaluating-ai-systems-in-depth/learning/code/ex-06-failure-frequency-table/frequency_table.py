"""Worked Example 6: Count Each Failure Mode's Frequency in the Sample."""  # => co-03: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from collections import Counter  # => co-04: a Counter is the exact right stdlib tool for a frequency table

# The taxonomy ex-05 produced -- but here we have the FULL 40-case sample it was clustered from,
# not just the 13 shown in ex-05's smaller demonstration.
FULL_SAMPLE_MODE_PER_CASE: list[str] = (  # => co-03: one mode name per ticket in the full sample, in reading order
    ["wrong-object-acted-on"] * 9  # => co-04: nine real cases landed in this mode
    + ["malformed-structured-output"] * 14  # => co-04: fourteen real cases -- the largest cluster
    + ["incorrect-aggregate-count"] * 6  # => co-04: six real cases
    + ["tone-mismatch-for-audience"] * 11  # => co-04: eleven real cases -- a fourth mode this larger sample reveals
)  # => co-04: closes FULL_SAMPLE_MODE_PER_CASE -- 40 cases total, four named modes


def build_frequency_table(mode_per_case: list[str]) -> Counter[str]:  # => co-04: the counting step itself
    """Count how many sampled cases fall under each named failure mode."""  # => co-04: documents build_frequency_table's contract -- no runtime output, just sets its __doc__
    return Counter(mode_per_case)  # => co-04: one line -- Counter does exactly this, with no hand-rolled loop needed


if __name__ == "__main__":  # => co-04: entry point -- runs only when this file executes directly, not on import
    table = build_frequency_table(FULL_SAMPLE_MODE_PER_CASE)  # => co-04: run the count over the full 40-case sample
    for mode_name, count in table.most_common():  # => co-04: print modes ranked from most to least frequent
        share = count / len(FULL_SAMPLE_MODE_PER_CASE)  # => co-04: this mode's share of the whole sample
        print(f"{mode_name}: {count} cases ({share:.0%})")  # => co-04: one ranked line per mode

    total_counted = sum(table.values())  # => co-04: every counted case must sum back to the sample size
    assert total_counted == len(FULL_SAMPLE_MODE_PER_CASE), "the counts must sum to exactly the full sample size"  # => co-04: the floor this example demonstrates
    assert len(table) == 4, "this sample must cluster into exactly four counted modes"  # => co-04: sanity check on the fixture
    most_common_mode, most_common_count = table.most_common(1)[0]  # => co-04: the single most frequent mode, by count
    print(f"MATCH: {total_counted}/{len(FULL_SAMPLE_MODE_PER_CASE)} cases counted; most frequent mode is {most_common_mode!r} ({most_common_count} cases)")  # => co-04
    # => co-04: "malformed-structured-output" is the LARGEST cluster here -- but ex-07 shows frequency alone is not the full ranking
