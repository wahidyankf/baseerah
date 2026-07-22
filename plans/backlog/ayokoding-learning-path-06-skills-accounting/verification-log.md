# Verification Log — ayokoding-learning-path-06-skills-accounting

The machine-checkable ledger for this plan's carried verification debt. Every line below is asserted
by an acceptance clause in [delivery.md](./delivery.md); the human-readable statement of each item,
its named primary source, and its escape hatch live in
[tech-docs §Open verification items](./tech-docs.md#open-verification-items-oi-1-through-oi-4).

## Why this file exists

The research seeding this plan originally marked only **three** items as directly fetched and
verified: the AAOIFI Financial Accounting Standards index, AAOIFI's adoption-by-country page, and
IAI's PSAK Syariah index. A follow-up `web-researcher` grounding run on **2026-07-22** re-confirmed
and extended two of the three open items below (OI-1, OI-3). A4 forbids promoting any unresolved item
to fact silently, so every marker travels here as a **status line a grep can check** rather than as
prose a reviewer has to interpret.

## Status lines (grep-checkable — one per item, first column anchored)

Each line begins at column 0 and matches `^OI-<n>: <STATUS>`. Valid statuses:

| Status          | Meaning                                                                                                                                            |
| --------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `OPEN`          | Not yet resolved. **Blocks** the phase named in tech-docs.                                                                                         |
| `RESOLVED`      | Checked against the named primary source. Line carries the source URL and the access date.                                                         |
| `SCOPED-AROUND` | The primary source could not be reached; the affected course teaches the structure without publishing the specific claim. Line carries the reason. |
| `ROUTED`        | Not a research item — a cross-plan seam handed to its owning plan. Line carries the target.                                                        |

Do **not** delete a line when it resolves — rewrite its status in place, so the ledger stays a
complete record.

OI-1: RESOLVED — IAI's published PSAK Syariah standard list (`iaiglobal.or.id`), re-confirmed via the
2026-07-22 `web-researcher` grounding run — the operative series is PSAK 101-110; PSAK 59 is
superseded. Residual: the exact PPSAK ratification date for PSAK 101 was NOT confirmed by this run —
course #20 (`sharia-accounting-and-aaoifi-standards`) cites the series only, never a specific
ratification date, until that residual is separately resolved.
OI-2: OPEN
OI-3: RESOLVED — AAOIFI's adoption-by-country index, re-confirmed via the 2026-07-22 `web-researcher`
grounding run, for the adoption-relationship claim specifically: Malaysia is not on AAOIFI's
mandatory-adoption list; Indonesia uses AAOIFI as a basis, not an adoption. Residual: governance
mechanics beyond this specific relationship (e.g. the internal provisions of Bank Negara Malaysia's
Shariah Governance Policy 2019) were not directly fetched by this run and remain subject to the
standing "fast-moving facts, re-verify at authoring" rule in tech-docs.md.
OI-4: OPEN

## Item summaries

- **OI-1** `RESOLVED` (with a stated residual) — **Indonesian PSAK numbering.** The prior conflict
  between a "PSAK 59 / SIFAS 101-109" generation and a "PSAK 101-110" series is resolved: **PSAK
  101-110 is the operative series; PSAK 59 was superseded.** Source: IAI's published PSAK Syariah
  standard list, re-confirmed 2026-07-22. The exact PPSAK ratification date for PSAK 101 remains
  unconfirmed; the corpus's rule for this residual is to cite the series and never a specific
  ratification date. Course #20's authoring proceeds without further blocking on this item.
- **OI-2** `OPEN` — **Riba doctrinal basis.** Still sourced only from Wikipedia, which is not a
  primary source. Primary source to check: an **AAOIFI Shari'ah Standard** or an **IFSB publication**.
  The practical consequence is well-attested (profit must arise from trade, leasing, partnership or
  service risk, never a predetermined return on a pure loan); the minority time-value-of-money
  position is **not settled** and is not this corpus's to settle. **This item is explicitly left
  OPEN by this rewrite — it is not resolved by the 2026-07-22 grounding run and must not be restated
  as fact.** Blocks course #20.
- **OI-3** `RESOLVED` (for the adoption-relationship claim; governance minutiae beyond it remain
  re-verify-at-authoring) — **Three-jurisdiction adoption relationship.** Confirmed: Malaysia does
  **not** appear on AAOIFI's published mandatory-adoption list; MASB standards are IFRS-converged,
  with Sharia treatment handled through Bank Negara policy documents rather than AAOIFI adoption;
  Indonesia's position is "AAOIFI as basis", not adoption. Source: AAOIFI's adoption-by-country index,
  re-confirmed 2026-07-22. Governance-mechanics detail beyond this specific relationship — the
  internal provisions of Bank Negara Malaysia's Shariah Governance Policy 2019 itself — was not
  directly fetched by this run; any course citing those provisions specifically still follows the
  standing re-verify-at-authoring rule. Blocks courses #20, #21, #24 (the adoption-relationship
  portion of the block is now clear; the governance-minutiae portion is unchanged).
