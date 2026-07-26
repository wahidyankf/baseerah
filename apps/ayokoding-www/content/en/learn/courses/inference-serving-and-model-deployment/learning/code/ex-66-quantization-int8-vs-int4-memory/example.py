"""Example 66: Quantization INT8 vs INT4 Memory."""


def model_memory_gb(param_count: int, bits_per_param: int) -> float:  # => co-19: total storage for ALL parameters
    return param_count * bits_per_param / 8 / (1024**3)  # => params times bits, converted bits->bytes->GiB


param_count = 7_000_000_000  # => the SAME 7B parameter count used throughout the quantization examples
memory_by_precision = {bits: round(model_memory_gb(param_count, bits), 2) for bits in (16, 8, 4)}  # => co-19: three points
print(memory_by_precision)  # => Output: {16: 13.04, 8: 6.52, 4: 3.26}

savings_int8_vs_fp16 = memory_by_precision[16] - memory_by_precision[8]  # => co-19: the middle-ground precision's savings
savings_int4_vs_fp16 = memory_by_precision[16] - memory_by_precision[4]  # => co-19: the aggressive precision's savings
print(round(savings_int8_vs_fp16, 2), round(savings_int4_vs_fp16, 2))  # => Output: 6.52 9.78

assert savings_int4_vs_fp16 > savings_int8_vs_fp16  # => co-19: INT4 saves MORE than INT8, at a steeper quality cost
# => Example 67 shows a middle path -- blending precisions instead of picking one for the whole model
print("ex-66 OK")  # => a self-check marker confirming INT4's larger memory saving held over INT8
