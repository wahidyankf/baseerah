# 81 · Enterprise Java & the JVM (By Example, Java †)

**prd row**: Pass 4 · Concurrency & Systems · By Example · Java † · Learn 181 / Drill 281 · Nvim-ready
Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: enterprise Java as it is actually built and run — the Spring/Spring Boot ecosystem,
dependency injection as the organizing principle, and the JVM underneath (JIT, garbage collection, the
memory model). The primer [`80-just-enough-java`](./80-just-enough-java.md) gives you the language;
this topic gives you the framework conventions and the runtime that a large Java shop lives inside, with
the trade-offs of the heavyweight-framework approach made explicit rather than assumed. `†`: Java on a
current LTS JDK, Spring Boot, and a build tool (Maven/Gradle).

## Why this exists · the big idea

- **The problem before the solution**: wiring a large application by hand — constructing every service,
  passing every dependency, managing every lifecycle — collapses under its own weight; enterprise codebases
  needed a way to declare _what_ depends on _what_ and let the framework assemble it, plus a runtime that
  stays fast without manual memory management.
- **Keep-this-if-you-forget-everything**: inversion of control is the load-bearing idea — components declare
  their dependencies and the container supplies them, so wiring becomes configuration instead of code, and
  the JVM's managed runtime (JIT + GC) buys portability and speed at the cost of a warm-up and a tuning
  surface you must understand.
