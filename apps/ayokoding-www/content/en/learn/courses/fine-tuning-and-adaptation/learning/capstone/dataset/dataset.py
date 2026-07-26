# learning/capstone/dataset/dataset.py
"""Capstone Step 2: The Dataset (exercises co-09, co-10, co-11, co-12, co-13, co-15, co-16)."""  # => co-10: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import json  # => co-10: the committed dataset-splits artefact this step writes, read by train/train.py next
from pathlib import Path  # => co-10: locates the prior step's decision record and this step's own committed artefact
from typing import NamedTuple, TypedDict, cast  # => co-10: NamedTuple per case, TypedDict for the committed artefact, cast for the typed JSON read

DECISION_RECORD_PATH = Path(__file__).parent.parent / "decision" / "decision_record.json"  # => co-06: the prior step's own committed artefact
RESULT_PATH = Path(__file__).parent / "dataset_splits.json"  # => co-10: this step's own committed artefact -- train/train.py reads it next


class DecisionRecord(TypedDict):  # => co-06: mirrors decision.py's own committed shape -- only the fields this step actually needs
    decision: str  # => co-06: must read "go" for this step to proceed
    base_model_id: str  # => co-30: carried forward, unchanged, through every remaining step


class TrainingCase(NamedTuple):  # => co-09: one instruction/response pair, tagged with its sourcing strategy
    case_id: str  # => co-15: the id every split and the leakage check operate on
    instruction: str  # => co-09: the ticket-triage input
    response: str  # => co-09: the target output
    source: str  # => co-13: "production_traffic" or "expert_authored" -- each carries a distinct bias, per this course's own ex-21/ex-22


# => co-09,co-10,co-13: 20 curated instruction/response pairs -- a MIX of production traffic and expert authoring, per co-13's own sourcing discipline
DATASET: list[TrainingCase] = [  # => co-10: one row per case, in collection order
    TrainingCase("case-01", "Triage: customer cannot log in after a password reset.", "Priority: P2. Category: access.", "production_traffic"),  # => co-13
    TrainingCase("case-02", "Triage: customer was charged twice for the same invoice.", "Priority: P2. Category: billing.", "production_traffic"),  # => co-13
    TrainingCase("case-03", "Triage: production API returning 500 errors for all customers.", "Priority: P1. Category: outage.", "expert_authored"),  # => co-13
    TrainingCase("case-04", "Triage: customer requests a CSV export feature.", "Priority: P4. Category: feature-request.", "production_traffic"),  # => co-13
    TrainingCase("case-05", "Triage: customer's dashboard shows stale data for three days.", "Priority: P2. Category: bug.", "production_traffic"),  # => co-13
    TrainingCase("case-06", "Triage: customer cannot find the invoice download button.", "Priority: P3. Category: usability.", "production_traffic"),  # => co-13
    TrainingCase("case-07", "Triage: single-sign-on integration broken for one enterprise customer.", "Priority: P1. Category: access.", "expert_authored"),  # => co-13
    TrainingCase("case-08", "Triage: customer asks how to change their billing email.", "Priority: P4. Category: billing.", "production_traffic"),  # => co-13
    TrainingCase("case-09", "Triage: bulk import silently drops rows over 10,000.", "Priority: P2. Category: bug.", "expert_authored"),  # => co-13
    TrainingCase("case-10", "Triage: customer wants a dark-mode theme option.", "Priority: P4. Category: feature-request.", "production_traffic"),  # => co-13
    TrainingCase("case-11", "Triage: all customers on the EU region cannot log in.", "Priority: P1. Category: outage.", "expert_authored"),  # => co-13
    TrainingCase("case-12", "Triage: customer disputes a refund that was never processed.", "Priority: P2. Category: billing.", "production_traffic"),  # => co-13
    TrainingCase("case-13", "Triage: mobile app crashes on opening for some Android versions.", "Priority: P2. Category: bug.", "production_traffic"),  # => co-13
    TrainingCase("case-14", "Triage: customer requests an API rate-limit increase.", "Priority: P3. Category: feature-request.", "expert_authored"),  # => co-13
    TrainingCase("case-15", "Triage: customer cannot reset their password, reset email never arrives.", "Priority: P2. Category: access.", "production_traffic"),  # => co-13
    TrainingCase("case-16", "Triage: a single enterprise customer's data export job hangs indefinitely.", "Priority: P1. Category: bug.", "expert_authored"),  # => co-13
    TrainingCase("case-17", "Triage: customer asks for a plain-language explanation of an invoice line item.", "Priority: P4. Category: billing.", "production_traffic"),  # => co-13
    TrainingCase("case-18", "Triage: customer reports the search feature returns no results for common terms.", "Priority: P2. Category: bug.", "production_traffic"),  # => co-13
    TrainingCase("case-19", "Triage: customer requests a Slack notification integration.", "Priority: P4. Category: feature-request.", "expert_authored"),  # => co-13
    TrainingCase("case-20", "Triage: customer's team cannot invite new seats past their plan limit.", "Priority: P3. Category: billing.", "production_traffic"),  # => co-13
]  # => co-10: closes DATASET -- 20 cases, comfortably inside this course's own "a few hundred" quality-over-quantity range for a small capstone


