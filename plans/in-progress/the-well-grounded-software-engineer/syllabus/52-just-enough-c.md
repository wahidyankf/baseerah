# 52 · Just Enough C (Primer §, C †)

**prd row**: Pass 4 · Concurrency & Systems · Primer § · C † · Learn 152 / Drill 252 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `§` primer — **just enough C** to be productive in the OS and systems-programming topics
([`53-linux-os`](./53-linux-os.md), [`54-windows-os`](./54-windows-os.md),
[`55-system-programming`](./55-system-programming.md)). The compiler/`make` toolchain, syntax/types,
a pointers intro, arrays/structs, `stdio`, the preprocessor, and a minimal `Makefile`.

## Prerequisites

- **Prior topics**: [topic 04 Just Enough Python](./04-just-enough-python.md) (a high-level contrast) and
  [topic 05 Just Enough Bash](./05-just-enough-bash.md) (compilers, `make`,
  the build loop).
- **Tools & environment**: a macOS/Linux terminal; **gcc/clang** + **make**; Neovim/VSCode with a C LSP
  (DD-17).
- **Assumed knowledge**: variables/functions/loops in some language (topic 04); running CLI tools + a build
  step (topic 05).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified (nuance): **C23 is published as ISO/IEC 9899:2024** (2024-10-31), and **GCC 15
  (~Apr 2025) made `-std=c23`/`gnu23` the default C dialect** — so C23 is now the practical default in
  current GCC. Clang has only **partial** C23 support (`-std=c23` since Clang 18); MSVC lags. The successor
  **C2y** is an early WG14 draft with no release date. Given GCC's default shift, consider leading with C23
  while keeping **C17 as the conservative portability baseline** (esp. vs Clang's partial coverage);
  re-verify Clang C23 completeness at authoring time. (gcc.gnu.org/projects/c-status.html / clang.llvm.org/c_status.html)
- 2026-07-12 — verified: gcc/clang + make toolchain, `Makefile` conventions, and `-Wall -Wextra` are
  evergreen/unchanged.

## Items

- `gcc` / `clang` and `make` from the CLI; compile & link a program.
- Syntax & types; a pointers intro; arrays; structs; `stdio`.
- The preprocessor; headers; a minimal `Makefile`.

## Worked examples

Colocated under `just-enough-c/learning/code/`; each built via gcc/clang + make (DD-20/DD-30).

- **beginner** — compile & run a C program with `gcc`.
- **intermediate** — pointers + structs.
- **advanced** — a small multi-file build driven by `make`.

## Capstone spec — intra-topic (primer → light consolidation)

- **Goal**: build a small multi-file C program driven by a `Makefile` that exercises the primer's surface —
  pointers, arrays, structs, `stdio`, headers, and the preprocessor — compiling cleanly with warnings on,
  proving readiness for the OS/systems topics.
- **Concepts exercised**: [ ] a `Makefile`-driven multi-file build [ ] pointers + arrays [ ] structs
  [ ] `stdio` I/O [ ] headers + the preprocessor.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — a program split across a header + two source files using structs +
     pointers + `stdio`. Verify it compiles warning-clean (`-Wall -Wextra`).
  2. `Makefile` — build + clean targets. Verify `make` produces the binary and `make clean` removes
     artifacts.
  3. Run it on sample input. Verify the output matches the expected result.
- **Acceptance criteria**: the multi-file build works via `make`; pointers/structs/`stdio` behave; the
  program compiles warning-clean and produces correct output.
- **Done bar**: runnable end-to-end + web-verified.

---

← Previous: [51 · Linux App Development](./51-linux-app-development.md) · Next: [53 · Linux OS](./53-linux-os.md) →