- **Big ideas touched**: `coupling-vs-cohesion` (dependency injection inverts control so modules couple to
  interfaces, not concrete constructors — the framework's whole reason to exist), `abstraction-and-its-cost`
  (Spring's auto-configuration hides enormous machinery; the leverage is real and so is the leak when the
  magic misfires).

## Prerequisites

- **Prior topics**: [topic 80 Just Enough Java](./80-just-enough-java.md) (the language, records, streams,
  the memory model at a glance) and [topic 42 Software Architecture](./42-software-architecture.md) (layering,
  boundaries, and where a DI container fits an architecture).
- **Tools & environment**: a macOS/Linux/Windows machine; a **JDK** pinned to a current LTS; **Spring Boot**
  via **Maven or Gradle**; `curl` for exercising endpoints; Neovim/VSCode with the Java LSP (DD-17). Keep the
  JDK and Spring Boot versions unpinned in prose — re-pull at authoring time.
- **Assumed knowledge**: classes/interfaces and generics (topic 80); collections/streams (topic 80);
  architectural layering and dependency direction (topic 42); building/serving an HTTP API (topic 11).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: dependency injection, Spring/Spring Boot's convention-over-configuration model,
  and the JVM's JIT + generational-GC design are evergreen and correctly left version-unpinned. Java's LTS
  cadence (a new LTS roughly every two years) means "current LTS JDK" and the matching Spring Boot line
  should be re-pulled at authoring time rather than pinned here.
- 2026-07-12 — verified: garbage-collector specifics (G1 as the default collector, ZGC as a low-pause
  alternative) are stable enough to name generically but move between JDK releases — describe them by role
  (throughput vs pause-time) rather than committing to a per-version default. (docs.oracle.com/en/java)

## Items

- Dependency injection & inversion of control: constructor injection, the container, bean lifecycle and
  scopes — the framework's spine.
- Spring Boot conventions: auto-configuration, starters, `application.yml`, profiles, and the
  convention-over-configuration bargain.
- Building a web service: controllers, service/repository layers, validation, and an error-handling model.
- Persistence with the ecosystem: JPA/Hibernate mapping, transactions, and the N+1 trap in an ORM context.
- The JVM as a runtime: class loading, the JIT (interpret → compile → optimize), and warm-up.
- Garbage collection & memory: generational GC, choosing a collector by role (throughput vs pause), and
  reading heap/GC behaviour under load.
- Observability & packaging: metrics/health endpoints (Actuator-style), and shipping a runnable artifact.

## Tensions & trade-offs — when NOT to reach for this

- **Framework magic vs debuggability**: auto-configuration and classpath scanning assemble a working app
  with almost no code — until something wires wrong, and now you are debugging a graph you never wrote. The
  same leverage that speeds the happy path lengthens the failure path; the cost is a steep "what is actually
  happening" tax when the abstraction leaks.
- **Startup and footprint**: a JIT-warmed, reflection-heavy Spring app is superb for long-lived servers and
  poor for short-lived, cold-start workloads (serverless, CLIs) — where startup time and memory dominate, the
  heavyweight-framework approach is the wrong default; a lighter runtime or ahead-of-time compilation fits
  better.
- **When NOT to reach for it**: a small service, a script, or a latency-critical cold-start path does not
  need a DI container and an ORM. Reach for the enterprise stack when team size, longevity, and integration
  breadth make its conventions pay for their weight — not because Java implies Spring.

## Lineage — why it beat the alternative

- Enterprise Java's shape is a reaction to its own past. J2EE/EJB tried to standardize enterprise concerns
  but did so with heavyweight, ceremony-laden components; Spring won by inverting that — a lightweight IoC
  container plus POJOs, letting the framework wire plain objects instead of forcing them into an EJB mold.
  Spring Boot then removed the remaining XML-and-boilerplate friction with opinionated auto-configuration, so
  "convention over configuration" became the enterprise default. Underneath, the JVM's write-once-run-anywhere
  bargain plus a maturing JIT made managed-runtime performance acceptable for server workloads. What this
  hands forward: the DI + layered-service discipline reinforces [`42-software-architecture`](./42-software-architecture.md),
  and the managed-runtime intuition (JIT, GC, memory model) is the counterweight to the manual-memory world
  of the systems-programming topics.

## Worked examples

Colocated under `enterprise-java-and-the-jvm/learning/code/`; each runnable via Maven/Gradle and exercised
from the CLI with `curl` (DD-20/DD-30).

- **beginner** — a Spring Boot service with constructor-injected components and a health/metrics endpoint;
  start it and hit it with `curl`.
- **intermediate** — a controller → service → JPA-repository stack with validation and a structured error
  envelope; observe (and then fix) an N+1 query.
- **advanced** — put the service under a small load and read its JIT warm-up and GC behaviour; try a
  different collector and compare pause vs throughput.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a small but idiomatic Spring Boot service — constructor-injected layers, JPA persistence
  with a transaction boundary, validation and an error model, and health/metrics endpoints — then exercise
  it under load and observe the JVM's JIT and GC behaviour, demonstrating both the framework conventions and
  the runtime beneath them.
- **Concepts exercised**: [ ] constructor dependency injection [ ] Spring Boot auto-configuration + profiles
  [ ] controller/service/repository layering [ ] JPA persistence with a transaction [ ] validation + error
  envelope [ ] health/metrics endpoint [ ] JIT warm-up + GC observation under load.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — a Spring Boot app with constructor-injected service + repository beans
     and an Actuator-style health/metrics endpoint. Verify the app starts and `curl` of the health endpoint
     returns healthy.
  2. Add a controller → service → JPA-repository CRUD path with validation and a structured error response.
     Verify a valid write persists (`curl`) and an invalid one returns the error envelope, not a stack trace.
  3. Wrap a multi-step write in a transaction and reproduce/fix an N+1 query. Verify the transaction rolls
     back on failure and the N+1 is gone (query count drops).
  4. Drive the service under a small load; capture JIT warm-up and GC behaviour, then swap the collector.
     Verify warm-up is observable and the throughput/pause trade-off between collectors is visible.
- **Acceptance criteria**: the app starts and serves; DI wiring is explicit (constructor injection);
  persistence + transaction + validation behave correctly; N+1 is resolved; JIT/GC behaviour is observed and
  the collector trade-off is demonstrated.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Java Concurrency in Practice** — Brian Goetz, Tim Peierls, Joshua Bloch, Joseph Bowbeer, David Holmes,
  Doug Lea (2006). The definitive treatment of the Java Memory Model and concurrent programming, essential
  for correct enterprise JVM code.
- **Spring in Action** — Craig Walls (6th ed., 2022). The long-running, most widely used introduction to the
  Spring Framework and Spring Boot ecosystem.
- **Java Performance** — Scott Oaks (2nd ed., 2020). Authoritative, in-depth guide to JVM tuning, garbage
  collection, and profiling for production Java systems.
- **Optimizing Java** — Benjamin J. Evans, James Gough, Chris Newland (2018). Practical JVM
  performance-engineering techniques by well-known JVM/Java Champions.

**Papers & articles**

- **The Java Virtual Machine Specification, Java SE 21 Edition** — Oracle America, Inc. (2023). The official
  normative reference for JVM bytecode, the class-file format, and execution semantics.
  <https://docs.oracle.com/javase/specs/jvms/se21/jvms21.pdf>

---

← Previous: [80 · Just Enough Java](./80-just-enough-java.md) · Next: [82 · Lisp](./82-lisp.md) →
