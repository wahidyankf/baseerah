"""Example 16: Latency Vocabulary."""

from dataclasses import dataclass  # => stdlib only -- no external metrics library needed for this shape

# => four names, one formula each -- confusing them is the single most common serving-latency mistake


@dataclass
class GenerationTrace:  # => co-16: the four metrics this topic uses to describe serving latency
    prefill_ms: float  # => one number -- prefill has no per-token breakdown, it is one parallel pass
    decode_step_ms: list[float]  # => one entry per emitted token


def compute_metrics(trace: GenerationTrace) -> dict[str, float]:  # => derives all four metrics from one trace
    ttft_ms = trace.prefill_ms + trace.decode_step_ms[0]  # => co-16: time to FIRST token, prefill + step 1
    inter_token_ms = sum(trace.decode_step_ms[1:]) / (len(trace.decode_step_ms) - 1)  # => avg gap AFTER token 1
    total_s = (trace.prefill_ms + sum(trace.decode_step_ms)) / 1000  # => wall-clock for this ONE request
    tokens_per_sec_per_user = len(trace.decode_step_ms) / total_s  # => co-16: THIS user's own rate
    return {  # => bundling all four so callers read one dict instead of four loose variables
        "ttft_ms": ttft_ms,  # => the metric users FEEL first -- how long until anything shows up
        "inter_token_ms": inter_token_ms,  # => the metric users feel AFTER that -- how smoothly it streams
        "tokens_per_sec_per_user": tokens_per_sec_per_user,  # => this ONE user's rate, not the fleet's
    }  # => end of the four-metric dict


trace = GenerationTrace(prefill_ms=100.0, decode_step_ms=[20.0, 20.0, 20.0, 20.0, 20.0])  # => 5 equal steps
# => equal steps here isolate the vocabulary -- real traces have UNEQUAL per-step latencies
metrics = compute_metrics(trace)  # => one call, four derived metrics
print(metrics["ttft_ms"])  # => Output: 120.0
print(metrics["inter_token_ms"])  # => Output: 20.0
print(round(metrics["tokens_per_sec_per_user"], 2))  # => Output: 25.0

assert metrics["ttft_ms"] == 120.0  # => co-16: TTFT bundles prefill AND the first decode step
assert metrics["inter_token_ms"] == 20.0  # => co-16: ITL is the steady-state per-token gap thereafter
assert metrics["ttft_ms"] != metrics["inter_token_ms"]  # => co-16: these are genuinely DIFFERENT metrics
print("ex-16 OK")  # => a self-check marker confirming all four derived metrics matched expectations
