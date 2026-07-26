---
title: "Overview"
date: 2026-07-26T00:00:00+07:00
draft: false
weight: 1
---

This page is the spaced-repetition companion to the Inference Serving & Model Deployment topic:
recall first, then applied judgment, then a hands-on kata, then a self-check checklist, then
elaborative-interrogation prompts that ask _why_, not just _what_. Every answer is hidden in a
`<details>` block; try each item yourself before opening it.

Every kata below is self-contained and deterministic -- hand-constructed fixtures only, no GPU, no
downloaded model weights, and no dependency on this topic's own worked-example or capstone code.

## Recall Q&A

Twenty-eight short-answer questions, one per concept (`co-01` through `co-28`). Answer from memory,
then check.

**Q1 (co-01 -- inference is not ordinary request/response).** Name the three properties that make
serving an LLM different from an ordinary web request/response cycle.

<details>
<summary>Answer</summary>

The unit of work is a token, not a request; the request's total cost is unknown when it arrives (you
cannot know output length in advance); and the dominant resource is memory held for the duration of
the generation, not CPU consumed at the start.

</details>

**Q2 (co-02 -- prefill phase).** Why can every prompt token be processed in parallel during prefill?

<details>
<summary>Answer</summary>

Because the model already has the entire prompt in hand before generation starts -- there is no
sequential dependency between prompt tokens the way there is between generated tokens, so prefill is
a parallel, compute-bound pass.

</details>

**Q3 (co-03 -- decode phase).** Why can decode never be parallelized within a single request, no
matter how much compute is available?

<details>
<summary>Answer</summary>

Because each new token depends on every token generated before it -- token N+1 cannot be computed
until token N exists. This sequential dependency is structural, not a hardware limitation.

</details>

**Q4 (co-04 -- why decode is bandwidth-bound).** Why does adding more GPU compute not speed up
decode?

<details>
<summary>Answer</summary>

Each decoded token requires reading the full weight set (and the growing cache) from GPU memory, so
decode throughput tracks memory bandwidth, not arithmetic throughput. A bandwidth-bound operation
gets no benefit from more FLOPs.

</details>

**Q5 (co-05 -- KV-cache purpose).** What problem does caching keys and values solve?

<details>
<summary>Answer</summary>

Without a cache, generating the Nth token would require reprocessing all N-1 prior tokens' attention
every single step, scaling quadratically with sequence length. Caching avoids that recomputation
entirely.

</details>

**Q6 (co-06 -- KV-cache size arithmetic).** What four quantities does cache size scale with?

<details>
<summary>Answer</summary>

