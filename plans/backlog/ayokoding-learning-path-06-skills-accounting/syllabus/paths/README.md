# Path Mirrors — Skills Paths: Accounting

This folder holds the **human-readable path mirrors** of the two machine-consumed ordering data files
this plan owns. A **path** is an ordered, prerequisite-consistent list of **course IDs** over this
plan's [course specs](../courses/README.md); it composes course building blocks in a chosen order and
adds **zero new bodies** (every shared course body lives once and is referenced by both manifests,
A11). Each mirror is transcribed into its manifest's `courseOrder` at
`apps/ayokoding-www/src/features/course-paths/manifests/skills/<subject>.yaml`.

Both path ids are written in **full 2-segment canonical form**, category segment included
(`skills/<subject>`); the bare subject slug is invalid and `PathManifestSchema.safeParse` rejects it
(R2, plan 02's ruling). Both carry `arc: immediately-effective` — a required manifest field, recorded
as data and omitted from the URL (R8).

## The paths

| Path id                          | Persona                                              | Shape                                                                                  | Mirror                                                          |
| -------------------------------- | ---------------------------------------------------- | -------------------------------------------------------------------------------------- | --------------------------------------------------------------- |
| `skills/conventional-accounting` | Systems builder learning conventional accounting     | 19 shared courses (Stages 1–2); **terminates at Stage 2** — a complete competence      | [manifest mirror](./manifest-skills-conventional-accounting.md) |
| `skills/sharia-accounting`       | Systems builder learning Sharia-compliant accounting | The **same 19** shared courses in the same order, then **5** Sharia-specific (Stage 3) | [manifest mirror](./manifest-skills-sharia-accounting.md)       |

The first 19 entries of `skills/sharia-accounting`'s `courseOrder` are byte-identical, in order, to
the entirety of `skills/conventional-accounting`'s — the mechanical expression of "shared, authored
once, referenced by both" (A11). See
[tech-docs §Two manifests, nineteen shared courses](../../tech-docs.md#two-manifests-nineteen-shared-courses-a10--a11).

## How to read a mirror

Each mirror is the ordered list of course IDs grouped by stage, with a short composition rationale.
Order is a per-path property; it is **not** a spec property (the [course specs](../courses/README.md)
are order-neutral). Every mirror is **prerequisite-consistent** — a valid topological entry into the
corpus's prerequisite DAG, so every course appears after all of its prerequisites.

---

← Back to the [syllabus root README](../README.md)
