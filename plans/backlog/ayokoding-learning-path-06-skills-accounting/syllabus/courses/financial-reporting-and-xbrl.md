# Financial Reporting and XBRL (Annotated-concept)

**Course ID**: `financial-reporting-and-xbrl` · **Format**: Annotated-concept.

**Short summary**: How financial statements become machine-readable regulatory filings, and why the
tagging taxonomy is itself standard-specific.

**Scope note**: a landscape and judgment framework — XBRL's tagging model, taxonomies, and the filing
mandates that require it — not a mechanism the reader executes step by step. Extends #14's
standard-plurality theme into the reporting-format layer: the tags available depend on which
standard's taxonomy applies.

## Why this exists · the big idea

- **The problem before the solution**: a financial statement meant only for a human reader can use any
  layout; a filed regulatory report increasingly must also be machine-readable, and that requirement
  changes what "correct" means — a statement can be numerically correct and still fail a filing if it
  is tagged wrong.
- **Keep-this-if-you-forget-everything**: XBRL tagging is not formatting — it is asserting, machine-
  readably, which taxonomy element a reported figure represents, and mis-tagging a figure to the wrong
  element is a new, XBRL-specific failure mode that has nothing to do with whether the underlying
  number is arithmetically correct.
- **Big ideas touched**: `standard-plurality`, extended from #14's accounting-standard divergence into
  the reporting-format layer — the US GAAP Financial Reporting Taxonomy and the IFRS Taxonomy are
  different, standard-specific tag sets, not one universal tagging scheme.

## Prerequisites

- **Prior courses**: `financial-reporting-standards-ifrs-vs-gaap` (#14).
- **Assumed knowledge**: #14's IFRS-vs-GAAP divergences, as the reason the taxonomies themselves
  diverge.

## Accuracy notes

- XBRL's tagging model (elements, contexts, instance documents) and the existence of jurisdiction-
  specific mandates (e.g. SEC filing requirements in the US, ESMA's ESEF mandate in the EU) are
  documented `[Judgment call — cited generically, structure only, no taxonomy text reproduced per A8]`.
  Flagged `[Needs Verification]` pending the Phase 1 coverage pass, since filing-mandate details
  change over time and must be re-verified at authoring.

## Concepts

- **co-01 · xbrl-overview** — eXtensible Business Reporting Language: a standardized, machine-readable
  markup for financial data, distinct from a human-readable statement layout.
- **co-02 · taxonomy** — the defined set of reportable elements (e.g. "Revenue," "AssetsCurrent") a
  filer can tag figures against; taxonomies are standard-specific.
- **co-03 · element-tagging** — attaching a taxonomy element to a specific reported figure, asserting
  what that figure represents.
- **co-04 · instance-document** — the actual filing: a set of tagged facts for a specific reporting
  period and entity.
- **co-05 · context-and-dimensional-tagging** — attaching period, entity, and dimensional context
  (e.g. by segment or by currency) to a tagged fact, so the same element can represent different
  sliced figures.
- **co-06 · taxonomy-extension** — a filer-specific addition to the standard taxonomy for a figure the
  base taxonomy has no element for, used sparingly since over-extension defeats comparability.
- **co-07 · filing-mandate** — the jurisdiction-specific requirement to file in XBRL (or a related
  machine-readable format) as a condition of regulatory filing, distinct from the accounting standard
  itself.
- **co-08 · taxonomy-divergence** — the US GAAP Financial Reporting Taxonomy and the IFRS Taxonomy are
  separately maintained, standard-specific tag sets — a figure correct under one accounting standard
  still needs the matching taxonomy's element, not a translated tag from the other.

## Tensions & trade-offs — when tagging effort is and is not worth it

- **Detail tagging vs. block tagging**: tagging every individual figure gives maximal machine
  readability but costs the most effort; block-tagging a whole note as one element is cheaper but
  loses granularity — the choice is a real cost/comparability tradeoff, not a formality.
- **Extension vs. forcing a fit**: extending the taxonomy for a genuinely novel figure preserves
  meaning; forcing a novel figure into the closest existing element to avoid an extension preserves
  comparability but can misrepresent what the figure actually is — neither choice is free.

## Worked examples

Grouped by theme; no fixed Beginner/Intermediate/Advanced bands (Annotated-concept). Every example
cites the `co-NN` it exercises.

### Theme A · Tagging mechanics

- **ex-01 · tag-a-revenue-figure** — tag a reported revenue figure against the correct taxonomy
  element for a stated standard — verify the element name matches the correct taxonomy (US GAAP vs
  IFRS) for that standard. (co-02, co-03)
- **ex-02 · attach-context-to-a-tagged-fact** — attach period and entity context to a tagged figure so
  it is unambiguous in a multi-period filing — verify the context distinguishes it from the prior
  period's same-element fact. (co-05)
- **ex-03 · dimensional-tag-a-segment-figure** — tag revenue broken out by two business segments using
  dimensional context — verify each segment's figure is separately identifiable and the two sum to the
  total. (co-05)

### Theme B · Where taxonomy divergence bites

- **ex-04 · cross-taxonomy-mistag-failure** — tag a figure prepared under IFRS against a US GAAP
  taxonomy element with a similar-sounding name but a different definition — verify the filing is
  internally well-formed (it validates) while the tag misrepresents what the figure is, and name the
  observable signal (a taxonomy-conformance review) that would catch it. (co-08, silent-failure)
- **ex-05 · decide-detail-vs-block-tagging** — for a disclosure note with five distinct sub-figures,
  decide between detail-tagging each and block-tagging the whole note — verify the decision against
  the stated cost/comparability tradeoff, not a default. (co-06)

## Applied synthesis (no build — A6)

Take one small set of reported figures (revenue, an asset balance, one segment breakout) by hand
through taxonomy selection, element tagging, and context attachment for a stated standard, then
identify what would change if the same figures were refiled under the other standard's taxonomy.
Verify the tagging choices and the cross-standard comparison. No system is built — the synthesis is
the hand-worked tagging exercise and its cross-standard comparison.

## Read more

- **XBRL International — XBRL Specification** (xbrl.org). Named nominatively as the standard-setting
  body for the XBRL specification; structure only, no specification text reproduced.
- **US SEC — EDGAR Filer Manual** (sec.gov). Named nominatively as a real, public filing-mandate
  reference; not transcribed.

## In which paths

- `conventional-accounting` — Stage 2 · Most conventional systems a mid-size company runs, plus how to
  architect (not build) a ledger system.
- `sharia-accounting` — Stage 2 · same; the shared spine both paths cover identically.

---

← Back to the [syllabus index](../README.md)
