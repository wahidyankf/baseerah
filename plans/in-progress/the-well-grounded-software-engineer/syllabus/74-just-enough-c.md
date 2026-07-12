# 74 · Just Enough C (Primer §, C †)

**prd row**: Pass 4 · Concurrency & Systems · Primer § · C † · Learn 174 / Drill 274 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `§` primer — **just enough C** to be productive in the OS and systems-programming topics
([`75-linux-os`](./75-linux-os.md), [`76-windows-os`](./76-windows-os.md),
[`77-system-programming`](./77-system-programming.md)). The compiler/`make` toolchain, syntax/types,
a pointers intro, arrays/structs, `stdio`, the preprocessor, and a minimal `Makefile`.

## Why this exists · the big idea

- **The problem before the solution**: the OS and systems topics that follow are written against a machine
  that speaks C — without a working grip on pointers, structs, and the compile/link loop, the memory and
  syscall material is unreadable. C is the just-enough key that unlocks it.
- **Keep-this-if-you-forget-everything**: C is a thin, honest layer over the machine — a pointer is just an
  address, a struct is just laid-out bytes — and almost nothing is hidden from you, which is both its power
  and its danger.
- **Big ideas touched**: `abstraction-and-its-cost` — C buys portability over assembly while hiding almost
  nothing; you manage memory and layout yourself, and the machine leaks through every pointer;
  `taming-state` — manual memory means you own each allocation's lifetime by hand, the discipline the later
  systems topics are built on.

## Prerequisites

- **Prior topics**: [topic 4 Just Enough Python](./04-just-enough-python.md) (a high-level contrast) and
  [topic 5 Just Enough Bash](./05-just-enough-bash.md) (compilers, `make`,
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

## Read more

**Books**

- **The C Programming Language**, 2nd ed. — Brian W. Kernighan & Dennis M. Ritchie (1988, Prentice Hall). "K&R" — the field-defining, canonical primer for C, updated for ANSI C.
- **Expert C Programming: Deep C Secrets** — Peter van der Linden (1994, Prentice Hall). The classic deep-dive into C idioms, quirks, and compiler/linker internals.

**Papers & articles**

- **ISO/IEC 9899:2011 (C11)**, public committee draft N1570 — ISO/IEC JTC1/SC22/WG14 (2011). The definitive language standard; N1570 is the last public draft, freely published by the standards committee itself. <https://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf>

---

← Previous: [73 · Building Production CLI Tools](./73-building-production-cli-tools.md) · Next: [75 · Linux OS](./75-linux-os.md) →
