"""Example 28: Beginner Recap -- End-to-End Admission Pipeline."""

from dataclasses import dataclass  # => stdlib only -- the recap needs no new dependency

BYTES_PER_TOKEN = 98304  # => co-06
PREFILL_MS_PER_TOKEN = 0.5  # => co-02
DECODE_MS_PER_TOKEN = 20.0  # => co-03
# => three constants, one function -- everything Examples 1-27 built up, compressed into one pipeline


@dataclass
class ServeResult:  # => co-01..co-18 recap: everything this beginner tier taught, in one small pipeline
    admitted: bool  # => co-07/co-18: the cache-gate verdict, decided BEFORE any latency is computed
    total_ms: float  # => co-02/co-03: only meaningful when admitted is True


def serve_request(cache_budget_bytes: int, prompt_tokens: int, output_tokens: int) -> ServeResult:
    required_bytes = output_tokens * BYTES_PER_TOKEN  # => co-06: this request's steady-state cache need
    if required_bytes > cache_budget_bytes:  # => co-07/co-18: cache is the gate, not compute
        return ServeResult(admitted=False, total_ms=0.0)  # => refused BEFORE any prefill/decode work starts
    prefill_ms = prompt_tokens * PREFILL_MS_PER_TOKEN  # => co-02
    decode_ms = output_tokens * DECODE_MS_PER_TOKEN  # => co-03
    return ServeResult(admitted=True, total_ms=prefill_ms + decode_ms)  # => co-02+co-03: the two phases summed


fits = serve_request(cache_budget_bytes=1 * 1024**3, prompt_tokens=200, output_tokens=100)  # => a modest request
too_big = serve_request(cache_budget_bytes=1 * 1024**3, prompt_tokens=200, output_tokens=50_000)  # => same budget
# => same function, same budget -- ONLY the output length differs between these two calls
print(fits.admitted, fits.total_ms)  # => Output: True 2100.0
print(too_big.admitted)  # => Output: False -- refused BEFORE latency was ever computed

assert fits.admitted is True  # => co-18: fits comfortably within a 1 GiB cache budget
assert too_big.admitted is False  # => co-07: 50,000 tokens of cache blows straight through the budget
# => this small pipeline is the seed the intermediate tier grows into: batching, scheduling, paging
print("ex-28 OK")  # => a self-check marker confirming the full pipeline -- gate, then latency -- held