- **OI-4** routed, **already answered** — plan 02's doc-level rule _"A path may omit a prerequisite
  only if it also omits every course that needs it"_ used to read as forbidding this plan's
  link-don't-walk manifests. Plan 02 has since published a dated ruling resolving it —
  `tech-docs.md §"Link-don't-walk: prerequisite omission is permitted (OI-4 ruling, 2026-07-21)"` —
  which rules Direction A **PERMITTED** and names this plan's OI-4 explicitly. Plan 02's
  **implemented** `checkPrerequisiteConsistency` already permitted it even before the ruling; the
  ruling brings the prose in line. Phase 0 confirms and records the ruling in the status line above
  (never edited from here). Blocks nothing mechanically.

## Verified facts carried in (do not re-litigate, do re-confirm at authoring)

`[Verified]` AAOIFI Financial Accounting Standards numbers for the contract types this corpus covers:
**FAS 3** (Mudaraba), **FAS 4** (Musharaka), **FAS 7** (Salam), **FAS 9** (Zakah — now taught, course
\#22), **FAS 10** (Istisnaa), **FAS 28** (Murabaha and deferred payment sales), **FAS 32–34** (Ijarah
through sukuk-holder reporting — Sukuk now taught, course #23). AAOIFI keeps **Financial Accounting
Standards** and **Shari'ah Standards** as two separate series — "what to book" versus "what makes the
contract compliant". **FAS numbers outside this list are `[Unverified]`** and are re-verified or
dropped, never published on trust.

`[Verified]` (2026-07-22 grounding run) — **licensing posture**: IAI (Indonesia) is the strictest of
the four bodies this corpus touches, with **no educational exception at all**; AAOIFI is free to read
but has no published permission-to-reproduce policy (treated as closed); **no public-domain chart of
accounts exists anywhere**, so every chart of accounts in this corpus is originally authored. Full
table: [tech-docs §Licensing and IP Compliance](./tech-docs.md#licensing-and-ip-compliance-a8).

## Carried residuals — verification-marker resolution status at the Phase 4 gate

**Historical note (superseded by the 2026-07-22 grounding pass — do not re-derive from this
section's earlier wording).** Earlier authoring-time drafts of this register named
`sharia-accounting-and-aaoifi-standards.md` (one marker) and
`sharia-ledger-system-architecture.md` (three markers) as carrying `[Needs Verification]`
placeholders pending Phase 1's coverage pass. The exhaustive grounding pass that followed resolved
every placeholder in the corpus before Phase 1 began — a repo-wide check across all 24
`syllabus/courses/*.md` files (excluding `README.md`), reproducible with
`find syllabus/courses -maxdepth 1 -name '*.md' ! -name 'README.md' -print0 | xargs -0 grep -l
'\[Needs Verification\]' | wc -l`, confirms:

- **0** files carry a `[Needs Verification]` marker — the placeholder tag is retired from this
  corpus entirely.
- **48** `[Verified...]`-tagged claims remain (across 20 files) — resolved facts with a stated
  primary source.
- **18** `[Web-cited...]`-tagged claims remain (across 10 files) — resolved facts with an inline
  URL and access date.
- **4** `[Unverified...]`-tagged claims are **honestly retained**, per `A4`/`A8`, in exactly these
  files (each with a stated, non-asserting reason, either inline in the tag or in the immediately
  surrounding prose — never a bare unexplained tag):
  - `managerial-and-cost-accounting.md` — a primary-source-level claim corroborated only by
    secondary study guides.
  - `sharia-accounting-and-aaoifi-standards.md` — any FAS number outside the `[Verified]`
    whitelist, explicitly marked re-verify-or-drop before being written.
  - `sharia-ledger-system-architecture.md` — the balance-sheet presentation of profit-sharing
    investment-account balances, deliberately not asserted (Apache Fineract's own two mentions in
    this file already carry `[Web-cited]`, not `[Unverified]` — they resolved separately).
  - `zakah-computation-and-reporting-for-systems.md` — a version- and jurisdiction-volatile value,
    deliberately not asserted.

**This is the target state, not a gap.** A retained `[Unverified]` tag is a legitimate terminal
state under `A4` — "the corpus honestly declines to assert this, with a reason" — not the same
thing as the old `[Needs Verification]` placeholder, which meant "this claim has not been resolved
either way yet." Because the placeholder tag no longer appears anywhere in the corpus, the Phase 4
gate's checks in `delivery.md` were rewritten to (a) confirm the placeholder's absence and (b) use
an anti-vacuity companion that confirms the corpus's verification labelling is genuine — a
substantial number of `[Verified]`/`[Web-cited]`/`[Unverified]` tags actually present — rather than
checking for files named against a register that no longer describes any real marker.

### Not registered here — OI-ledger residuals are a different mechanism

A **residual of a `RESOLVED` OI item** is not a `[Needs Verification]` marker on a course claim, and
does not belong in the register above. The two are tracked separately and must not be conflated:

- **OI-1's PPSAK residual.** The operative series (**PSAK 101-110**) and the supersession of
  **PSAK 59** are `[Verified]`; only the exact **PPSAK ratification date** for PSAK 101 is
  unconfirmed. Course #20 states this as
  `[Verified, PSAK numbering — OI-1 resolved]` with the residual named inline — it carries **no**
  `[Needs Verification]` marker for the date, because the course cites the _series_ and never a date,
  so there is no unverified claim to mark. Tracked as OI-1's residual in the `## Item summaries`
  above, not in the register.
- **OI-3's governance-mechanics residual** is tracked the same way, for the same reason.

Do not "fix" the register by adding these; doing so would name markers that do not exist, which is
the exact defect this section exists to prevent.

### Forward constraints — not yet markers, must not become unmarked assertions

The two items below came from the 2026-07-22 factual-grounding pass. **Neither is currently a
`[Needs Verification]` marker in any course file**, and neither is registered above, because the
text they concern **has not been authored yet**. They are constraints on Phase 1/Phase 2 authoring,
recorded here so they are not silently resolved the wrong way. Do not cite them as existing
residuals; do not treat their absence from the corpus as a defect.

- **`financial-statements-and-close-cycle.md` (course #3), when it enumerates the primary
  statements** — whether the statement of changes in equity counts as a **fourth** primary statement
  under IAS 1 / ASC 220 is unresolved. The course currently scopes itself to balance sheet, income
  statement and cash flow (its `## Scope note`). If authoring adds the fourth statement, it must
  carry `[Needs Verification]` and be registered above, never asserted as fact.
- **`financial-statements-and-close-cycle.md`'s `ex-04 · derive-the-cash-flow-statement`** — the
  worked example does not state whether it uses the **direct** or **indirect** method. When the
  example is written out at authoring time it must state which, or carry `[Needs Verification]` and
  be registered above. Silence is acceptable in the syllabus spec; silence in the authored body is
  not.

**Every other marker is an authoring-time placeholder** that Phase 1's coverage pass must resolve or
promote into this register with a stated reason before the Phase 4 gate passes.
