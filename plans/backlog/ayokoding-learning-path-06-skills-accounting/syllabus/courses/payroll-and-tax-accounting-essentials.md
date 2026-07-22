# Payroll and Tax Accounting Essentials (By Example)

**Course ID**: `payroll-and-tax-accounting-essentials` · **Format**: By Example.

**Short summary**: Gross-to-net payroll mechanics, employer payroll-tax liabilities, and the
jurisdiction-dependent nature of tax accounting.

**Scope note**: the accounting entries payroll and indirect tax produce, not payroll processing
software or tax-filing procedure. Jurisdiction-specific rates and forms are explicitly out of scope —
this course teaches the accounting pattern that any jurisdiction's specific rules plug into.

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
  serves a second jurisdiction.

## Prerequisites

- **Prior courses**: `chart-of-accounts-and-data-modeling` (#2).
- **Assumed knowledge**: #2's schema.

## Accuracy notes

- Gross-to-net payroll structure and the employer/employee tax-liability split are stable, structural
  domain knowledge; specific rates, brackets, and jurisdiction rules are explicitly out of scope and
  not asserted `[Judgment call — structure only, no jurisdiction-specific rate or rule is stated as
fact anywhere in this course]`.

## Concepts

1. **co-01 · gross-pay** — an employee's full earned compensation before any withholding or deduction.
2. **co-02 · employee-tax-withholding** — amounts withheld from gross pay on the employee's behalf and
   remitted to a tax authority, recorded as a liability until remitted.
3. **co-03 · voluntary-deductions** — non-tax deductions from gross pay (benefits premiums, retirement
   contributions) the employee has authorised.
4. **co-04 · net-pay** — gross pay minus all withholdings and deductions; the amount actually paid to
   the employee.
5. **co-05 · employer-payroll-tax-liability** — taxes the employer owes on top of gross pay, never
   withheld from the employee — a separate liability, not a pass-through.
6. **co-06 · total-employer-cost** — gross pay plus employer-only payroll taxes and any employer-paid
   benefit contributions; the true cost of employing someone, larger than gross pay.
7. **co-07 · payroll-accrual** — recognising payroll expense in the period earned, even when the actual
   cash payment date falls in a later period.
8. **co-08 · indirect-tax-liability** — sales tax or VAT collected from a customer (or paid to a
   vendor) and held as a liability (or asset/expense) until remitted, structurally similar to
   employee-tax-withholding.
9. **co-09 · jurisdiction-scoped-configuration** — rates, brackets, and rules vary by jurisdiction; a
   sound system keeps this variance in configuration data, not in posting logic.

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
