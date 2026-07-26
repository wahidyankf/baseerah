"""Example 18: Precision Formats and Bytes."""

BYTES_PER_VALUE = {"fp32": 4, "fp16": 2, "bf16": 2, "int8": 1}  # => co-18/co-19: precision sets byte width
# => the SAME parameter count means very different byte counts depending on which format is chosen


def weights_bytes(num_params: int, precision: str) -> int:  # => co-18: total weight bytes at a precision
    return num_params * BYTES_PER_VALUE[precision]  # => a lookup, then a multiply -- the whole formula


num_params = 7_000_000_000  # => a 7-billion-parameter model
fp16_bytes = weights_bytes(num_params, "fp16")  # => the common serving default
int8_bytes = weights_bytes(num_params, "int8")  # => same model, quarter-width integers
print(round(fp16_bytes / 1024**3, 2), round(int8_bytes / 1024**3, 2))  # => Output: 13.04 6.52

assert fp16_bytes == num_params * 2  # => co-18: 2 bytes per parameter at fp16
assert int8_bytes == fp16_bytes // 2  # => co-19 preview: halving byte width halves memory, exactly
print("ex-18 OK")  # => a self-check marker confirming both precision-to-byte conversions held
