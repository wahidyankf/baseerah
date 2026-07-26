"""Capstone step 3 -- tune/: plot the throughput/latency frontier, select an operating point against
the stated SLO, and make the quantization decision by evaluating the quantized model on a real,
learner-measured quality delta -- never a published figure.

Reuses the cost model from Examples 33-34 and the quantization decision shape from Examples 51-53.
"""

from dataclasses import dataclass

BASE_STEP_MS = 15.0
MS_PER_EXTRA_SLOT = 1.0


def itl_at_batch(batch_size: int) -> float:  # => co-15/co-16: the SAME cost model used throughout this course
    return BASE_STEP_MS + MS_PER_EXTRA_SLOT * batch_size


def throughput_at_batch(batch_size: int) -> float:
    return batch_size * 1000.0 / itl_at_batch(batch_size)


def frontier(batch_sizes: list[int]) -> list[tuple[int, float, float]]:
    # => co-15: (batch_size, throughput, itl_ms) -- the durable trade, traced across configurations
    return [(b, round(throughput_at_batch(b), 1), round(itl_at_batch(b), 1)) for b in batch_sizes]


def pick_operating_point(batch_sizes: list[int], itl_slo_ms: float) -> int:
    # => co-15: choose the LARGEST batch size that still satisfies the stated SLO -- maximize throughput
    #    subject to the constraint, never the other way around
    candidates = [b for b in batch_sizes if itl_at_batch(b) <= itl_slo_ms]
    if not candidates:
        raise ValueError(f"no batch size satisfies ITL SLO of {itl_slo_ms}ms")
    return max(candidates)


@dataclass
class QuantizationCandidate:  # => co-19: same shape as Example 53, but quality_score is MEASURED, not assumed
    name: str
    memory_gb: float
    measured_quality_delta: float  # => co-19: the learner's OWN measured degradation, e.g. from an eval suite


def decide_quantization(candidates: list[QuantizationCandidate], max_tolerated_quality_delta: float) -> dict[str, str]:
    # => co-19: accept only candidates within the tolerated MEASURED quality delta; prefer smallest memory
    accepted = [c for c in candidates if c.measured_quality_delta <= max_tolerated_quality_delta]
    if not accepted:
        return {"decision": "reject_all", "reason": "no candidate's measured quality delta is within tolerance"}
    winner = min(accepted, key=lambda c: c.memory_gb)
    return {
        "decision": winner.name,
        "reason": f"smallest memory ({winner.memory_gb} GB) with measured quality delta {winner.measured_quality_delta} <= tolerance {max_tolerated_quality_delta}",
    }
