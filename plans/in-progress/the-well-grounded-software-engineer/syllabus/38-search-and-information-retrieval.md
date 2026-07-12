# 38 · Search & Information Retrieval (By Example, Python †)

**prd row**: Pass 3 · Build for the Real World · By Example · Python † · Learn 138 / Drill 238 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: making text findable, taught in three tiers — a raw inverted index by hand →
a real engine (tokenization, TF-IDF/BM25, ranking) → building a small index yourself — with the
relevance-vs-recall trade-off in the foreground. The vanilla tier grounds the intuition, the
practical tier shows what a production engine (Lucene/Elasticsearch/OpenSearch family) actually
does, and the build-your-own tier makes the machinery concrete. `†`: Python, fully type-annotated
(DD-34) — every snippet carries type hints in the mypy-clean spirit.

## Why this exists · the big idea

- **The problem before the solution**: a `LIKE '%term%'` scan finds substrings, not documents — it
  can't rank, can't handle "roughly this", and gets slower with every row. Users don't want the rows
  that _contain_ a word; they want the handful that are _about_ it, best-first, in milliseconds.
- **Keep-this-if-you-forget-everything**: search inverts the problem — instead of scanning documents
  for a query, you build a term → documents map once (the inverted index) and let the query do a
  cheap lookup, then _rank_ the hits by a relevance score. Index-then-rank is the whole game.
- **Big ideas touched**: `abstraction-and-its-cost` (a search engine hides tokenization, scoring, and
  merge logic behind one `search()` call — and the hidden analyzer choices leak straight into which
  documents you can ever find), `consistency-latency-throughput` (ranking quality, index freshness,
  and query latency trade against each other — a better score costs CPU, a fresher index costs write
  throughput).

## Prerequisites

- **Prior topics**: [topic 10 SQL Essentials](./10-sql-essentials.md) (the row-scan baseline search
  improves on) and [topic 7 Data Structures & Algorithms Essentials](./07-data-structures-and-algorithms-essentials.md)
  (hash maps, sorted postings, heaps for top-k).
- **Tools & environment**: a macOS/Linux terminal; **Python** at a recent stable release with type
  hints and `mypy`; a small text corpus (a docs folder or a public dataset dump); optionally a local
  search engine from the Lucene family (Elasticsearch/OpenSearch) via a container for the practical
  tier; Neovim/VSCode with the Python LSP (DD-17).
- **Assumed knowledge**: writing and querying a table (topic 10); dictionaries, sets, and sorting
  by a key (topic 07); reading a file line by line from Python (topic 04).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: BM25 (Okapi BM25) remains the default lexical ranking function across the
  Lucene family and is left correctly version-unpinned — the formula and its `k1`/`b` parameters are
  stable and standard. TF-IDF as the teaching stepping-stone to BM25 is unchanged.
- 2026-07-12 — verified (GAP for plan owner): the practical tier names the Lucene/Elasticsearch/
  OpenSearch family generically on purpose — pin a concrete engine + version only at drafting time,
  since the licensing split (Elastic vs the OpenSearch fork) and default analyzers shift between
  releases. The vanilla and build-your-own tiers are engine-independent and stable.

## Items

- **Tier 1 — the inverted index by hand**: tokenize a small corpus, build a `term → posting list`
  dictionary, and answer a boolean query (AND/OR/NOT) by merging posting lists. See why this beats
  a scan.
- Tokenization and normalization: case-folding, stemming/lemmatization, stop-words — and how each
  choice silently expands or shrinks what is findable.
- Scoring: term frequency, inverse document frequency, TF-IDF, then BM25 and why its saturation and
  length-normalization terms beat raw TF-IDF.
- **Tier 2 — a real engine**: analyzers, fields, the query DSL, and top-k ranking against a
  Lucene-family engine; mapping the earlier hand-built concepts onto its knobs.
- Relevance vs recall: precision/recall, the trade-off, and evaluating ranking quality (precision@k,
  a tiny relevance-judgment set).
- **Tier 3 — build your own small index**: a typed Python inverted index with BM25 top-k, persisted
  postings, and incremental document add — the framework demystified.

## Tensions & trade-offs — when NOT to reach for this

- **A dedicated search engine is a second source of truth**: it must be fed, kept in sync with the
  primary store, and reindexed when the schema or analyzer changes. For a small dataset, Postgres
  full-text search (or even a well-indexed `LIKE`) avoids a whole distributed system you'd otherwise
  operate and reconcile.
- **Lexical search doesn't understand meaning**: BM25 matches tokens, not concepts — "car" won't find
  "automobile". When semantic matching genuinely matters, vector/embedding search is the tool, but it
  adds model, index, and cost; hybrid (lexical + vector) is often right, pure-vector rarely is.
