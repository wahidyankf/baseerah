# ERP BOM and Routing Architecture (By Example)

**Course ID**: `erp-bom-and-routing-architecture` · **Format**: By Example · **Language**: — (domain, worked scenarios, no application code).

**Short summary**: Single/multi-level BOM, phantom BOMs, routing/operations, BOM explosion

**Scope note**: the entry point to the production-planning cluster (with courses 18-20) — bill of
materials and routing structures, including phantom BOM handling, one of the domain's hard parts
[Repo-grounded — domain-research grounding, Part 2]. Authorable in Stage A: its only prerequisite is
course 2's conceptual data model. License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: a multi-level product structure (an assembly containing
  sub-assemblies containing components) cannot be reasoned about informally once it gets more than a
  couple of levels deep — BOM explosion is the mechanical answer to "what do I actually need, all the
  way down".
- **Keep-this-if-you-forget-everything**: a phantom BOM assembly never itself appears in inventory —
  explosion passes straight through it to its own components, which is easy to model incorrectly.
- **Big ideas touched**: `bom-as-a-recursive-structure`; `phantom-assembly-as-a-modeling-trap`.

## Prerequisites

- **Prior topics**: [`erp-conceptual-data-model`](./erp-conceptual-data-model.md).
- **Cross-domain prerequisites**: none.
- **Assumed knowledge**: course 2's item-master vocabulary.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- Worked BOM examples use an originally-authored product structure throughout.

## Concepts

- **co-01 · single-level-bom** — a direct list of components for one assembly, one level deep.
- **co-02 · multi-level-bom** — an assembly whose components are themselves assemblies, recursively.
- **co-03 · phantom-bom** — a sub-assembly that never itself appears in inventory; explosion passes
  through it to its own components.
- **co-04 · routing** — the sequence of operations (and the work centers performing them) required to
  produce an assembly, distinct from its component list.
- **co-05 · work-center** — a resource (machine, line, or labor pool) an operation is scoped to.
- **co-06 · bom-explosion** — recursively resolving a multi-level BOM down to its base-level component
  quantities.
- **co-07 · phantom-explosion-trap** — the modeling error of treating a phantom assembly as a real
  stocked item during explosion.
- **co-08 · quantity-per-scaling** — a BOM line's "quantity per parent" multiplies through every level
  of explosion.

## Worked examples

Prose-based worked scenarios with originally-authored data (no runnable code). Every example cites
the `co-NN` it exercises.

### Beginner

- **ex-01 · single-level-bom-read** — given a single-level BOM, list its direct components and
  quantities. (co-01)
- **ex-02 · routing-vs-bom-contrast** — given an assembly's BOM and its routing, explain what each
  answers that the other does not. (co-04)

### Intermediate

- **ex-03 · multi-level-explosion** — given a three-level product structure, explode it to base-level
  component quantities. (co-02, co-06, co-08)
- **ex-04 · phantom-bom-identify** — given a product structure with one phantom sub-assembly, identify
  it and explain why it never appears in an inventory count. (co-03)

### Advanced

- **ex-05 · phantom-explosion-trap-demonstration** — show what goes wrong if the ex-04 phantom is
  mistakenly treated as a real stocked item during explosion — verify the resulting component
  quantities are wrong. (co-07)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: design a multi-level BOM (including one phantom sub-assembly) and its routing for a new
  product, and perform a full explosion by hand.
- **Concepts exercised**: [ ] multi-level BOM (co-02) [ ] phantom BOM (co-03) [ ] explosion (co-06,
  co-08).
- **Ordered steps**: 1) design the product structure; 2) mark the phantom sub-assembly; 3) design the
  routing; 4) explode the structure to base-level quantities.
- **Acceptance criteria**: the phantom assembly correctly passes through without appearing as a
  stocked quantity; explosion quantities are arithmetically correct.
- **Done bar**: a written design and worked explosion, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage A, course 17 of 26.
- `skills/sharia-erp` — Stage A, course 17 of 29.

---

← Back to the [syllabus index](../README.md)
