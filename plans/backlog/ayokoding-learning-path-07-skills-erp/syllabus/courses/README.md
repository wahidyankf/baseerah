# ERP Syllabus Corpus — Per-Course Detail Files

The **per-course detail layer** of the 30-course ERP corpus: one `<course-id>.md` syllabus file per
course, each carrying an explicit module/topic breakdown (DD-31). Start with the
**[syllabus root README](../README.md)** for the two-paths-over-one-corpus architecture, the stage
tables, and the reading ramp; the [tech-docs §The ERP catalog](../../tech-docs.md#the-erp-catalog-30-courses-settled)
is the single source of truth for course ids, formats, and prerequisite edges. **Order is not a
property of this folder** — it lives in the two [path mirrors](../paths/README.md).

Grouped below by **authoring stage** (A/B/C), matching the root README. 27 courses are shared by both
`skills/conventional-erp` and `skills/sharia-erp`; the 3 Stage-C courses are `sharia-erp`-only.

## Stage A — Foundations & Architecture (15 courses, no accounting precondition)

| Course id                                                                                         | Format            |
| ------------------------------------------------------------------------------------------------- | ----------------- |
| [`erp-foundations-and-history`](./erp-foundations-and-history.md)                                 | Annotated-concept |
| [`erp-conceptual-data-model`](./erp-conceptual-data-model.md)                                     | Annotated-concept |
| [`erp-module-map-and-architecture`](./erp-module-map-and-architecture.md)                         | Annotated-concept |
| [`erp-document-lifecycle-and-state-machines`](./erp-document-lifecycle-and-state-machines.md)     | Annotated-concept |
| [`erp-posting-rules-and-account-determination`](./erp-posting-rules-and-account-determination.md) | By Example        |
| [`erp-subledger-to-gl-architecture`](./erp-subledger-to-gl-architecture.md)                       | By Example        |
| [`erp-fiscal-calendar-and-period-close`](./erp-fiscal-calendar-and-period-close.md)               | Annotated-concept |
| [`erp-numbering-sequences-and-uom-conversion`](./erp-numbering-sequences-and-uom-conversion.md)   | Annotated-concept |
| [`erp-audit-trail-and-change-tracking`](./erp-audit-trail-and-change-tracking.md)                 | Annotated-concept |
| [`procure-to-pay-systems`](./procure-to-pay-systems.md)                                           | By Example        |
| [`order-to-cash-systems`](./order-to-cash-systems.md)                                             | By Example        |
| [`erp-procurement-and-fulfillment-exceptions`](./erp-procurement-and-fulfillment-exceptions.md)   | By Example        |
| [`erp-bom-and-routing-architecture`](./erp-bom-and-routing-architecture.md)                       | By Example        |
| [`erp-extension-and-customization`](./erp-extension-and-customization.md)                         | By Example        |
| [`erp-integration-patterns`](./erp-integration-patterns.md)                                       | By Example        |

## Stage B — Conventional Enterprise Depth (12 courses, gated on conventional-accounting)

| Course id                                                                                         | Format            |
| ------------------------------------------------------------------------------------------------- | ----------------- |
| [`record-to-report-systems`](./record-to-report-systems.md)                                       | By Example        |
| [`inventory-and-warehouse-management`](./inventory-and-warehouse-management.md)                   | By Example        |
| [`erp-inventory-costing-methods`](./erp-inventory-costing-methods.md)                             | By Example        |
| [`erp-inventory-integrity-and-concurrency`](./erp-inventory-integrity-and-concurrency.md)         | By Example        |
| [`production-planning-and-mrp`](./production-planning-and-mrp.md)                                 | By Example        |
| [`demand-and-supply-planning`](./demand-and-supply-planning.md)                                   | Annotated-concept |
| [`erp-availability-and-reservations`](./erp-availability-and-reservations.md)                     | By Example        |
| [`quality-management-and-inspection`](./quality-management-and-inspection.md)                     | By Example        |
| [`human-capital-management-and-hire-to-retire`](./human-capital-management-and-hire-to-retire.md) | Annotated-concept |
| [`multi-company-and-multi-currency-erp`](./multi-company-and-multi-currency-erp.md)               | By Example        |
| [`erp-security-and-controls`](./erp-security-and-controls.md)                                     | Annotated-concept |
| [`erp-analytics-and-reporting`](./erp-analytics-and-reporting.md)                                 | By Example        |

**Dangerous 3 ⚡ — `conventional-erp` ends here** (27 courses).

## Stage C — Sharia-Compliant Design (3 courses, `sharia-erp` only, gated on sharia-accounting)

| Course id                                                                                   | Format            |
| ------------------------------------------------------------------------------------------- | ----------------- |
| [`sharia-compliant-erp-design`](./sharia-compliant-erp-design.md)                           | Annotated-concept |
| [`islamic-contract-based-transaction-flows`](./islamic-contract-based-transaction-flows.md) | By Example        |
| [`zakat-and-sharia-compliance-modules`](./zakat-and-sharia-compliance-modules.md)           | Annotated-concept |

**Dangerous 4 ⚡ — `sharia-erp` ends here** (30 courses).

---

← Back to the [syllabus index](../README.md)
