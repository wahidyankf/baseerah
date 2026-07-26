"""Example 19: Weights Memory Arithmetic."""


def weights_gib(num_params: int, bytes_per_param: int) -> float:  # => co-18: weights are the FIRST budget line
    return (num_params * bytes_per_param) / 1024**3  # => bytes converted straight to GiB for readability


seven_b_fp16 = weights_gib(7_000_000_000, 2)  # => a mid-size model at the common serving precision
thirteen_b_fp16 = weights_gib(13_000_000_000, 2)  # => nearly double the parameters, same precision
print(round(seven_b_fp16, 1), round(thirteen_b_fp16, 1))  # => Output: 13.0 24.2

gpu_total_gib = 24.0  # => a common consumer/workstation GPU size
remaining_after_7b = gpu_total_gib - seven_b_fp16  # => co-18: weights are ALWAYS subtracted FIRST
remaining_after_13b = gpu_total_gib - thirteen_b_fp16  # => same subtraction, bigger model
print(round(remaining_after_7b, 1), round(remaining_after_13b, 1))  # => Output: 11.0 -0.2

assert remaining_after_7b > 0  # => co-18: the 7B model leaves headroom for cache and activations
assert remaining_after_13b < 0  # => co-18: the 13B model does NOT fit at all at fp16 on this GPU
# => quantizing to int8 (Example 18) would roughly halve this weight footprint
print("ex-19 OK")  # => a self-check marker confirming one model fits and the other does not
