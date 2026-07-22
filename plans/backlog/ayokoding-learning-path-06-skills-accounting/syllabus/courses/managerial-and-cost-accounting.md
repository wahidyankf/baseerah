# Managerial and Cost Accounting (By Example)

**Course ID**: `managerial-and-cost-accounting` · **Format**: By Example.

**Short summary**: Internal, decision-support accounting — cost behaviour and cost-volume-profit
analysis — distinct from the external financial reporting #3 covers.

**Scope note**: cost classification (fixed, variable, mixed), break-even analysis, and job vs. process
costing. This course's cost-classification vocabulary is a direct prerequisite for #10's inventory
costing, so it must land before inventory.

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

## Concepts

1. **co-01 · financial-vs-managerial-accounting** — audience (outsiders vs. insiders), rules (external
   standards vs. none), and purpose (report the past vs. decide the future).
2. **co-02 · fixed-cost** — a cost that does not change with production volume in the relevant range.
3. **co-03 · variable-cost** — a cost that changes proportionally with production volume.
4. **co-04 · mixed-cost** — a cost with both fixed and variable components.
5. **co-05 · cost-estimation-methods** — the high-low method and regression as ways to split a mixed
   cost into its fixed and variable components.
6. **co-06 · cost-volume-profit-analysis** — the relationship between cost behaviour, sales volume, and
   profit.
7. **co-07 · break-even-point** — the volume at which total revenue equals total cost.
8. **co-08 · job-costing** — accumulating cost per distinct job or order, used when output units differ
   meaningfully from each other.
9. **co-09 · process-costing** — accumulating cost per production process and averaging across
   homogeneous units, used when output units are essentially identical.
10. **co-10 · misclassification-outside-the-ledger** — a cost-classification error that never touches a
    posted journal entry, yet corrupts a pricing or volume decision built on it.

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
