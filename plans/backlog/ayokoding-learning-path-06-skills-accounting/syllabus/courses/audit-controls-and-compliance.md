# Audit, Controls, and Compliance (Annotated-concept)

**Course ID**: `audit-controls-and-compliance` · **Format**: Annotated-concept.

**Short summary**: Internal controls and audit as the discipline of catching this corpus's own
silent-failure theme before it reaches a decision-maker.

**Scope note**: a landscape and judgment framework — internal control frameworks, segregation of
duties, control testing, and audit trails — not a mechanism the reader executes step by step. This
course is explicitly the corpus's answer to its own recurring question: who, or what, is supposed to
catch a trial balance that foots while being wrong.

## Why this exists · the big idea

- **The problem before the solution**: every prior course from #4 onward has shown a way the ledger
  can balance while being substantively wrong. This course is where the reader learns the discipline
  built specifically to catch that class of error before it reaches a decision-maker.
- **Keep-this-if-you-forget-everything**: a control is preventive or detective, never both — design a
  system's controls knowing which kind each one is, and never assume a detective control (a
  reconciliation, a review) substitutes for a preventive one (a three-way match, a segregation of
  duties) that was skipped.
- **Big ideas touched**: `silent-failure` — this is the corpus's explicit treatment of the discipline
  that exists to catch it — and `form-vs-substance` — audit testing exists precisely because a
  transaction's paperwork (form) can be complete and consistent while its economic reality
  (substance) is not what the paperwork claims.

## Prerequisites

- **Prior courses**: `financial-statements-and-close-cycle` (#3).
- **Assumed knowledge**: #3's close cycle and statement derivation; the silent-failure examples from
  #4 through #13 as concrete material this course generalises from.

## Accuracy notes

- The COSO Internal Control–Integrated Framework's five components (control environment, risk
  assessment, control activities, information and communication, monitoring) and the fraud triangle
  (pressure, opportunity, rationalization) are stable, widely taught domain knowledge
  `[Judgment call — cited generically, framework name and structure only, no text reproduced per A8]`.
  Flagged `[Needs Verification]` pending the Phase 1 coverage pass.

## Concepts

- **co-01 · internal-control-framework** — a structured set of components (e.g. COSO's five) an
  organisation uses to design and evaluate its controls, named nominatively, never reproduced.
- **co-02 · control-environment** — the tone-at-the-top and organisational culture that determines
  whether controls are taken seriously or routinely overridden.
- **co-03 · preventive-control** — a control that stops an error or fraud before it happens (e.g. the
  three-way match at #6, a data-layer constraint at #2).
- **co-04 · detective-control** — a control that finds an error or fraud after it happens (e.g. a
  bank reconciliation, a variance review).
- **co-05 · segregation-of-duties** — no single person should be able to both execute and conceal a
  transaction (e.g. the person who approves a payment should not also be the person who can edit the
  vendor master).
- **co-06 · control-testing** — verifying that a described control actually operates as designed,
  distinct from merely confirming the control's existence on paper.
- **co-07 · audit-trail** — a complete, unbroken record of who did what and when, which every prior
  course's "never delete, always reverse" pattern (#4's `co-03`) exists in part to preserve.
- **co-08 · materiality** — not every error is worth investigating at the same intensity; materiality
  is the threshold judgment for where audit and control effort concentrates.
- **co-09 · management-override-risk** — the specific risk that the people with authority to bypass a
  control are also the people best positioned to benefit from bypassing it.
- **co-10 · fraud-triangle** — pressure, opportunity, and rationalization as the three conditions
  commonly present together when fraud occurs; a lens for where controls matter most.

## Tensions & trade-offs — when NOT to reach for more control

- **Control cost vs. risk reduction**: every control has a cost (time, friction, headcount); a control
  environment with no controls at all is negligent, but one with maximal controls everywhere is
  unusable — the judgment is proportioning control intensity to materiality (co-08) and risk, not
  maximising controls uniformly.
- **Preventive vs. detective, not either/or**: relying entirely on detective controls (catch it later)
  when a cheap preventive control (stop it now) is available is a design mistake; relying entirely on
  preventive controls with no detective backstop misses the controls that were themselves
  circumvented.

## Worked examples

Grouped by theme; no fixed Beginner/Intermediate/Advanced bands (Annotated-concept). Every example
cites the `co-NN` it exercises.

### Theme A · Classifying controls

- **ex-01 · classify-five-controls** — classify the three-way match (#6), a monthly bank
  reconciliation, a data-layer balance constraint (#2), a segregation-of-duties rule, and an annual
  physical inventory count as preventive or detective — verify each classification against co-03/co-04.
  (co-03, co-04)
- **ex-02 · design-a-segregation-of-duties-rule** — for a payment-approval workflow, identify which two
  roles must never be held by the same person — verify the rule blocks the specific concealment
  scenario it targets. (co-05)

### Theme B · Connecting controls to this corpus's own silent failures

- **ex-03 · map-a-prior-silent-failure-to-its-control** — for #9's mismatched-depreciation-method
  failure and #10's method-inconsistent-with-physical-flow failure, name the control (if any) that
  would most plausibly catch each — verify the mapping distinguishes a control that exists in this
  corpus's material from a gap this corpus has left open. (co-01, `silent-failure`)
- **ex-04 · audit-trail-vs-editable-history** — contrast a ledger that reverses errors (#4's pattern)
  against a hypothetical one that edits history in place — verify only the reversing pattern preserves
  a usable audit trail for a later investigator. (co-07)

### Theme C · Materiality and override risk

- **ex-05 · apply-a-materiality-threshold** — given two errors of different sizes relative to total
  assets, decide which merits investigation under a stated materiality threshold — verify the decision
  against the threshold, not against a fixed dollar amount. (co-08)
- **ex-06 · identify-override-risk** — in a scenario where the person who can post journal entries is
  also the person who reconciles the bank account, identify the override risk and the
  segregation-of-duties fix — verify the fix separates the two roles. (co-09, co-05)

## Applied synthesis (no build — A6)

Take one control failure from this corpus's own prior material (choose from #6, #9, #10, #13) and
design, on paper, a preventive control and a detective control that together would have caught it —
naming which role(s) each control depends on and why neither alone would have been sufficient. No
system is built — the synthesis is the paper control design and its justification.

## Read more

- **COSO — Internal Control–Integrated Framework** (coso.org). Named nominatively as the standard
  internal-control framework reference; framework structure only, no text reproduced.
- **Principles of Fraud Examination** — Wells (Wiley/ACFE). A standard text on the fraud triangle and
  fraud examination; cited nominatively.

## In which paths

- `conventional-accounting` — Stage 2 · Most conventional systems a mid-size company runs, plus how to
  architect (not build) a ledger system.
- `sharia-accounting` — Stage 2 · same; the shared spine both paths cover identically.

---

← Back to the [syllabus index](../README.md)
