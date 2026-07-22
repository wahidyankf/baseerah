# Managerial and Cost Accounting (By Example)

**Course ID**: `managerial-and-cost-accounting` · **Format**: By Example.

**Short summary**: Internal, decision-support accounting — cost behaviour, cost-volume-profit analysis,
and how a manufactured unit's cost is assembled — distinct from the external financial reporting #3
covers.

**Scope note**: cost classification (fixed, variable, mixed), break-even analysis, job vs. process
costing, manufacturing-overhead absorption, and the roll-up that turns component costs into one
finished-unit cost. This course's cost-classification vocabulary is a direct prerequisite for #10's
inventory costing, and so is its cost roll-up: **the per-unit cost #10's FIFO and weighted-average
methods consume is produced here**, not given. Both reasons put this course before inventory.

## Why this exists · the big idea

- **The problem before the solution**: financial accounting (#3) answers "how did the business
  perform?" for outsiders; managerial accounting answers "what should we do next?" for insiders — and
  a systems builder who conflates the two ends up building one reporting layer that serves neither
  well.
- **Keep-this-if-you-forget-everything**: cost classification is a modelling decision with real
  consequences — calling a cost "fixed" when it is actually variable (or vice versa) does not touch
  the general ledger at all, yet corrupts every decision built on it.
- **Big ideas touched**: `silent-failure` — this course's silent failure lives outside the ledger
  entirely: misclassifying a cost leaves every posted entry correct while the business decision built
  on the misclassification is not.

## Prerequisites

- **Prior courses**: `financial-statements-and-close-cycle` (#3).
- **Assumed knowledge**: #3's statement derivation.

## Accuracy notes

- Cost-volume-profit analysis and the job/process costing distinction are stable, widely taught
  managerial-accounting domain knowledge with no dynamic component to re-verify at authoring
  `[Verified — stable, non-dynamic domain fact]`.
- Overhead absorption, the absorption-vs-variable-costing contrast, and the cost roll-up (co-11
  through co-15) are domain reasoning about how a unit cost is assembled before #10 consumes it, not
  claims sourced from this plan's grounding file `[Needs Verification]` pending the Phase 1 coverage
  pass. Which of the two costings an external reporting standard requires is deliberately not asserted
  here — #14 is where standard divergence is adjudicated.

## Concepts

- **co-01 · financial-vs-managerial-accounting** — audience (outsiders vs. insiders), rules (external
  standards vs. none), and purpose (report the past vs. decide the future).
- **co-02 · fixed-cost** — a cost that does not change with production volume in the relevant range.
- **co-03 · variable-cost** — a cost that changes proportionally with production volume.
- **co-04 · mixed-cost** — a cost with both fixed and variable components.
- **co-05 · cost-estimation-methods** — the high-low method and regression as ways to split a mixed
  cost into its fixed and variable components.
- **co-06 · cost-volume-profit-analysis** — the relationship between cost behaviour, sales volume, and
  profit.
- **co-07 · break-even-point** — the volume at which total revenue equals total cost.
- **co-08 · job-costing** — accumulating cost per distinct job or order, used when output units differ
  meaningfully from each other.
- **co-09 · process-costing** — accumulating cost per production process and averaging across
  homogeneous units, used when output units are essentially identical.
- **co-10 · misclassification-outside-the-ledger** — a cost-classification error that never touches a
  posted journal entry, yet corrupts a pricing or volume decision built on it.
- **co-11 · manufacturing-overhead** — production costs that cannot be traced to one unit (factory
  rent, supervision, machine depreciation) and therefore have to be spread across units by a rule
  rather than assigned by observation.
- **co-12 · predetermined-overhead-rate** — the rate, set from estimated overhead over an estimated
  activity base (machine hours, labour hours), that lets a unit carry an overhead amount before the
  period's actual overhead is known.
- **co-13 · over-and-under-applied-overhead** — the gap between overhead applied at co-12's rate and
  overhead actually incurred, and the period-end decision about where that gap is written off.
- **co-14 · absorption-costing-vs-variable-costing** — absorption costing treats fixed manufacturing
  overhead as part of a unit's cost, so it sits in inventory until the unit sells; variable costing
  treats it as a period cost expensed as incurred. The same production and sales figures therefore
  yield two different profit numbers whenever inventory levels change, and neither is an arithmetic
  error.
- **co-15 · cost-roll-up** — assembling a finished unit's cost from its component materials, its direct
  labour, and its absorbed overhead, so the "unit cost" #10's costing methods take as input is itself a
  computed figure resting on co-11 through co-14's choices.

## Worked examples

### Beginner

- **ex-01 · classify-five-costs** — classify rent, direct materials, a sales commission, factory
  supervisor salary, and shipping cost as fixed, variable, or mixed — verify each classification
  against its behaviour under a volume change. (co-02, co-03, co-04)
- **ex-02 · split-a-mixed-cost** — apply the high-low method to a utility bill across two volume
  levels — verify the resulting fixed and variable components sum back to the original bills. (co-04,
  co-05)

### Intermediate

- **ex-03 · build-a-cvp-model** — build a cost-volume-profit model for one product from its fixed and
  variable costs — verify profit at three different volumes. (co-06)
- **ex-04 · compute-break-even** — compute the break-even unit volume from ex-03's model — verify
  profit is exactly zero at that volume. (co-07)
- **ex-05 · job-cost-a-custom-order** — accumulate direct materials, direct labour, and allocated
  overhead for one custom job — verify the total job cost. (co-08)
- **ex-06 · process-cost-a-production-run** — average total process cost across a batch of identical
  units — verify the per-unit cost. (co-09)
- **ex-07 · choose-job-vs-process** — given two production scenarios (custom furniture vs. bottled
  water), choose job or process costing for each — verify the choice matches each scenario's
  production-process reality, not a preference. (co-08, co-09)

### Advanced

- **ex-08 · cvp-with-mixed-costs** — build a full CVP model incorporating a mixed cost split via ex-02
  — verify break-even and profit-at-volume figures against independent recomputation. (co-04–co-07)
- **ex-09 · misclassification-pricing-failure** — misclassify a variable cost as fixed inside a pricing
  model, then verify the resulting price recommendation at a new volume is wrong even though no
  journal entry anywhere is incorrect. (co-10, silent-failure)
- **ex-10 · absorb-overhead-at-a-predetermined-rate** — set a predetermined rate from stated estimated
  overhead and estimated machine hours, then apply it to ex-05's custom job — verify the job's total
  cost now includes an overhead amount, and that the amount changes if the activity base changes while
  nothing about the job itself does. (co-11, co-12)
- **ex-11 · dispose-of-an-overhead-variance** — compare overhead applied under ex-10's rate against a
  stated actual overhead figure — verify the difference, state whether it is over- or under-applied,
  and verify that writing it off at period end leaves total cost across all units equal to overhead
  actually incurred. (co-12, co-13)
- **ex-12 · roll-up-a-finished-unit-cost** — assemble one finished unit's cost from stated component
  material costs, direct labour, and ex-10's absorbed overhead — verify the rolled-up figure, then
  state which single number #10's FIFO and weighted-average examples would consume from it. (co-15)
- **ex-13 · absorption-against-variable-costing** — take one period where production exceeds sales and
  compute profit twice from the identical data, once under absorption costing and once under variable
  costing — verify the two profit figures differ by the fixed overhead sitting in unsold inventory,
  that both are internally consistent, and that a reader shown only one figure could not tell which
  costing produced it. (co-14, co-15, silent-failure)

## Applied synthesis (no build — A6)

Take one product's cost data by hand through classification, a CVP model, and a break-even
calculation, then choose correctly between job costing and process costing for a stated production
scenario and justify the choice from the scenario's characteristics. Verify the break-even figure
against independent recomputation. No system is built — the synthesis is the hand-worked model and
the costing-method justification.

## Read more

- **Cost Accounting: A Managerial Emphasis** — Horngren, Datar & Rajan (Pearson). The standard
  managerial/cost accounting textbook; cited nominatively for a fuller treatment of CVP analysis and
  job/process costing.

## In which paths

- `conventional-accounting` — Stage 2 · Most conventional systems a mid-size company runs, plus how to
  architect (not build) a ledger system.
- `sharia-accounting` — Stage 2 · same; the shared spine both paths cover identically.

---

← Back to the [syllabus index](../README.md)
