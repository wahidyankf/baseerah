"""Example 22: Output Length Is a Random Variable."""

OUTPUT_LENGTHS = [12, 340, 8, 490, 15, 22, 501, 9]  # => co-21: a small SAMPLE of real observed output lengths
# => same request TYPE (a chat completion), wildly different actual lengths -- that's the randomness


def summarize(lengths: list[int]) -> dict[str, float]:  # => co-01/co-21: a fixed request count hides this spread
    return {"mean": sum(lengths) / len(lengths), "min": float(min(lengths)), "max": float(max(lengths))}
    # => three summary statistics, all derived from the SAME underlying sample


stats = summarize(OUTPUT_LENGTHS)  # => one call, three numbers back
print(len(OUTPUT_LENGTHS))  # => Output: 8 -- eight "identical" requests, by request count
print(stats["min"], stats["max"], round(stats["mean"], 1))  # => Output: 8.0 501.0 174.6

assert stats["max"] / stats["min"] > 50  # => co-21: over 50x spread between the cheapest and priciest request
assert stats["mean"] < stats["max"] / 2  # => the mean UNDERSTATES how expensive the worst requests are
print("ex-22 OK")  # => a self-check marker confirming the spread and mean-understatement both held
