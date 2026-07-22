# Islamic Contract Modeling for Systems (By Example)

**Course ID**: `islamic-contract-modeling-for-systems` · **Format**: By Example. Sharia-only
(`sharia-accounting` manifest).

**Short summary**: The accounting entries for Murabaha, Mudaraba, Musharaka, Salam, and Istisnaa
contracts — modelled by their economic substance, not by conventional-loan analogy.

**Scope note**: contract-by-contract mechanics, building directly on #20's doctrinal landscape.
Zakah (#22) and sukuk (#23) are each substantial enough to warrant their own course and are out of
scope here even though both connect to the contract types taught in this course.

## Why this exists · the big idea

- **The problem before the solution**: a systems builder who models a Murabaha as "a loan with a
  markup instead of interest" gets a system that is compliant in label only — the accounting treatment,
  the risk allocation, and the disclosure requirements are all different from a loan's, because the
  underlying contract is a trade, not a lending arrangement.
- **Keep-this-if-you-forget-everything**: a murabaha is modelled as a trade — the institution buys an
  asset, then resells it to the customer at a disclosed markup, on deferred payment terms — not as a
  loan with interest relabelled as profit.
- **Big ideas touched**: `form-vs-substance` — this is the corpus's most concrete demonstration of the
  theme: two contracts can produce an identical payment schedule while one is Sharia-compliant (trade,
  asset-backed, markup-based) and the other is not (a disguised interest-bearing loan).

## Prerequisites

- **Prior courses**: `sharia-accounting-and-aaoifi-standards` (#20),
  `chart-of-accounts-and-data-modeling` (#2).
- **Assumed knowledge**: #20's doctrinal landscape and standards ecosystem, #2's schema design.

## Accuracy notes

- `[Verified]` AAOIFI Financial Accounting Standards numbers this course applies: FAS 3 (Mudaraba),
  FAS 4 (Musharaka), FAS 7 (Salam), FAS 10 (Istisnaa), FAS 28 (Murabaha and deferred payment sales).
  The accounting mechanics below are restated in original words from general domain knowledge of these
  contract types, cross-referenced against the FAS numbers `[Verified]` for their existence and scope
  only — no standard text, clause, or example is reproduced.
- This course's specific journal-entry mechanics for each contract type are `[Judgment call — original
worked examples, not sourced from any AAOIFI text or reference implementation]`, per the licensing
  posture in [tech-docs §Licensing](../../tech-docs.md#licensing-and-ip-compliance-a8).

## Concepts

- **co-01 · murabaha-as-trade** — the institution purchases an asset, then resells it to the customer
  at a disclosed cost-plus-markup, on deferred payment terms; the markup is trade profit, not
  interest (FAS 28).
- **co-02 · murabaha-vs-conventional-loan-contrast** — the same payment schedule modelled two ways:
  as a murabaha (asset purchase, then resale) and as a loan (cash advanced, then repaid with
  interest) — structurally different even when cash flows look similar.
- **co-03 · mudaraba-profit-loss-sharing** — a partnership where one party provides capital and the
  other provides expertise/labour; profit is shared by an agreed ratio, loss is borne by the capital
  provider alone (absent misconduct by the managing partner) (FAS 3).
- **co-04 · musharaka-joint-partnership** — a partnership where multiple parties contribute capital and
  share both profit and loss by an agreed ratio, distinct from Mudaraba's asymmetric loss allocation
  (FAS 4).
- **co-05 · salam-forward-sale** — a sale with payment made in full at contract signing and delivery
  of the (typically fungible, quantifiable) goods deferred to a future date — the reverse timing of a
  conventional trade (FAS 7).
- **co-06 · istisnaa-manufacturing-contract** — a contract to manufacture or construct an asset to
  specification, with payment terms agreed at contract signing and delivery on completion (FAS 10).
- **co-07 · deferred-payment-markup-recognition** — recognising a murabaha's markup as profit over the
  deferred-payment period rather than entirely at the point of resale, echoing #5's revenue-timing
  discipline applied to a Sharia-specific contract.
- **co-08 · asset-risk-transfer-timing** — the point at which the institution, rather than the
  customer, bears risk of loss on the underlying asset — a determining factor in whether a contract is
  genuinely a trade (co-01) rather than a disguised loan.

## Worked examples

### Beginner

- **ex-01 · record-a-murabaha-purchase** — the institution purchases an asset it will resell to a
  customer — verify the purchase is recorded as an asset acquisition, not as a loan disbursement.
  (co-01)
- **ex-02 · record-a-murabaha-resale** — resell the asset from ex-01 to the customer at a disclosed
  markup on deferred terms — verify the markup is recorded as deferred trade profit, not interest
  income. (co-01, co-07)

### Intermediate

- **ex-03 · contrast-murabaha-and-loan-entries** — record the identical cash-flow schedule once as a
  murabaha (per ex-01/ex-02) and once as a conventional interest-bearing loan — verify the two produce
  different account structures despite similar period-by-period cash amounts.
  (co-02, `form-vs-substance`)
- **ex-04 · record-a-mudaraba-profit-split** — split a Mudaraba venture's period profit between capital
  provider and managing partner by an agreed ratio — verify a period loss is instead borne entirely by
  the capital provider. (co-03)
- **ex-05 · record-a-musharaka-profit-and-loss-split** — split both a profit period and a loss period
  for a Musharaka partnership by each partner's agreed ratio — verify both partners share the loss,
  unlike ex-04's Mudaraba. (co-04)
- **ex-06 · record-a-salam-contract** — record full payment at signing and the deferred delivery
  obligation for a Salam forward sale — verify the timing (payment first, delivery later) is the
  reverse of a conventional sale. (co-05)
- **ex-07 · record-an-istisnaa-contract** — record a manufacturing contract's payment terms at signing
  and recognise progress toward delivery — verify the treatment against co-06's structure. (co-06)

### Advanced

- **ex-08 · recognise-deferred-markup-over-time** — recognise a multi-period murabaha's markup evenly
  over the deferred-payment period rather than entirely at resale — verify each period's recognised
  profit and that the total matches the full markup by the final period. (co-07)
- **ex-09 · disguised-loan-failure** — a contract structured with murabaha-style documentation but
  where the "seller" never actually takes asset-risk before resale (the customer bears risk from the
  start) — verify every individual entry balances while the contract's economic substance is a
  disguised loan, and name the specific test (co-08's asset-risk-transfer-timing) that reveals it.
  (co-08, silent-failure, `form-vs-substance`)

## Applied synthesis (no build — A6)

Take one contract description (a murabaha for equipment financing) by hand through purchase, resale
with disclosed markup, and period-by-period markup recognition, then re-model the identical cash-flow
schedule as a conventional interest-bearing loan and compare the two account structures. Verify the
murabaha's asset-risk-transfer timing and the loan's absence of one. No system is built — the synthesis
is the two hand-worked models and their comparison.

## Read more

- **AAOIFI — Financial Accounting Standards (FAS 3, 4, 7, 10, 28)** (aaoifi.com). Named nominatively
  for each contract type's governing standard; no standard text reproduced.
- **An Introduction to Islamic Finance: Theory and Practice** — Muhammad Ayub (Wiley Finance). A
  standard reference on Islamic contract structures; cited nominatively.

## In which paths

- `sharia-accounting` — Stage 3 · Full competence, including how to architect (not build) a
  Sharia-compliant ledger.

---

← Back to the [syllabus index](../README.md)