Sequence length, batch size, number of layers, and number of attention heads (along with the
precision's bytes-per-value). Computing this precisely is the prerequisite to every later capacity
decision.

</details>

**Q7 (co-07 -- KV-cache is the scarce resource).** Why can a GPU be "full" while its compute sits
idle?

<details>
<summary>Answer</summary>

Because available cache memory, not compute, usually sets the maximum number of concurrent requests
a GPU can serve -- once cache is exhausted, no more requests can be admitted even if the GPU's
arithmetic units have spare capacity.

</details>

**Q8 (co-08 -- cache fragmentation).** What causes a contiguous cache allocator to strand capacity?

<details>
<summary>Answer</summary>

Variable and unpredictable generation lengths force a contiguous allocator to reserve the worst-case
length for every request up front -- most requests finish well short of that reservation, and the
unused remainder cannot be reclaimed for anyone else.

</details>

**Q9 (co-09 -- paged cache allocation).** What OS concept does paged cache allocation borrow, and
what does it fix?

<details>
<summary>Answer</summary>

Operating-system virtual-memory paging -- allocating fixed-size blocks instead of contiguous regions.
It fixes co-08's fragmentation by bounding waste to at most one partially-full block per request,
instead of the whole worst-case-minus-actual gap.

</details>

**Q10 (co-10 -- prefix sharing).** Why does prefix sharing save nothing on two prompts with no
common prefix?

<details>
<summary>Answer</summary>

Prefix sharing only deduplicates the literal, token-for-token identical portion at the start of two
sequences -- if that shared prefix has zero length, there is nothing to deduplicate, and both
requests need their own full, independent cache blocks.

</details>

**Q11 (co-11 -- static batching wastes).** What happens to a finished request's slot under static
batching?

<details>
<summary>Answer</summary>

It stays reserved and idle until every other request in the batch also finishes -- static batching
waits for the whole group, so an early-finishing request's slot is wasted for the remainder of the
batch's duration.

</details>

**Q12 (co-12 -- continuous batching).** What does "token granularity" mean for admission and
retirement?

<details>
<summary>Answer</summary>

Requests are admitted into a free slot and retired out of it the instant they finish, checked at
every single decode step -- not once per whole batch. This keeps the batch close to full throughout
instead of idling on early finishers.

</details>

**Q13 (co-13 -- scheduling and admission control).** Why can the identical set of requests see wildly
different tail latencies under different scheduling policies?

<details>
<summary>Answer</summary>

Because the scheduling policy decides the ORDER requests are served in, not the requests' own cost --
first-come-first-served can make many cheap requests wait behind one expensive one, while a
shortest-first policy processes cheap requests first, producing a very different tail with zero
change to the underlying work.

</details>

**Q14 (co-14 -- preemption and recompute).** What happens to a preempted request's progress?

<details>
<summary>Answer</summary>

It is discarded, not paused -- the request's cache is evicted and its generation must be recomputed
from scratch later. Preemption trades that one request's latency for the scheduler's ability to admit
other work right now.

</details>

**Q15 (co-15 -- throughput vs latency).** Why does no batch-size configuration optimize both
throughput and latency at once?

<details>
<summary>Answer</summary>

Because they move in the same direction from the same lever -- a larger batch raises aggregate
throughput and simultaneously raises each individual request's inter-token latency. Every
configuration is a deliberate point on that one trade-off, never a free win on both axes.

</details>

**Q16 (co-16 -- serving-latency vocabulary).** Name all four latency metrics this topic distinguishes.

<details>
<summary>Answer</summary>

Time-to-first-token (TTFT), inter-token latency (ITL), per-user tokens per second, and aggregate
throughput -- four distinct numbers, each optimized by opposing serving choices.

</details>

**Q17 (co-17 -- TTFT vs ITL tradeoff).** What is the cost of prioritizing a new request's prefill?

<details>
<summary>Answer</summary>

Every in-flight request's decode step must wait for that prefill to finish, worsening their ITL by
roughly the full prefill duration -- the new request's TTFT improves at every in-flight request's
expense.

</details>

**Q18 (co-18 -- GPU memory budget).** What four consumers must fit in one GPU's memory?

<details>
<summary>Answer</summary>

Weights, KV cache, activations, and framework overhead. The remainder after weights, activations, and
overhead is what actually buys concurrency via the cache-size arithmetic (co-06).

</details>

**Q19 (co-19 -- quantization tradeoff).** Why must a quantization quality claim be measured on your
own workload rather than trusted from a published number?

<details>
<summary>Answer</summary>

Because the quality cost is task-dependent -- a published degradation figure on one benchmark does
not transfer reliably to a different task or workload. The trade (lower precision buys memory and
speed at a quality cost) is durable; the specific measured cost is not.

</details>

**Q20 (co-20 -- model-parallelism basics).** What does tensor parallelism cost in exchange for
letting a too-large model fit on multiple devices?

<details>
<summary>Answer</summary>

Per-layer interconnect traffic between devices (which can dominate decode-step latency on a slow
link) and all fault tolerance -- a single device's failure fails the entire request, since there is
no redundancy.

</details>

**Q21 (co-21 -- capacity planning for token workloads).** Why is a single "average request" number
insufficient for capacity planning?

<details>
<summary>Answer</summary>

Because real traffic is a distribution with a long tail -- a handful of very long generations can
dominate resource consumption far more than their share of request count suggests. Planning against
the average hides exactly that tail.

</details>

**Q22 (co-22 -- load-testing a token service).** What happens when a load test substitutes a uniform
average length for the real length distribution?

<details>
<summary>Answer</summary>

It measures a workload that does not exist -- the resulting throughput or capacity number can
meaningfully understate (or overstate) the real wall-clock cost, because the real distribution's mix
of short and long requests batches differently than a uniform one.

</details>

**Q23 (co-23 -- autoscaling a GPU service).** Why can't a GPU autoscaling policy simply copy a
stateless CPU service's reactive playbook?

<details>
<summary>Answer</summary>

Because loading multi-gigabyte weights takes seconds, not milliseconds -- a policy that waits until
the queue is already deep before scaling out will still be mid-cold-start by the time the queue has
grown well past the threshold. A GPU-aware policy must project queue growth across the cold start.

</details>

**Q24 (co-24 -- deployment packaging).** What three things does a deployment manifest bundle as one
unit?

<details>
<summary>Answer</summary>

Weights, runtime, and configuration -- versioned together so the served artifact is a single,
identifiable, reproducible unit, not three independently-changing pieces.

</details>

**Q25 (co-25 -- rollout and rollback of a model).** Why does changing the served model need the same
discipline as any other production release?

<details>
<summary>Answer</summary>

Because it has quality consequences just like any code change -- a staged rollout with a guardrail
bounds the blast radius of a bad model to a small fraction of traffic, and a rollback rule reverses it
automatically the moment a regression is detected, rather than propagating a bad model to 100% of
traffic on a single all-or-nothing swap.

</details>

**Q26 (co-26 -- observability for serving).** Name the five signals that together usually explain a
serving problem.

<details>
<summary>Answer</summary>

Queue depth, batch occupancy, cache utilization, preemption rate, and the latency metrics from co-16
(especially ITL). Read together, this small set is usually enough to diagnose an incident without
inspecting individual request traces.

</details>

**Q27 (co-27 -- self-hosting decision).** What does self-hosting win on, and what does it lose on,
compared to a hosted API?

<details>
<summary>Answer</summary>

It wins on sustained high utilization, data residency, latency floors, and model control; it loses on
operational burden, idle cost, and elasticity. The right decision depends entirely on which side of
that trade your traffic and organization sit on.

</details>

**Q28 (co-28 -- total cost of ownership).** What does an honest self-hosting-versus-buy comparison
include that a naive "hourly GPU price vs. per-token API price at full load" comparison omits?

<details>
<summary>Answer</summary>

Idle hours, engineering time, on-call burden, and -- most importantly -- the utilization you will
REALISTICALLY achieve, not full load. The identical GPU at the identical hourly rate can be cheaper or
far more expensive than the API purely as a function of that realistic utilization.

</details>

## Applied problems

Eight scenarios spanning the whole topic. Each describes a realistic situation without naming the
specific concept -- decide what applies and why, then check your reasoning against the worked
solution. Every scenario below is invented for this drill and does not reuse any function, fixture,
or domain from the 75 worked examples or the capstone.

**AP1.** A team benchmarks their new serving stack by sending 100 identical, short "hello" prompts
and reports the resulting throughput number as "our production capacity." Three weeks after launch,
the service falls over under real traffic well before that number is reached. What went wrong?

<details>
<summary>Worked solution</summary>

The load test used a uniform, unrealistically cheap workload instead of the real prompt/output length
distribution (co-21, co-22) -- 100 identical short prompts batch far more efficiently than a realistic
mix containing occasional long generations. The reported number describes a workload that does not
exist in production; the fix is to characterize the real length distribution and load-test against
it directly.

</details>

**AP2.** An engineer notices their service's GPU utilization dashboard shows 40% compute utilization
even though the service is refusing new connections with "at capacity" errors. They conclude the
capacity limit must be a bug, since the GPU is clearly not maxed out. Are they right?

<details>
<summary>Worked solution</summary>

No -- this is exactly co-07's point: available cache memory, not compute, usually sets the
concurrency ceiling. A GPU can be compute-idle at 40% while completely full on KV cache, refusing new
admissions correctly. The "bug" is a wrong mental model, not a real defect; the fix is to dashboard
cache utilization alongside compute utilization (co-26).

</details>

**AP3.** A team migrates from a naive request-per-process server to a continuous-batching engine and
reports a 3x throughput improvement, attributed vaguely to "the new framework being faster." A
skeptical reviewer asks for the specific mechanism. What's the actual, attributable cause?

<details>
<summary>Worked solution</summary>

The specific mechanism is co-11/co-12: the old server idled every finished request's slot until the
whole batch completed, while the new engine admits and retires at token granularity, eliminating that
idle-slot waste. "The new framework is faster" is not an explanation; "continuous batching eliminates
static batching's idle-slot waste" is -- and it is directly measurable by comparing slot-steps used
versus slot-steps wasted on the identical workload.

</details>

**AP4.** A model swap is deployed to 100% of traffic in one shot because the error rate looked
perfectly clean in a quick manual smoke test. A week later, users report the new version answers
correctly but noticeably more slowly, and nobody notices for days because no dashboard tracks it. What
two things should have been in place before this deploy?

<details>
<summary>Worked solution</summary>

A staged rollout with BOTH an error-rate and a latency guardrail (co-25) -- a model swap is a
production change requiring the same staged discipline as any release, and error rate alone (which
stayed clean here) is not sufficient; and serving observability tracking ITL/p99 latency continuously
(co-26), so a slow-but-correct regression is visible immediately rather than discovered days later
through user reports.

</details>

**AP5.** An operator compares self-hosting against a hosted API using the GPU's advertised hourly
rate and the model's peak, fully-loaded throughput, and concludes self-hosting is dramatically
cheaper. Six months after migrating, the actual bill is higher than the API would have cost. What was
wrong with the comparison?

<details>
<summary>Worked solution</summary>

The comparison used full-load utilization instead of realistic utilization (co-27, co-28) -- real
traffic is spiky, and a GPU bills for every hour it is on regardless of how busy it actually is during
that hour. The honest comparison divides the same hourly rate by REALISTIC (not peak) effective
throughput, which the syllabus's own tensions section calls out as "almost always wrong" when done at
full load.

</details>

**AP6.** A team implements a strict priority scheduler that always serves "premium" tier requests
before "free" tier ones. After a traffic surge in the premium tier that never fully subsides, free
tier requests stop completing entirely -- not slowly, but never. What's the failure mode, and what's
the minimal fix?

<details>
<summary>Worked solution</summary>

This is starvation (co-13): strict priority with no fairness guarantee can starve a lower class
completely if higher-priority work keeps arriving. The minimal fix is a fairness mechanism -- e.g.
weighted round-robin -- that guarantees every class a nonzero share of slots every round, priority
only breaking ties within that guaranteed share.

</details>

**AP7.** A GPU autoscaling policy is copied directly from an existing CPU-service autoscaler: scale
out when queue depth crosses a threshold. Under load, the service repeatedly falls behind even though
the policy fires "in time" by the CPU-service's own definition. What's missing?

<details>
<summary>Worked solution</summary>

The policy is reactive, not proactive (co-23) -- it ignores the multi-second cold start required to
load model weights onto a new GPU replica. By the time the naive threshold fires and a new replica
finishes loading, the queue has grown well past what the threshold predicted. The fix projects queue
growth across the measured cold-start window and scales out earlier.

</details>

**AP8.** An engineer quantizes a model to INT4, checks that it "still answers reasonably" on five
manually-tried prompts, and ships it to save memory. A month later, a systematic quality drop is
discovered on a specific class of numerical-reasoning questions the five manual prompts never
happened to cover. What discipline would have caught this before shipping?

<details>
<summary>Worked solution</summary>

Evaluating the quantized candidate on the team's own eval suite and measuring an actual quality delta
(co-19), rather than a small number of hand-picked prompts -- this is the exact same "vibe check"
failure mode that applies to any model change: five manually-tried cases cannot reveal a regression on
a case class nobody thought to try. A quantization decision record should cite a measured delta
against a real, fixed eval set, never an impression from ad hoc spot checks.

</details>

## Code katas

Five self-contained exercises. Every kata runs on hand-constructed, in-memory data using only the
Python 3.13 standard library -- no GPU, no downloaded weights, and no dependency on this topic's own
worked-example or capstone files, so every kata is runnable anywhere Python 3.13 is installed.

### Kata 1 -- Compute KV-cache bytes from first principles

Write `kv_cache_bytes(num_layers: int, num_heads: int, head_dim: int, seq_len: int, bytes_per_value:
int) -> int` implementing the formula from co-06 (`2 * layers * heads * head_dim * seq_len *
bytes_per_value`). Confirm it by hand-computing the expected result for `num_layers=4, num_heads=8,
head_dim=64, seq_len=256, bytes_per_value=2` before running your function, then confirm your function's
output matches your hand-computed value exactly.

### Kata 2 -- A continuous-batching occupancy tracker, hand-built from scratch

Write `run_continuous_batch(output_lengths: list[int], max_slots: int) -> list[int]` that simulates
continuous batching (admit at token granularity, retire the instant a request finishes) and returns
the occupancy (active count) at every step. Construct your own workload of at least 5 requests with
varied lengths and `max_slots=3`, predict the occupancy trace on paper first, then confirm your
function's output matches your prediction (co-12).

### Kata 3 -- A static-versus-continuous slot-step comparator

Using your Kata-2 function (or a fresh one), write `static_slot_steps(output_lengths: list[int]) ->
int` returning `max(output_lengths) * len(output_lengths)`, and `continuous_slot_steps(output_lengths:
list[int]) -> int` returning `sum(output_lengths)`. Construct a workload with at least one very long
and several very short requests, and confirm the ratio between the two functions' results is larger
than 2x on your own constructed workload (co-11, co-12).

### Kata 4 -- A block-based (paged) allocator, hand-built from scratch

Write `blocks_needed(token_len: int, block_size: int) -> int` (round up to the nearest whole block)
and `paged_bytes(lengths: list[int], block_size: int, bytes_per_token: int) -> int`. Construct a
workload of your own with at least one very long and several very short requests, compute the
contiguous-allocation cost by hand (`len(lengths) * max(lengths) * bytes_per_token`), and confirm your
paged allocator's result is meaningfully smaller (co-08, co-09).

### Kata 5 -- A two-guardrail canary evaluator

Write `evaluate_stage(error_rate: float, p99_latency_ms: float, error_guardrail: float,
latency_guardrail_ms: float) -> str` returning `"advance"` or a specific `"halt: <reason>"` string
naming which guardrail failed. Construct three of your own scenarios -- one healthy on both axes, one
that fails only the latency guardrail, one that fails only the error-rate guardrail -- and confirm
each returns the correct, specific halt reason rather than a generic failure (co-25).

## Self-check checklist

Confirm each item without checking the learning track first. If you hesitate, that concept needs
another pass.

- [ ] I can name the three properties that make LLM serving unlike ordinary request/response. (co-01)
- [ ] I can explain why prefill is parallel and compute-bound while decode is sequential. (co-02,
      co-03)
- [ ] I can explain why more GPU compute does not speed up decode. (co-04)
- [ ] I can explain what problem the KV cache solves and state its size formula's four inputs. (co-05,
      co-06)
