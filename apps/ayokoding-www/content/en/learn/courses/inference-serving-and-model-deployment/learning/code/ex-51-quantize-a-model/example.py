"""Example 51: Quantize a Model."""


def quantize_int8(weights: list[float], scale: float) -> list[int]:  # => co-19: linear symmetric quantization
    return [round(w / scale) for w in weights]  # => each float mapped to its nearest INT8 code


def dequantize_int8(quantized: list[int], scale: float) -> list[float]:  # => co-19: reconstruct an APPROXIMATE float
    return [q * scale for q in quantized]  # => the inverse mapping -- approximate, never exact


weights = [0.12, -0.45, 0.98, -1.2, 0.03]  # => a tiny stand-in for one row of a real weight matrix
scale = max(abs(w) for w in weights) / 127  # => co-19: INT8 spans [-127, 127] -- scale maps the float range onto it
quantized = quantize_int8(weights, scale)  # => the compressed, integer-only representation
dequantized = dequantize_int8(quantized, scale)  # => reconstructed floats -- close, but NOT identical to the originals
print(quantized)  # => Output: [13, -48, 104, -127, 3]
print([round(d, 4) for d in dequantized])  # => Output: [0.1228, -0.4535, 0.9827, -1.2, 0.0283]

max_error = max(abs(w - d) for w, d in zip(weights, dequantized))  # => the worst single reconstruction error
print(round(max_error, 4))  # => Output: 0.0035

assert max_error < scale  # => co-19: quantization error is bounded by roughly half a scale step, always
# => real quantizers (GPTQ, AWQ) use calibration data instead of this toy min/max scale -- same idea, tighter fit
print("ex-51 OK")  # => a self-check marker confirming reconstruction error stayed within the expected bound
