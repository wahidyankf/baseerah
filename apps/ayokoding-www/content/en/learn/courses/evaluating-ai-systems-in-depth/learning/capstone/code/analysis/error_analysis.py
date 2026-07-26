"""Capstone Step 1: Read a Batch of Real Failures, Open-Code Them, and Cluster Into a Taxonomy."""  # => co-02/co-03/co-04: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import json  # => co-02: raw_failures.jsonl is read as real, logged JSON lines -- not invented after the fact
from pathlib import Path  # => co-02: locates raw_failures.jsonl relative to this file, not the caller's cwd
from typing import NamedTuple  # => co-02: FailureRecord and TaxonomyEntry are typed records, not bare dicts

RAW_FAILURES_PATH = Path(__file__).parent / "raw_failures.jsonl"  # => co-02: resolved relative to THIS file, so it runs correctly from any working directory


class FailureRecord(NamedTuple):  # => co-02: one real, logged failure -- the raw material for open coding
    request: str  # => co-02: the real request
    reply: str  # => co-02: what the agent actually replied
    open_code: str  # => co-02: a short, specific description of what went wrong -- written by READING the failure, not guessed from a category list


class TaxonomyEntry(NamedTuple):  # => co-04: one clustered failure mode -- a NAME plus its own frequency
    mode_name: str  # => co-04: the analyst's own coined name for this cluster of open codes
    frequency: int  # => co-06: how many of the eight raw failures fall into this mode


def load_raw_failures(path: Path = RAW_FAILURES_PATH) -> tuple[FailureRecord, ...]:  # => co-02: reads the real, logged batch of failures from disk
    """Return every line of `path` as a `FailureRecord`, parsed from JSON."""  # => co-02: documents load_raw_failures's contract -- no runtime output, just sets its __doc__
    records: list[FailureRecord] = []  # => co-02: accumulates one FailureRecord per JSONL line
    for line in path.read_text(encoding="utf-8").splitlines():  # => co-02: reads the file once, line by line -- JSONL, not a single JSON array
        data = json.loads(line)  # => co-02: parses this line's raw JSON
        records.append(FailureRecord(request=data["request"], reply=data["reply"], open_code=data["open_code"]))  # => co-02: builds a typed record from the parsed fields
    return tuple(records)  # => co-02: returns this computed value to the caller


def classify_open_code(open_code: str) -> str:  # => co-04: clusters ONE open code into a named mode -- the analyst's own judgment call, made explicit and checkable
    """Return the taxonomy mode name that best matches `open_code`'s wording."""  # => co-04: documents classify_open_code's contract -- no runtime output, just sets its __doc__
    text = open_code.lower()  # => co-04: case-insensitive matching
    if (
        "without any board" in text or "no board" in text or "no ticket" in text or "equally plausible" in text or "guessed a specific" in text or "single guessed ticket" in text
    ):  # => co-04: acted without resolving a genuine ambiguity first
        return "skips-clarifying-question"  # => co-04: matches ex-01's original taxonomy mode
    if "instead of the requested" in text:  # => co-04: acted on the WRONG specific target that WAS named, not an ambiguous one
        return "wrong-object-acted-on"  # => co-04: matches ex-06's original taxonomy mode
    if "counted" in text and "true" in text:  # => co-04: a numeric aggregate that does not match the real count
        return "incorrect-aggregate-count"  # => co-04: matches ex-06's original taxonomy mode
    if "ignoring the stated" in text:  # => co-04: a stated filter condition was present but not applied
        return "ignores-stated-filter-condition"  # => co-04: matches ex-49/ex-77's later-discovered mode
    return "uncategorized"  # => co-04: an explicit fallback -- never silently drops a failure that does not fit a known mode


def build_taxonomy(records: tuple[FailureRecord, ...]) -> tuple[TaxonomyEntry, ...]:  # => co-06: turns classified records into a frequency-ranked taxonomy
    """Return one `TaxonomyEntry` per distinct mode found in `records`, sorted by descending frequency."""  # => co-06: documents build_taxonomy's contract -- no runtime output, just sets its __doc__
    counts: dict[str, int] = {}  # => co-06: tallies how many records fall into each mode
    for r in records:  # => co-04: classifies every real failure
        mode = classify_open_code(r.open_code)  # => co-04: the mode this specific failure belongs to
        counts[mode] = counts.get(mode, 0) + 1  # => co-06: increments this mode's tally
    entries = [TaxonomyEntry(mode_name=name, frequency=freq) for name, freq in counts.items()]  # => co-06: one entry per distinct mode
    return tuple(sorted(entries, key=lambda e: e.frequency, reverse=True))  # => co-06: returns this computed value to the caller -- highest-frequency mode first


if __name__ == "__main__":  # => co-02: entry point -- runs only when this file executes directly, not on import
    failures = load_raw_failures()  # => co-02: load the real, logged batch
    taxonomy = build_taxonomy(failures)  # => co-04/co-06: cluster and rank into a taxonomy
    print(f"Loaded {len(failures)} real failures from {RAW_FAILURES_PATH.name}")  # => co-02: prints the loaded count
    for entry in taxonomy:  # => co-06: prints the ranked taxonomy
        print(f"  {entry.mode_name}: {entry.frequency} occurrences")  # => co-06

    assert len(failures) == 8, "the capstone's own raw-failure batch must contain exactly the eight logged failures on disk"  # => co-02: the rule this example proves
    assert taxonomy[0].mode_name in {"skips-clarifying-question", "wrong-object-acted-on"}, "the DOMINANT mode by frequency must be one of the two most common patterns in this batch"  # => co-06: the rule this example proves
    assert all(e.mode_name != "uncategorized" for e in taxonomy), "every real failure in this curated batch must classify into a NAMED mode, not fall through to the uncategorized fallback"  # => co-04
    print(f"MATCH: {len(failures)} real failures cluster into {len(taxonomy)} named modes, ranked by frequency, with '{taxonomy[0].mode_name}' dominant at {taxonomy[0].frequency} occurrences")  # => co-06
    # => co-04: Step 2 next derives operationalized criteria FROM these exact modes, in criteria.md and labeling-guide.md