- [ ] I can explain why cache, not compute, usually sets the concurrency ceiling. (co-07)
- [ ] I can explain how contiguous allocation strands capacity and how paged allocation fixes it.
      (co-08, co-09)
- [ ] I can explain when prefix sharing helps and when it buys nothing. (co-10)
- [ ] I can explain static batching's idle-slot waste and how continuous batching eliminates it.
      (co-11, co-12)
- [ ] I can explain how scheduling policy alone can change tail latency by an order of magnitude.
      (co-13)
- [ ] I can explain what preemption costs the evicted request. (co-14)
- [ ] I can explain why no batch size optimizes both throughput and latency at once. (co-15)
- [ ] I can name all four serving-latency metrics and what each measures. (co-16)
- [ ] I can explain the TTFT-versus-ITL trade from prioritizing prefill. (co-17)
- [ ] I can name the four consumers of GPU memory and which one the cache-size formula fills. (co-18)
- [ ] I can explain why a quantization quality claim must be measured, not trusted from a published
      number. (co-19)
- [ ] I can explain what tensor parallelism costs in interconnect traffic and fault tolerance. (co-20)
- [ ] I can explain why a single "average request" number is insufficient for capacity planning.
      (co-21)
- [ ] I can explain what goes wrong when a load test substitutes a uniform length for the real
      distribution. (co-22)
