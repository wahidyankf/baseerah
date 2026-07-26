"""Worked Example 18: A Sampling Frame Mismatch."""  # => co-07: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import random  # => co-07: builds the simulated traffic log and draws the frame-based sample

N_TRAFFIC = 3000  # => co-07: total real requests a system actually received
TIMEOUT_RATE = 0.08  # => co-07: fraction of real requests that time out -- these NEVER complete, and never write a log line
PASS_RATE_AMONG_COMPLETED = 0.85  # => co-07: among requests that DO complete, the fraction judged a good answer
FRAME_SAMPLE_SIZE = 80  # => co-07: cases drawn from the (incomplete) sampling frame


if __name__ == "__main__":  # => co-07: entry point -- runs only when this file executes directly, not on import
    rng = random.Random(6)  # => co-07: builds the fixed simulated traffic this whole example draws from
    all_traffic: list[tuple[str, bool]] = []  # => co-07: every REAL request, whether or not it ever reaches the completion log
    for _ in range(N_TRAFFIC):  # => co-07: simulate the full real traffic stream, timeouts included
        if rng.random() < TIMEOUT_RATE:  # => co-07: this request times out -- the user got no answer at all
            all_traffic.append(("timeout", False))  # => co-07: counts as a failed interaction from the user's point of view, but writes NO log line
        else:  # => co-07: this request completes and gets scored normally
            passed = rng.random() < PASS_RATE_AMONG_COMPLETED  # => co-07: whether the completed answer was judged good
            all_traffic.append(("completed", passed))  # => co-07: this request DOES appear in the completion log

    true_rate_all_traffic = sum(1 for _, passed in all_traffic if passed) / len(all_traffic)  # => co-07: the honest question -- "does a real user get a good answer" -- timeouts count as failures
    print(f"True pass rate over ALL real traffic (timeouts count as failures): {true_rate_all_traffic:.4f}")  # => co-07

    log_frame = [passed for kind, passed in all_traffic if kind == "completed"]  # => co-07: the SAMPLING FRAME -- what the completion log actually contains
    print(f"Completion-log frame size: {len(log_frame)} of {N_TRAFFIC} total requests ({len(log_frame) / N_TRAFFIC:.1%})")  # => co-07: the frame is SMALLER than the real population, by construction

    frame_sample = random.Random(95).sample(log_frame, FRAME_SAMPLE_SIZE)  # => co-07: an HONEST random sample -- but drawn from the wrong frame
    frame_estimate = sum(frame_sample) / FRAME_SAMPLE_SIZE  # => co-07: this sample's resulting estimate
    print(f"Estimate from sampling the completion log: {frame_estimate:.4f}")  # => co-07: looks like a normal, well-sampled number

    gap = frame_estimate - true_rate_all_traffic  # => co-07: the systematic gap this mismatch introduces -- NOT sampling error, a different question entirely
    print(f"Gap between the frame estimate and the true all-traffic rate: {gap:.4f}")  # => co-07
    assert gap > 0.05, "sampling only the completion log must overstate the true all-traffic pass rate by a material margin"  # => co-07: the mismatch claim itself
    print("MATCH: a perfectly honest random sample of the WRONG frame still answers the WRONG question")  # => co-07
    # => co-07: 'we sampled at random' is necessary but not sufficient -- the sampling frame itself must match the population the decision is actually about
