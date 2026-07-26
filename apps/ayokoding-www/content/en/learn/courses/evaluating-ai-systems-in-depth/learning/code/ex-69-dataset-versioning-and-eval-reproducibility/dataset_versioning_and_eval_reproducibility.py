"""Worked Example 69: Version the Eval Dataset So a Past Score Is Reproducible."""  # => co-21: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import hashlib  # => co-22: a content hash makes a dataset version VERIFIABLE, not just a claimed label
from typing import NamedTuple  # => co-21: DatasetVersion is a typed record pairing a version label with a verifiable hash


class DatasetVersion(NamedTuple):  # => co-21: one snapshot of the eval dataset, with a verifiable content hash
    version_label: str  # => co-21: a human-readable version tag
    case_texts: tuple[str, ...]  # => co-21: the exact case texts in THIS version, in order
    content_hash: str  # => co-22: a hash over the case texts -- proves two "v1" claims are actually identical, or are not


def compute_dataset_hash(case_texts: tuple[str, ...]) -> str:  # => co-22: derives a short, stable content hash from the dataset's own texts
    """Return a short SHA-256 hex digest over the concatenation of `case_texts`, joined by newlines."""  # => co-22: documents compute_dataset_hash's contract -- no runtime output, just sets its __doc__
    joined = "\n".join(case_texts)  # => co-22: a deterministic, order-sensitive join
    return hashlib.sha256(joined.encode("utf-8")).hexdigest()[:12]  # => co-22: returns this computed value to the caller -- truncated for readability, still collision-resistant enough for this purpose


V1_CASES = ("Move this to done.", "Close ticket #12.")  # => co-21: the dataset as it existed at v1
V2_CASES = ("Move this to done.", "Close ticket #12.", "Archive tickets older than 90 days.")  # => co-21: v2 adds ex-49's newly-discovered case

V1 = DatasetVersion(version_label="v1-2026-01", case_texts=V1_CASES, content_hash=compute_dataset_hash(V1_CASES))  # => co-21: v1's own snapshot, hash computed FROM its actual content
V2 = DatasetVersion(version_label="v2-2026-03", case_texts=V2_CASES, content_hash=compute_dataset_hash(V2_CASES))  # => co-21: v2's own snapshot, hash computed FROM its actual content


def score_was_measured_on(reported_hash: str, known_versions: tuple[DatasetVersion, ...]) -> str | None:  # => co-22: recovers WHICH dataset version a historical score was actually measured against
    """Return the `version_label` of the `known_versions` entry whose `content_hash` equals `reported_hash`, or None if no match."""  # => co-22: documents score_was_measured_on's contract -- no runtime output, just sets its __doc__
    for v in known_versions:  # => co-22: scans known versions for a hash match
        if v.content_hash == reported_hash:  # => co-22: found the exact version this score belongs to
            return v.version_label  # => co-22: returns this computed value to the caller
    return None  # => co-22: no known version matches -- the dataset that produced this score is not reproducible from current records


if __name__ == "__main__":  # => co-22: entry point -- runs only when this file executes directly, not on import
    print(f"V1 hash: {V1.content_hash} ({len(V1.case_texts)} cases)")  # => co-21: prints v1's hash and size
    print(f"V2 hash: {V2.content_hash} ({len(V2.case_texts)} cases)")  # => co-21: prints v2's hash and size

    a_historical_score_report_hash = V1.content_hash  # => co-22: a score report from months ago cites this exact hash
    recovered_version = score_was_measured_on(a_historical_score_report_hash, (V1, V2))  # => co-22: recover which version that old score was measured against
    print(f"Historical score's dataset hash {a_historical_score_report_hash!r} resolves to version: {recovered_version}")  # => co-22

    assert V1.content_hash != V2.content_hash, "adding a case must change the dataset's content hash -- versions are distinguishable, not just labeled differently"  # => co-22: the rule this example proves
    assert recovered_version == "v1-2026-01", "a historical score's cited hash must resolve back to the EXACT dataset version it was measured against"  # => co-22: the rule this example proves
    print(f"MATCH: the dataset's content hash makes '{recovered_version}' recoverable from a bare hash string, so an old score stays reproducible even as the dataset grows to v2")  # => co-22
    # => co-22: ex-70 next returns to the noise floor, this time checking whether it stays STABLE across MANY repeated runs, not just five