- [ ] I can explain why GPU autoscaling cannot copy a stateless CPU service's reactive policy. (co-23)
- [ ] I can name the three things a deployment manifest bundles as one versioned unit. (co-24)
- [ ] I can explain why a model swap needs the same staged-rollout discipline as a code release.
      (co-25)
- [ ] I can name the five signals that together usually explain a serving incident. (co-26)
- [ ] I can state what self-hosting wins on and what it loses on, compared to a hosted API. (co-27)
- [ ] I can explain what an honest TCO comparison includes that a full-load comparison omits. (co-28)

## Elaborative interrogation & self-explanation

Six prompts that ask you to explain WHY, connecting two or more concepts, rather than recall a single
fact. Write your own answer before checking the discussion.

**E1.** Co-07 says cache, not compute, is usually the scarce resource. Co-19 (quantization) and co-09
(paged allocation) are both presented as ways to "buy back capacity." Why does it make sense that
BOTH of these very different techniques target the same resource?

<details>
<summary>Discussion</summary>

Because co-07 establishes that cache memory -- not compute -- is what actually limits concurrency, any
technique that wants to increase how many requests a GPU can serve must ultimately act on memory, not
arithmetic throughput. Quantization shrinks the bytes each cached value costs; paged allocation
shrinks the bytes wasted to fragmentation. They attack the identical bottleneck from two different
angles (how much each unit costs vs. how much is wasted per unit), which is exactly why a real
deployment typically uses both together rather than treating them as competing choices.

