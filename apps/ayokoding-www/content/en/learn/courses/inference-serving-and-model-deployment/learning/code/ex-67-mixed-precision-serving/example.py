"""Example 67: Mixed-Precision Serving."""


def blended_memory_gb(param_count: int, fp16_fraction: float, quantized_bits: int) -> float:
    # => co-19: keep a FRACTION of params at fp16 (typically attention/output layers) and quantize the rest
    fp16_params = param_count * fp16_fraction  # => the portion kept at full precision, for quality
    quantized_params = param_count * (1 - fp16_fraction)  # => the remainder, quantized down for memory
    fp16_bytes = fp16_params * 16 / 8  # => full-precision portion's byte cost
    quantized_bytes = quantized_params * quantized_bits / 8  # => quantized portion's byte cost
    return (fp16_bytes + quantized_bytes) / (1024**3)  # => the two portions summed, converted to GiB


param_count = 7_000_000_000  # => the SAME 7B parameter count used throughout the quantization examples
pure_int8 = blended_memory_gb(param_count, fp16_fraction=0.0, quantized_bits=8)  # => nothing kept at fp16
mixed_10pct_fp16 = blended_memory_gb(param_count, fp16_fraction=0.10, quantized_bits=8)  # => 10% kept for quality
pure_fp16 = blended_memory_gb(param_count, fp16_fraction=1.0, quantized_bits=8)  # => everything at full precision
print(round(pure_int8, 2), round(mixed_10pct_fp16, 2), round(pure_fp16, 2))  # => Output: 6.52 7.17 13.04

assert pure_int8 < mixed_10pct_fp16 < pure_fp16  # => co-19: mixed precision sits STRICTLY between the two pure extremes
# => production quantizers pick this fraction per-layer, not as one global knob, for the same reason
print("ex-67 OK")  # => a self-check marker confirming mixed precision lands strictly between the two extremes
