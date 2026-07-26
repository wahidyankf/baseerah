# learning/code/ex-23-synthetic-generation/synthetic_generation.py
"""Worked Example 23: Synthetic Generation."""  # => co-14: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-14: a generation run's own recorded shape -- speed and volume, measured, not assumed


@dataclass(frozen=True)  # => co-14: frozen -- a completed generation run's numbers should not mutate after the fact
class GenerationRun:  # => co-14: what generating training data from a larger teacher model actually looks like, measured
    examples_generated: int  # => co-14: how many synthetic examples this run produced
    wall_clock_minutes: float  # => co-14: how long the run took, end to end
    cost_usd: float  # => co-14: the teacher-model API cost for this run


EXPERT_AUTHORING_RATE_PER_HOUR = 300.0 / 8  # => co-13: from ex-22 -- 300 examples took roughly a full working day, illustratively
SYNTHETIC_RUN = GenerationRun(examples_generated=2_500, wall_clock_minutes=18.0, cost_usd=6.75)  # => co-14: a larger teacher model, prompted to generate triage examples

if __name__ == "__main__":  # => co-14: entry point -- runs only when this file executes directly, not on import
    synthetic_rate_per_hour = SYNTHETIC_RUN.examples_generated / (SYNTHETIC_RUN.wall_clock_minutes / 60)  # => co-14: synthetic examples produced per hour
    print(f"Synthetic generation: {SYNTHETIC_RUN.examples_generated} examples in {SYNTHETIC_RUN.wall_clock_minutes:.0f} minutes, ${SYNTHETIC_RUN.cost_usd:.2f}")  # => co-14
    print(f"Synthetic rate: {synthetic_rate_per_hour:,.0f} examples/hour vs. expert authoring: {EXPERT_AUTHORING_RATE_PER_HOUR:.1f} examples/hour")  # => co-13,co-14
    speed_ratio = synthetic_rate_per_hour / EXPERT_AUTHORING_RATE_PER_HOUR  # => co-14: how many times faster synthetic generation is
    print(f"Synthetic generation is {speed_ratio:,.0f}x faster than expert authoring, per example")  # => co-14
    assert speed_ratio > 100, "synthetic generation must be dramatically faster than expert authoring for this demo to land"  # => co-14
    print("MATCH: synthetic generation is fast and cheap -- ex-24 shows what it costs in a different currency: silent error propagation")  # => co-14
    # => co-13,co-14: co-14's speed advantage is real -- and it is bounded by the teacher's own quality, which ex-24 measures next
