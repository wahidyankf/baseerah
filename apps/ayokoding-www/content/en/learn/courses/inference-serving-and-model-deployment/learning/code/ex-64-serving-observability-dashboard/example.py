"""Example 64: Serving Observability Dashboard."""

from dataclasses import dataclass  # => stdlib only -- aggregating traces needs no framework


@dataclass
class RequestTrace:  # => co-26/co-16: one completed request's observed metrics
    ttft_ms: float  # => co-16: this request's time-to-first-token
    itl_ms: float  # => co-16: this request's steady-state inter-token latency
    preempted: bool  # => co-14: whether this request was ever evicted mid-generation


def build_dashboard(traces: list[RequestTrace], queue_depth: int, batch_occupancy: float) -> dict[str, float | int]:
    # => co-26: aggregates the signals that EXPLAIN a serving problem -- queue, occupancy, latency, preemption
    ttfts = sorted(t.ttft_ms for t in traces)  # => sorted so a percentile can be read off by index
    itls = sorted(t.itl_ms for t in traces)  # => sorted so a percentile can be read off by index
    preemption_rate = sum(1 for t in traces if t.preempted) / len(traces)  # => co-14: fraction of traces evicted
    return {  # => co-26: one dashboard snapshot -- every field a real on-call engineer would want
        "queue_depth": queue_depth,  # => co-13: how many requests are waiting, right now
        "batch_occupancy": batch_occupancy,  # => co-12: how full the active batch is, right now
        "ttft_p50_ms": ttfts[len(ttfts) // 2],  # => co-16: the median time-to-first-token
        "itl_p50_ms": itls[len(itls) // 2],  # => co-16: the median inter-token latency
        "preemption_rate": round(preemption_rate, 2),  # => co-14: how often eviction is happening
    }


traces = [  # => co-26: five completed requests, two of which were preempted mid-generation -- a small fleet snapshot
    RequestTrace(120, 18, False),  # => a normal, uninterrupted request
    RequestTrace(140, 20, False),  # => a normal, uninterrupted request
    RequestTrace(500, 45, True),  # => co-14: this request was PREEMPTED -- its latency reflects that cost
    RequestTrace(130, 19, False),  # => a normal, uninterrupted request
    RequestTrace(600, 50, True),  # => co-14: preempted again -- two of five, a signal worth noticing
]
dashboard = build_dashboard(traces, queue_depth=12, batch_occupancy=0.92)  # => co-26: one call, five signals
print(dashboard)
# => Output: {'queue_depth': 12, 'batch_occupancy': 0.92, 'ttft_p50_ms': 140, 'itl_p50_ms': 20, 'preemption_rate': 0.4}
# => co-26: Example 74 feeds this EXACT shape into a rule chain to diagnose what's actually wrong

assert dashboard["preemption_rate"] == 0.4  # => co-26: 2 of 5 traces were preempted -- a real, actionable number
# => a single p50 latency number alone would NEVER have surfaced this preemption signal
# => co-26: dashboards built from raw traces beat dashboards built from pre-averaged summaries
print("ex-64 OK")  # => a self-check marker confirming the dashboard's preemption-rate arithmetic held
# => this is the last co-26 example -- capacity, batching, and preemption all converge into one view
