# Payroll and Tax Accounting Essentials (By Example)

**Course ID**: `payroll-and-tax-accounting-essentials` · **Format**: By Example.

**Short summary**: Gross-to-net payroll mechanics, employer payroll-tax liabilities, indirect tax, and
the income-tax accounting that forces a system to carry a tax basis alongside its book basis.

**Scope note**: the accounting entries payroll, indirect tax, and income tax produce — not payroll
processing software, tax-filing procedure, or tax planning. Jurisdiction-specific rates and forms are
explicitly out of scope; this course teaches the accounting pattern that any jurisdiction's specific
rules plug into. **Income tax is covered to the depth a systems builder needs to model it**: why the
tax authority's measure of income is not book profit, why the same asset or liability has to carry a
second, tax-basis measurement in parallel with its reporting carrying amount, how differences between
those two bases become deferred tax balances, and where the recoverability judgment sits. **It is not
covered further than that**: no jurisdiction's computation of taxable profit, no rate or threshold, and
none of the specialised corners of the governing standards (ASC 740 under US GAAP, IAS 12 under IFRS)
— intra-group transfers, share-based payment, uncertain tax positions — which are named here as the
standards that govern the area and are not worked through.

## Why this exists · the big idea

- **The problem before the solution**: payroll is the transaction type nearly every business has, and
  it is also the one most likely to be modelled as "pay the employee the agreed amount" when the
  accounting reality is a gross amount split across withholdings, employee deductions, and separate
  employer-only liabilities.
- **Keep-this-if-you-forget-everything**: gross pay is not what the employer pays out, and net pay is
  not what payroll costs the employer — three different numbers (gross, net, and total employer cost)
  all matter and none of them equal each other.
- **Big ideas touched**: `standard-plurality` — payroll tax and indirect tax rules are inherently
  jurisdiction-dependent; a system that hard-codes one jurisdiction's rates or rules into its core
  posting logic (rather than into a configurable, jurisdiction-scoped layer) breaks the moment it
  serves a second jurisdiction. Also `estimation-under-uncertainty` — a deferred tax asset is carried
  only as far as future taxable profit is expected to absorb it, which makes a judgment about the
  future into a balance-sheet number.

## Prerequisites

