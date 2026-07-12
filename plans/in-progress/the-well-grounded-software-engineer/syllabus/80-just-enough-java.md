# 80 · Just Enough Java (Primer §, Java †)

**prd row**: Pass 4 · Concurrency & Systems · Primer § · Java † · Learn 180 / Drill 280 · Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `§` primer — **just enough modern Java** to be productive on the JVM
([`81-enterprise-java-and-the-jvm`](./81-enterprise-java-and-the-jvm.md)). Build tooling (Maven/Gradle),
records, sealed types, and pattern matching; generics; collections and streams; and the memory model at a
glance. `†`: Java, built and run with a standard JVM build tool.

## Why this exists · the big idea

- **The problem before the solution**: Java carries decades of reputation for verbosity, and an engineer
  arriving from a modern typed language expects the boilerplate of 2005 — meanwhile the language has quietly
  modernized, and the enterprise JVM pass deserves that modern baseline rather than time spent on ceremony.
- **Keep-this-if-you-forget-everything**: modern Java is a statically typed, garbage-collected language whose
  recent additions — records, sealed types, pattern matching, and streams — remove most of the old
  boilerplate; know these and the collections/streams API and you can read and write idiomatic current Java.
- **Big ideas touched**: `taming-state` (the JVM's garbage collector and memory model handle the mutable
  shared state that systems languages made you manage by hand — records push you toward immutable data),
  `abstraction-and-its-cost` (the JVM abstracts away the machine for portability and safety — the cost is a
  runtime and a memory model you occasionally have to reason about explicitly).

## Prerequisites

- **Prior topics**: [topic 8 Object-Oriented Programming Essentials](./08-object-oriented-programming-essentials.md)
  (classes, interfaces, inheritance — Java is the canonical mainstream OO language these map onto).
- **Tools & environment**: a macOS/Linux/Windows machine; a **JDK** pinned to a current LTS and a standard
  build tool (**Maven** or **Gradle**); Neovim/VSCode with the Java LSP (DD-17).
- **Assumed knowledge**: classes/interfaces/inheritance (topic 08); static types and generics from an
  earlier typed language (topics 13/70); running a CLI build tool (topic 05).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: keep the JDK at "a current LTS" in shipped text rather than a pinned number —
  Java ships an LTS every few years (the JLS reference below is the SE 21 edition), and records, sealed
  types, pattern matching, generics, and the streams/collections API are stable, finalized language
  features. Re-pull the exact current LTS at authoring time.
- 2026-07-12 — verified: Maven and Gradle are both current, actively maintained build tools — reference
  them by role and keep any specific version unpinned; the primer stays on the standard library, so no
  third-party dependency version is claimed.

## Items

- Build tooling: a project with Maven or Gradle from the terminal — compile, run, and test.
- Syntax & types; records and sealed types for concise, closed data models.
- Pattern matching (`instanceof` patterns and `switch` patterns) over sealed types.
- Generics and the collections framework: `List`/`Map`/`Set` and their idioms.
- Streams: `map`/`filter`/`collect` pipelines over collections.
- The JVM memory model at a glance: heap/stack, garbage collection, and object identity.

## Worked examples

Colocated under `just-enough-java/learning/code/`; each runnable via the build tool (DD-20/DD-30).

- **beginner** — a Maven/Gradle project that compiles and runs a small program using a record.
- **intermediate** — a sealed type hierarchy consumed with `switch` pattern matching.
- **advanced** — a streams pipeline (`map`/`filter`/`collect`) over a collection, with a unit test.

## Capstone spec — intra-topic (primer → light consolidation)

- **Goal**: build a small modern-Java program that exercises the primer's surface — a record, a sealed type
  consumed with pattern matching, a generic collection, and a streams pipeline — built and run with
  Maven/Gradle plus a passing test, proving readiness for the enterprise JVM pass.
- **Concepts exercised**: [ ] a record [ ] a sealed type + pattern matching [ ] a generic collection [ ] a
  streams pipeline [ ] a Maven/Gradle build [ ] a unit test.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — a Maven/Gradle project with a record and a generic collection. Verify
     the build compiles and running it produces the expected output.
  2. Add a sealed type hierarchy consumed with `switch` pattern matching. Verify each variant is handled
     exhaustively and the match compiles without a default fall-through.
  3. Add a streams pipeline over the collection and a unit test. Verify the pipeline produces the expected
     result and the test passes under the build tool.
- **Acceptance criteria**: the record and generic collection work; the sealed type is matched exhaustively;
  the streams pipeline produces the expected result; the Maven/Gradle build and unit test pass.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Effective Java**, 3rd ed. — Joshua Bloch (2018). The essential idiomatic-Java reference by the former
  Java architect at Sun Microsystems and Google.
- **Head First Java**, 3rd ed. — Kathy Sierra, Bert Bates, Trisha Gee (2022). The most widely recommended
  beginner-friendly introduction to core Java language mechanics.
- **Core Java, Volume I — Fundamentals**, 12th ed. — Cay S. Horstmann (2021). Comprehensive, precise
  reference to Java language fundamentals; a longstanding classic.
- **Java: The Complete Reference**, 13th ed. — Herbert Schildt, Danny Coward (2024). Long-running,
  comprehensive single-volume Java language reference, updated for Java SE 21.

**Papers & articles**

- **The Java Language Specification, Java SE 21 Edition** — Oracle America, Inc. (2023). The official
  normative definition of Java syntax and semantics. <https://docs.oracle.com/javase/specs/jls/se21/jls21.pdf>

---

← Previous: [79 · Modern System Programming](./79-modern-system-programming.md) · Next: [81 · Enterprise Java & the JVM](./81-enterprise-java-and-the-jvm.md) →
