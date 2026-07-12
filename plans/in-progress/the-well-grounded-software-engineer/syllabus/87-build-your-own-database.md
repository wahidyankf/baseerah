# 87 · Build Your Own Database (By Example, Python †)

**prd row**: Pass 5 · Internals & Lead at Altitude · By Example · Python † · Learn 187 / Drill 287 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: demystify the database by rebuilding its core — a pager over a single file, a B-tree (or an
LSM alternative) for indexed storage, a write-ahead log for durability, a tiny SQL-ish query layer, and
crash recovery. This is the build-your-own tier of
[`36-database-internals-and-storage-engines`](./36-database-internals-and-storage-engines.md): that topic
explained B-trees, the WAL, the buffer pool, and MVCC; here you implement enough of them that a durable,
crash-safe key-value/table store stops being magic. `†`: Python, fully type-annotated (DD-34), verified
with `pytest`.

## Why this exists · the big idea

- **The problem before the solution**: "the database handles durability and consistency for you" is true
  and unhelpful until you have written the code that survives a crash mid-write — only then do the WAL,
  fsync, and page-at-a-time discipline stop being incantations and become mechanisms you can reason about.
- **Keep-this-if-you-forget-everything**: a database is a durable, ordered map built on three moves — put
  data in fixed-size pages, keep an index (B-tree/LSM) so lookups are logarithmic not linear, and write your
  intent to a log _before_ the data so a crash is recoverable. Everything else is optimization on top of
  those three.
- **Big ideas touched**: `consistency-latency-throughput` (the WAL/fsync boundary is exactly where you trade
  latency for durability, and page caching trades memory for throughput), `layering-and-leaks` (pager →
  B-tree → query layer is a clean stack, and the leaks — page splits, torn writes, recovery — are where the
  real learning is).

## Prerequisites

- **Prior topics**: [topic 36 Database Internals & Storage Engines](./36-database-internals-and-storage-engines.md)
  (B-tree vs LSM, the write-ahead log, buffer pool, MVCC — the concepts this topic makes concrete) and
  [topic 10 SQL Essentials](./10-sql-essentials.md) (the query surface you build a tiny version of).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** with type hints (mypy-clean spirit, DD-34);
  `pytest`; the standard library only for file I/O and `struct`-style packing (no external DB engine);
  Neovim/VSCode with the Python LSP (DD-17).
- **Assumed knowledge**: storage-engine concepts — pages, B-trees, the WAL, recovery (topic 36); SQL basics
  and what a query must do (topic 10); Python file/bytes handling and classes (topic 04).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: the core structures — fixed-size pages behind a pager, B-tree (or LSM+SSTables)
  indexing, write-ahead logging with fsync at the durability boundary, and log-based crash recovery — are
  evergreen and correctly left version-unpinned. The design deliberately targets a single-file store with
  the standard library, so there is no third-party version to pin.
- 2026-07-12 — verified (SCOPE note for plan owner): full ACID with concurrent transactions (MVCC, locking)
  is a large stretch; the tractable target is single-writer durability + crash recovery, with MVCC/isolation
  named as the forward direction. Keep the query layer "SQL-ish" (a parsed subset), not a conformant SQL
  engine. (Petrov, _Database Internals_; cstack db_tutorial)

## Items

- The pager: fixed-size pages over a single file, and a page cache/buffer pool with eviction.
- Indexed storage: a B-tree with insert, search, and node splits (or an LSM: memtable + SSTables + compaction).
- Durability: the write-ahead log, fsync, and why intent-before-data makes a crash recoverable.
- Crash recovery: replaying/truncating the WAL on startup to restore a consistent state.
- A tiny query layer: parse a SQL-ish subset (`insert`/`select`/`where`) and execute it against the store.
- Putting it together: a durable table/key-value store you can kill mid-write and reopen intact.

## Worked examples

Colocated under `build-your-own-database/learning/code/`; Python (fully type-annotated, DD-34) + `pytest`
(DD-20/DD-30). Durability is proven by killing the process mid-write and reopening.

- **beginner** — a pager: read/write fixed-size pages to a file with a small page cache; verify round-trips.
- **intermediate** — a B-tree over the pager with insert/search and node splits; verify ordered lookups
  survive many inserts.
- **advanced** — add a WAL + crash recovery and a SQL-ish `insert`/`select`/`where` layer; verify a
  simulated crash mid-write recovers to a consistent state.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a minimal but durable database — a pager with a page cache, a B-tree (or LSM) index, a
  write-ahead log with crash recovery, and a tiny SQL-ish query layer — such that it survives a process kill
  mid-write and reopens with a consistent state, fully covered by `pytest`.
- **Concepts exercised**: [ ] a pager + page cache over a single file [ ] a B-tree (or LSM) index with splits
  [ ] a write-ahead log with fsync [ ] crash recovery on startup [ ] a SQL-ish `insert`/`select`/`where`
  layer [ ] durability under a simulated crash [ ] `pytest` coverage of each stage.
- **Ordered steps**:
  1. `.../learning/capstone/code/pager.py` — fixed-size pages + a page cache over one file. Verify pages
     round-trip and eviction works (tests).
  2. `btree.py` — a B-tree with insert/search and node splitting on top of the pager. Verify ordered lookups
     stay correct across enough inserts to force splits (tests).
  3. `wal.py` + `recovery.py` — write-ahead logging with fsync and startup replay/truncate. Verify a process
     killed mid-write reopens to a consistent state, losing only uncommitted work (tests).
  4. `query.py` — parse and execute a SQL-ish `insert`/`select`/`where` subset against the store. Verify
     queries return correct rows and a full write → crash → reopen → select cycle is intact.
- **Acceptance criteria**: the pager/cache and B-tree behave under load; the WAL makes writes durable;
  recovery restores consistency after a mid-write kill; the query subset executes correctly; `pytest` covers
  each stage.
- **Done bar**: runnable end-to-end + survives a simulated crash + tests green + web-verified.

## Read more

**Books**

- **Database Internals** — Alex Petrov (2019). Canonical modern deep dive into storage engines (B-trees,
  LSM-trees) and distributed data systems — the blueprint for what you build here.
- **Designing Data-Intensive Applications** — Martin Kleppmann (2017). The field-defining book on the
  principles behind reliable, scalable data systems.

**Papers & articles**

- **Build Your Own Database From Scratch in Go** — James Smith. Widely cited incremental guide building a
  B+tree-based database; the key-value-store portion is free on the author's site (the full relational
  chapters are in the paid edition). <https://build-your-own.org/database/>
- **Let's Build a Simple Database** — cstack. Free, complete, widely referenced tutorial implementing a
  SQLite-like database from scratch in C. <https://cstack.github.io/db_tutorial/>
- **Architecture of a Database System** — Joseph M. Hellerstein, Michael Stonebraker, James Hamilton (2007).
  Highly cited survey of DBMS-internals architecture; free official PDF.
  <http://db.cs.berkeley.edu/papers/fntdb07-architecture.pdf>

---

← Previous: [86 · Build Your Own Git](./86-build-your-own-git.md) · Next: [88 · Build Your Own Raft / Replicated KV](./88-build-your-own-raft.md) →
