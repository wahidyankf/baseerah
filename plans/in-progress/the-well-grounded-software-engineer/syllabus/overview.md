# Syllabus Overview — The Well-Grounded Software Engineer

This `syllabus/` folder is the **per-topic detail layer** for the section defined in
[prd.md](../prd.md) — **the single source of truth** for topic set, pass, format, primary language,
weights, and editor-readiness. This folder never restates weights; it adds the dimension the prd
table cannot hold: for **each** topic, the concrete **Items** (subtopics the learning subtree and
drilling page must cover), the named **Worked examples**, the **Capstone spec** (intra-topic, and any
inter-topic capstone anchored at that topic), and the dated **Accuracy notes** from the pre-authoring
`web-researcher` sweep.

Per the [Syllabus as a Folder decision (DD-29)](../prd.md#syllabus-as-a-folder-dd-29), the detail is
split one file per topic — `NN-<slug>.md`, where `NN` = **order of appearance** (`01`, `02`, … `61`)
matching the prd journey index. The [README](./README.md) indexes all 61 files.

## How to read a topic file

Topic order, slugs, format, and primary language come from the [prd table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks).
Each `NN-<slug>.md` file carries these sections in order:

1. **Header** — title, prd row echo (pass, format, primary language), scope note.
2. **Prerequisites** — what a reader must already have to follow the material successfully (see
   [Prerequisites Clarity Rule](#prerequisites-clarity-rule-hard-rule-dd-31) below).
3. **Accuracy notes (web-verified)** — dated `web-researcher` findings folded in **before** authoring
   (DD-28): current versions, current API/CLI syntax, license status, CVE status, best-practice deltas.
4. **Items** — the concrete subtopics the learning subtree and drilling page must cover (a **floor**,
   not a cap — DD-8).
5. **Worked examples** — the named examples the topic authors. For **By Example**/**Primer** topics they
   map to the `by-example/{beginner,intermediate,advanced}` progression; for **Annotated-concept** topics
   they map to per-theme worked-example pages (with WCAG-accessible Mermaid diagrams where code does not
   fit).
6. **Capstone spec** — the full spec (see [Capstone Policy](#capstone-policy-dd-27) below) for this
   topic's intra-topic capstone, and — where this topic is the anchor of a pass boundary or a
   cross-cutting junction — the inter-topic capstone spec too.
7. **Navigation footer** — explicit **← Previous** / **Next →** links to the adjacent material in
   reading order (see [Prev/Next Navigation Rule](#prevnext-navigation-rule-hard-rule-dd-32) below).

Every `apps-ayokoding-www-*-maker` step in [delivery.md](../delivery.md) reads the matching
`NN-<slug>.md` file and must cover **every listed item and worked example**, and build **every listed
capstone**, to the mastery bar (DD-8).

## Legend (from the prd table)

- `†` — platform-/subject-mandated language exception to the Python primary (e.g. SQL, Cypher, Lua, Go,
  Elixir, Kotlin, Swift, C#, C, YAML/HCL, Bash, PowerShell, Scheme/Clojure, OCaml/Haskell/F#).
- `*` — concept-centric topic; **Python** wherever code appears, otherwise prose + diagrams.
- `**` — survey topic (Programming Paradigms) anchored in Python, other languages shown illustratively.
- `§` — tool primer (vanilla Neovim, no plugins); `:set`/ex-commands and motions, not a language.
- `‡` — leadership/governance topic, minimal-to-no code; prose, worked scenarios, diagrams.
- `◆` — parallel app-domain topic; `▲` — parallel Product & Delivery track topic.

## Cross-cutting authoring guarantees

**Coverage is a floor, not a cap** (DD-8): the items/examples/capstones below are the minimum surface a
topic must reach to leave a reader well-grounded; a maker may add more, never fewer.

**Raw-form-first tooling** (DD-17): every topic assumes the reader edits in **Neovim** and drives
build/run/test/debug/git from the terminal on a macOS/Linux-compatible environment, learning the raw
command rather than an IDE gesture. IDE-mandatory app domains (iOS→Xcode, Android→Android
Studio/Gradle, Windows→Visual Studio/.NET) are called out in place and still show the CLI form where
possible. See [prd.md §Tooling & Environment Stance](../prd.md#tooling--environment-stance-raw-form-first).

**Editor prologue first** (Pass 0): the editor is taught before any programming topic —
`just-enough-nvim` (vanilla, zero plugins) → `just-enough-lua` (the config language) → `extending-neovim`
(plugins, LSP, DAP, Treesitter, completion).

**Split-and-interleave** (DD-11): seven subjects ship an Essentials topic early and an Advanced topic on
a later pass (DS&A, SQL, Backend, OOP, Networking, Frontend, Security). Each Essentials file states what
it defers; each Advanced file back-references its Essentials.

**Free-to-use-and-teachable-first materials** (DD-21): every language, tool, database, dataset, and
standard named is free to obtain and legal to author training material on — Tier-1 OSS/public-domain by
default, Tier-2 free-but-proprietary (Xcode, Android SDK, Visual Studio Community) only where a domain
requires it. ISO 27001 and SOC 2 are landscape context only, never reproduced. See
[prd.md §Materials policy](../prd.md#personas-materials-policy--free-to-use-and-teachable-first-hard-rule-dd-21).

**CVE-free dependencies** (DD-23): every dependency an example asks the reader to install is
standard-library-first, pinned to an exact CVE-clean version, verified across NVD / GitHub Advisories /
Snyk / the vendor page / CISA KEV. See
[prd.md §CVE-free dependencies](../prd.md#cve-free-dependencies--safe-supply-chain-first-hard-rule-dd-23).

**Colocated runnable code** (DD-24): each topic's runnable files live in its colocated Hugo page-bundle
`code/` directory (`<slug>/learning/code/`, `<slug>/learning/capstone/code/`, `<slug>/drilling/code/`,
and `<capstone-slug>/code/` for inter-topic capstones), excluded from the app's Nx build/test/lint gates
but held to the runnable-example rule.

## Accuracy Verification (HARD RULE, DD-28)

**Every topic is web-verified before it is authored.** The pre-authoring `web-researcher` sweep records
its dated findings in this file's **Accuracy notes (web-verified)** block; the maker authors against
those findings, and `apps-ayokoding-www-facts-checker` re-verifies the rendered pages. The sweep runs
**sequentially, one topic at a time**, to bound token usage. See
[prd.md §Accuracy Verification Rule](../prd.md#accuracy-verification-rule-hard-rule-dd-28).

## Prerequisites Clarity Rule (HARD RULE, DD-31)

**Every topic states its prerequisites up front, so a reader knows exactly what they must already have to
follow the material successfully — before they start.** Each `NN-<slug>.md` carries a **Prerequisites**
section, and the authored topic surfaces the same list in its `learning/_index.md` intro. Prerequisites
are stated in four concrete kinds (omit a kind only when genuinely none apply):

- **Prior topics** — the earlier topics in journey order whose knowledge this topic builds on, named and
  linked (e.g. "topic 04 Just Enough Python", "topic 08 SQL Essentials"). This makes the spiral's
  dependencies explicit rather than implied by order.
- **Tools & environment** — the exact toolchain the reader must have installed and working, with pinned
  versions (e.g. "Python 3.x", "the `capstone-forge-ready` Neovim forge", "Docker"), plus the OS/platform
  assumption (macOS/Linux terminal by default; any Partial-editor platform SDK called out — DD-25).
- **Assumed knowledge/skills** — concepts the reader is expected to already be comfortable with (e.g.
  "reading and writing basic Python", "using the terminal and git"), distinct from what this topic
  teaches.
- **First-topic exception** — [topic 01](./01-just-enough-nvim.md) is the true entry point: its only
  prerequisites are a computer with a macOS/Linux-compatible terminal and the willingness to learn; it
  assumes **no** prior programming.

Prerequisites must be honest and minimal: list what is genuinely required, not a wish-list. A topic that
silently assumes knowledge it never names, or a tool it never told the reader to install, violates both
this rule and Follow-Along Completeness (DD-30). `apps-ayokoding-www-*-checker` verifies the rendered
`learning/_index.md` states prerequisites; `plan-checker` flags a topic file missing the section.

## Prev/Next Navigation Rule (HARD RULE, DD-32)

**Every material file carries an explicit link to the previous and the next material in reading order** —
so a reader can walk the whole journey forward or backward without guessing what comes next. This binds
**both** layers:

- **This plan's `syllabus/NN-<slug>.md` files** — each ends with a `---` rule then a navigation footer:
  `← Previous: [NN-1 · Title](./NN-1-<slug>.md) · Next: [NN+1 · Title](./NN+1-<slug>.md) →`. The first
  file ([01](./01-just-enough-nvim.md)) points **Previous** at [README](./README.md); the last file
  ([61](./61-engineering-management.md)) points **Next** at [overview](./overview.md) (journey complete).
  Order is the prd journey index (01 → 61); inter-topic capstone specs, being anchored inside a topic
  file, inherit that file's footer.
- **The eventual `apps/ayokoding-www` content pages** — every authored page (`_index.md` for each
  `<slug>/`, `learning/`, `drilling/`, `capstone/`, and each worked-example/drill leaf) ends with the
  same **← Previous / Next →** footer, threading the section's reading order: within a topic
  learning → drilling → intra-topic capstone; across topics topic N → topic N+1 in journey order; and
  the inter-topic capstone bundles sit at their pass/junction boundary in the chain. Links are relative
  (`../<slug>/`) or the site URL where a relative link cannot express the hop; every link resolves (no
  dangling prev/next). This is a superset of the platform's `_index.md` weight ordering — the weights fix
  the order; DD-32 makes the prev/next hop **explicit on the page**.
- **Multi-page topics thread through _every_ internal page (HARD sub-rule)** — most topics are not a
  single page: a topic's subtree is `learning/_index.md` → each `learning/` worked-example/theme **leaf**
  in weight order → `learning/capstone/_index.md` → `drilling/_index.md` → each `drilling/` **leaf** in
  weight order (and any inter-topic `capstone-*/_index.md` at its boundary). The prev/next chain must walk
  **that full page sequence in true reading order** — never skipping a leaf and never jumping straight
  topic-index → topic-index over the leaves in between. Concretely: a topic's **first** page (its
  `learning/_index.md`) sets **Previous** to the **last page of the previous topic** (that topic's final
  drill leaf, or its intra-topic capstone if drilling has no leaves), and a topic's **last** page sets
  **Next** to the **first page of the next topic** (its `learning/_index.md`). Every intermediate leaf
  points Previous/Next at its true neighbour leaf. The maker computes this per topic from the actual page
  inventory it authored — the linear order is: prev-topic's last leaf → this topic's learning index →
  learning leaves (weight order) → intra-capstone → drilling index → drilling leaves (weight order) →
  next-topic's learning index. `apps-ayokoding-www-*-checker` verifies the walk is complete and in order
  across a multi-page topic (no skipped or mis-ordered leaf, no cross-boundary mistargeting).
- **Prev/next commonly point _within the same topic_** — this is expected, not an error. Because most
  topics are multi-page, the majority of prev/next hops stay inside one topic (learning index → a
  learning leaf → the next learning leaf → the intra-topic capstone → the drilling index → a drill leaf).
  A prev/next link only crosses a topic boundary at a topic's **first** page (Previous → prior topic) and
  **last** page (Next → next topic). So both a same-topic target and a cross-topic target are valid — the
  test is only that the link points to the true neighbour in the linear reading order above.

The maker authors the footer; `apps-ayokoding-www-*-checker` verifies every rendered page has a resolving
prev/next footer in the correct journey position; `plan-checker` flags a `syllabus/` file missing the
footer. A prev/next link that 404s or points out of order is a defect, not a nicety.

## Follow-Along Completeness (HARD RULE, DD-30)

Every worked example and every capstone is **followable step-by-step, code-by-code, line-by-line, with
no hidden assumptions**: prerequisites + exact pinned versions + install/run commands stated up front;
no elided `...`-only listings presented as runnable (fragments are assembled into a complete listing on
the same page); every command shown verbatim with its observable expected result. See
[prd.md §Follow-Along Completeness Rule](../prd.md#follow-along-completeness-rule-hard-rule-dd-30).

## Capstone Policy (DD-27)

Every topic ships an **intra-topic capstone**, and the section ships **10 inter-topic capstones** (6
pass-boundary + 4 cross-cutting). All capstones are self-contained, follow-along-complete (DD-30), and
web-verified (DD-28). Size is uncapped; correctness, accuracy, detail, and clarity are the bar. See
[prd.md §Capstone Policy](../prd.md#capstone-policy-dd-27).

**Intra-topic capstone — scaled by topic kind:**

- **Subject topics** → a full runnable capstone (one cohesive project exercising the topic end-to-end).
- **The 11 _Just Enough_ primers** → a light consolidation exercise (a short program using the
  just-learned features together).
- **Leadership/governance `‡` topics** → a design/decision capstone producing an artifact (decision
  record, governance matrix, runbook); no code.

**Inter-topic capstones — inline milestone bundles at the section root:**

| Capstone slug                      | Kind          | Junction (topics integrated)                               | Anchored in file                     |
| ---------------------------------- | ------------- | ---------------------------------------------------------- | ------------------------------------ |
| `capstone-forge-ready`             | pass-boundary | Pass 0 (01–03: nvim + lua + extending)                     | `03-extending-neovim.md`             |
| `capstone-first-working-software`  | pass-boundary | Pass 1 (04–14: build → store → test → secure)              | `14-security-essentials.md`          |
| `capstone-full-stack-app`          | cross-cutting | Frontend (12) + Backend (09) + SQL (08)                    | `14-security-essentials.md`          |
| `capstone-solid-core`              | pass-boundary | Pass 2 (15–25)                                             | `25-project-management.md`           |
| `capstone-real-world-delivery`     | pass-boundary | Pass 3 (26–40)                                             | `40-defensive-security.md`           |
| `capstone-secure-service`          | cross-cutting | Backend (09) + Security Essentials (14) + IT Security (38) | `40-defensive-security.md`           |
| `capstone-data-pipeline`           | cross-cutting | Data Engineering (36) + SQL/NoSQL (08/26) + a queue        | `40-defensive-security.md`           |
| `capstone-concurrency-and-systems` | pass-boundary | Pass 4 (41–59)                                             | `59-site-reliability-engineering.md` |
| `capstone-concurrency-showdown`    | cross-cutting | Concurrency Core (19) + CSP/Go (42) + Actor/Elixir (44)    | `59-site-reliability-engineering.md` |
| `capstone-lead-at-altitude`        | pass-boundary | Pass 5 (60–61)                                             | `61-engineering-management.md`       |

**Every capstone spec states**: (a) goal/outcome, (b) a concepts-exercised checklist, (c) an ordered
step outline (each step naming a file + the code + the verify command), (d) testable acceptance
criteria, and (e) the done bar = **"runnable end-to-end + web-verified"** (or "produces the stated
artifact + web-verified" for `‡` leadership capstones).

## Per-topic file template

Each `NN-<slug>.md` is authored to this skeleton:

```markdown
# NN · <Title> (<Format>, <Primary language>)

**prd row**: Pass <P> · <Format> · <Primary language> · Learn <Lwt> / Drill <Dwt>.

**Scope note**: <what this topic covers; what it defers to an Advanced/later topic>.

## Prerequisites

- **Prior topics**: <linked earlier topics this builds on, or "none — this is the entry point">.
- **Tools & environment**: <pinned toolchain the reader must have installed + OS/platform assumption>.
- **Assumed knowledge**: <concepts the reader must already be comfortable with>.

## Accuracy notes (web-verified)

- <YYYY-MM-DD> — <web-researcher finding: current version / API / license / CVE / best practice>.

## Items

- <concrete subtopic> …

## Worked examples

- **<name>** (<beginner|intermediate|advanced or theme>) — <one-line spec>.

## Capstone spec — intra-topic (<kind>)

- **Goal**: …
- **Concepts exercised**: [ ] … [ ] …
- **Ordered steps**: 1. `<file>` — <code> — verify `<command>` …
- **Acceptance criteria**: …
- **Done bar**: runnable end-to-end + web-verified.

<!-- Inter-topic capstone spec block appended only in the anchor files listed in the Capstone Policy table -->

---

← Previous: [<NN-1> · <Title>](./<NN-1>-<slug>.md) · Next: [<NN+1> · <Title>](./<NN+1>-<slug>.md) →
```
