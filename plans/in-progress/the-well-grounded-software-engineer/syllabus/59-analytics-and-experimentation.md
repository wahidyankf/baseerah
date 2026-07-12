# 59 · Analytics & Experimentation (By Example, Python †)

**prd row**: Pass 3 · Build for the Real World · By Example · Python † · Learn 159 / Drill 259 · Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: measuring what you ship without fooling yourself — event instrumentation, funnels and
cohorts, A/B testing, statistical significance and its traps, and reading a metric honestly. This is the
data-literacy pass every shipping engineer needs: the usable persistence layer is
[`10-sql-essentials`](./10-sql-essentials.md) and the discipline of a controlled comparison extends
[`15-software-testing`](./15-software-testing.md). Pulled earlier in the spiral because its prerequisites
are light. `†`: Python, fully type-annotated (DD-34, mypy-clean spirit), driving a query engine and a
small statistics stack.

## Why this exists · the big idea

- **The problem before the solution**: a shipped feature you cannot measure is a guess with a deploy
  button — teams argued from opinion and anecdote because they had no trustworthy way to tell whether a
  change helped, hurt, or did nothing.
- **Keep-this-if-you-forget-everything**: a number is only as honest as the way it was collected and
  compared — instrument deliberately, randomize the comparison, and assume every surprising result is a
  measurement artifact until it survives scrutiny.
- **Big ideas touched**: `correctness-vs-pragmatism` (a perfectly clean statistical result yields to a
  decision you can defend and ship — you choose a significance bar and a stopping rule, then live with the
  disciplined compromise), `determinism-vs-emergence` (product metrics are emergent signal from thousands
  of independent user choices, not a deterministic output — you measure the aggregate, and separate signal
  from noise rather than reading each event as truth).

## Prerequisites

- **Prior topics**: [topic 10 SQL Essentials](./10-sql-essentials.md) (aggregation, joins, and
  group-by over event tables) and [topic 15 Software Testing](./15-software-testing.md) (the mindset of a
  controlled comparison with a stated hypothesis).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** (fully type-annotated) with a pinned
  CVE-clean data/stats stack (a dataframe library plus a statistics/hypothesis-test module); a local SQL DB
  holding an events table; Neovim/VSCode with the Python LSP (DD-17).
