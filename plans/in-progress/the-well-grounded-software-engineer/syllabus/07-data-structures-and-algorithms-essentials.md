# 7 · Data Structures & Algorithms Essentials (By Example, Python)

**prd row**: Pass 1 · Core Foundations · By Example · Python · Learn 107 / Drill 207 · Nvim-ready
Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the **usable slice** — the structures and algorithms a working engineer reaches for
daily. Deep paradigms (amortized/Θ/Ω rigor, graph/DP/greedy families) are deferred to
[`25-advanced-algorithms`](./25-advanced-algorithms.md) (split-and-interleave, DD-11).

## Why this exists · the big idea

- **The problem before the solution**: the same task can run instantly or crawl depending on how you
  store the data — choosing the wrong structure is a bug you feel only at scale.
- **Keep-this-if-you-forget-everything**: pick the structure that makes the _common_ operation cheap; the
  data-structure choice is the real decision, and the algorithm often follows from it.
- **Big ideas touched**: `abstraction-and-its-cost` — every structure trades one operation's cost for
  another (a hash buys O(1) lookup and charges ordering; a list buys order and charges search).

## Prerequisites

- **Prior topics**: [topic 4 Just Enough Python](./04-just-enough-python.md) (all examples are Python).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** with `pytest` installed in a `venv`.
- **Assumed knowledge**: reading/writing basic Python — functions, lists, dicts, loops (from topic 04);
  no prior algorithms background required.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: `heapq`, `collections.deque`, and `bisect` stdlib APIs are current/unchanged in
  Python 3.14; list `.append()` is amortized O(1), dict lookup average-case O(1) (degrades only under
  pathological collisions). (docs.python.org / wiki.python.org TimeComplexity)

## Items

- **Complexity intuition**: Big-O basics and why it matters (amortized/Θ/Ω rigor → Advanced Algorithms).
- **Linear structures**: dynamic array (`list`), stack, queue, `deque`; singly-linked-list basics.
- **Hashing**: `dict`/`set`, hash-map/hash-set usage, when O(1) lookup helps.
- **Trees basics**: binary tree, BST intro, heap / priority queue via `heapq`.
- **Searching & sorting basics**: linear vs binary search; the built-in sort; simple sorts conceptually.
- **Recursion basics**; iterate-vs-recurse.

## Worked examples

Colocated under `data-structures-and-algorithms-essentials/learning/code/`; each a runnable Python module
with a `pytest` assertion (DD-20/DD-30).

- **beginner** — reverse a list / linked list; balanced parentheses with a stack; two-sum with a dict.
- **intermediate** — binary search (first/last occurrence); min-heap top-k; BFS over a dict-of-lists
  graph.
- **advanced** — memoized fibonacci; build a small BST with insert/search.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a small "job scheduler" that ingests tasks with priorities and dependencies and emits a
  valid run order — exercising a heap (priority), a dict/set (lookup + seen-tracking), a queue/BFS
  (dependency traversal), and complexity reasoning, all in one runnable program with tests.
- **Concepts exercised**: [ ] `heapq` priority queue [ ] `dict`/`set` lookups [ ] BFS over an adjacency
  dict [ ] cycle detection [ ] Big-O reasoning documented per operation.
- **Ordered steps**:
  1. `.../learning/capstone/code/scheduler.py` — parse tasks `{id, priority, deps}`; build adjacency +
     in-degree. Verify `pytest` on a fixture graph.
  2. Topologically order by dependency, breaking ties by priority via `heapq`. Verify the emitted order
     respects all deps and prefers higher priority on ties.
  3. Detect a dependency cycle and raise a clear error. Verify a cyclic fixture raises.
  4. Document the Big-O of each phase in the module docstring.
- **Acceptance criteria**: `pytest` green on acyclic + cyclic fixtures; documented complexities correct;
  program runs from the CLI on a sample input.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Introduction to Algorithms (CLRS)** — Cormen, Leiserson, Rivest, Stein (4th ed., 2022). The standard rigorous algorithms/data-structures reference; the field's most-cited textbook.
- **The Algorithm Design Manual** — Steven Skiena (3rd ed., 2020). Practical companion to CLRS, prized for "war stories" and its algorithm catalog for working engineers.
- **Algorithms** — Sedgewick, Wayne (4th ed., 2011). Implementation-focused classic pairing runnable code with rigorous analysis. <https://algs4.cs.princeton.edu/home/>

---

← Previous: [6 · Version Control & Git](./06-version-control-and-git.md) · Next: [8 · Object-Oriented Programming Essentials](./08-object-oriented-programming-essentials.md) →
