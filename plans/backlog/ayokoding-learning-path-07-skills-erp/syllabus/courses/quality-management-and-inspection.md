# Quality Management and Inspection (By Example)

**Course ID**: `quality-management-and-inspection` · **Format**: By Example · **Language**: — (domain, worked scenarios, no application code).

**Short summary**: Inspection triggers and lots, characteristics and sampling rules, usage decisions,
dispositions, quality traceability

**Scope note**: the dedicated treatment of quality management, promoted out of the touchpoint-only
coverage courses 3 and 14 carry (DD-36) — how an inspection obligation is raised from a movement, how
a verdict is recorded, and what that verdict then does to the stock, the source document, and the
value. Stays on the **ERP-mechanics** side of the boundary: it models the gate a quality system needs
and the data that gate writes, never statistical-quality engineering as a discipline and never a
certification programme. Transitively requires `inventory-and-cogs-accounting` via its inventory
prerequisite. License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: "it passed inspection" is a verdict about a **quantity at a
  moment**, but an ERP is asked to answer it about an item for as long as that stock exists — and
  without a lot-shaped record of who inspected what, against which specification, and by what rule,
  the verdict lives in an email while the stock sits in the same bin as everything else.
- **Keep-this-if-you-forget-everything**: quality is a **gate on a movement, not a module a
  transaction visits** — the inspection lot exists so a quantity can be held in a state that is on
  hand but not available, and the usage decision is the single act that ends that state.
- **Big ideas touched**: `inspection-lot-as-the-quality-side-document`;
  `usage-decision-as-the-single-write-back-point`; `a-verdict-under-sampling-is-an-inference`.

## Prerequisites

- **ERP prereqs**: [`inventory-and-warehouse-management`](./inventory-and-warehouse-management.md),
  [`erp-procurement-and-fulfillment-exceptions`](./erp-procurement-and-fulfillment-exceptions.md),
  [`erp-bom-and-routing-architecture`](./erp-bom-and-routing-architecture.md).
- **Accounting prereqs**: `inventory-and-cogs-accounting` (transitive via course 14).
- **Assumed knowledge**: course 14's stock-type vocabulary and its inspection-hold touchpoint (co-08);
  course 12's exception vocabulary; course 4's document-state-machine vocabulary; course 17's
  routing-and-operation vocabulary.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- This course is authored from **domain reasoning alone**. The grounding research treats quality
  management only at its inventory and procurement touchpoints, so the decomposition below is
  `[Needs Verification]` **as a whole** pending the Phase 1.2a coverage pass. That pass's coverage
  question for this course is explicitly two-sided: whether the field recognises this decomposition,
  and whether an area a practitioner would expect (supplier quality rating and vendor scorecards, or
  calibration of measuring equipment, are the two most likely candidates) is missing from it.
- **ISO 9001** is **named** as the governing quality-management-system standard family, and
  GMP/HACCP as sector-specific regimes. No clause, clause numbering, threshold, or table from any of
  them is reproduced or paraphrased anywhere in this course (`A8`). Whether a particular requirement
  originates in one of those standards is `[Needs Verification]` and is never asserted.
- Statistical acceptance sampling has **published schemes**. This course teaches the sampling rule
  only as a **recorded input that makes a verdict reproducible**, and names those schemes
  nominatively. No sampling table, sample-size code letter, or acceptance number is reproduced;
  any claim about what a specific published scheme prescribes is `[Needs Verification]` and is not
  made here.
- Concept co-13's cost-of-quality framing is this corpus's own reasoning about where a quality event
  lands in the valuation model, not a sourced taxonomy, and is `[Needs Verification]`.

## Concepts

- **co-01 · inspection-trigger** — the configured rule deciding whether a given event raises an
  inspection obligation at all: a goods receipt for a flagged item, a production confirmation at a
  routing operation, a pre-delivery check, or a recurring re-check of stock already held. The trigger
  is a property of the item, source and event type together — never a decision a clerk makes
  case-by-case, or the same material is inspected inconsistently.
- **co-02 · inspection-lot** — the quality-side document a trigger creates: it binds a **specific
  quantity of a specific movement** to an inspection obligation and carries its own lifecycle
  (created → in inspection → decided), exactly as course 4's other documents do. It is the reason a
  verdict can be found again later; a result recorded without a lot has nothing durable to attach to.
- **co-03 · inspection-characteristic** — one named thing being measured, with its specification: a
  quantitative characteristic carries a target and a permitted range, a qualitative (attribute)
  characteristic carries a permitted set of outcomes. A lot is inspected against a **set** of
  characteristics, which is why a lot verdict is never a single measurement.
- **co-04 · sampling-rule-as-a-recorded-input** — how many units of the lot were actually examined and
  how they were chosen, recorded on the lot itself. Recording it is what makes the verdict
  reproducible and auditable rather than an opinion; a 100 % inspection is just the boundary case of
  this same rule.
- **co-05 · inference-and-its-two-error-directions** — a verdict from a sample is an **inference about
  the lot**, so it can be wrong in two distinct directions: accepting a lot that should have been
  rejected, and rejecting one that should have been accepted. The two have different costs and
  different owners, and tightening a sampling rule trades one against the other rather than removing
  both.
- **co-06 · result-recording-and-conformance** — the recorded value for one characteristic, compared
  against that characteristic's specification, yielding conforming or nonconforming **for that
  characteristic only**.
- **co-07 · lot-verdict-aggregation** — the explicit rule turning per-characteristic results into one
  lot verdict (any-nonconforming-fails, or a weighting that lets a minor characteristic fail without
  failing the lot). The rule must be stated in the design, because two defensible rules give opposite
  verdicts on the same recorded results.