</details>

**E2.** Co-12 (continuous batching) and co-09 (paged allocation) are presented as separate techniques
in this course's spine, but the lineage material credits paged allocation as the mechanism that made
prefix sharing (co-10) -- and by extension, much of continuous batching's real-world efficiency --
practical. What's the connection?

<details>
<summary>Discussion</summary>

Continuous batching's admit/retire-at-token-granularity mechanism works regardless of how cache is
allocated, but it becomes dramatically more efficient once cache is addressed in fixed blocks (co-09)
rather than contiguous per-request regions -- fixed blocks let two requests reference the SAME block
for a shared prefix (co-10) instead of each needing its own identical contiguous memory. Without
paging, continuous batching still eliminates idle-slot waste, but it cannot additionally deduplicate
shared prefixes, leaving real capacity on the table.

</details>

**E3.** Co-17 shows that prioritizing prefill improves a new request's TTFT while stalling in-flight
requests' ITL. Co-39's chunked prefill is presented as resolving this without forcing a strict choice.
Why doesn't chunking simply become "the answer" that makes co-17's tension irrelevant?

<details>
<summary>Discussion</summary>

Chunking bounds the WORST-CASE stall to one chunk's cost instead of the whole prefill's cost, but it
does not eliminate the tension entirely -- there is still a real trade between chunk size (smaller
chunks bound the stall more tightly but add scheduling overhead and slightly delay the new request's
own TTFT) and how aggressively prefill is interleaved with decode. Co-17's underlying trade-off is
durable; chunking is a mechanism that makes the trade's worst case much more tolerable, not a way to
make the trade disappear.

