# Business Rationale — Architecture Procedural Track

## Problem

The existing software-architecture content on ayokoding-web teaches two paradigm tracks (OOP and FP). Both tracks misrepresent Go, Rust, and C content when readers attempt to apply them:

- **OOP track** assumes inheritance hierarchies as the polymorphism backbone — a Java reader internalises Visitor, Bridge, Abstract Factory family hierarchies, and Template Method patterns that **do not translate to Go** (no inheritance), Rust (no inheritance, ownership-driven instead), or C (no classes).
- **FP track** assumes garbage-collected persistent immutable sharing and higher-kinded effect abstraction — an F# / Haskell reader internalises Reader monads, Free monads, Kleisli composition, monadic computation expressions that **do not translate to Rust** (no HKT, no generic Functor / Monad trait) and **do not exist in Go or C**.

Without a third track, learners reaching for Go (large and growing audience), Rust (systems / WASM / embedded), or C (kernel / embedded / firmware) either:

1. Misread the OOP track and write Go code that fights the language's design (deep inheritance-style packaging, fat interfaces, `factory` indirection).
2. Misread the FP track and try to express Functor / Monad / Reader patterns in Rust that the type system cannot generalise without ad-hoc workarounds.
3. Find no architectural guidance at all for C, the language with the longest production track record for FSMs and OS-level architectural patterns.

## Audience

- **Go developers** — the most numerous untapped reader segment. Go is a top-10 production language; mature DDD / hexagonal literature exists (Boyle 2022, Three Dots Labs); the OOP track misrepresents it.
- **Rust developers** — growing systems / WASM / serverless audience; the typestate FSM idiom is unique to Rust and deserves first-class teaching, not a footnote.
- **Embedded / kernel / firmware engineers** — the C function-pointer-table FSM idiom (Samek) is canonical for embedded systems with rich literature; currently zero coverage on the site.
- **Polyglot architects** evaluating language choice — paradigm-honest framing helps decide _which language fits which architectural commitment_, not just "can Go do hexagonal?" (yes) but "what does hexagonal cost in Go vs Java?" (less, because structural typing eliminates the `implements` ceremony).

## Why Not Stretch the Existing Tracks

Two alternatives were considered and rejected:

1. **Add Rust as another FP tab; Go as another OOP tab; skip C.** Rejected because Go has no inheritance (forcing it into OOP teaches the wrong instincts) and Rust has no HKT (forcing it into FP teaches concepts that won't generalise). C has no fit in either.
2. **Single "polyglot" track covering all 3 + existing languages.** Rejected because the paradigm-fit framing the site already commits to (Norvig, Seemann, Wlaschin, Hickey, Evans, Fowler) is the strongest pedagogical feature — diluting it loses the educational anchor.

The honest split is **three paradigm-aligned tracks**, each with one clear conceptual lens.

## Cost / Benefit

**Cost (one-shot)**:

- Overview-level paradigm framing (already shipped 2026-05-20): ~5,000 words across 11 files (5 FP overview additions, 5 OOP overview additions, parent overview + \_index update).
- Procedural-track scaffolding (already shipped 2026-05-20): 10 files (5 overview.md + 5 \_index.md), ~4,500 words.

**Cost (rolling out tier content)**:

- 15 tier files × 5,000–10,000 lines each at 1.0–2.25 annotation density = ~120,000–225,000 annotated lines of code + prose. Multi-session deliverable spanning weeks of authoring time.

**Benefit**:

- Honest paradigm framing for Go (largest audience gap closed).
- First-class Rust typestate teaching (unique-to-Rust idiom, deserves first-class treatment).
- Coverage of canonical C FSM literature (Samek, Linux kernel patterns) that has zero presence today.
- Strengthened paradigm-fit pedagogy across all three tracks — readers can choose which lens fits their language.

## Decision

Ship overview-level updates immediately (already done 2026-05-20). Roll out tier content under the [delivery.md](./delivery.md) checklist as a multi-session deliverable.

## Out of Scope

- C beyond FSM. No DDD-in-C, no hexagonal-in-C — these have no canonical literature.
- Rust as an OOP-track-tab. Rust rejects inheritance by design; force-fitting it into OOP would be miseducation.
- Replacing existing OOP or FP tracks. The three tracks coexist.
