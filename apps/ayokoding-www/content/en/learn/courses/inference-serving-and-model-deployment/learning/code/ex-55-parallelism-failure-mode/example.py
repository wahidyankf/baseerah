"""Example 55: Parallelism Failure Mode."""


def forward_pass_with_device_failure(num_devices: int, failed_device: int | None) -> str:
    # => co-20: tensor parallelism has NO redundancy -- every device must complete for the result to be valid
    if failed_device is not None and 0 <= failed_device < num_devices:  # => a valid device index actually failed
        return "request_failed: partial result on other devices is unrecoverable, must restart from scratch"
    return "request_succeeded: all devices completed their shard"  # => no failure -- every shard finished normally


healthy = forward_pass_with_device_failure(4, None)  # => the baseline: no failure anywhere
one_failed = forward_pass_with_device_failure(4, failed_device=2)  # => a SINGLE device out of 4 fails
# => contrast a single-instance deployment: a device failure there costs ONE request, not the fleet
print(healthy)  # => Output: request_succeeded: all devices completed their shard
print(one_failed)  # => Output: request_failed: partial result on other devices is unrecoverable, must restart from scratch

assert healthy.startswith("request_succeeded")  # => the no-failure baseline succeeds, as expected
# => co-20: this all-or-nothing coupling is the direct cost of splitting ONE request across devices
assert one_failed.startswith("request_failed")  # => co-20: a SINGLE device failure fails the WHOLE request
print("ex-55 OK")  # => a self-check marker confirming tensor parallelism's all-or-nothing failure mode held