- **Relevance tuning is unbounded**: analyzers, boosts, and score functions can be tuned forever.
  Without a relevance-judgment set to measure against, tuning is guessing — reach for evaluation
  before reaching for another boost.

## Lineage — why it beat the alternative

- Search grew out of library and legal-database retrieval: the inverted index predates the web, and
  the probabilistic ranking work behind BM25 was hardened at the TREC evaluations in the early 1990s.
  Brin and Page's link-analysis (PageRank) then showed that _who points at a document_ ranks web
  pages better than term statistics alone. The lasting winner for text is index-then-rank with BM25,
  because it is cheap, explainable, and strong without training data — which is exactly why the Lucene
  family made it the default. This topic hands its scoring and indexing intuition to
  [topic 39 Backend at Scale](./39-backend-at-scale.md) (search as a service to operate) and its
  storage mechanics to [topic 87 Build Your Own Database](./87-build-your-own-database.md).

## Worked examples

Colocated under `search-and-information-retrieval/learning/code/`; each runnable from the CLI, every
Python snippet fully type-annotated and `mypy`-clean (DD-20/DD-30/DD-34).

- **beginner** — build a typed inverted index over a small corpus and answer a boolean AND/OR query
  by merging posting lists; compare timing against a naive substring scan.
- **intermediate** — add tokenization/normalization and TF-IDF then BM25 scoring; return ranked
  top-k and inspect how a stop-word or stemmer choice changes the results.
- **advanced** — evaluate two analyzer/scoring configurations against a tiny relevance-judgment set
  (precision@k), and persist the index so documents can be added incrementally.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a small typed search library over a real text corpus that indexes documents,
  ranks queries with BM25 top-k, supports incremental add, and reports a relevance metric against a
  judgment set — then contrast one query's results with a Lucene-family engine to prove the concepts
  transfer.
- **Concepts exercised**: [ ] inverted index + posting-list merge [ ] tokenization/normalization
  [ ] TF-IDF → BM25 scoring [ ] top-k ranking [ ] precision@k evaluation [ ] incremental indexing.
- **Ordered steps**:
  1. `.../learning/capstone/code/index.py` — a typed inverted index with tokenization and persisted
     postings. Verify a boolean query returns the correct document set and `mypy` is clean.
  2. `.../learning/capstone/code/rank.py` — add TF-IDF then BM25 top-k scoring. Verify the ranked
     order matches a hand-computed BM25 score on a 3-document fixture.
  3. `.../learning/capstone/code/evaluate.py` — run precision@k over a small relevance-judgment set
     across two analyzer configs. Verify the metric changes as expected when a stemmer is toggled.
  4. `.../learning/capstone/code/incremental.py` — add a new document without a full rebuild. Verify
     it becomes findable and its BM25 score is consistent with a from-scratch rebuild.
- **Acceptance criteria**: boolean and ranked queries are correct; BM25 matches the hand computation;
  precision@k responds to analyzer changes; incremental add matches a rebuild; all Python is
  type-annotated and `mypy`-clean.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Introduction to Information Retrieval** — Christopher D. Manning, Prabhakar Raghavan, Hinrich
  Schütze (2008). Free, standard graduate textbook on indexing, ranking, and evaluation for search
  systems. <https://nlp.stanford.edu/IR-book/>
- **Relevant Search: With Applications for Solr and Elasticsearch** — Doug Turnbull, John Berryman
  (2016). Standard practitioner's guide to tuning ranking and relevance in Lucene-based search
  engines.
- **Lucene in Action** — Michael McCandless, Erik Hatcher, Otis Gospodnetić (2nd ed., 2010).
  Canonical implementation-level reference for building search applications on Apache Lucene.

**Papers & articles**

- **The Anatomy of a Large-Scale Hypertextual Web Search Engine** — Sergey Brin, Lawrence Page
  (1998). The original Google/PageRank paper defining link-based ranking for large-scale web search.
  <http://infolab.stanford.edu/pub/papers/google.pdf>
- **Okapi at TREC** — Stephen E. Robertson, Stephen Walker, Micheline Hancock-Beaulieu, A. Gull,
  M. Lau (1992). Origin of the probabilistic ranking work that produced BM25, the ranking function
  underlying most modern inverted-index search engines. <https://trec.nist.gov/pubs/trec1/papers/02.txt>

---

← Previous: [37 · Data Engineering](./37-data-engineering.md) · Next: [39 · Backend at Scale](./39-backend-at-scale.md) →
