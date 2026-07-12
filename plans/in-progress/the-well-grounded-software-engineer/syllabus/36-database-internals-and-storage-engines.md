# 36 · Database Internals & Storage Engines (By Example, Python †)

**prd row**: Pass 3 · Build for the Real World · By Example · Python † · Learn 136 / Drill 236 · Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: what's under the database — B-tree versus LSM-tree storage engines, the write-ahead
log, the buffer pool, MVCC, and on-disk page layout. This explains the query-performance topic above
it (why an index is a B-tree, why writes are cheap in one engine and reads in another) and feeds the
build-your-own-database pass at [`87-build-your-own-database`](./87-build-your-own-database.md). `†`:
fully type-annotated Python examples (DD-34) that model the structures at small scale.

## Why this exists · the big idea

- **The problem before the solution**: treating the database as an opaque box means you can't explain
  why a workload is slow, why the same schema is fast on one engine and slow on another, or what
  actually happens on `COMMIT` — the abstraction hides exactly the costs you must reason about at
  scale.
- **Keep-this-if-you-forget-everything**: durability and performance come from the same few ideas — an
  append-only log for crash-safe writes, a page/buffer-pool cache for reads, and an index structure
  (B-tree or LSM) chosen to favor either reads or writes.
- **Big ideas touched**: `consistency-latency-throughput` (B-tree vs LSM is a read-latency-vs-write-
  throughput choice, and the WAL and buffer pool trade durability guarantees against latency),
  `layering-and-leaks` (the SQL abstraction leaks its storage engine — page size, index type, and MVCC
  version bloat all surface as performance you have to explain).

## Prerequisites

- **Prior topics**: [topic 10 SQL Essentials](./10-sql-essentials.md) and
  [topic 26 Advanced SQL & Query Performance](./26-advanced-sql-and-query-performance.md).
- **Tools & environment**: a macOS/Linux terminal; a local relational DB whose internals you can
  inspect (Postgres-style MVCC and/or an embedded B-tree/LSM store); a hex/page viewer; **Python 3.x**
  (fully type-annotated) to model the structures; Neovim/VSCode with the Python LSP (DD-17).
- **Assumed knowledge**: writing SQL and reading an `EXPLAIN` plan (topics 10, 26); how an index
  changes a query plan (topic 26); reading typed Python (topic 04).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: the internals taught here (B-tree and LSM-tree structure, write-ahead logging,
  buffer pool, MVCC, slotted-page layout) are long-settled, engine-independent concepts and correctly
  unpinned. The canonical WAL/recovery reference remains ARIES (1992) and the canonical LSM reference
  the O'Neil et al. paper (1996).
- 2026-07-12 — verified: specific engine defaults (page sizes, MVCC vacuum behavior, compaction
  strategy) vary by product and version — the file describes them as engine-dependent rather than
  asserting one product's numbers.

## Items

- On-disk layout: pages/blocks, the slotted-page format, and why databases think in fixed-size pages.
- The buffer pool: caching hot pages in memory, eviction, and the read path.
- B-tree storage engines: structure, in-place updates, and read-optimized behavior — the classic
  index.
- LSM-tree storage engines: memtable + SSTables, compaction, and write-optimized behavior; the
  read/write trade-off against B-trees.
- The write-ahead log: durability, crash recovery (ARIES-style redo/undo), and why the log is written
  before the page.
- MVCC: multiple versions for snapshot isolation, how readers avoid blocking writers, and the cost
  (version bloat / vacuum).

## Tensions & trade-offs — when NOT to reach for this

- **B-tree vs LSM is a workload bet, not a winner**: LSM wins write-heavy/ingest workloads and
  compresses well but pays on reads and suffers unpredictable compaction stalls; B-trees win
  read-heavy/point-lookup workloads but amplify random writes. Choosing by fashion instead of by
  workload is the mistake.
- **Internals knowledge can be premature**: for most CRUD apps the engine's defaults are fine, and
  reaching for storage-engine tuning before a measured bottleneck is effort spent where it doesn't pay
  — go through topic 26's `EXPLAIN`-driven approach first.
- **When NOT to go deeper**: if you're neither operating the database at scale nor picking the engine,
  the leaky details (compaction tuning, vacuum, WAL sizing) are someone else's job — learn enough to
  reason, not to reinvent.

