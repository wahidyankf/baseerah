"""Example 1: Serve a Model Locally."""


class TinyModel:  # => a stand-in for a real served model -- deterministic, no GPU or download needed
    def __init__(self) -> None:  # => sets up fixed, deterministic model state -- no randomness anywhere
        self.vocab: list[str] = ["the", "cat", "sat", "on", "a", "mat"]  # => fixed 6-token vocabulary
        self.weights_bytes: int = 2_000_000_000  # => co-24: weights are part of the deployed artefact

    def generate(self, prompt: str, max_tokens: int) -> list[str]:  # => the served model's ONE job
        prompt_words = prompt.split()  # => co-01: split on whitespace -- a stand-in for real tokenization
        # => a trivial DETERMINISTIC "generation": echo the prompt, then pad from a fixed vocabulary
        padding = (self.vocab * 2)[: max(0, max_tokens - len(prompt_words))]  # => repeats vocab so slicing never runs dry
        return (prompt_words + padding)[:max_tokens]  # => same input -> same output, every single time


def handle_completion_request(model: TinyModel, prompt: str, max_tokens: int) -> dict[str, object]:
    # => co-01/co-24: this function IS the served HTTP endpoint's handler, minus the socket
    prompt_tokens = len(prompt.split())  # => co-01: tokens counted, not characters and not requests
    completion = model.generate(prompt, max_tokens)  # => the actual served computation happens here
    return {  # => building the response payload the client actually receives
        "text": " ".join(completion),  # => what a client sees in the HTTP response body
        "prompt_tokens": prompt_tokens,  # => co-01: billed/measured in tokens, never bytes or requests
        "completion_tokens": len(completion),  # => co-01: the OTHER billed dimension -- output tokens
    }  # => end of the response dict


model = TinyModel()  # => "loading" the model -- a real server reads weights_bytes off disk right here
response = handle_completion_request(model, prompt="a cat", max_tokens=5)  # => collapses a client request to a call
print(response["text"])  # => Output: a cat the cat sat
print(response["prompt_tokens"], response["completion_tokens"])  # => Output: 2 5

assert response["text"] == "a cat the cat sat"  # => confirms deterministic generation
assert response["prompt_tokens"] == 2  # => "a cat" is two whitespace-split tokens
assert response["completion_tokens"] == 5  # => exactly max_tokens, as requested
print("ex-01 OK")  # => a self-check marker, confirming all three assertions above held