- **co-08 · usage-decision** — the terminal act that closes the inspection lot **and** releases the
  held quantity into a new status. This is the single point where quality writes back to inventory,
  which is what keeps the two models coupled without merging them (course 14, co-08).
- **co-09 · quality-status-as-a-property-of-a-quantity** — inspection-blocked, unrestricted and
  rejected are statuses of **quantities**, not of items: the same item routinely holds quantities in
  several statuses at once. This is the modeling reason "on hand" and "available" are separate
  numbers rather than synonyms (course 14, co-01).
- **co-10 · disposition-outcomes** — the distinct endings a usage decision can choose — release,
  restricted release under a documented concession, rework, scrap, or return to the supplier — each
  with a **different** downstream consequence: a different stock movement, a different valuation
  effect, and in the supplier case a procurement exception (course 12) rather than a purely internal
  one.
- **co-11 · inspection-point-placement** — where in a flow the gate is placed: on receiving, in
  process at a named routing operation (course 17), before delivery (course 11), or recurring against
  stored stock. Placement is a design decision with a cost: the later the gate, the more value has
  already been added to material that may be scrapped.
- **co-12 · batch-and-serial-traceability** — the identity granularity a verdict attaches to. Without
  batch or serial identity, a defect discovered later has an item-wide blast radius; with it, the same
  defect resolves to a bounded set of receipts and deliveries, which is what makes containment and
  segregation possible at all.
- **co-13 · quality-cost-and-valuation-touchpoint** — a quality event is not only a quantity event:
  scrap writes off value, rework consumes further cost, and a supplier return carries a commercial
  consequence — so a disposition lands on the costing model (course 15) as well as on the stock model
  (course 14).
- **co-14 · nonconformance-record-and-corrective-action** — the record that **outlives** the lot: a
  nonconformance attributed to its source (supplier, work center, batch) plus the corrective action
  raised against it. This is what turns quality data from a gate into something analysable later
  (course 27), and it is the concept most often missing from a first design.

## Worked examples

Prose-based worked scenarios with originally-authored data (no runnable code). Every example cites
the `co-NN` it exercises.

### Beginner

- **ex-01 · inspection-trigger-classification** — given four events (a goods receipt of an
  inspection-flagged item, a goods receipt of an unflagged item, an internal stock transfer, and a
  production confirmation at a flagged operation), decide which raise an inspection lot and which do
  not. (co-01, co-02)
- **ex-02 · characteristic-conformance-read** — given one quantitative characteristic with a target
  and a permitted range and three recorded measurements, mark each conforming or nonconforming, then
  do the same for one attribute characteristic. (co-03, co-06)

### Intermediate

- **ex-03 · lot-verdict-under-two-rules** — given five characteristic results where exactly one is
  nonconforming, state the lot verdict under an any-fail rule, then under a rule that treats that
  characteristic as minor, and name which rule the design must record. (co-06, co-07)
- **ex-04 · usage-decision-stock-trace** — trace one receipt's on-hand and available quantities at
  three moments: while the lot is open, after a release, and — re-running from the same starting
  point — after a rejection instead. (co-08, co-09)
- **ex-05 · disposition-consequence-map** — given one nonconforming lot, map each of the five
  dispositions to the stock movement it causes, its effect on value, and whether it raises a
  procurement exception or stays internal. (co-10, co-13)

### Advanced

- **ex-06 · inspection-point-placement-trade-off** — given a three-operation routing, place the gate
  after operation one and then after operation three, and quantify what each placement costs when the
  same defect is found: the material scrapped, and the work already added to it. (co-11)
- **ex-07 · batch-traceability-blast-radius** — given a defect reported on delivered goods, trace
  backward to the receipt and forward to every delivery that consumed the same batch; then re-run the
  same trace for an item held without batch identity and state what the containment set becomes.
  (co-12, co-14)
- **ex-08 · sampling-and-the-wrong-verdict** — given a lot of 200 units where 5 were examined and
  passed, state which of the two error directions this verdict is exposed to, what a tightened rule
  would change and what it would cost, and exactly which fields must sit on the lot for a reviewer to
  reconstruct the verdict a year later. (co-04, co-05)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: design the quality gate for one purchased item family and one manufactured item family —
  triggers, characteristics, sampling rule, aggregation rule, dispositions, and the write-back point
  into stock — and write one worked trace through it.
- **Concepts exercised**: [ ] inspection trigger and lot (co-01, co-02) [ ] characteristics and
  sampling rule (co-03, co-04) [ ] verdict aggregation (co-07) [ ] usage decision and quality status
  (co-08, co-09) [ ] dispositions (co-10) [ ] traceability granularity (co-12).
- **Ordered steps**: 1) state which events raise an inspection lot for each family and why; 2) define
  each family's characteristics with their specifications; 3) state the sampling rule and what is
  recorded about it; 4) state the aggregation rule that turns results into a verdict; 5) enumerate the
  permitted dispositions and each one's stock and value consequence; 6) choose batch or serial
  granularity and justify it; 7) trace one lot end to end through a rejection.
- **Acceptance criteria**: the aggregation rule is explicit and would give a determinate verdict on
  ex-03's results; the traced quantity is counted as on hand but not as available while the lot is
  open; every disposition names both a stock consequence and a value consequence; the chosen
  traceability granularity is justified against a stated containment requirement.
- **Done bar**: a written design and one worked trace, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage B, course 21 of 27.
- `skills/sharia-erp` — Stage B, course 21 of 30.

---

← Back to the [syllabus index](../README.md)
