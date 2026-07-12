# 77 · System Programming (By Example, C †)

**prd row**: Pass 4 · Concurrency & Systems · By Example · C † · Learn 177 / Drill 277 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: programming close to the metal in C — the memory model (stack/heap/`malloc`/`free`),
undefined behavior & safety, manual resource management (C has no RAII — cleanup is explicit), low-level
data (bits/unions/endianness/serialization), building & linking (the ABI), and interfacing with the OS.
Builds on the OS topics ([`75-linux-os`](./75-linux-os.md)) and the CS memory foundations
([`19-computer-science-foundations`](./19-computer-science-foundations.md)).

## Why this exists · the big idea

- **The problem before the solution**: C hands you the machine with no guardrails — no garbage collector,
  no RAII, no bounds checks — so a single mismanaged pointer becomes a use-after-free, a buffer overflow,
  or silent undefined behaviour that ships and corrupts memory in production. This topic is about doing it
  safely, by discipline.
- **Keep-this-if-you-forget-everything**: without RAII, every resource you acquire is one you must
  explicitly release on every path — ownership in C is a discipline you enforce by hand, not a guarantee
  the language gives you.
- **Big ideas touched**: `taming-state` — manual `malloc`/`free`, fd ownership, and by-hand cleanup are a
  discipline for containing mutable resources the language won't manage for you; `layering-and-leaks` —
  linking, the ABI, endianness, and syscalls are where your program meets the layers beneath it, and
  undefined behaviour is the machine leaking through the abstraction.

## Prerequisites

- **Prior topics**: [topic 74 Just Enough C](./74-just-enough-c.md) (the language),
  [topic 75 Linux OS](./75-linux-os.md) (syscalls, the process/memory model), and
  [topic 19 Computer Science Foundations](./19-computer-science-foundations.md) (data representation,
  memory).
- **Tools & environment**: a macOS/Linux terminal; **gcc/clang** + make + **valgrind**/**AddressSanitizer**
  for memory checking; Neovim/VSCode (DD-17).
- **Assumed knowledge**: C pointers/structs + a `make` build (topic 74); the process/memory/syscall model
  (topic 75); binary/number representation (topic 19).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: stack/heap, `malloc`/`free`, alignment, UB categories (buffer overflow,
  use-after-free, integer overflow), ASan (built into GCC/Clang) + Valgrind, static-vs-dynamic linking,
  and ABI concepts are evergreen/unchanged. The `__attribute__((cleanup))` note is correctly scoped as a
  **GCC/Clang extension** (not standard C, not portable to MSVC) presented among a portability spectrum.

## Items

- Memory model: stack vs heap, pointers, `malloc`/`free`, alignment, ownership discipline.
- Undefined behavior & safety: buffer overflows, use-after-free, integer overflow.
- Manual resource management: file descriptors, scope-based cleanup by hand (the goto-cleanup pattern,
  `__attribute__((cleanup))`), error/`errno` handling. (C has no RAII — cleanup is explicit.)
- Low-level data: bit manipulation, structs/unions, endianness, serialization.
- Building & linking: compilation units, headers, static vs dynamic linking, the ABI (concept).
- Interfacing with the OS: syscalls, signals, basic sockets.

## Worked examples

Colocated under `system-programming/learning/code/`; C, memory-checked with ASan/valgrind (DD-20/DD-30).

- **beginner** — a dynamic array in C with correct `malloc`/`realloc`/`free`; bit-manipulation utilities.
- **intermediate** — a linked structure with disciplined ownership + cleanup; `errno`-based error handling.
- **advanced** — a small allocator or memory pool; a minimal socket client/server; a serialization routine
  with endianness handling.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a small systems component in C — a dynamic data structure or memory pool with disciplined
  manual ownership + cleanup, a serialization routine that handles endianness, and a minimal socket
  client/server — that runs **memory-clean under AddressSanitizer/valgrind** (no leaks, no use-after-free,
  no overflow), demonstrating safe systems programming without RAII.
- **Concepts exercised**: [ ] correct `malloc`/`realloc`/`free` ownership [ ] scope-based manual cleanup
  (goto-cleanup / `__attribute__((cleanup))`) [ ] `errno`-based error handling [ ] endianness-aware
  serialization [ ] a minimal socket client/server [ ] a clean ASan/valgrind run.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — a dynamic structure or memory pool with disciplined alloc/free +
     cleanup. Verify it runs **leak-free** under valgrind/ASan.
  2. Add an endianness-aware serialization routine. Verify round-tripping a value across serialize/
     deserialize preserves it regardless of host endianness.
  3. Add a minimal socket client/server exchanging serialized data. Verify a message round-trips and the
     whole program stays ASan/valgrind-clean.
- **Acceptance criteria**: ownership + cleanup are disciplined; serialization is endianness-correct; the
  socket exchange works; the entire program is **memory-clean** under AddressSanitizer/valgrind.
- **Done bar**: runnable end-to-end + memory-clean + web-verified.

## Read more

**Books**

- **The C Programming Language**, 2nd ed. — Brian W. Kernighan, Dennis M. Ritchie (1988). The original, definitive book on C, written by the language's co-creator; the field's baseline reference for decades.
- **Advanced Programming in the UNIX Environment**, 3rd ed. — W. Richard Stevens, Stephen A. Rago (2013). The canonical guide to Unix/POSIX systems programming: syscalls, processes, files, threads, and IPC.
- **The Linux Programming Interface** — Michael Kerrisk (2010). The most comprehensive modern single-volume reference for Linux/Unix system calls and the C library, by the Linux man-pages maintainer.
- **Operating Systems: Three Easy Pieces** — Remzi H. Arpaci-Dusseau, Andrea C. Arpaci-Dusseau. Free, widely-adopted OS textbook covering virtualization, concurrency, and persistence from first principles. <https://pages.cs.wisc.edu/~remzi/OSTEP/>
- **The Art of Unix Programming** — Eric S. Raymond (2003). Canonical treatment of Unix design philosophy and programming culture, freely licensed by the author. <http://www.catb.org/esr/writings/taoup/html/>

---

← Previous: [76 · Windows OS](./76-windows-os.md) · Next: [78 · Just Enough Rust](./78-just-enough-rust.md) →
