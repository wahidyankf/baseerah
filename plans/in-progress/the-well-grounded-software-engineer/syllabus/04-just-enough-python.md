# 04 · Just Enough Python (Primer, Python)

**prd row**: Pass 1 · First Working Software · Primer · Python · Learn 104 / Drill 204 · Nvim-ready Yes ·
VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: just enough Python to be productive in the Pass 1–3 Python topics; **not** full mastery.
OOP gets a preview here and full treatment in
[`07-object-oriented-programming-essentials`](./07-object-oriented-programming-essentials.md). Python is
the book's primary language (CPython, PSF-license, Tier-1 DD-21).

## Prerequisites

- **Prior topics**: [topic 01 Just Enough Nvim](./01-just-enough-nvim.md) (to edit/run files); the
  [`capstone-forge-ready`](./03-extending-neovim.md) forge is recommended but not required.
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** installed (`python3 --version`) with
  `venv` + `pip`; the `black` and `ruff` CLIs (or installed via `pip`).
- **Assumed knowledge**: basic terminal use; no prior Python required (this is the reader's Python
  starting point).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28). Re-confirm version pins at authoring.

- 2026-07-12 — verified: current CPython stable **3.14.6** (2026-06-10); `python3 -m venv` + `pip`
  workflow unchanged. (python.org)
- 2026-07-12 — verified: `black` **26.5.1**, `ruff` **0.15.20** — both current, CVE-clean, CLI unchanged.
  (black.readthedocs.io / astral.sh)
- 2026-07-12 — verified: `json` stdlib, f-strings, and `match` are unchanged in 3.14 (PEP 750 t-strings
  are an additive complement to f-strings, not a replacement). (docs.python.org whatsnew/3.14)

## Items

- **Running Python raw**: the `python3` interpreter & REPL, running a script, `python -m venv` + `pip`
  from the terminal (no IDE); `black`/`ruff` via CLI.
- **Core syntax**: variables, primitive types (int/float/str/bool/None), operators, f-strings.
- **Collections**: list, tuple, dict, set; slicing; comprehensions.
- **Control flow**: `if`/`elif`/`else`, `for`/`while`, `range`, `enumerate`, `zip`.
- **Functions**: `def`, positional/keyword args, defaults, `*args`/`**kwargs`, return, lambdas, scope.
- **Modules & packages**: `import`, the standard library, `if __name__ == "__main__"`.
- **Errors**: `try`/`except`/`finally`, raising, common built-in exceptions.
- **Files & I/O**: `open` with `with`, reading/writing text, JSON via `json`.
- **OOP preview**: a class at a glance (full treatment in topic 07).

## Worked examples

Colocated under `just-enough-python/learning/code/`; each runs with `python3 <file>` (DD-20/DD-30).

- **beginner** — a runnable script taking `sys.argv`; REPL exploration of types & collections.
- **intermediate** — a function-structured program reading/writing JSON with error handling.
- **advanced** — a small multi-module CLI (`argparse`) run from the terminal, with a `pytest` test.

## Capstone spec — intra-topic (primer → light consolidation)

- **Goal**: write one small (~80–150-line) multi-module Python CLI that reads/validates a JSON input,
  transforms it, writes JSON output, and ships one `pytest` test — exercising collections, functions,
  modules, error handling, and file I/O together.
- **Concepts exercised**: [ ] `argparse` CLI [ ] `venv`+`pip` [ ] collections + comprehensions
  [ ] `try/except` with a raised custom error [ ] `json` read/write with `with` [ ] `if __name__` guard
  [ ] one `pytest` test.
- **Ordered steps**:
  1. `just-enough-python/learning/capstone/code/` — `app/__main__.py` + `app/transform.py` + `tests/`.
     Verify `python3 -m venv .venv && .venv/bin/pip install pytest` succeeds.
  2. Implement `transform.py` (pure function). Verify `.venv/bin/pytest` passes.
  3. Wire `argparse` in `__main__.py` reading a JSON file. Verify `python3 -m app in.json` prints/writes
     the expected JSON and exits 0; a bad file exits non-zero with a clear message.
- **Acceptance criteria**: `pytest` green; the CLI round-trips the sample JSON; invalid input handled
  cleanly; `ruff`/`black` clean.
- **Done bar**: runnable end-to-end + web-verified.

---

← Previous: [03 · Extending Neovim](./03-extending-neovim.md) · Next: [05 · Just Enough Bash](./05-just-enough-bash.md) →