</details>

**E4.** Co-28's total-cost-of-ownership concept insists that realistic utilization -- not full load --
must drive the build-versus-buy calculation (co-27). Why is "the GPU is billed by the hour regardless
of use" specifically the fact that makes utilization matter this much, when a hosted API has no
equivalent idle cost?

<details>
<summary>Discussion</summary>

A hosted API bills strictly per token actually consumed -- there is no such thing as an "idle" API
hour costing money. A self-hosted GPU, by contrast, is a fixed-cost resource that bills for every hour
it exists, whether it served one token or a million during that hour. This asymmetry means the
self-hosted side of the comparison is extremely sensitive to how much of its billed time is genuinely
productive, while the hosted-API side is not sensitive to utilization at all -- which is exactly why
plugging in a full-load throughput number for the self-hosted side, when real traffic is spiky,
systematically favors self-hosting in a way that does not survive contact with real operations.

</details>

**E5.** Co-14 (preemption) and co-13 (scheduling fairness) both involve a scheduler making a choice
that costs one request to benefit another. What's the concrete difference between a well-designed
preemption policy and the starvation failure mode co-13 warns about?

<details>
<summary>Discussion</summary>

A well-designed preemption policy evicts occasionally, under genuine cache pressure, to admit specific
higher-priority or more-urgent work -- the cost (recomputed progress) is real but bounded and
purposeful. Starvation is what happens when a scheduling rule systematically and repeatedly
disadvantages the SAME class of request with no bound -- not an occasional, purposeful trade but a
structural exclusion. The fix for starvation (a fairness guarantee, co-13) and the fix for preemption
thrashing (a minimum-progress guard, co-14) are the same shape of fix -- both bound how much any one
class of request can be repeatedly disadvantaged -- because both failure modes are the identical
underlying problem: an unbounded, repeated cost imposed on the same victim.

</details>

**E6.** This course's scope guard explicitly excludes prompting, fine-tuning, and agent design. Why
does co-01's framing ("the unit of work is a token, cost is unknown on arrival, memory is the
dominant resource") draw that boundary as cleanly as it does?

<details>
<summary>Discussion</summary>

Co-01's three properties are specifically about RUNNING a model that already exists, with weights
already fixed -- they say nothing about how good the model's answers are, how its weights were
produced, or how a chain of calls is orchestrated into an agent. Everything this course teaches
(the KV cache, batching, scheduling, capacity planning, deployment) follows directly from those three
serving-specific properties; none of it changes if you swap in a better-fine-tuned model or a
smarter prompt. That is exactly the boundary this course's scope guard draws: if changing the
question changes co-01's three properties, it's out of scope; if it doesn't, it's this course's job.

</details>

---

← Previous: [Capstone](../learning/capstone/overview.md)
