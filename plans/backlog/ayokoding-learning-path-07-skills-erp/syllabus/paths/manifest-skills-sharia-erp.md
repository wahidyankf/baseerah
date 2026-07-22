# Path Manifest — `skills/sharia-erp` (Enterprise Resource Planning, Sharia-Compliant)

The **ordered manifest** for the Sharia-compliant-ERP skills path: a **curated, prerequisite-consistent**
ordered list of **course IDs** over this plan's full 29-course corpus — the same 26 shared courses
`skills/conventional-erp` teaches, **plus** 3 Sharia-exclusive courses interleaved after the shared
corpus. **Covers all the basics** (A10) — a reader entering this path cold gets the full 26-course
foundation; it is never an add-on assuming the conventional path. This is the authoritative reading
order for this path; a course page under `?path=skills/sharia-erp` follows it for prev/next +
breadcrumb.

This file is the **human-readable mirror** of the manifest. The **machine-consumed source of truth** is
the standalone data file `apps/ayokoding-www/src/features/course-paths/manifests/skills/sharia-erp.yaml`.
Per `A11` — cited directly from
[`ayokoding-learning-path-02-schema-and-prerequisite-dag/tech-docs.md:417,424,615`](../../../ayokoding-learning-path-02-schema-and-prerequisite-dag/tech-docs.md#design-decisions)
— every shared id below **references** the same course body `skills/conventional-erp` teaches; **no
body is duplicated**. The manifest carries an explicit `arc: immediately-effective` field (R8). Path
landing served at `/en/learn/paths/skills/sharia-erp`.

## Composition (29 courses, terminal — 26 shared + 3 Sharia-exclusive)

### Stage A — Foundations & Architecture (courses 1-12, 17, 21-22; 15 total; identical to `conventional-erp`'s Stage A)

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

### Stage B — Conventional Enterprise Depth (courses 13-16, 18-20, 23-26; 11 total; identical insertion positions to `conventional-erp`'s Stage B)

1. `record-to-report-systems`
2. `inventory-and-warehouse-management`
3. `erp-inventory-costing-methods`
4. `erp-inventory-integrity-and-concurrency` — **Dangerous 2 ⚡**
5. `production-planning-and-mrp`
6. `demand-and-supply-planning`
7. `erp-availability-and-reservations`
8. `human-capital-management-and-hire-to-retire`
9. `multi-company-and-multi-currency-erp` — **Dangerous 3 ⚡ — the shared corpus ends here; `conventional-erp` stops at this id, `sharia-erp` continues**

### Stage C — Sharia-Compliant Design (courses 27-29; 3 total; `sharia-erp` exclusive — interleaved after the shared corpus)

1. `sharia-compliant-erp-design`
2. `islamic-contract-based-transaction-flows`
3. `zakat-and-sharia-compliance-modules` — **Dangerous 4 ⚡ — path ENDS HERE**

### Remaining shared Stage B ids (interleaved before Stage C, per catalog order)

To keep the shared 26-course foundation identical in composition between both manifests while placing
the 3 Sharia-exclusive ids at their correct prerequisite-consistent position, the final two shared
Stage B ids sit **between** `multi-company-and-multi-currency-erp` and Stage C:

1. `erp-security-and-controls`
2. `erp-analytics-and-reporting`

> **Note on section order above**: each `###` subsection here restarts its own local numbering; the
> **reading position** in this manifest's actual `courseOrder` is the sequence of subsections
> top-to-bottom (Stage A, then Stage B up to `multi-company-and-multi-currency-erp`, then Stage C's
> three ids, then the two remaining Stage B ids listed last). `erp-security-and-controls` and
> `erp-analytics-and-reporting` (both catalog Stage B) read **after** the three Sharia-exclusive
> courses in this manifest, because `sharia-compliant-erp-design` prerequisites
> `multi-company-and-multi-currency-erp` only, not either of those two ids — so the Sharia-exclusive
> block inserts immediately after `multi-company-and-multi-currency-erp`, ahead of them, in this
> manifest's ramp. `conventional-erp`'s own manifest keeps the catalog's order (those two ids
> immediately follow `multi-company-and-multi-currency-erp`) since it never inserts the Sharia block.

## Growth history (falsifiable checks)

- **Before Stage B growth**: `courseOrder` has exactly 15 entries; no Stage B or Stage C id is
  present.
- **After Stage B growth**: `courseOrder` has exactly 26 entries (the same 26 shared ids
  `conventional-erp` reaches); no Stage C id is present yet.
- **After Stage C growth**: `courseOrder` has exactly 29 entries; every previously-published id
  retains its relative order; the 3 Sharia-exclusive ids are inserted at the position described
  above.

## Order rationale

See [tech-docs.md §The ERP catalog](../../tech-docs.md#the-erp-catalog-29-courses-settled),
[§Authoring stages vs reading ramp](../../tech-docs.md#authoring-stages-vs-reading-ramp-dd-3), and
[§Two paths, one corpus (A10/A11)](../../tech-docs.md#two-paths-one-corpus-a10--a11).

---

← Back to the [syllabus index](../README.md)
