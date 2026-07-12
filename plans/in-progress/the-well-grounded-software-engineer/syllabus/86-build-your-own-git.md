# 86 · Build Your Own Git (By Example, Python †)

**prd row**: Pass 5 · Internals & Lead at Altitude · By Example · Python † · Learn 186 / Drill 286 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: demystify Git by rebuilding its core — the content-addressed object store
(blobs/trees/commits), refs, the index/staging area, and the everyday commands `commit`/`log`/`checkout`
— written against a real `.git` directory so your reimplementation and the real `git` binary can read
each other's output. This is the build-your-own tier of [`06-version-control-and-git`](./06-version-control-and-git.md):
that topic taught the object-model intuition and the CLI; here you make it concrete by implementing it.
`†`: Python, fully type-annotated (DD-34), verified with `pytest`.

## Why this exists · the big idea

- **The problem before the solution**: Git feels like magic — a fast, distributed, tamper-evident history
  — and that mystery makes it scary to use fully and easy to misuse; the "solution" is to stop treating it
  as a black box and rebuild the handful of ideas underneath, at which point the whole tool becomes obvious.
- **Keep-this-if-you-forget-everything**: Git is a content-addressed key-value store with a thin porcelain
  on top — every object is named by the hash of its content, commits point to trees point to blobs, and
  refs are just named pointers into that graph. Once you see that, branches, merges, and history stop being
  mysterious.
- **Big ideas touched**: `abstraction-and-its-cost` (Git's porcelain hides the object model; rebuilding the
  plumbing shows what the leverage costs and where it leaks — detached HEAD, dangling objects),
  `layering-and-leaks` (blobs → trees → commits → refs is a clean layered stack, and the index is the layer
  that most surprises people until you build it).

## Prerequisites

- **Prior topics**: [topic 6 Version Control & Git](./06-version-control-and-git.md) (the object-model
  intuition — commits/trees/blobs/refs — and everyday CLI fluency this topic makes concrete) and
  [topic 4 Just Enough Python](./04-just-enough-python.md) (the implementation language).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** with type hints (mypy-clean spirit, DD-34);
  `pytest`; the real **`git`** binary installed so you can cross-check your objects against genuine ones;
  Neovim/VSCode with the Python LSP (DD-17).
- **Assumed knowledge**: Git's object model and the porcelain commands (topic 06); Python classes, files,
  and bytes handling (topic 04); hashing intuition (topic 07/17).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: Git's on-disk model — zlib-compressed, SHA-named loose objects under
  `.git/objects`, the `blob`/`tree`/`commit` object headers, refs as files under `.git/refs`, and the
  index format — is stable and correctly left version-unpinned. The reimplementation targets loose objects
  and the index; packfiles are an optional stretch, not required for a readable clone.
- 2026-07-12 — verified (GAP for plan owner): Git is mid-transition from SHA-1 to SHA-256 object naming.
  Build against the repository's default hash (SHA-1 for compatibility with a stock `git init`) and note
  SHA-256 as the forward direction — keep the hash algorithm a named parameter rather than hard-coded.
  (git-scm.com/docs, hash-function-transition)

## Items

- The content-addressed object store: hashing content, the `blob`/`tree`/`commit` object formats, and
  zlib storage under `.git/objects`.
- Plumbing first: `hash-object` and `cat-file` equivalents — write and read objects your `git` can also read.
- Trees & blobs: serializing a directory snapshot into a tree of blobs.
- Commits & refs: building a commit object, and refs/`HEAD` as named pointers into the graph.
- The index / staging area: the file that sits between the working tree and the next commit.
- Porcelain on top: `commit`, `log` (walk the commit graph), and `checkout` (materialize a tree).

## Worked examples

Colocated under `build-your-own-git/learning/code/`; Python (fully type-annotated, DD-34) + `pytest`
(DD-20/DD-30). Every object your code writes is cross-checked against the real `git` binary.

- **beginner** — `hash-object`/`cat-file` equivalents: write a blob into `.git/objects` and read it back;
  verify the real `git cat-file` agrees.
- **intermediate** — snapshot a directory into a tree of blobs and build a commit object with a parent;
  verify `git log` on the real repo shows your commit.
- **advanced** — implement the index plus `commit`/`log`/`checkout` so a full add → commit → checkout cycle
  round-trips and interoperates with the real `git`.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a minimal but interoperable Git — a content-addressed object store (blobs/trees/commits),
  refs and `HEAD`, an index, and the `commit`/`log`/`checkout` porcelain — that reads and writes a real
  `.git` directory, so your implementation and the stock `git` binary can consume each other's objects,
  fully covered by `pytest`.
- **Concepts exercised**: [ ] hash-object + cat-file plumbing [ ] blob/tree serialization [ ] commit objects
  with parents [ ] refs + `HEAD` [ ] an index/staging area [ ] `commit`/`log`/`checkout` porcelain
  [ ] cross-interop with the real `git` [ ] `pytest` coverage of each stage.
- **Ordered steps**:
  1. `.../learning/capstone/code/objects.py` — hash, write, and read `blob`/`tree`/`commit` objects to
     `.git/objects`. Verify the real `git cat-file -p` prints your objects correctly (tests).
  2. `refs.py` + `index.py` — refs/`HEAD` as pointers and an index that stages blobs. Verify staging a file
     records it in the index and a ref update moves `HEAD` (tests).
  3. `porcelain.py` — `commit`, `log` (walk parents), and `checkout` (materialize a tree). Verify an
     add → commit → checkout cycle round-trips and `git log` on the same repo shows the identical history.
- **Acceptance criteria**: objects are byte-compatible with the real `git` (it can read yours, you can read
  its); refs/`HEAD`/index behave; the commit graph walks correctly; checkout materializes the right tree;
  `pytest` covers each stage.
- **Done bar**: runnable end-to-end + interoperates with the real `git` + tests green + web-verified.

## Read more

**Books**

- **Pro Git** — Scott Chacon, Ben Straub (2nd ed., 2014). The official, comprehensive Git book, whose
  internals chapters (objects, refs, packfiles) are exactly what you reimplement here; freely licensed.
  <https://git-scm.com/book/en/v2>

**Papers & articles**

- **Write Yourself a Git!** — Thibault Polge. Free, widely cited tutorial that walks through implementing
  Git's core plumbing commands from scratch. <https://wyag.thb.lt/>
- **Git from the Bottom Up** — John Wiegley (2009). Early, influential free explainer of Git's object model
  and internals. <https://jwiegley.github.io/git-from-the-bottom-up/>

---

← Previous: [85 · Compilers, Parsers & Transpilers](./85-compilers-parsers-and-transpilers.md) · Next: [87 · Build Your Own Database](./87-build-your-own-database.md) →
