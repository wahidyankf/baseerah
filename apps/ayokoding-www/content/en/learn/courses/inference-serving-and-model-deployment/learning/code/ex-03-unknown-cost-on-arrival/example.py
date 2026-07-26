"""Example 3: Unknown Cost on Arrival."""

from dataclasses import dataclass  # => stdlib only -- no external dependency needed for this shape

# => a request is a piece of DATA the server receives, distinct from the WORK it will cause


@dataclass
class Request:  # => what the server sees the MOMENT a request arrives
    prompt: str  # => the input text -- fully known at arrival time
    max_tokens: int  # => an UPPER bound the client sets -- not the actual length the model will emit


def simulate_actual_output_length(prompt: str, max_tokens: int) -> int:
    # => co-21: the TRUE output length depends on when the model emits an end-of-sequence token,
    # => modeled here as prompt-dependent -- NOT knowable from the request alone at admission time
    if "explain" in prompt.lower():  # => an open-ended prompt tends to run long
        return min(max_tokens, 500)  # => still capped by the client's own upper bound
    return min(max_tokens, 10)  # => a closed-ended prompt tends to stop early


request_a = Request(prompt="Summarize: cats are mammals.", max_tokens=500)  # => looks unremarkable
request_b = Request(prompt="Explain photosynthesis in depth.", max_tokens=500)  # => looks the same too

actual_a = simulate_actual_output_length(request_a.prompt, request_a.max_tokens)  # => resolved AFTER the fact
actual_b = simulate_actual_output_length(request_b.prompt, request_b.max_tokens)  # => same function, same shape
print(request_a.max_tokens, request_b.max_tokens)  # => Output: 500 500 -- identical UPPER bound
print(actual_a, actual_b)  # => Output: 10 500 -- wildly different ACTUAL cost

assert request_a.max_tokens == request_b.max_tokens  # => the request alone gives no hint
assert actual_a != actual_b  # => co-21: true cost is only known AFTER generation finishes
print("ex-03 OK")  # => a self-check marker confirming both assertions held
