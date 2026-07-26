"""Capstone step 1 -- serve/: serve the model, instrument the four latency metrics, and document the
GPU memory budget with the cache arithmetic that yields a maximum concurrency.

Reuses the exact formulas taught in Examples 1, 11, 14, 16, and 59 -- nothing here is new arithmetic,
only their assembly into one importable module the rest of the capstone builds on.
"""

from dataclasses import dataclass


@dataclass
class TinyModel:  # => co-01/co-24: a stand-in for a small, self-hosted open-weights model
    name: str = "example-org/example-7b"


def handle_completion_request(model: TinyModel, prompt: str, max_output_tokens: int) -> dict[str, int | str]:
    # => co-01: the served endpoint -- request cost is UNKNOWABLE until generation actually finishes
    output_tokens = min(max_output_tokens, len(prompt) % 47 + 3)  # => deterministic, prompt-dependent length
    return {"model": model.name, "prompt_tokens": len(prompt.split()), "output_tokens": output_tokens}


def kv_cache_bytes_per_request(num_layers: int, num_heads: int, head_dim: int, seq_len: int, bytes_per_value: int) -> int:
    # => co-06: the cache-size formula from Example 11 / Example 59, unchanged
    return 2 * num_layers * num_heads * head_dim * seq_len * bytes_per_value


def gpu_memory_budget(
    total_gpu_bytes: int,
    weights_bytes: int,
    activations_bytes: int,
    framework_overhead_bytes: int,
    bytes_per_request: int,
) -> dict[str, int]:
    # => co-18: weights + cache + activations + overhead must all fit -- the remainder buys concurrency
    remainder_for_cache = total_gpu_bytes - weights_bytes - activations_bytes - framework_overhead_bytes
    max_concurrency = remainder_for_cache // bytes_per_request  # => co-07: cache budget SETS the ceiling
    return {
        "total_gpu_bytes": total_gpu_bytes,
        "weights_bytes": weights_bytes,
        "activations_bytes": activations_bytes,
        "framework_overhead_bytes": framework_overhead_bytes,
        "remainder_for_cache_bytes": remainder_for_cache,
        "bytes_per_request": bytes_per_request,
        "max_concurrency": max_concurrency,
    }


@dataclass
class GenerationTrace:  # => co-16: one completed request's four distinct latency metrics
    ttft_ms: float
    total_ms: float
    output_tokens: int


def compute_metrics(trace: GenerationTrace) -> dict[str, float]:
    # => co-16: TTFT, ITL, per-user tokens/sec, and (separately) aggregate throughput are FOUR distinct numbers
    decode_ms = trace.total_ms - trace.ttft_ms
    itl_ms = decode_ms / max(trace.output_tokens - 1, 1)
    tokens_per_sec = trace.output_tokens / (trace.total_ms / 1000.0)
    return {
        "ttft_ms": trace.ttft_ms,
        "itl_ms": round(itl_ms, 2),
        "tokens_per_sec": round(tokens_per_sec, 2),
    }
