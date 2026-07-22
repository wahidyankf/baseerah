# Path Mirrors — Curated Orderings Over the ERP Corpus

This folder holds the **path mirrors** — the human-readable authoritative orderings each machine
manifest's `courseOrder` is transcribed from (DD-22). A **path** is an ordered,
prerequisite-consistent list of **course ids** over the
[ERP course corpus](../courses/README.md); it composes existing course bodies in a chosen order and
adds **zero new bodies** (every course a mirror names is authored once under `courses/`). This plan
owns exactly two paths, both under the `skills/` URL category
(`/en/learn/paths/skills/<subject>`) — the accounting siblings live in
`ayokoding-learning-path-06-skills-accounting`.

Both paths cover all the basics (A10/A11): the 27 shared course ids are authored once and
**referenced by both** mirrors; `sharia-erp` interleaves 3 Sharia-exclusive ids rather than
duplicating any file. See [tech-docs §Two paths, one corpus](../../tech-docs.md#two-paths-one-corpus-a10--a11).

## The two mirrors

- [`manifest-skills-conventional-erp.md`](./manifest-skills-conventional-erp.md) — the authoritative
  **27-id** ordering `<CONVMAN>` (`manifests/skills/conventional-erp.yaml`) is transcribed from.
  Terminal boundary: **Dangerous 3** at `erp-analytics-and-reporting`.
- [`manifest-skills-sharia-erp.md`](./manifest-skills-sharia-erp.md) — the authoritative **30-id**
  ordering `<SHARMAN>` (`manifests/skills/sharia-erp.yaml`) is transcribed from: the same 27 shared
  ids plus the 3 Stage-C Sharia-exclusive ids appended. Terminal boundary: **Dangerous 4** at
  `zakat-and-sharia-compliance-modules`.

Both mirrors carry `arc: immediately-effective` (R8/DD-7), even though the URL omits the arc segment.

---

← Back to the [syllabus index](../README.md)
