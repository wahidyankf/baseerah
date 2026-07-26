"""Example 49: Fairness Across Request Classes."""


def weighted_round_robin_order(class_weights: dict[str, int], rounds: int) -> list[str]:
    # => co-13: each class gets `weight` slots per round -- proportional, not all-or-nothing
    order: list[str] = []  # => accumulates the full schedule across every round
    for _ in range(rounds):  # => co-13: EVERY round repeats the SAME proportional split
        for cls, weight in class_weights.items():  # => co-13: EVERY class appears in EVERY round
            order.extend([cls] * weight)  # => this class gets exactly its configured weight of slots
    return order  # => the full schedule -- inspectable for class-by-class fairness


weights = {"interactive": 3, "batch": 1}  # => interactive gets 3x the slots, but batch is NEVER zero
schedule = weighted_round_robin_order(weights, rounds=4)  # => 4 rounds of the same proportional split
interactive_count = schedule.count("interactive")  # => total interactive slots across all 4 rounds
batch_count = schedule.count("batch")  # => total batch slots across all 4 rounds
print(interactive_count, batch_count)  # => Output: 12 4

assert interactive_count == batch_count * 3  # => co-13: exactly the configured 3:1 weighting, every round
assert batch_count > 0  # => co-13: unlike strict priority (Example 37), "batch" is NEVER fully starved
# => weighted round robin buys bounded wait for low-priority traffic, at a small throughput cost
print("ex-49 OK")  # => a self-check marker confirming the weighted split held exactly across all rounds
