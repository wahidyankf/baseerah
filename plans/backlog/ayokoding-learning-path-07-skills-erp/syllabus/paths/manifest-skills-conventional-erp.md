# Path Manifest — `skills/conventional-erp` (Enterprise Resource Planning, Conventional)

The **ordered manifest** for the conventional-ERP skills path: a **curated, prerequisite-consistent**
ordered list of **course IDs** over this plan's own 26-course conventional-ERP corpus (a subset of the
[syllabus index](../README.md), which additionally holds the 3 Sharia-exclusive courses referenced
only by the sibling `skills/sharia-erp` manifest). This is the authoritative reading order for this
path; a course page under `?path=skills/conventional-erp` follows it for prev/next + breadcrumb.

This file is the **human-readable mirror** of the manifest. The **machine-consumed source of truth** is
the standalone data file
`apps/ayokoding-www/src/features/course-paths/manifests/skills/conventional-erp.yaml` (a standalone
YAML data file in the `course-paths` feature, never `courseOrder` frontmatter on any `_index.md`). The
manifest also carries an explicit `arc: immediately-effective` field, present even though the URL
grammar omits it (R8). Path landing served at `/en/learn/paths/skills/conventional-erp`.

## Composition (26 courses, terminal — no further growth)

Entirely this plan's own corpus; no course is linked in from the existing software-engineering library
or the accounting corpus — those are **linked, not included**, per
[tech-docs.md §Requirement L-4](../../tech-docs.md#landing-content-requirements-what-plan-03-cannot-infer).

### Stage A — Foundations & Architecture (courses 1-12, 17, 21-22; 15 total; publishes first)

1. `erp-foundations-and-history`
2. `erp-conceptual-data-model`
3. `erp-module-map-and-architecture`
4. `erp-document-lifecycle-and-state-machines`
5. `erp-posting-rules-and-account-determination`
6. `erp-subledger-to-gl-architecture`
7. `erp-fiscal-calendar-and-period-close`
8. `erp-numbering-sequences-and-uom-conversion`
9. `erp-audit-trail-and-change-tracking` — **Dangerous 1 ⚡**
10. `procure-to-pay-systems`
11. `order-to-cash-systems`
12. `erp-procurement-and-fulfillment-exceptions`
13. `erp-bom-and-routing-architecture`
14. `erp-extension-and-customization`
15. `erp-integration-patterns`

### Stage B — Conventional Enterprise Depth (courses 13-16, 18-20, 23-26; 11 total; grown after accounting's conventional-accounting boundary)

1. `record-to-report-systems`
2. `inventory-and-warehouse-management`
3. `erp-inventory-costing-methods`
4. `erp-inventory-integrity-and-concurrency` — **Dangerous 2 ⚡**
5. `production-planning-and-mrp`
6. `demand-and-supply-planning`
7. `erp-availability-and-reservations`
8. `human-capital-management-and-hire-to-retire`
9. `multi-company-and-multi-currency-erp`
10. `erp-security-and-controls`
11. `erp-analytics-and-reporting` — **Dangerous 3 ⚡ — path ENDS HERE**

## Growth history (falsifiable checks)

- **Before Stage B growth**: `courseOrder` has exactly 15 entries; `erp-analytics-and-reporting` and
  every other Stage B id are **absent**.
- **After Stage B growth**: `courseOrder` has exactly 26 entries; every Stage A id retains its
  original relative order; every Stage B id is present exactly once.

## Order rationale

See [tech-docs.md §The ERP catalog](../../tech-docs.md#the-erp-catalog-29-courses-settled) and
[§Authoring stages vs reading ramp](../../tech-docs.md#authoring-stages-vs-reading-ramp-dd-3).

---

← Back to the [syllabus index](../README.md)