- **Assumed knowledge**: writing an aggregate SQL query over a table (topic 10); stating a hypothesis and
  a pass/fail bar (topic 15); reading typed Python (topic 04).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: the statistical core is stable and correctly left version-unpinned — A/B test
  mechanics (randomized assignment, power/sample-size, p-values, confidence intervals), the classic traps
  (peeking/optional-stopping, multiple comparisons, Simpson's paradox, novelty and primacy effects), and
  funnel/cohort analysis are settled practice, not tooling that goes stale. Keep the Python stats stack at
  "a recent stable" in shipped text.
- 2026-07-12 — verified (GAP for plan owner): no specific analytics vendor or dataframe/stats library
  version is claimed in the body — re-verify exact package versions and any hosted-tool names once the
  worked examples are drafted.

## Items

- Event instrumentation: designing a tracking plan, naming events and properties, idempotent
  client/server events, and avoiding double-counting.
- Funnels & cohorts: conversion funnels, retention cohorts, and segmentation over the events table
  (SQL-first).
- Choosing metrics: a north-star metric, guardrail metrics, and why ratio metrics are treacherous.
- A/B testing mechanics: hypothesis, randomized assignment, sample size and statistical power, and the
  minimum detectable effect.
- Significance and its traps: p-values and confidence intervals, peeking/optional-stopping, multiple
  comparisons, and Simpson's paradox.
- Reading a metric honestly: novelty/primacy effects, seasonality, survivorship, and telling correlation
  from a randomized causal claim.

## Tensions & trade-offs — when NOT to reach for this

- **When NOT to A/B test**: below a traffic threshold you cannot reach statistical power, and a test just
  delays a decision you should make on judgment plus qualitative signal. Underpowered tests produce
  confident-looking noise — worse than no test.
- **Goodhart's law / metrics theater**: the moment a metric becomes a target it stops measuring what you
  cared about. Optimizing a proxy (clicks, session length) can actively harm the real goal (user value,
  retention) while every dashboard turns green.
- **Peeking is not impatience, it's a bug**: repeatedly checking a running experiment and stopping at the
  first significant reading inflates the false-positive rate badly. Fixing a sample size (or using a proper
  sequential method) up front is not optional rigor — it is the difference between a result and an
  artifact.

## Lineage — why it beat the alternative

- Controlled experimentation descends from R. A. Fisher's agricultural randomized trials and the clinical
  RCT: the insight that a randomized control group is the only clean way to separate an intervention's
  effect from everything else changing at once. The web made this cheap and continuous — Google, Microsoft,
  and Amazon industrialized online controlled experiments in the 2000s, replacing argue-from-opinion product
  decisions with measured ones. The through-line: randomize the comparison so the counterfactual is real,
  and treat surprising numbers as artifacts until proven otherwise. The instrumentation and honest-metric
  discipline built here feeds the telemetry and usage signals of the tools you ship next, including
  [`73-building-production-cli-tools`](./73-building-production-cli-tools.md).

## Worked examples

Colocated under `analytics-and-experimentation/learning/code/`; each runnable + exercised from the CLI,
Python fully type-annotated (DD-20/DD-30/DD-34).

- **beginner** — instrument a handful of events into a table, then compute a conversion funnel and a
  retention cohort with SQL + typed Python.
- **intermediate** — analyze a fixed-sample A/B test: compute the effect size, a confidence interval, and
  a p-value with a stated significance bar, and decide ship/no-ship.
- **advanced** — demonstrate a trap and its fix: show how peeking inflates false positives on a simulated
  no-effect experiment, then correct it with a pre-committed sample size (or a sequential rule) and a
  guardrail metric.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: run one honest experiment end to end — instrument the events, build the funnel/cohort view,
  design an A/B test with a pre-committed sample size, analyze it with a confidence interval and a stated
  significance bar, and write a decision memo that names the guardrail metrics and the traps you controlled
  for — all in fully type-annotated Python.
- **Concepts exercised**: [ ] event instrumentation + tracking plan [ ] funnel + retention cohort
  [ ] north-star + guardrail metrics [ ] randomized assignment + pre-committed sample size [ ] confidence
  interval + significance decision [ ] a named-trap check (peeking / multiple comparisons / Simpson's).
- **Ordered steps**:
  1. `.../learning/capstone/code/instrument/` — emit typed events into the DB and build a funnel + cohort
     query. Verify the funnel counts reconcile with the raw event rows (no double-counting).
  2. `.../learning/capstone/code/design/` — state the hypothesis, north-star + guardrail metrics, and
     compute the required sample size for a chosen minimum detectable effect. Verify the power calculation
     runs and outputs a concrete N.
  3. `.../learning/capstone/code/analyze/` — randomize assignment on a provided dataset, compute the effect
     size + confidence interval + p-value. Verify a known-null dataset does not read as significant.
  4. `.../learning/capstone/decision-memo.md` — the ship/no-ship call, the guardrail readings, and the
     specific trap you controlled (peeking, multiple comparisons, or Simpson's). Verify the memo's numbers
     match the analysis output.
- **Acceptance criteria**: instrumentation reconciles with raw events; the sample size is committed before
  analysis; the significance decision follows from a CI + stated bar; a named trap is explicitly checked;
  all Python is type-annotated and runs from the CLI.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Trustworthy Online Controlled Experiments: A Practical Guide to A/B Testing** — Ron Kohavi, Diane
  Tang, Ya Xu (2020). Written by experimentation leaders at Google, LinkedIn, and Microsoft; the standard
  modern reference for running trustworthy A/B tests at scale.
- **Lean Analytics: Use Data to Build a Better Startup Faster** — Alistair Croll, Benjamin Yoskovitz
  (2013). Widely read reference connecting product analytics to actionable business metrics.

**Papers & articles**

- **Seven Pitfalls to Avoid when Running Controlled Experiments on the Web** — Thomas Crook, Brian Frasca,
  Ron Kohavi, Roger Longbotham (2009), KDD. Highly cited paper on common statistical and practical mistakes
  in online experimentation. <https://dl.acm.org/doi/10.1145/1557019.1557139>

---

← Previous: [58 · IT Governance, Risk & Compliance](./58-it-governance-grc.md) · Next: [60 · Just Enough Go](./60-just-enough-go.md) →
