# Path Manifest — `skills/conventional-erp` (Enterprise Resource Planning, Conventional)

The **ordered manifest** for the conventional-ERP skills path: a **curated, prerequisite-consistent**
ordered list of **course IDs** over this plan's own 27-course conventional-ERP corpus (a subset of the
[syllabus index](../README.md), which additionally holds the 3 Sharia-exclusive courses referenced
only by the sibling `skills/sharia-erp` manifest). This is the authoritative reading order for this
path; a course page under `?path=skills/conventional-erp` follows it for prev/next + breadcrumb.

This file is the **human-readable mirror** of the manifest. The **machine-consumed source of truth** is
the standalone data file
`apps/ayokoding-www/src/features/course-paths/manifests/skills/conventional-erp.yaml` (a standalone
YAML data file in the `course-paths` feature, never `courseOrder` frontmatter on any `_index.md`). The
manifest also carries an explicit `arc: immediately-effective` field, present even though the URL
grammar omits it (R8). Path landing served at `/en/learn/paths/skills/conventional-erp`.

## Composition (27 courses, terminal — no further growth)

Entirely this plan's own corpus; no course is linked in from the existing software-engineering library
or the accounting corpus — those are **linked, not included**, per
[tech-docs.md §Requirement L-4](../../tech-docs.md#landing-content-requirements-what-plan-03-cannot-infer).

**This list is the reading ramp, in `courseOrder` order.** Per
[tech-docs.md §Authoring stages vs reading ramp](../../tech-docs.md#authoring-stages-vs-reading-ramp-dd-3),
**authoring order is not reading order** — the manifest fixes what a _reader_ walks, while the
delivery checklist fixes what an _author_ writes next. Each entry is therefore annotated with its
authoring stage rather than grouped by it. Grouping by stage would place
`erp-bom-and-routing-architecture` (Stage A, read 17th) ahead of `record-to-report-systems` (Stage B,
read 13th) and desynchronise every course's own self-declared "course N of 27" position.

1. `erp-foundations-and-history` — Stage A
2. `erp-conceptual-data-model` — Stage A
3. `erp-module-map-and-architecture` — Stage A
4. `erp-document-lifecycle-and-state-machines` — Stage A
5. `erp-posting-rules-and-account-determination` — Stage A
6. `erp-subledger-to-gl-architecture` — Stage A
7. `erp-fiscal-calendar-and-period-close` — Stage A
8. `erp-numbering-sequences-and-uom-conversion` — Stage A
9. `erp-audit-trail-and-change-tracking` — Stage A — **Dangerous 1 ⚡**
10. `procure-to-pay-systems` — Stage A
11. `order-to-cash-systems` — Stage A
12. `erp-procurement-and-fulfillment-exceptions` — Stage A
13. `record-to-report-systems` — Stage B — the hard accounting edge lands here
14. `inventory-and-warehouse-management` — Stage B
15. `erp-inventory-costing-methods` — Stage B
16. `erp-inventory-integrity-and-concurrency` — Stage B — **Dangerous 2 ⚡**
17. `erp-bom-and-routing-architecture` — Stage A (authored early, read here)
18. `production-planning-and-mrp` — Stage B
19. `demand-and-supply-planning` — Stage B
20. `erp-availability-and-reservations` — Stage B
21. `quality-management-and-inspection` — Stage B
22. `erp-extension-and-customization` — Stage A (authored early, read here)
23. `erp-integration-patterns` — Stage A (authored early, read here)
24. `human-capital-management-and-hire-to-retire` — Stage B
25. `multi-company-and-multi-currency-erp` — Stage B
26. `erp-security-and-controls` — Stage B
27. `erp-analytics-and-reporting` — Stage B — **Dangerous 3 ⚡ — path ENDS HERE**

## Growth history (falsifiable checks)

- **Before Stage B growth**: `courseOrder` has exactly 15 entries — the Stage A ids only, holding
  their relative reading order (final-ramp positions 1-12, 17, 22, 23); every Stage B id is
  **absent**.
- **After Stage B growth**: `courseOrder` has exactly 27 entries; every Stage A id retains its
  original relative order; every Stage B id is present exactly once, **inserted at its reading
  position rather than appended** — Stage B ids interleave with Stage A ids, they do not follow them.

## Order rationale

See [tech-docs.md §The ERP catalog](../../tech-docs.md#the-erp-catalog-30-courses-settled) and
[§Authoring stages vs reading ramp](../../tech-docs.md#authoring-stages-vs-reading-ramp-dd-3).

---

← Back to the [syllabus index](../README.md)
