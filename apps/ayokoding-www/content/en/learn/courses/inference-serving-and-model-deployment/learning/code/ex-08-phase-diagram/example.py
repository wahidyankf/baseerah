"""Example 8: Request Lifecycle -- Phase Diagram in Code."""

from dataclasses import dataclass, field  # => stdlib only -- tracing phases needs no framework

# => a trace object exists SOLELY to make an otherwise-invisible lifecycle observable in a test
# => real servers emit this same information as structured logs or tracing spans, not print()


@dataclass
class RequestTrace:  # => records which phases a single request actually passed through
    phases: list[str] = field(default_factory=list[str])  # => explicit generic keeps type-checking strict
    cache_written: bool = False  # => flips true the instant prefill writes the KV cache


def run_request(trace: RequestTrace, output_tokens: int) -> None:  # => simulates one request's full lifecycle
    trace.phases.append("prefill")  # => co-02: phase 1 -- process the whole prompt at once
    trace.cache_written = True  # => co-05: prefill WRITES the KV cache -- this is where it's populated
    for _ in range(output_tokens):  # => co-03: phase 2 -- one decode step per output token
        trace.phases.append("decode")  # => each step both READS and APPENDS to the cache (co-05)


trace = RequestTrace()  # => starts empty: no phases recorded, cache not yet written
run_request(trace, output_tokens=3)  # => exactly one prefill call, then three decode steps
print(trace.phases)  # => Output: ['prefill', 'decode', 'decode', 'decode']
print(trace.cache_written)  # => Output: True

assert trace.phases[0] == "prefill"  # => co-02: prefill always happens first, exactly once
# => this ordering is not incidental -- a decode step needs SOMETHING in the cache to read
assert trace.phases.count("decode") == 3  # => co-03: one decode step per requested output token
assert trace.cache_written is True  # => co-05: the cache exists because prefill wrote into it
print("ex-08 OK")  # => a self-check marker confirming every phase-ordering assertion held
