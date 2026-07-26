"""Example 11: Compute Cache Size."""


def kv_cache_bytes(
    num_layers: int,  # => depth of the transformer stack -- more layers, more K/V pairs to store
    num_heads: int,  # => attention heads within each layer -- one K/V pair PER head
    head_dim: int,  # => the size of ONE head's key or value vector
    seq_len: int,  # => tokens processed so far -- this is the dimension that GROWS during decode
    bytes_per_value: int,  # => precision width: 2 for fp16/bf16, 1 for int8, 4 for fp32
    batch_size: int = 1,  # => defaults to a single sequence -- scale up for concurrent requests
) -> int:  # => co-06: the formula every capacity decision in this topic reduces to
    # => 2x for K AND V, one value per (layer, head, dim, token), times batch and precision width
    return 2 * num_layers * num_heads * head_dim * seq_len * bytes_per_value * batch_size
    # => six multiplied factors -- change ANY one and the byte count moves proportionally


small_model_cache = kv_cache_bytes(  # => one worked configuration, used throughout this topic
    num_layers=24,  # => transformer depth -- one K/V pair stored PER layer
    num_heads=16,  # => attention heads -- one K/V pair stored PER head
    head_dim=64,  # => per-head dimensionality -- part of the "one value" unit above
    seq_len=2048,  # => co-06: cache size grows with EVERY token processed so far
    bytes_per_value=2,  # => fp16 == 2 bytes/value
)  # => closes the call -- five dimensions multiplied together produce one byte count
print(small_model_cache)  # => Output: 201326592
# => 192 MiB is the cost of ONE 2048-token sequence, before a single OTHER request is admitted
print(small_model_cache / (1024**2))  # => Output: 192.0 -- expressed in mebibytes

assert small_model_cache == 2 * 24 * 16 * 64 * 2048 * 2  # => the formula, spelled out, matches exactly
assert small_model_cache / (1024**2) == 192.0  # => 192 MiB of cache for ONE 2048-token sequence
print("ex-11 OK")  # => a self-check marker confirming the formula and its unit conversion held
