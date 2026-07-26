"""Example 36: Scheduling Policy Affects Tails."""


def fcfs_wait_times(arrival_order: list[tuple[str, int]]) -> dict[str, int]:  # => co-13: first come, first served
    waits: dict[str, int] = {}  # => records how long each request waited before its turn started
    clock = 0  # => the simulated clock -- advances by each job's cost as it is processed
    for rid, cost in arrival_order:  # => processed strictly in arrival order, regardless of cost
        waits[rid] = clock  # => this request starts exactly when the clock currently reads
        clock += cost  # => the NEXT request must wait for this one's full cost first
    return waits  # => one wait time per request, under a strict arrival-order policy


def shortest_first_wait_times(arrival_order: list[tuple[str, int]]) -> dict[str, int]:  # => co-13: SRPT-style
    remaining = sorted(arrival_order, key=lambda item: item[1])  # => cheapest request processed FIRST
    waits: dict[str, int] = {}  # => same bookkeeping as fcfs_wait_times, different processing order
    clock = 0  # => resets to zero -- this is an independent simulation, not a continuation of fcfs
    for rid, cost in remaining:  # => now iterating in COST order, not arrival order
        waits[rid] = clock  # => same recording logic as fcfs_wait_times, applied to the new order
        clock += cost  # => same clock-advance logic as fcfs_wait_times, applied to the new order
    return waits  # => one wait time per request, under a cheapest-first policy


jobs = [("big", 100), ("small1", 1), ("small2", 1), ("small3", 1)]  # => one big job, three tiny ones
# => the SAME four jobs are fed to both scheduling functions below -- only the ORDER differs
fcfs = fcfs_wait_times(jobs)  # => processes "big" first, purely because it arrived first
srpt = shortest_first_wait_times(jobs)  # => processes all three tiny jobs before touching "big"
print(fcfs["small3"], srpt["small3"])  # => Output: 102 2 -- a 50x difference in wait, same three jobs

assert fcfs["small3"] == 102  # => co-13: FCFS makes every tiny job wait behind the big one first
assert srpt["small3"] < fcfs["small3"]  # => co-13: reordering by cost sharply cuts the SMALL jobs' tail wait
print("ex-36 OK")  # => a self-check marker confirming the scheduling-policy wait-time gap held
