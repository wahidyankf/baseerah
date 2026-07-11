# 15 · Computer Science Foundations (Annotated-concept, Python \*)

**prd row**: Pass 2 · Solidify the Core · Annotated-concept · Python \* · Learn 115 / Drill 215 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the CS bedrock a self-taught-style engineer usually skips — data representation, logic,
machine organization, automata, computability/complexity, and information theory — at intuition depth,
grounded in small runnable Python demonstrations (`*`: Python where code appears, else prose + diagrams).
Depth in specific areas is spread across [`20-advanced-algorithms`](./20-advanced-algorithms.md) and
[`57-type-systems`](./57-type-systems.md); this topic builds the mental model they hang on.

## Prerequisites

- **Prior topics**: [topic 04 Just Enough Python](./04-just-enough-python.md) (the small demonstrations
  are Python); [topic 06 Data Structures & Algorithms Essentials](./06-data-structures-and-algorithms-essentials.md)
  gives the stack/heap and Big-O vocabulary this topic deepens.
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x**; a REPL for number/representation demos.
- **Assumed knowledge**: reading/writing basic Python; comfort with arithmetic and simple algebra (no
  formal CS or discrete-math background assumed — the topic builds it).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: IEEE-754 (754-2019 current revision, no successor), two's-complement, and UTF-8
  (RFC 3629, unchanged since 2003) are stable specs; P-vs-NP remains open in 2026 (Clay Millennium Prize
  unclaimed) — the syllabus's "intuition" framing claims no resolution, so it is safe; Chomsky-hierarchy
  terminology unchanged. Python `struct`/`bin`/`hex` behavior long-stable — spot-check `docs.python.org` at
  authoring. (ieee.org / ietf.org / claymath.org)

## Items

- Number systems & data representation: binary/hex, two's complement, IEEE-754 floats, endianness,
  Unicode/UTF-8.
- Boolean logic & digital abstraction: gates, truth tables, combinational vs sequential.
- Computer organization: CPU/registers/ALU, memory hierarchy (cache/RAM/disk), the stack & heap.
- Automata & formal languages: finite automata, regular vs context-free, the Chomsky hierarchy (survey).
- Computability & complexity classes: Turing machines (concept), the halting problem, P vs NP (intuition).
- Information & encoding: entropy intuition, lossless vs lossy, checksums/hashing basics.

## Worked examples

Colocated under `computer-science-foundations/learning/code/`; runnable Python + WCAG-accessible Mermaid
where code does not fit (DD-20/DD-30).

- **representation** — represent & convert integers/floats; demonstrate float rounding error and its
  mitigation (`0.1 + 0.2`), shown in the REPL.
- **automata** — hand-trace a small finite automaton recognizing a language; map a regex to its FA
  (diagram + a tiny Python simulator).
- **organization** — walk a function call through the stack frame; show cache-friendly vs cache-hostile
  array traversal and time both.
- **complexity** — classify sample problems tractable vs intractable with reasoning (P/NP intuition).

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a small "CS foundations toolkit" in Python — a base/representation converter (incl. an
  IEEE-754 float inspector), a finite-automaton simulator that accepts/rejects strings for a given
  language, and a stack-frame + cache-traversal timing demo — each output explained against the theory.
- **Concepts exercised**: [ ] two's-complement + IEEE-754 representation [ ] a regex→FA mapping run by a
  simulator [ ] call-stack tracing [ ] cache-friendly vs hostile access timing [ ] a checksum/hash demo.
- **Ordered steps**:
  1. `.../learning/capstone/code/represent.py` — int/float ↔ binary/hex converter + float-bit inspector.
     Verify it prints the exact bit pattern for a known value and demonstrates `0.1+0.2 != 0.3`.
  2. `automaton.py` — an FA simulator; feed it accept/reject strings for one regular language. Verify each
     string is classified correctly against a hand-traced expectation.
  3. `memory.py` — time row-major vs column-major traversal of a 2-D array. Verify the cache-friendly order
     is measurably faster and explain why.
- **Acceptance criteria**: each tool runs from the CLI with the documented output; the FA matches the
  hand trace; the timing demo shows the expected ordering; every result is tied back to the theory.
- **Done bar**: runnable end-to-end + web-verified.

---

← Previous: [14 · Security Essentials](./14-security-essentials.md) · Next: [16 · Object-Oriented Design & Patterns](./16-object-oriented-design-and-patterns.md) →
