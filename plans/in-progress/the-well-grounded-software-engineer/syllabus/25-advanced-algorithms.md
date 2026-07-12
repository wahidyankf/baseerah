# 25 · Advanced Algorithms (By Example, Python)

**prd row**: Pass 2 · Depth, Design & Craft · By Example · Python · Learn 125 / Drill 225 · Nvim-ready Yes ·
VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the deep algorithms pass — rigorous complexity, advanced trees and graphs, algorithmic
paradigms (D&C, greedy, DP, backtracking), and the problem-solving patterns for interviews and real work.
The everyday basics are the prerequisite
[`07-data-structures-and-algorithms-essentials`](./07-data-structures-and-algorithms-essentials.md); this
topic is where they become a toolkit.

## Why this exists · the big idea

- **The problem before the solution**: some problems look intractable until you know the paradigm that
  cracks them — brute force silently explodes from milliseconds to millennia as the input grows.
- **Keep-this-if-you-forget-everything**: most hard problems reduce to a known shape — divide, be greedy,
  memoize, or backtrack — and recognizing which shape you're holding is the actual skill.
- **Big ideas touched**: `abstraction-and-its-cost` — every paradigm is a resource trade (DP buys time
  with memory; greedy buys speed by giving up a guarantee; the analysis is deciding which trade is worth it).

## Prerequisites

- **Prior topics**: [topic 7 Data Structures & Algorithms Essentials](./07-data-structures-and-algorithms-essentials.md)
  (arrays, hashmaps, trees, Big-O, recursion) and [topic 4 Just Enough Python](./04-just-enough-python.md);
  [topic 19 Computer Science Foundations](./19-computer-science-foundations.md) sharpens the complexity
  reasoning.
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** (`heapq`, `collections`, `functools`
  from the stdlib); `pytest` to check algorithm correctness on edge cases.
- **Assumed knowledge**: recursion, Big-O notation, and the basic data structures from topic 07; reading a
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

## Read more

**Books**

- **Introduction to Algorithms** — Thomas H. Cormen, Charles E. Leiserson, Ronald L. Rivest, Clifford Stein (2009, 3rd ed.). The standard reference ("CLRS") for algorithms, data structures, and complexity analysis.
- **Algorithm Design** — Jon Kleinberg & Éva Tardos (2005). Widely adopted text teaching algorithmic design paradigms — greedy, divide-and-conquer, DP, network flow — through motivating problems.
- **The Algorithm Design Manual** — Steven S. Skiena (1997; 2nd ed. 2008). Practitioner-oriented reference pairing "war stories" with a catalog of algorithmic techniques and data structures.
- **Approximation Algorithms** — Vijay V. Vazirani (2001). The standard graduate reference on approximation algorithms and hardness of approximation for NP-hard problems.

**Papers & articles**

- **A Note on Two Problems in Connexion with Graphs** — Edsger W. Dijkstra (1959). Original paper introducing Dijkstra's shortest-path algorithm, still taught in every graph algorithms course.

---

← Previous: [24 · Concurrency & Parallelism](./24-concurrency-and-parallelism.md) · Next: [26 · Advanced SQL & Query Performance](./26-advanced-sql-and-query-performance.md) →
