# 20 · Advanced Algorithms (By Example, Python)

**prd row**: Pass 2 · Solidify the Core · By Example · Python · Learn 120 / Drill 220 · Nvim-ready Yes ·
VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the deep algorithms pass — rigorous complexity, advanced trees and graphs, algorithmic
paradigms (D&C, greedy, DP, backtracking), and the problem-solving patterns for interviews and real work.
The everyday basics are the prerequisite
[`06-data-structures-and-algorithms-essentials`](./06-data-structures-and-algorithms-essentials.md); this
topic is where they become a toolkit.

## Prerequisites

- **Prior topics**: [topic 06 Data Structures & Algorithms Essentials](./06-data-structures-and-algorithms-essentials.md)
  (arrays, hashmaps, trees, Big-O, recursion) and [topic 04 Just Enough Python](./04-just-enough-python.md);
  [topic 15 Computer Science Foundations](./15-computer-science-foundations.md) sharpens the complexity
  reasoning.
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** (`heapq`, `collections`, `functools`
  from the stdlib); `pytest` to check algorithm correctness on edge cases.
- **Assumed knowledge**: recursion, Big-O notation, and the basic data structures from topic 06; reading a
  simple recurrence.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: `heapq` and `functools.cache` (added 3.9, a simplified `lru_cache(maxsize=None)`)
  are current unchanged stdlib APIs. The Master-theorem statement (comparing `f(n)` against `n^(log_b a)`)
  is a stable unchanged mathematical result; complexity facts stable. (docs.python.org / CLRS canon)

## Items

- Complexity rigor: amortized analysis, Θ/Ω, space–time trade-offs, recurrence & Master-theorem intuition.
- Advanced trees: balanced trees (AVL / red-black overview), tries, segment / Fenwick trees (survey).
- Graph algorithms: BFS/DFS deep, topological sort, Dijkstra, Bellman-Ford, union-find, MST (Kruskal/Prim).
- Sorting deep: merge/quick/heap sort with invariants; counting/radix; stability.
- Algorithmic paradigms: divide & conquer, greedy, dynamic programming (1D/2D), backtracking,
  two-pointers, sliding window.
- Problem-solving patterns for interviews and real work.

## Worked examples

Colocated under `advanced-algorithms/learning/code/`; each runnable with edge-case tests (DD-20/DD-30).

- **beginner** — merge-sort with invariants; DFS/BFS traversals.
- **intermediate** — DP (edit distance / LCS / knapsack); Dijkstra shortest path; union-find connected
  components.
- **advanced** — backtracking (N-queens / word search); a greedy-vs-DP contrast; topological sort of a
  real dependency graph.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a small "algorithm workbench" that solves one substantial problem end to end — e.g. a
  task scheduler over a dependency DAG that computes a topological order, a critical path via DP, and a
  shortest-cost path via Dijkstra — with complexity stated and verified against edge-case tests.
- **Concepts exercised**: [ ] graph representation + BFS/DFS [ ] topological sort (cycle detection)
  [ ] a DP formulation (critical path / longest path) [ ] Dijkstra with a heap [ ] stated + justified
  complexity [ ] edge-case tests (empty, cyclic, disconnected).
- **Ordered steps**:
  1. `.../learning/capstone/code/graph.py` — the DAG model + topological sort with cycle detection. Verify
     a cyclic input is rejected and a valid DAG yields a correct order.
  2. `critical_path.py` — DP longest-path/critical-path over the DAG. Verify it matches a hand-computed
     small example.
  3. `shortest.py` — Dijkstra over a weighted variant with `heapq`. Verify it matches a known shortest path
     and handles an unreachable node.
  4. State each routine's time/space complexity; add a `pytest` suite of edge cases. Verify all pass.
- **Acceptance criteria**: every algorithm is correct on the edge-case suite; complexities are stated and
  defended; the workbench runs end to end on a sample project graph.
- **Done bar**: runnable end-to-end + web-verified.

---

← Previous: [19 · Concurrency & Parallelism (Core)](./19-concurrency-and-parallelism.md) · Next: [21 · Advanced Networking](./21-advanced-networking.md) →
