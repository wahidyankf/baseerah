# Lease and Intangible Asset Accounting (By Example)

**Course ID**: `lease-and-intangible-asset-accounting` · **Format**: By Example.

**Short summary**: Lease classification and intangible-asset accounting.

**Scope note**: leased assets and intangibles specifically; #9 covers owned tangible assets. This is
the corpus's headline "misclassified lease" risk example, named explicitly in `brd.md`.

## Why this exists · the big idea

- **The problem before the solution**: lease classification is a well-documented, standard-shifted
  area (ASC 842 / IFRS 16) that most systems builders get wrong by defaulting to pre-reform
  intuitions — most leases now belong on the balance sheet, not off it.
- **Keep-this-if-you-forget-everything**: classification depends on the economic substance of the
  arrangement (who effectively controls and consumes the asset), not on what the contract calls
  itself.
- **Big ideas touched**: `form-vs-substance` — the ASC 842 / IFRS 16 reform moved lease classification
  from a bright-line legal-form test toward one grounded in economic substance; this course's headline
  risk (a misclassified lease) is exactly a form-over-substance mistake.

## Prerequisites

- **Prior courses**: `fixed-assets-and-depreciation` (#9).
- **Assumed knowledge**: #9's capitalisation and depreciation mechanics.

## Accuracy notes

- The ASC 842 / IFRS 16 lease-classification shift is stable, documented domain knowledge `[Judgment
call — restated conceptually; no standard text reproduced per A8]`.

## Concepts

- **co-01 · operating-vs-finance-lease** — the two classifications, and why ASC 842 / IFRS 16 moved
  most leases onto the balance sheet regardless of classification.
- **co-02 · right-of-use-asset** — the capitalised asset representing the lessee's right to use the
  leased item over the lease term.
- **co-03 · lease-liability** — the capitalised obligation to make future lease payments, recognised
  alongside the right-of-use asset.
- **co-04 · lease-schedule-mechanics** — the amortisation schedule for a capitalised lease's asset and
  liability over its term.
- **co-05 · identifiable-intangible-asset** — an intangible with separable, determinable value
  (patents, licences), distinct from goodwill.
- **co-06 · goodwill** — the unidentifiable excess of purchase price over the fair value of net
  identifiable assets acquired.
- **co-07 · amortisation-vs-impairment-only-treatment** — finite-lived intangibles are amortised over
  their useful life; goodwill and indefinite-lived intangibles are tested for impairment instead.
- **co-08 · capitalised-software-development-costs** — the threshold between research costs (expensed)
  and development costs (capitalisable) for internally developed software.

## Worked examples

### Beginner

- **ex-01 · classify-a-lease** — classify a five-year equipment lease as operating or finance under
  current standards — verify the classification against the stated criteria. (co-01)
- **ex-02 · identify-vs-goodwill** — split an acquisition's intangible value into an identifiable
  patent and unidentifiable goodwill — verify each is recorded in the correct account. (co-05, co-06)

### Intermediate

- **ex-03 · build-a-rou-schedule** — build the right-of-use asset and lease liability schedule for
  ex-01's lease — verify the asset and liability amortise to zero at the lease term's end. (co-02,
  co-03, co-04)
- **ex-04 · amortise-an-intangible** — amortise the identifiable patent from ex-02 over its useful life
  — verify the amortisation schedule and carrying value at each period end. (co-05, co-07)
- **ex-05 · impairment-test-goodwill** — test goodwill for impairment at a conceptual level, without
  amortising it — verify goodwill is written down only when its recoverable value falls below carrying
  value, never amortised routinely. (co-06, co-07)
- **ex-06 · capitalise-vs-expense-software-costs** — classify a software project's costs into a
  research phase (expensed) and a development phase (capitalised) — verify each phase's costs land in
  the correct treatment. (co-08)

### Advanced

- **ex-07 · full-lease-lifecycle** — take one lease from classification through its full amortisation
  schedule to term end — verify the right-of-use asset and lease liability both reach zero
  simultaneously. (co-01–co-04)
- **ex-08 · misclassified-lease-failure** — classify a lease as operating when its economic substance
  meets the finance-lease criteria — verify the trial balance still foots while the balance sheet is
  understated (the right-of-use asset and lease liability are missing entirely). (co-01, silent-failure)

## Applied synthesis (no build — A6)

Classify one lease under current standards by hand, build its full right-of-use asset and liability
schedule, and separately distinguish an identifiable intangible from goodwill in one acquisition
scenario. Verify the lease schedule amortises to zero at term end and the goodwill is correctly
excluded from routine amortisation. No system is built — the synthesis is the hand-worked schedules
and classifications.

## Read more

- **IFRS Foundation — IFRS 16 Leases** (ifrs.org). The IFRS Foundation publishes its **own** free
  teaching materials for classroom use by recognised institutions under attribution and
  non-commercial terms; **the Standards text itself still requires a separate licence to reproduce**
  `[Verified]`; named nominatively, no clause text reproduced.
- **Intermediate Accounting** — Kieso, Weygandt & Warfield (Wiley). Cited nominatively for a fuller
  treatment of intangible assets and goodwill.

## In which paths

- `conventional-accounting` — Stage 2 · Most conventional systems a mid-size company runs, plus how to
  architect (not build) a ledger system.
- `sharia-accounting` — Stage 2 · same; the shared spine both paths cover identically.

---

← Back to the [syllabus index](../README.md)
