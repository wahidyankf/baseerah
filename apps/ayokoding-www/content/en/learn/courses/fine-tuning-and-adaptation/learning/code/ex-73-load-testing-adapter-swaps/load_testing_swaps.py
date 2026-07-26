# learning/code/ex-73-load-testing-adapter-swaps/load_testing_swaps.py
"""Worked Example 73: Load-Testing Adapter Swaps."""  # => co-29: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-29: one immutable row per simulated swap, its measured latency recorded


class SwapMeasurement(NamedTuple):  # => co-21: a single hot-swap event under simulated load, and how long it took
    swap_id: int  # => co-21: which swap in the sequence this is
    from_adapter: str  # => co-21: the adapter that was active before this swap
    to_adapter: str  # => co-21: the adapter that became active after this swap
    swap_latency_ms: float  # => co-29: measured time for the swap itself -- NOT a full base reload


SWAP_LATENCY_BUDGET_MS = 50.0  # => co-29: the serving stack's own budget for a hot-swap to stay invisible to the caller

# => co-29,co-21: 6 simulated swaps under load, alternating between two adapters on the SAME loaded base
SWAP_MEASUREMENTS: list[SwapMeasurement] = [  # => co-29: one row per swap, in the order they were run
    SwapMeasurement(swap_id=1, from_adapter="billing-tone-v1", to_adapter="escalation-tone-v1", swap_latency_ms=8.2),  # => co-29: swap 1
    SwapMeasurement(swap_id=2, from_adapter="escalation-tone-v1", to_adapter="billing-tone-v1", swap_latency_ms=7.9),  # => co-29: swap 2
    SwapMeasurement(swap_id=3, from_adapter="billing-tone-v1", to_adapter="escalation-tone-v1", swap_latency_ms=9.1),  # => co-29: swap 3
    SwapMeasurement(swap_id=4, from_adapter="escalation-tone-v1", to_adapter="billing-tone-v1", swap_latency_ms=8.6),  # => co-29: swap 4
    SwapMeasurement(swap_id=5, from_adapter="billing-tone-v1", to_adapter="escalation-tone-v1", swap_latency_ms=11.3),  # => co-29: swap 5, a slight spike under load
    SwapMeasurement(swap_id=6, from_adapter="escalation-tone-v1", to_adapter="billing-tone-v1", swap_latency_ms=8.4),  # => co-29: swap 6
]  # => co-29: closes SWAP_MEASUREMENTS

FULL_BASE_RELOAD_LATENCY_MS = 4_200.0  # => co-29: for comparison -- what reloading the WHOLE base model would cost instead of swapping an adapter


if __name__ == "__main__":  # => co-29: entry point -- runs only when this file executes directly, not on import
    for measurement in SWAP_MEASUREMENTS:  # => co-29: show every swap's measured latency, in order
        print(f"  swap {measurement.swap_id}: {measurement.from_adapter} -> {measurement.to_adapter} took {measurement.swap_latency_ms:.1f}ms")  # => co-29
    max_swap_latency = max(m.swap_latency_ms for m in SWAP_MEASUREMENTS)  # => co-29: the worst observed swap under load
    avg_swap_latency = sum(m.swap_latency_ms for m in SWAP_MEASUREMENTS) / len(SWAP_MEASUREMENTS)  # => co-29: the typical swap cost under load
    print(f"Max swap latency: {max_swap_latency:.1f}ms | Average: {avg_swap_latency:.1f}ms | Budget: {SWAP_LATENCY_BUDGET_MS:.0f}ms")  # => co-29
    assert max_swap_latency < SWAP_LATENCY_BUDGET_MS, "every single swap must stay comfortably under the 50ms latency budget, even the worst one under load"  # => co-29
    reload_vs_swap_ratio = FULL_BASE_RELOAD_LATENCY_MS / max_swap_latency  # => co-21,co-29: how much more expensive a full base reload would be, for comparison
    print(f"A full base reload would cost {reload_vs_swap_ratio:.0f}x the worst measured swap latency")  # => co-21,co-29
    assert reload_vs_swap_ratio > 200, "an adapter swap must be dramatically cheaper than a full base reload under this scenario's numbers"  # => co-21,co-29
    print("MATCH: every hot-swap under simulated load stays under 12ms, over 350x cheaper than the 4.2-second cost of reloading the whole base model")  # => co-21,co-29
    # => co-21,co-29: this is the load-tested version of ex-46's single demonstration swap -- the operational claim holds under repeated, sustained use, not just once