## Lineage — why it beat the alternative

- Early databases wrote updates in place and hoped a crash didn't strike mid-write; the write-ahead log
  (formalized by ARIES, 1992) made durability and recovery correct by logging intent before mutating
  pages, and it remains the backbone of relational engines. The B-tree ruled indexing for decades
  because disks favored its shallow, read-optimized structure; the LSM-tree (O'Neil et al., 1996) then
  won the write-heavy, internet-scale workloads behind Bigtable, Cassandra, and RocksDB by turning
  random writes into sequential ones. The through-line — a log for durability, a structure chosen for
  the read/write balance — is exactly what [`87-build-your-own-database`](./87-build-your-own-database.md)
  reconstructs, and it explains the query-performance behavior taught in
  [`26-advanced-sql-and-query-performance`](./26-advanced-sql-and-query-performance.md).

## Worked examples

Colocated under `database-internals/learning/code/`; each models one internal at small scale and is
runnable, fully type-annotated Python (DD-20/DD-30/DD-34).

- **beginner** — a page/slotted-record layout: pack and unpack fixed-size pages, and walk a tiny
  on-disk B-tree.
- **intermediate** — a minimal LSM: a memtable + flush to sorted segments + a merge/compaction pass;
  compare its write path to the B-tree's.
- **advanced** — a write-ahead log with crash recovery (redo on restart) and a small MVCC
  snapshot-read demonstration.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: model the core of a storage engine — a paged B-tree or LSM index, a write-ahead log with
  recovery, and a snapshot read — proving you understand how a database achieves durability and how the
  index choice sets the read/write trade-off.
- **Concepts exercised**: [ ] a slotted-page layout [ ] a B-tree or LSM index [ ] a buffer-pool read
  path [ ] a write-ahead log [ ] crash recovery (redo) [ ] an MVCC snapshot read [ ] fully
  type-annotated Python.
- **Ordered steps**:
  1. `.../learning/capstone/code/pages.py` — page pack/unpack + a buffer pool. Verify a record
     round-trips through a page and a hot page is served from the pool, not disk.
  2. `.../index.py` — a B-tree (or LSM) index over the pages. Verify point lookups and a range scan
     return correct results.
  3. `.../wal.py` — write-ahead logging with a simulated crash + restart. Verify committed writes
     survive the crash and uncommitted ones do not (redo/undo).
  4. `.../mvcc.py` — a snapshot read. Verify a reader sees a consistent snapshot while a concurrent
     writer proceeds.
- **Acceptance criteria**: pages round-trip; the index answers lookups and ranges; the WAL recovers
  committed data after a crash; the snapshot read stays consistent under a concurrent write.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Database Internals: A Deep Dive into How Distributed Data Systems Work** — Alex Petrov (2019). The
  modern canonical text on storage engines, B-trees, LSM-trees, and distributed consensus.
- **Designing Data-Intensive Applications** — Martin Kleppmann (2017). Covers storage-engine internals
  — indexing structures, WAL, replication — as a core part of its systems treatment.
- **Readings in Database Systems, 5th Edition (The Red Book)** — Peter Bailis, Joseph M. Hellerstein,
  Michael Stonebraker, eds. (2015). Free, curated collection of foundational and modern database-systems
  papers with expert commentary. <http://www.redbook.io/>

**Papers & articles**

- **The Log-Structured Merge-Tree (LSM-Tree)** — Patrick O'Neil, Edward Cheng, Dieter Gawlick,
  Elizabeth O'Neil (1996). The original paper defining the LSM-tree structure behind Bigtable,
  Cassandra, RocksDB, and LevelDB. <https://link.springer.com/article/10.1007/s002360050048>
- **ARIES: A Transaction Recovery Method Supporting Fine-Granularity Locking and Partial Rollbacks
  Using Write-Ahead Logging** — C. Mohan, Don Haderle, Bruce Lindsay, Hamid Pirahesh, Peter Schwarz
  (1992). The canonical write-ahead-logging and recovery algorithm implemented by most production
  relational databases. <https://dl.acm.org/doi/10.1145/128765.128770>

---

← Previous: [35 · Graph Databases](./35-graph-databases.md) · Next: [37 · Data Engineering](./37-data-engineering.md) →
