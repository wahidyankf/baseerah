# Product Requirements — Architecture Procedural Track

## Scope

Add a third paradigm track (`in-procedural-by-example`) to the software-architecture section of ayokoding-web with Go (canonical), Rust, and C (sidebar) coverage. Adjust FP overviews to teach Rust-as-FP-adjacent with explicit concept adjustments. Adjust OOP overviews to redirect Go-curious readers to the procedural track for the canonical formulation.

**Stage 1 (shipped 2026-05-20)** — overview-level scaffolding:

- 5 FP overview.md updates with Rust paradigm-fit section + six concept adjustments.
- 5 OOP overview.md updates with Go partial-fit / FSM language-reach note.
- 5 new `in-procedural-by-example/` directories with overview.md + \_index.md.
- Parent `_index.md` and `overview.md` exposing three paradigm tracks.
- 5 topic `_index.md` updates linking the new procedural subsection.

**Stage 2 (pending — rollout under delivery.md)** — tier content:

- 15 tier files (`beginner.md`, `intermediate.md`, `advanced.md` per topic) with 1.0–2.25 annotation density, Go-canonical with Rust tabs where the idiom changes.

## Acceptance Criteria (Gherkin)

### Stage 1 — Already Met

```gherkin
Feature: Three paradigm tracks exposed on the software-architecture index
  Scenario: Parent _index.md lists three tracks per topic
    Given I open /en/learn/software-engineering/software-architecture
    Then I see Patterns and Principles with three children (OOP, FP, Procedural)
    And I see DDD with three children (FP, OOP, Procedural)
    And I see Hexagonal Architecture with three children (FP, OOP, Procedural)
    And I see FSM with three children (OOP, FP, Procedural)
    And I see Cases with three children (FP, OOP, Procedural)

Feature: FP overviews acknowledge Rust with concept adjustments
  Scenario: Patterns-and-principles FP overview teaches Rust adjustments
    Given I open the FP overview for patterns-and-principles
    Then I see a section "Rust as an FP-Adjacent Member — With Concept Adjustments"
    And the section enumerates ownership/affine types as a Rust-only concept
    And the section explains no-HKT (no generic Functor/Monad trait)
    And the section explains the `?` operator is not monadic bind
    And the section cites Blandy & Orendorff, Programming Rust
    And the section cites the GAT stabilisation blog post
    And the section links to the in-procedural-by-example track

  Scenario: All five FP overviews carry this section
    Given I open each of (patterns-and-principles, ddd, hexagonal, fsm, cases) FP overviews
    Then each has a Rust section with concept adjustments
    And the section text is topic-specific (DDD aggregates; hexagonal ports; FSM typestate; etc.)

Feature: OOP overviews redirect Go-curious readers
  Scenario: Patterns-and-principles OOP overview has Go partial-fit note
    Given I open the OOP overview for patterns-and-principles
    Then I see a section titled "Where Go Fits (Partial)"
    And the section cites Rob Pike's 2012 SPLASH keynote
    And the section enumerates which OOP patterns translate vs do not translate to Go
    And the section links to the in-procedural-by-example track

  Scenario: Hexagonal OOP overview marks Go as strong fit
    Given I open the OOP overview for hexagonal-architecture
    Then I see a section "Where Go Fits (Strong)"
    And the section cites Cockburn's original hexagonal architecture
    And the section cites Boyle 2022 and Three Dots Labs as Go references

  Scenario: FSM OOP overview enumerates three native idioms
    Given I open the OOP overview for fsm
    Then I see a section "Where Go and C Fit"
    And the section names Rust typestate, Go looplab/fsm, C Samek function-pointer table
    And each is linked to its canonical literature

Feature: Procedural-track overviews are substantive
  Scenario: Each new overview has paradigm framing
    Given I open the overview.md for each in-procedural-by-example directory
    Then I see why this paradigm is honest for Go (Pike SPLASH 2012 cited)
    And I see what languages this track covers (Go, Rust, C — with C in sidebar where applicable)
    And I see the comparison table to OOP and FP track formulations
    And I see authority basis with at least 6 cited sources
    And I see sibling-track cross-links to OOP and FP overviews
    And I see a rollout-plan link to plans/in-progress/architecture-procedural-track
```

### Stage 2 — Future Tier Content (Definition of Done)

```gherkin
Feature: Procedural-track tier content is authored
  Scenario: Each topic has beginner / intermediate / advanced markdown files
    Given the in-procedural-by-example directory for any of the five topics
    Then beginner.md, intermediate.md, advanced.md exist
    And each follows the five-part example format used in OOP and FP tracks
    And annotation density is 1.0–2.25 comment lines per code line per tab
    And Go is the canonical language tab
    And Rust appears where the idiom changes
    And C appears only in the FSM track sidebar
    And the running domain is procurement-platform-be (matches sibling tracks)

  Scenario: Cross-track parity maintained
    Given the same example number appears in all three tracks (where applicable)
    Then the conceptual title matches across tracks
    And paradigm notes explain how the procedural formulation differs from OOP and FP
```

## Citation Anchors (Required in Stage 1, Carried Through Stage 2)

Every overview and tier file MUST cite at minimum:

- Rob Pike — Go at Google (2012 SPLASH) for Go paradigm framing.
- Matthew Boyle — _Domain-Driven Design with Golang_ (Packt, 2022) for Go DDD.
- Three Dots Labs (threedots.tech) for Go reference implementation.
- Alistair Cockburn — Hexagonal Architecture (2005) for hexagonal definition.
- Hoverbear or Crichton — for Rust typestate FSM idiom.
- Miro Samek — _Practical UML Statecharts in C/C++_ (Routledge, 2008) for C FSM.
- looplab/fsm GitHub for the canonical Go FSM library.
- Blandy, Orendorff & Tindall — _Programming Rust_ for Rust paradigm framing.
- GAT stabilisation blog post for Rust's no-HKT explicit statement.

No fabricated sources. Every citation must be verifiable.

## Non-Requirements

- This plan does NOT replace the OOP or FP tracks. All three coexist.
- This plan does NOT require Rust to appear in every example tab across the existing tracks. Rust appears where the idiom translates one-to-one; for ownership-driven idioms, the procedural track is the canonical location.
- This plan does NOT cover DDD-in-C or hexagonal-in-C. These have no canonical literature.
