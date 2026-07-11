# 58 · Compilers, Parsers & Transpilers (By Example, Python)

**prd row**: Pass 4 · Concurrency & Systems · By Example · Python · Learn 158 / Drill 258 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: how a language processor works front-to-back — lexing, parsing (to an AST), semantic
analysis, and either interpreting/evaluating or emitting code — by building a small language end-to-end in
Python. **Motivation (DD-16)**: in the AI-assisted era, compilers/type-checkers/linters are your
**guardrails** — understanding how they parse and reason about code makes you a sharper reader and reviewer
of both hand-written and AI-generated code.

## Prerequisites

- **Prior topics**: [topic 04 Just Enough Python](./04-just-enough-python.md) (the implementation language),
  [topic 15 Computer Science Foundations](./15-computer-science-foundations.md) (trees, recursion, grammars),
  and [topic 57 Type Systems](./57-type-systems.md) (ADTs + pattern matching make an AST + evaluator natural
  — the immediately-prior topic).
- **Tools & environment**: **Python 3.x**; `pytest`; optionally a parser-generator to contrast with the
  hand-written recursive-descent parser; Neovim/VSCode (DD-17).
- **Assumed knowledge**: Python classes + recursion (topic 04); trees + grammar intuition (topic 15); sum
  types / pattern matching as a way to shape an AST (topic 57).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: the pipeline (lexer → parser/AST → semantic analysis → interpret/emit),
  recursive-descent + Pratt parsing (precedence-climbing operator parsing), tree-walking interpreter +
  environments/scopes, and transpilation are evergreen/unchanged. The file keeps parser-generators generic
  ("optionally a parser-generator"), avoiding a version-pinned tool — good; nothing to correct.

## Items

- The pipeline: source → lexer/tokens → parser/AST → semantic analysis → interpret or emit.
- Lexing: tokenizing input; handling whitespace/comments/errors.
- Parsing: grammars, recursive descent, precedence (Pratt parsing), building an AST.
- Evaluation: a tree-walking interpreter; environments/scopes.
- Transpilation: emitting target code (e.g. to Python or JS) instead of interpreting.
- **The guardrail lens (DD-16)**: how type-checkers/linters use the same front-end to catch errors — why
  understanding this makes you a better reviewer of AI-generated code.

## Worked examples

Colocated under `compilers-parsers-and-transpilers/learning/code/`; Python + `pytest` (DD-20/DD-30).

- **beginner** — a lexer that tokenizes a small expression language (+ tests).
- **intermediate** — a recursive-descent/Pratt parser building an AST with correct precedence (+ tests).
- **advanced** — a tree-walking interpreter that evaluates the AST; a transpiler variant that emits target
  code instead.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a small but complete language processor in Python — lexer → recursive-descent/Pratt parser
  (correct precedence) → AST → **both** a tree-walking interpreter that evaluates programs **and** a
  transpiler that emits equivalent target code — fully covered by `pytest`, demonstrating the front-end that
  every compiler/type-checker/linter shares.
- **Concepts exercised**: [ ] a lexer/tokenizer with error handling [ ] a recursive-descent/Pratt parser
  with correct precedence [ ] an AST [ ] a tree-walking interpreter with scopes/environments [ ] a
  transpiler emitting target code [ ] `pytest` coverage of each stage.
- **Ordered steps**:
  1. `.../learning/capstone/code/lexer.py` — tokenize the source language. Verify tests cover tokens,
     whitespace/comments, and a lexer error.
  2. `parser.py` — recursive-descent/Pratt parser → AST with correct operator precedence. Verify
     precedence-sensitive expressions parse to the right tree (tests).
  3. `interpreter.py` — evaluate the AST with scopes; `transpiler.py` — emit equivalent target code. Verify
     the interpreter produces correct results and the transpiled output, when run, matches the interpreter.
- **Acceptance criteria**: the full pipeline works; precedence is correct; interpreter results and
  transpiler output agree; `pytest` covers each stage; the guardrail framing is stated.
- **Done bar**: runnable end-to-end + tests green + web-verified.

---

← Previous: [57 · Type Systems](./57-type-systems.md) · Next: [59 · Site Reliability Engineering](./59-site-reliability-engineering.md) →