def audit_conflicts(dataset: list[TrainingCase]) -> list[tuple[str, str]]:  # => co-12: find any instruction that maps to more than one distinct response
    """Return conflicting (instruction, response) pairs: an instruction repeated with a DIFFERENT response than its first occurrence."""  # => co-12: documents audit_conflicts's contract -- no runtime output, just sets its __doc__
    seen: dict[str, str] = {}  # => co-12: instruction -> the first response seen for it
    conflicts: list[tuple[str, str]] = []  # => co-12: accumulates any instruction seen twice with a DIFFERENT response
    for case in dataset:  # => co-12: walk every case in collection order
        if case.instruction in seen and seen[case.instruction] != case.response:  # => co-12: the SAME instruction, a DIFFERENT target response
            conflicts.append((case.instruction, case.response))  # => co-12: record the conflicting pair
        seen[case.instruction] = case.response  # => co-12: remember this instruction's (latest) response
    return conflicts  # => co-12: returns this computed value to the caller


class DatasetSplits(TypedDict):  # => co-15: the committed shape train/train.py reads next
    decision_confirmed: bool  # => co-06: this step must have confirmed the prior step's own "go"
    total_examples: int  # => co-10: the full dataset's own size
    train_ids: list[str]  # => co-15: the training split's case ids
    val_ids: list[str]  # => co-15: the validation split's case ids
    test_ids: list[str]  # => co-15: the held-out test split's case ids
    audit_clean: bool  # => co-12: did the consistency audit find zero conflicts
    leakage_found: bool  # => co-16: did any id appear in more than one split
    production_traffic_share: float  # => co-13: documented sourcing bias -- the fraction sourced from real traffic
    base_model_id: str  # => co-30: carried forward, unchanged, from the decision step


if __name__ == "__main__":  # => co-10: entry point -- runs only when this file executes directly, not on import
    decision_raw = cast(DecisionRecord, json.loads(DECISION_RECORD_PATH.read_text(encoding="utf-8")))  # => co-06: read the prior step's own committed artefact
    assert decision_raw["decision"] == "go", "the dataset step must only proceed once the decision step's own gate reached go"  # => co-06
    print(f"Decision confirmed: {decision_raw['decision']!r} for base {decision_raw['base_model_id']!r}")  # => co-06,co-30

    conflicts = audit_conflicts(DATASET)  # => co-12: run the consistency audit
    audit_clean = len(conflicts) == 0  # => co-12: a clean audit means zero conflicting instruction/response pairs
    print(f"Consistency audit conflicts: {conflicts} | Audit clean: {audit_clean}")  # => co-12
    assert audit_clean, "this capstone's own dataset must be free of planted or accidental conflicts before training proceeds"  # => co-12

    production_traffic_count = sum(1 for c in DATASET if c.source == "production_traffic")  # => co-13: how many cases came from real traffic
    production_traffic_share = production_traffic_count / len(DATASET)  # => co-13: the documented sourcing bias this dataset carries
    print(f"Sourcing: {production_traffic_count} of {len(DATASET)} cases from production traffic ({production_traffic_share:.0%})")  # => co-13

    train_cases, val_cases, test_cases = DATASET[:14], DATASET[14:17], DATASET[17:]  # => co-15: 70/15/15, matching this course's own ex-25 split ratio
    train_ids, val_ids, test_ids = [c.case_id for c in train_cases], [c.case_id for c in val_cases], [c.case_id for c in test_cases]  # => co-15
    print(f"Train: {len(train_ids)} | Validation: {len(val_ids)} | Test: {len(test_ids)}")  # => co-15
    assert len(train_ids) + len(val_ids) + len(test_ids) == len(DATASET), "every case must land in exactly one split"  # => co-15

    all_split_ids = train_ids + val_ids + test_ids  # => co-16: every id across every split, concatenated
    leakage_found = len(set(all_split_ids)) != len(all_split_ids)  # => co-16: a leak is any id appearing more than once across splits
    print(f"Leakage check -- duplicate ids across splits: {leakage_found}")  # => co-16
    assert not leakage_found, "no case id may appear in more than one split -- a leaked id would invalidate every downstream eval"  # => co-16

    splits: DatasetSplits = {  # => co-15: the full committed artefact -- every field traceable to a check run above
        "decision_confirmed": True,  # => co-06
        "total_examples": len(DATASET),  # => co-10
        "train_ids": train_ids,  # => co-15
        "val_ids": val_ids,  # => co-15
        "test_ids": test_ids,  # => co-15
        "audit_clean": audit_clean,  # => co-12
        "leakage_found": leakage_found,  # => co-16
        "production_traffic_share": production_traffic_share,  # => co-13
        "base_model_id": decision_raw["base_model_id"],  # => co-30
    }  # => co-15: closes splits
    RESULT_PATH.write_text(json.dumps(splits, indent=2, sort_keys=True) + "\n", encoding="utf-8")  # => co-15: commits the artefact train/train.py reads next
    print(f"MATCH: dataset splits committed to {RESULT_PATH.name} -- audit clean, splits disjoint, zero leakage")  # => co-12,co-15,co-16
    # => co-10,co-12,co-15,co-16: the dataset is the whole job -- every downstream step's result is only as trustworthy as this step's own audit and split discipline
