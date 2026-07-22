# Sukuk and Islamic Capital Markets Accounting (Annotated-concept)

**Course ID**: `sukuk-and-islamic-capital-markets-accounting` · **Format**: Annotated-concept. **NEW
course (A9)**. Sharia-only (`sharia-accounting` manifest).

**Short summary**: How sukuk differ from conventional bonds in structure and accounting, and what
sukuk-holder reporting requires.

**Scope note**: a landscape and judgment framework — sukuk structures and sukuk-holder reporting — not
a mechanism the reader executes step by step. A real domain gap the original catalog never taught
despite AAOIFI FAS 32–34 being `[Verified]` in the seeding research. Builds on #21's Ijarah-adjacent
contract concepts and #12's FX-translation mechanics for cross-currency sukuk issuances.

## Why this exists · the big idea

- **The problem before the solution**: a sukuk is often described to newcomers as "an Islamic bond,"
  which is a useful first approximation and a dangerous final one — a conventional bond is a debt
  instrument paying interest; a sukuk (in its compliant forms) represents an ownership or usufruct
  interest in an underlying asset, paying a profit share or rental return, not interest.
- **Keep-this-if-you-forget-everything**: sukuk-holders' return is tied to the performance or usufruct
  of an underlying asset, structurally different from a bondholder's fixed interest coupon — even when
  the cash-flow schedule looks similar.
- **Big ideas touched**: `form-vs-substance`, extended to capital markets — and `standard-plurality`,
  extended a fourth time: sukuk accounting under AAOIFI FAS 32–34 is a distinct standards track from
  conventional bond accounting, not a relabelled version of it.

## Prerequisites

- **Prior courses**: `islamic-contract-modeling-for-systems` (#21),
  `multi-currency-accounting-and-fx-translation` (#12).
- **Assumed knowledge**: #21's Ijarah-adjacent contract reasoning, #12's translation mechanics for
  cross-currency issuances.

## Accuracy notes

- `[Verified]` AAOIFI FAS 32–34 (Ijarah through sukuk-holder reporting) are this course's anchor
  standards. Specific issuance structures beyond the general asset-backed-vs-asset-based distinction
  are `[Judgment call — cited generically from general domain knowledge, not sourced from the seeding
grounding file; flagged`[Needs Verification]`pending the Phase 1 coverage pass]`.

## Concepts

1. **co-01 · sukuk-vs-bond-contrast** — a sukuk represents an ownership or usufruct interest in an
   underlying asset; a conventional bond represents a debt claim paying interest — the "Islamic bond"
   description is a useful approximation, not a structural equivalence.
2. **co-02 · asset-backed-sukuk** — a sukuk structure genuinely backed by transferred ownership of an
   underlying asset, with sukuk-holders bearing the asset's performance risk.
3. **co-03 · asset-based-sukuk** — a sukuk structure referencing an asset without a genuine ownership
   transfer, a more contested structure closer to the debt-like end of the spectrum, and a case where
   `form-vs-substance` scrutiny matters most.
4. **co-04 · ijarah-sukuk-structure** — a common sukuk structure built on an Ijarah (lease) contract,
   where sukuk-holders receive rental-derived returns tied to the leased asset.
5. **co-05 · profit-distribution-vs-interest-coupon** — sukuk-holders receive a profit or rental
   distribution tied to underlying performance; bondholders receive a fixed interest coupon regardless
   of the issuer's underlying asset performance.
6. **co-06 · sukuk-issuance-accounting** — recognising the proceeds of a sukuk issuance and the
   corresponding asset/usufruct transfer at the issuer's books.
7. **co-07 · sukuk-holder-reporting** — the disclosure requirements specific to reporting a
   sukuk-holder's position, distinct from conventional bondholder disclosure, per AAOIFI FAS 32–34.
8. **co-08 · cross-currency-sukuk-translation** — a sukuk issued in a currency other than the issuer's
   functional currency requires #12's translation mechanics applied to a Sharia-specific instrument.

## Tensions & trade-offs — where the sukuk/bond line gets contested

- **Asset-backed vs. asset-based, in practice**: not every sukuk in the market is unambiguously one or
  the other — some structures sit closer to the asset-based end while marketed with asset-backed
  language, which is precisely the `form-vs-substance` scrutiny this course exists to teach, not a
  settled binary a reader can apply mechanically.
- **When the "Islamic bond" framing is useful vs. misleading**: useful as a first-approximation
  explanation to an audience unfamiliar with Islamic finance; misleading the moment it is used to
  justify accounting a sukuk identically to a bond.

## Worked examples

Grouped by theme; no fixed Beginner/Intermediate/Advanced bands (Annotated-concept). Every example
cites the `co-NN` it exercises.

### Theme A · Structural contrast

- **ex-01 · contrast-sukuk-and-bond-cash-flows** — lay out an Ijarah-sukuk's rental-based distribution
  schedule beside a conventional bond's interest-coupon schedule for a similar principal amount —
  verify the two schedules can look numerically similar while representing structurally different
  claims. (co-01, co-05)
- **ex-02 · classify-asset-backed-vs-asset-based** — given two sukuk structure descriptions, one with a
  genuine asset ownership transfer and one referencing an asset without transfer, classify each —
  verify the classification rests on ownership transfer, not on marketing language. (co-02, co-03,
  `form-vs-substance`)

### Theme B · Issuer and holder accounting

- **ex-03 · record-a-sukuk-issuance** — record the proceeds and corresponding asset/usufruct transfer
  for an Ijarah-sukuk issuance at the issuer's books — verify both sides of the entry. (co-04, co-06)
- **ex-04 · report-a-sukuk-holder-position** — prepare a sukuk-holder disclosure distinct from a
  conventional bondholder disclosure, per FAS 32–34's reporting requirement — verify the disclosure
  reflects the holder's asset/usufruct interest, not a debt claim. (co-07)
- **ex-05 · translate-a-cross-currency-sukuk** — translate a sukuk issued in a foreign currency into
  the issuer's reporting currency using #12's mechanics — verify the translated distribution schedule.
  (co-08)

### Theme C · Where the line gets contested

- **ex-06 · asset-based-mislabelled-as-asset-backed-failure** — a sukuk marketed and disclosed as
  asset-backed but structured without a genuine ownership transfer — verify every individual entry
  balances while the disclosure misrepresents the holder's actual claim, and name the observable
  signal (no transferred title in the underlying legal documentation) that would reveal it. (co-02,
  co-03, silent-failure, `form-vs-substance`)

## Applied synthesis (no build — A6)

Take one Ijarah-sukuk issuance by hand through issuer-side recognition, a sukuk-holder disclosure, and
a cross-currency translation, then classify the structure as asset-backed or asset-based from its
described ownership terms and justify the classification. Verify the issuer's entry, the holder
disclosure, and the classification each rest on the structural distinctions this course teaches. No
system is built — the synthesis is the hand-worked issuance and classification.

## Read more

- **AAOIFI — FAS 32, 33, 34** (aaoifi.com). Named nominatively as this course's anchor standards; no
  standard text reproduced.
- **Sukuk Structures: Legal Engineering Under Governing Laws** and comparable Islamic capital markets
  literature — cited nominatively as domain references, not transcribed.

## In which paths

- `sharia-accounting` — Stage 3 · Full competence, including how to architect (not build) a
  Sharia-compliant ledger.

---

← Back to the [syllabus index](../README.md)
