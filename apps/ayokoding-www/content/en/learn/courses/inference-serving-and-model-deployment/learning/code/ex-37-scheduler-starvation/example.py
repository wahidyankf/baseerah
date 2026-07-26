"""Example 37: Scheduler Starvation."""


def strict_priority_admit_order(queue: list[tuple[str, int]], slots: int, rounds: int) -> list[str]:
    # => co-13: STRICT priority -- the highest-priority items always win a free slot, no exceptions
    served: list[str] = []  # => accumulates who actually got served, round by round
    remaining = sorted(queue, key=lambda item: -item[1])  # => always sorted by priority, high to low
    for _ in range(rounds):  # => co-13: the SAME top entries win, round after round, forever
        for rid, _priority in remaining[:slots]:  # => co-13: only the TOP `slots` entries ever run
            served.append(rid)  # => low_priority never appears in this slice, so it never appears here
    return served  # => a record of who was served -- low priority may never appear at all


def fair_share_admit_order(queue: list[tuple[str, int]], slots: int, rounds: int) -> list[str]:
    # => co-13: round-robin FIX -- every request gets a turn, priority only breaks ties within a round
    served: list[str] = []  # => accumulates who actually got served, round by round
    idx = 0  # => a rotating pointer -- guarantees EVERY queue entry eventually comes back around
    for _ in range(rounds):  # => co-13: unlike strict priority, EVERY round advances the rotation
        for _ in range(slots):  # => fills each of the available slots via the rotating pointer
            served.append(queue[idx % len(queue)][0])  # => co-13: cycles through EVERYONE, low-priority included
            idx += 1  # => advances the rotation -- never re-checks priority to decide who's next
    return served  # => a record of who was served -- rotation guarantees nobody is skipped forever


queue = [("urgent", 10), ("urgent2", 9), ("low_priority", 1)]  # => low_priority never wins strict priority
strict = strict_priority_admit_order(queue, slots=2, rounds=5)  # => 5 rounds, only the top 2 EVER served
fair = fair_share_admit_order(queue, slots=2, rounds=5)  # => same queue, same rounds, rotation instead
print("low_priority" in strict)  # => Output: False -- STARVED across all 5 rounds
print("low_priority" in fair)  # => Output: True -- the round-robin fix guarantees it a turn
# => same queue, same slot count, same round count -- the POLICY alone decides who gets served

assert "low_priority" not in strict  # => co-13: strict priority starves it completely
assert "low_priority" in fair  # => co-13: fairness fix guarantees forward progress for every request
print("ex-37 OK")  # => a self-check marker confirming starvation happened under one policy, not the other
