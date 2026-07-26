# learning/code/ex-30-adapter-vs-full-cost/adapter_vs_full_cost.py
"""Worked Example 30: Adapter vs. Full Cost."""  # => co-18: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

# => co-17: ex-28's own measured full fine-tune numbers, reused here for a fair, apples-to-apples comparison
FULL_CHECKPOINT_SIZE_GB = 1.84  # => co-17: from ex-28
FULL_PEAK_MEMORY_GB = 5.52  # => co-17: from ex-28
FULL_TRAINING_MINUTES = 46.0  # => co-08: from ex-28
FULL_TRAINING_COST_USD = 0.61  # => co-08: from ex-28

# => co-18: the SAME target behaviour change from ex-29, measured on the adapter this time
ADAPTER_CHECKPOINT_SIZE_MB = 2.75  # => co-18: from ex-29's ~688K trainable parameters, stored at fp32
ADAPTER_PEAK_MEMORY_GB = 2.10  # => co-18: base weights loaded (frozen, no optimizer state) + small adapter optimizer state + activations
ADAPTER_TRAINING_MINUTES = 9.0  # => co-08,co-18: far fewer gradients to compute and store
ADAPTER_TRAINING_COST_USD = 0.12  # => co-08,co-18

if __name__ == "__main__":  # => co-18: entry point -- runs only when this file executes directly, not on import
    checkpoint_ratio = (ADAPTER_CHECKPOINT_SIZE_MB / 1024) / FULL_CHECKPOINT_SIZE_GB  # => co-18: adapter checkpoint as a fraction of the full checkpoint
    memory_ratio = ADAPTER_PEAK_MEMORY_GB / FULL_PEAK_MEMORY_GB  # => co-18: adapter peak memory as a fraction of full peak memory
    time_ratio = ADAPTER_TRAINING_MINUTES / FULL_TRAINING_MINUTES  # => co-08: adapter training time as a fraction of full training time
    cost_ratio = ADAPTER_TRAINING_COST_USD / FULL_TRAINING_COST_USD  # => co-08: adapter training cost as a fraction of full training cost
    print(f"Checkpoint size: full {FULL_CHECKPOINT_SIZE_GB:.2f} GB vs. adapter {ADAPTER_CHECKPOINT_SIZE_MB:.2f} MB ({checkpoint_ratio:.1%})")  # => co-18
    print(f"Peak memory: full {FULL_PEAK_MEMORY_GB:.2f} GB vs. adapter {ADAPTER_PEAK_MEMORY_GB:.2f} GB ({memory_ratio:.0%})")  # => co-18
    print(f"Training time: full {FULL_TRAINING_MINUTES:.0f} min vs. adapter {ADAPTER_TRAINING_MINUTES:.0f} min ({time_ratio:.0%})")  # => co-08
    print(f"Training cost: full ${FULL_TRAINING_COST_USD:.2f} vs. adapter ${ADAPTER_TRAINING_COST_USD:.2f} ({cost_ratio:.0%})")  # => co-08
    assert checkpoint_ratio < 0.01, "the adapter checkpoint must be well under 1% of the full checkpoint's size"  # => co-18
    assert memory_ratio < 0.5, "the adapter's peak memory must be meaningfully lower than the full fine-tune's"  # => co-17,co-18
    print("MATCH: for comparable behaviour change (ex-27 vs. ex-29), the adapter costs a small fraction on every axis measured")  # => co-08,co-17,co-18
    # => co-08,co-17,co-18: this is the fraction a full fine-tune should have to argue against, per this course's own tension between the two techniques
