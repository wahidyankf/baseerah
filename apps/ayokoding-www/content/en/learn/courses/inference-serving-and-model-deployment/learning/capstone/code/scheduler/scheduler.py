"""Capstone step 2 -- scheduler/: continuous batching over a paged cache with prefix sharing, plus an
admission and scheduling policy with preemption under pressure.

Reuses the mechanics taught across Examples 29-46 -- static/continuous batching, paged allocation,
prefix sharing, and preemption -- assembled into one scheduler comparable against a contiguous,
static-batching baseline.
"""

import math
from dataclasses import dataclass, field

BLOCK_TOKENS = 16  # => co-09: fixed block size, same principle as OS virtual-memory paging


@dataclass
class SimRequest:
    id: str
    prompt_prefix_tokens: int  # => co-10: the SHARED portion, if any, with other requests
    output_tokens: int
    tokens_emitted: int = 0

    @property
    def finished(self) -> bool:
        return self.tokens_emitted >= self.output_tokens


def blocks_needed(token_len: int) -> int:  # => co-09: round UP to the nearest whole block
    return math.ceil(token_len / BLOCK_TOKENS) if token_len > 0 else 0


def paged_cache_bytes(requests: list[SimRequest], bytes_per_token: int) -> int:
    # => co-08/co-09/co-10: paged allocation WITH prefix sharing -- the shared prefix's blocks count ONCE
    shared_prefixes = {r.prompt_prefix_tokens for r in requests if r.prompt_prefix_tokens > 0}
    shared_blocks = sum(blocks_needed(p) for p in shared_prefixes)  # => co-10: paid for ONCE, not per-request
    unique_blocks = sum(blocks_needed(r.output_tokens) for r in requests)  # => co-09: each request's own output
    return (shared_blocks + unique_blocks) * BLOCK_TOKENS * bytes_per_token


def contiguous_cache_bytes(requests: list[SimRequest], bytes_per_token: int, max_seq_len: int) -> int:
    # => co-08: the naive baseline -- reserves the WORST-CASE length, contiguously, per request, no sharing
    return len(requests) * max_seq_len * bytes_per_token


@dataclass
class ContinuousBatchScheduler:  # => co-12/co-13/co-14: admission, continuous batching, preemption
    max_batch_slots: int
    cache_budget_blocks: int
    active: list[SimRequest] = field(default_factory=list[SimRequest])
    queued: list[SimRequest] = field(default_factory=list[SimRequest])
    preemption_count: int = 0
    occupancy_trace: list[int] = field(default_factory=list[int])

    def _blocks_in_use(self) -> int:
        return sum(blocks_needed(r.output_tokens) for r in self.active)

    def submit(self, request: SimRequest) -> None:
        self.queued.append(request)  # => co-13: every arrival is queued first, admitted only when room exists

    def _admit_from_queue(self) -> None:
        # => co-13/co-14: bound preemption attempts to the active set's own size -- within ONE scheduling
        #    tick, a well-behaved scheduler preempts each currently-active request at most once, rather
        #    than cycling indefinitely when demand structurally exceeds the cache budget (see Example 46's
        #    thrashing failure mode, which this bound is specifically designed to avoid triggering here).
        preemption_budget = len(self.active)
        while len(self.active) < self.max_batch_slots and self.queued:
            candidate = self.queued[0]
            if self._blocks_in_use() + blocks_needed(candidate.output_tokens) <= self.cache_budget_blocks:
                self.active.append(self.queued.pop(0))
                continue
            if not self.active or preemption_budget <= 0:
                break  # => co-13: no room, and no further preemption budget this tick -- candidate stays queued
            # => co-14: under pressure, preempt the LARGEST cache holder to try to make room
            victim = max(self.active, key=lambda r: r.tokens_emitted)
            victim.tokens_emitted = 0
            self.active.remove(victim)
            self.queued.append(victim)
            self.preemption_count += 1
            preemption_budget -= 1

    def step(self) -> None:
        self._admit_from_queue()
        for r in self.active:
            r.tokens_emitted += 1  # => co-12: every active request takes one decode step
        self.occupancy_trace.append(len(self.active))
        self.active = [r for r in self.active if not r.finished]  # => co-12: retire IMMEDIATELY, not at batch end

    def run_to_completion(self) -> int:
        steps = 0
        while self.active or self.queued:
            self.step()
            steps += 1
        return steps
