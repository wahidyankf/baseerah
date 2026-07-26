"""Example 68: Tensor-Parallel vs Pipeline-Parallel."""


def tensor_parallel_traffic_per_step(activation_bytes: int, num_devices: int) -> int:
    # => co-20: TP exchanges activations on EVERY layer -- frequent, and it adds up fast
    num_layers = 32  # => a stand-in transformer depth, used only to size the traffic estimate
    return activation_bytes * (num_devices - 1) * num_layers  # => co-20: traffic scales with BOTH devices AND layers


def pipeline_parallel_traffic_per_step(activation_bytes: int, num_stages: int) -> int:
    # => co-20: PP exchanges activations only at STAGE BOUNDARIES -- rarer, but adds pipeline "bubble" idle time
    return activation_bytes * (num_stages - 1)  # => co-20: traffic scales with stages, NOT with layer count


activation_bytes = 4_000_000  # => per-layer-boundary activation volume that must be exchanged
tp_traffic = tensor_parallel_traffic_per_step(activation_bytes, num_devices=4)  # => co-20: the layer-by-layer cost
pp_traffic = pipeline_parallel_traffic_per_step(activation_bytes, num_stages=4)  # => co-20: the boundary-only cost
print(tp_traffic, pp_traffic)  # => Output: 384000000 12000000

assert tp_traffic > pp_traffic  # => co-20: TP moves FAR more data per step -- it needs a fast interconnect to pay off
# => Example 69 converts this traffic gap into actual wall-clock milliseconds on real links
print("ex-68 OK")  # => a self-check marker confirming TP's far higher per-step traffic held
