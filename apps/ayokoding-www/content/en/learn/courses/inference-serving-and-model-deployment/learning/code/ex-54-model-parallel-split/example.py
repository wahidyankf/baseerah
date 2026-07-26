"""Example 54: Model-Parallel Split. [GPU] illustrative reference numbers; simulator runs offline, deterministic."""


def shard_weights_bytes(total_weight_bytes: int, num_devices: int) -> int:
    # => co-20: tensor parallelism splits EACH weight matrix evenly across every device
    return total_weight_bytes // num_devices  # => exact split assumed for this illustration


def all_reduce_bytes_per_step(activation_bytes: int, num_devices: int) -> int:
    # => co-20: every device must exchange its partial activations with every OTHER device, once per layer
    return activation_bytes * (num_devices - 1) * 2  # => a simplified ring-all-reduce traffic estimate


total_weight_bytes = 14_000_000_000  # => a 7B-parameter model stored in fp16 (2 bytes/param)
shards = {n: shard_weights_bytes(total_weight_bytes, n) for n in (1, 2, 4, 8)}  # => co-20: one entry per device count
print(shards)  # => Output: {1: 14000000000, 2: 7000000000, 4: 3500000000, 8: 1750000000}

activation_bytes = 4_000_000  # => per-step activation volume that must be exchanged between devices
traffic = {n: all_reduce_bytes_per_step(activation_bytes, n) for n in (1, 2, 4, 8)}  # => co-20: traffic per device count
print(traffic)  # => Output: {1: 0, 2: 8000000, 4: 24000000, 8: 56000000}

assert shards[8] == total_weight_bytes // 8  # => co-20: each of 8 devices holds exactly one eighth of the weights
assert traffic[8] > traffic[2]  # => co-20: more devices means MORE interconnect traffic per step, not less
# => Example 69 measures what happens when this traffic outgrows the interconnect's real bandwidth
print("ex-54 OK")  # => a self-check marker confirming the shard-shrinks/traffic-grows tradeoff held
