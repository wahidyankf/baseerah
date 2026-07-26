# learning/capstone/train/train.py
"""Capstone Step 3: The Training Run (exercises co-17, co-18, co-19, co-20, co-23, co-24)."""  # => co-20: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import json  # => co-20: the committed training-result artefact this step writes, read by evaluate/evaluate.py next
from pathlib import Path  # => co-20: locates the prior step's dataset splits and this step's own committed artefact
from typing import TypedDict, cast  # => co-20: types the committed artefacts this step reads and writes

DATASET_SPLITS_PATH = Path(__file__).parent.parent / "dataset" / "dataset_splits.json"  # => co-15: the prior step's own committed artefact
RESULT_PATH = Path(__file__).parent / "train_result.json"  # => co-20: this step's own committed artefact -- evaluate/evaluate.py reads it next


class DatasetSplits(TypedDict):  # => co-15: mirrors dataset.py's own committed shape -- only the fields this step actually needs
    audit_clean: bool  # => co-12: must be True before any training proceeds
    leakage_found: bool  # => co-16: must be False before any training proceeds
    train_ids: list[str]  # => co-15: how many training examples this run actually had
    val_ids: list[str]  # => co-15: how many validation examples this run's early-stopping check actually had
    base_model_id: str  # => co-30: carried forward, unchanged, from the dataset step


# => co-20: a rank sweep against held-out validation, base FROZEN throughout (co-18) -- matches this course's own ex-31 shape
RANK_SWEEP: dict[int, float] = {  # => co-20: rank -> measured validation pass rate
    1: 0.78,  # => co-20: too little capacity
    2: 0.85,  # => co-20
    4: 0.90,  # => co-20
    8: 0.94,  # => co-20: the last big jump before the curve flattens
    16: 0.945,  # => co-20: diminishing returns already visible
    32: 0.95,  # => co-20: essentially flat
    64: 0.95,  # => co-20: plateaued -- more capacity buys nothing further on this dataset
}  # => co-20: closes RANK_SWEEP

# => co-23,co-24: the CHOSEN rank's own per-epoch validation curve -- this is what early stopping actually watches
VAL_CURVE_FOR_CHOSEN_RANK: list[float] = [0.70, 0.84, 0.91, 0.88, 0.83]  # => co-23: peaks at epoch 3 (index 2), then falls -- overfitting past that point


class TrainResult(TypedDict):  # => co-20: the committed shape evaluate/evaluate.py reads next
    chosen_rank: int  # => co-20: selected from the sweep, not defaulted
    rank_sweep: dict[str, float]  # => co-20: the full sweep, for anyone auditing the choice later (JSON keys are always strings)
    chosen_epoch: int  # => co-23,co-24: selected by validation, not by a configured epoch count
    val_curve: list[float]  # => co-23: the full per-epoch curve the stopping decision was made from
    val_pass_rate: float  # => co-24: the chosen checkpoint's own validation pass rate
    adapter_size_mb: float  # => co-18: the adapter's own on-disk size at the chosen rank
    base_model_id: str  # => co-30: pinned, carried forward unchanged


if __name__ == "__main__":  # => co-20: entry point -- runs only when this file executes directly, not on import
    splits_raw = cast(DatasetSplits, json.loads(DATASET_SPLITS_PATH.read_text(encoding="utf-8")))  # => co-15: read the prior step's own committed artefact
    assert splits_raw["audit_clean"] and not splits_raw["leakage_found"], "training must only proceed on an audited, leak-free dataset"  # => co-12,co-16
    print(f"Training on {len(splits_raw['train_ids'])} train / {len(splits_raw['val_ids'])} validation cases (audited, leak-free)")  # => co-15

    for rank, val_score in RANK_SWEEP.items():  # => co-20: show the whole sweep, in order
        print(f"  rank={rank}: validation pass rate {val_score:.1%}")  # => co-20
    ranks_sorted = sorted(RANK_SWEEP)  # => co-20: walk the sweep in increasing rank order to find the plateau
    chosen_rank = next(
        r for i, r in enumerate(ranks_sorted[:-1]) if RANK_SWEEP[ranks_sorted[i + 1]] - RANK_SWEEP[r] < 0.01
    )  # => co-20: the first rank where ONE MORE step buys under 1 point -- the justified, not defaulted, choice
    print(f"Chosen rank: {chosen_rank} (justified against the sweep -- one more doubling of rank buys under 1 validation point)")  # => co-20
    assert chosen_rank in RANK_SWEEP, "the chosen rank must be one of the ranks the sweep actually measured"  # => co-20
    assert chosen_rank == 8, "this capstone's own scenario is designed so the plateau is first reached at rank 8"  # => co-20

    for epoch, val_score in enumerate(VAL_CURVE_FOR_CHOSEN_RANK, start=1):  # => co-23: show the chosen rank's own per-epoch curve
        print(f"  epoch {epoch}: validation pass rate {val_score:.0%}")  # => co-23
    chosen_epoch = max(range(1, len(VAL_CURVE_FOR_CHOSEN_RANK) + 1), key=lambda e: VAL_CURVE_FOR_CHOSEN_RANK[e - 1])  # => co-23,co-24: the epoch VALIDATION says is best, not the last epoch trained
    chosen_val_pass_rate = VAL_CURVE_FOR_CHOSEN_RANK[chosen_epoch - 1]  # => co-24: that epoch's own validation pass rate
    print(f"Chosen epoch (early stopping on validation): {chosen_epoch} (val {chosen_val_pass_rate:.0%})")  # => co-23,co-24
    assert chosen_epoch == 3, "early stopping must select epoch 3, this scenario's own true validation peak"  # => co-23
    assert chosen_epoch != len(VAL_CURVE_FOR_CHOSEN_RANK), "the stopping point must be set by validation, not by training to the configured epoch count"  # => co-23,co-24

    adapter_size_mb = round(0.29 * chosen_rank, 2)  # => co-18: adapter size scales with rank -- a small, composable artefact regardless
    print(f"Adapter size at rank {chosen_rank}: {adapter_size_mb} MB")  # => co-18

    result: TrainResult = {  # => co-20: the full committed artefact -- every field traceable to a decision made above
        "chosen_rank": chosen_rank,  # => co-20
        "rank_sweep": {str(rank): score for rank, score in RANK_SWEEP.items()},  # => co-20: JSON object keys are always strings
        "chosen_epoch": chosen_epoch,  # => co-23,co-24
        "val_curve": VAL_CURVE_FOR_CHOSEN_RANK,  # => co-23
        "val_pass_rate": chosen_val_pass_rate,  # => co-24
        "adapter_size_mb": adapter_size_mb,  # => co-18
        "base_model_id": splits_raw["base_model_id"],  # => co-30: matches decision.py's own pin, carried through the dataset step unchanged
    }  # => co-20: closes result
    RESULT_PATH.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")  # => co-20: commits the artefact evaluate/evaluate.py reads next
    print(f"MATCH: training result committed to {RESULT_PATH.name} -- rank chosen from the sweep, epoch chosen by validation, not defaulted")  # => co-20,co-23,co-24
    # => co-17,co-18,co-19,co-20,co-23,co-24: a parameter-efficient adapter, base frozen throughout, its capacity and stopping point both evidenced, not assumed
