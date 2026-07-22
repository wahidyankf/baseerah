# Course Specs — Skills Paths: Accounting

The **per-course spec layer** for this plan's accounting corpus: one **`<course-id>.md`** detail file
per course, each carrying the concept/worked-example breakdown an author writes the course body from
directly. Start with the **[syllabus root README](../README.md)** for the shared-spine-plus-Sharia-extension
architecture and the inherited file shape (DD-627); the
[tech-docs §The twenty-four-course catalog](../../tech-docs.md#the-twenty-four-course-catalog) is the
single source of truth for IDs, formats, prerequisites, and stage. **Order is NOT a spec property** —
it lives in the two [path mirrors](../paths/README.md) this plan transcribes into the manifests'
`courseOrder`.

Each course is authored **once**; the nineteen shared courses are referenced by both
`conventional-accounting.yaml` and `sharia-accounting.yaml` (A11), and the five Sharia-specific courses
by `sharia-accounting.yaml` only.

## Shared spine (19 courses — referenced by both manifests, authored once)

| #   | Course spec                                                                                         | Format            | Stage |
| --- | --------------------------------------------------------------------------------------------------- | ----------------- | ----- |
| 1   | [`accounting-foundations`](./accounting-foundations.md)                                             | By Example        | 1     |
| 2   | [`chart-of-accounts-and-data-modeling`](./chart-of-accounts-and-data-modeling.md)                   | By Example        | 1     |
| 3   | [`financial-statements-and-close-cycle`](./financial-statements-and-close-cycle.md)                 | By Example        | 1     |
| 4   | [`journal-entries-and-posting-mechanics`](./journal-entries-and-posting-mechanics.md)               | By Example        | 2     |
| 5   | [`accrual-accounting-and-revenue-recognition`](./accrual-accounting-and-revenue-recognition.md)     | By Example        | 2     |
| 6   | [`accounts-payable-and-procure-to-pay`](./accounts-payable-and-procure-to-pay.md)                   | By Example        | 2     |
| 7   | [`accounts-receivable-and-order-to-cash`](./accounts-receivable-and-order-to-cash.md)               | By Example        | 2     |
| 8   | [`managerial-and-cost-accounting`](./managerial-and-cost-accounting.md)                             | By Example        | 2     |
| 9   | [`fixed-assets-and-depreciation`](./fixed-assets-and-depreciation.md)                               | By Example        | 2     |
| 10  | [`inventory-and-cogs-accounting`](./inventory-and-cogs-accounting.md)                               | By Example        | 2     |
| 11  | [`lease-and-intangible-asset-accounting`](./lease-and-intangible-asset-accounting.md)               | By Example        | 2     |
| 12  | [`multi-currency-accounting-and-fx-translation`](./multi-currency-accounting-and-fx-translation.md) | By Example        | 2     |
| 13  | [`consolidation-and-multi-entity-accounting`](./consolidation-and-multi-entity-accounting.md)       | By Example        | 2     |
| 14  | [`financial-reporting-standards-ifrs-vs-gaap`](./financial-reporting-standards-ifrs-vs-gaap.md)     | Annotated-concept | 2     |
| 15  | [`audit-controls-and-compliance`](./audit-controls-and-compliance.md)                               | Annotated-concept | 2     |
| 16  | [`payroll-and-tax-accounting-essentials`](./payroll-and-tax-accounting-essentials.md)               | By Example        | 2     |
| 17  | [`treasury-and-cash-management`](./treasury-and-cash-management.md)                                 | By Example        | 2     |
| 18  | [`financial-reporting-and-xbrl`](./financial-reporting-and-xbrl.md)                                 | Annotated-concept | 2     |
| 19  | [`general-ledger-system-architecture`](./general-ledger-system-architecture.md)                     | By Example        | 2     |

## Sharia-specific extension (5 courses — `sharia-accounting.yaml` only)

| #   | Course spec                                                                                         | Format            | Stage |
| --- | --------------------------------------------------------------------------------------------------- | ----------------- | ----- |
| 20  | [`sharia-accounting-and-aaoifi-standards`](./sharia-accounting-and-aaoifi-standards.md)             | Annotated-concept | 3     |
| 21  | [`islamic-contract-modeling-for-systems`](./islamic-contract-modeling-for-systems.md)               | By Example        | 3     |
| 22  | [`zakah-computation-and-reporting-for-systems`](./zakah-computation-and-reporting-for-systems.md)   | By Example        | 3     |
| 23  | [`sukuk-and-islamic-capital-markets-accounting`](./sukuk-and-islamic-capital-markets-accounting.md) | Annotated-concept | 3     |
| 24  | [`sharia-ledger-system-architecture`](./sharia-ledger-system-architecture.md)                       | By Example        | 3     |

## How to read a course spec

Each `<course-id>.md` carries these sections in order, inherited from plan 02's spec shape (DD-627),
adapted for a non-code, no-build domain: **H1 + top matter** (`Course ID` / `Format`, summary, scope
note — no `Language` field), **Why this exists · the big idea**, **Prerequisites**, **Accuracy notes**
(carrying `[Verified]` / `[Unverified]` / `[Needs Verification]` markers verbatim, A4), **Concepts**
(`co-NN`, floor ≥ 8), **Worked examples** (`ex-NN`, each citing its `co-NN`), **Applied synthesis (no
build — A6)** in place of plan 02's Capstone spec, **Read more** (nominative citation only, A8), and
**In which paths**.

---

← Back to the [syllabus root README](../README.md)