- **Prior courses**: `chart-of-accounts-and-data-modeling` (#2).
- **Assumed knowledge**: #2's schema.

## Accuracy notes

- Gross-to-net payroll structure and the employer/employee tax-liability split are stable, structural
  domain knowledge; specific rates, brackets, and jurisdiction rules are explicitly out of scope and
  not asserted `[Judgment call — structure only, no jurisdiction-specific rate or rule is stated as
fact anywhere in this course]`.
- Income-tax accounting (co-10 through co-14) is cited by standard name only — ASC 740 under US GAAP,
  IAS 12 under IFRS — with no clause text, threshold wording, or numbering layout reproduced, per A8.
  The book-basis/tax-basis, temporary-difference and deferred-balance structure taught here is domain
  reasoning rather than a claim sourced from this plan's grounding file `[Needs Verification]` pending
  the Phase 1 coverage pass, and the recognition wording each standard uses for a deferred tax asset is
  deliberately not stated.

## Concepts

- **co-01 · gross-pay** — an employee's full earned compensation before any withholding or deduction.
- **co-02 · employee-tax-withholding** — amounts withheld from gross pay on the employee's behalf and
  remitted to a tax authority, recorded as a liability until remitted.
- **co-03 · voluntary-deductions** — non-tax deductions from gross pay (benefits premiums, retirement
  contributions) the employee has authorised.
- **co-04 · net-pay** — gross pay minus all withholdings and deductions; the amount actually paid to
  the employee.
- **co-05 · employer-payroll-tax-liability** — taxes the employer owes on top of gross pay, never
  withheld from the employee — a separate liability, not a pass-through.
- **co-06 · total-employer-cost** — gross pay plus employer-only payroll taxes and any employer-paid
  benefit contributions; the true cost of employing someone, larger than gross pay.
- **co-07 · payroll-accrual** — recognising payroll expense in the period earned, even when the actual
  cash payment date falls in a later period.
- **co-08 · indirect-tax-liability** — sales tax or VAT collected from a customer (or paid to a
  vendor) and held as a liability (or asset/expense) until remitted, structurally similar to
  employee-tax-withholding.
- **co-09 · jurisdiction-scoped-configuration** — rates, brackets, and rules vary by jurisdiction; a
  sound system keeps this variance in configuration data, not in posting logic.
- **co-10 · current-income-tax-expense** — the tax owed on the period's taxable profit, where taxable
  profit is the tax authority's measure of income and is not the book profit the income statement
  reports; the two are computed from the same transactions under different rules.
- **co-11 · parallel-tax-basis** — the same asset or liability carries two measurements at once: the
  carrying amount used for reporting, and a separate amount recognised under tax rules. Nothing else in
  this corpus asks a system to hold two simultaneous measurements of one item, and a schema storing
  only the reporting amount cannot express the three concepts below at all.
- **co-12 · temporary-vs-permanent-difference** — a temporary difference between the two bases reverses
  in a later period and therefore carries a future tax effect (an expense accrued now but deductible
  only when paid is the plainest case); a permanent difference never reverses and therefore carries
  none.
- **co-13 · deferred-tax-asset-and-liability** — the balance recognised now for the future tax effect
  of a temporary difference: a liability where the reversal will increase a future period's tax, an
  asset where it will reduce it.
- **co-14 · deferred-tax-asset-recoverability** — a deferred tax asset is carried only as far as future
  taxable profit is expected to absorb it, so its measurement is a judgment revisited every period, not
  a figure the period's transactions determine.

## Worked examples

### Beginner

- **ex-01 · gross-to-net-single-employee** — compute one employee's net pay from gross pay, stated
  withholding, and one voluntary deduction — verify net pay equals gross minus withholding minus
  deduction. (co-01, co-02, co-03, co-04)
- **ex-02 · post-a-payroll-liability** — post the withholding from ex-01 as a liability rather than an
  expense — verify the liability clears only when actually remitted to the tax authority. (co-02)

### Intermediate

- **ex-03 · compute-employer-payroll-tax** — compute an employer-only payroll tax on top of ex-01's
  gross pay — verify this amount is a separate liability, never netted against the employee's
  withholding. (co-05)
- **ex-04 · compute-total-employer-cost** — sum gross pay, employer payroll tax, and an employer benefit
  contribution — verify the total exceeds gross pay and equals the true cost of employment. (co-06)
- **ex-05 · accrue-payroll-across-a-period-boundary** — accrue payroll expense for days worked before
  period end where the actual pay date falls after — verify the accrual and its reversal when cash is
  paid net to the correct expense recognised in the correct period. (co-07)
- **ex-06 · post-an-indirect-tax-collection** — collect sales tax on a sale and post it as a liability
  distinct from revenue — verify revenue is not overstated by the tax amount. (co-08)

### Advanced

- **ex-07 · full-payroll-cycle** — take one pay period for three employees with different deduction
  profiles through gross pay, withholding, employer tax, and net pay — verify total employer cost
  reconciles to the sum of all postings. (co-01–co-06)
- **ex-08 · hard-coded-jurisdiction-failure** — a system that hard-codes one jurisdiction's tax rate
  directly into its posting logic is deployed to a second jurisdiction with a different rate — verify
  every individual entry still balances (gross still splits correctly into net plus withholding) while
  every entry uses the wrong rate, and name the fix (moving the rate into jurisdiction-scoped
  configuration). (co-09, silent-failure)
- **ex-09 · book-profit-against-taxable-profit** — take ex-05's period-end payroll accrual, deductible
  for tax only when the cash is actually paid in the following period — verify book profit and taxable
  profit differ for that period by exactly the accrual, and that the difference disappears once the
  cash is paid. (co-07, co-10, co-11)
- **ex-10 · deferred-tax-across-the-reversal** — apply a stated tax rate to ex-09's difference and
  carry the resulting deferred tax balance across both periods — verify the balance arises in the first
  period and unwinds to zero in the second, and that total tax expense across the two periods equals
  total tax paid. (co-12, co-13)
- **ex-11 · permanent-difference-contrast** — a permanently non-deductible expense placed alongside
  ex-09's temporary one — verify it changes current tax expense but produces no deferred balance, then
  verify that a system tracking only the reporting basis reports a plausible, internally consistent tax
  expense for both while being able to distinguish neither. (co-11, co-12, silent-failure)
- **ex-12 · assess-a-deferred-tax-asset** — the same deferred tax asset held by an entity whose
  expected future taxable profit is stated first as more than sufficient and then as insufficient —
  verify the carried amount differs between the two cases even though the underlying difference is
  identical, and state which number a reader could not have recomputed from the transactions alone.
  (co-13, co-14, `estimation-under-uncertainty`)

## Applied synthesis (no build — A6)

Take one employee's pay period by hand through gross pay, employee withholding, one voluntary
deduction, employer-only payroll tax, and total employer cost, and separately accrue that period's
payroll expense across a stated period boundary. Verify net pay, total employer cost, and the accrual
against independent recomputation. No system is built — the synthesis is the hand-worked payroll cycle.

## Read more

- **Payroll Accounting** — Bieg & Toland (Cengage). A standard payroll accounting textbook; cited
  nominatively for a fuller treatment of gross-to-net mechanics.

## In which paths

- `conventional-accounting` — Stage 2 · Most conventional systems a mid-size company runs, plus how to
  architect (not build) a ledger system.
- `sharia-accounting` — Stage 2 · same; the shared spine both paths cover identically.

---

← Back to the [syllabus index](../README.md)
