"""Example 25: Sequential Decode Cannot Parallelize Within a Request."""


def decode_sequentially(seed_token: str, steps: int) -> list[str]:  # => co-03: EACH step needs the prior token
    tokens = [seed_token]  # => the chain starts from exactly one known token
    for _ in range(steps):  # => co-03: steps run ONE AFTER ANOTHER -- nothing here could run concurrently
        next_token = tokens[-1] + "'"  # => a deterministic stand-in: each new token depends on the LAST one
        tokens.append(next_token)  # => the chain only ever grows by reading its OWN most recent entry
    return tokens  # => the full ordered chain, proof that every step depended on its predecessor


correct = decode_sequentially("a", 3)  # => three sequential steps, each depending on the one before
print(correct)  # => Output: ["a", "a'", "a''", "a'''"]

# => now show the dependency is REAL: change what "the last token" resolves to partway through
tokens = ["a"]
tokens.append(tokens[-1] + "'")  # => step 1 depends on tokens[-1] AT THAT MOMENT ("a")
tokens[1] = "b"  # => simulate the model choosing a DIFFERENT actual token than expected at step 1
tokens.append(tokens[-1] + "'")  # => step 2 uses the NEW tokens[-1] == "b", not the original "a'"
print(tokens)  # => Output: ['a', 'b', "b'"]

assert correct == ["a", "a'", "a''", "a'''"]  # => the undisturbed sequential chain
assert tokens[-1] == "b'"  # => co-03: step 2 truly depends on step 1's ACTUAL result, not a precomputed guess
# => this is precisely why decode cannot be sped up by simply throwing more compute at one request
print("ex-25 OK")  # => a self-check marker confirming decode's step-to-step dependency is real, not cosmetic
