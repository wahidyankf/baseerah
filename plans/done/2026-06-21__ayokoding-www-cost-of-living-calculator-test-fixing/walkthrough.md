# Cognitive Walkthrough — Cost-of-Living Calculator (Usability)

Spec-blind first-time-user transcript from `web-usability-tester`, 2026-06-20. Each step rates the four
cognitive-walkthrough questions (Q1 right result? Q2 correct action visible? Q3 action associated with
goal? Q4 progress made visible?). Findings raised are linked to their `UWT-###`.

## Task 1 — "Cost to live in Bangkok, 2 adults + 1 preschool child" (en, 1280 + 375 px)

1. **Land on page** — Q1–Q4 pass; one friction: H1 "Salary Savings Calculator" ≠ tab title "Cost of Living Calculator" → **UWT-001**.
2. **Adults → 2** — labelled select, essentials chips update. Pass.
3. **Preschool children → 1** — childcare/school chips update. Pass.
4. **Filter Country = Thailand, City = Bangkok** — three cascading selects visible; Region control's purpose unexplained but works; featured row updates. Pass (at 375 px the selects stack cleanly).
5. **Read Bangkok result** — "Total THB 43,000" in a primary chip, prominent. Pass.
   - Residual: multi-city comparison shows bare numbers without inline currency → **UWT-009**.

## Task 2 — "Monthly savings at $5,000 living in Jakarta" (en, 1280 px)

1. **Click Savings tab** — inactive tab reads "SavingsSee how much you'd save" (fused) → **UWT-002**. On click the table shows all-red negatives before any salary → **UWT-003** (severe trust failure).
2. **Enter salary 5000** — input "Gross monthly salary (before tax) USD" found above the table (label-inline-output pattern, slightly non-standard position); table updates to positive figures. Pass.
3. **Find Jakarta** — must scroll (no per-city filter inside Savings); "Savings after lifestyle" column never defined on this tab (context deficit, UWT-006 spirit). Pass with friction.

## Task 3 — "Minimum job title to save $500/mo in Singapore" (en, 1280 px)

1. **Click Minimum role tab** — fused label again (**UWT-002**); role table appears with no target entered → **UWT-007**.
2. **Set "Baseline source"** — opaque label, no help → **UWT-006**.
3. **Enter target 500** — currency defaults to USD though user said Singapore (friction); table re-sorts; "Best city" shows Austin for all top roles.
4. **Find Singapore result** — Minimum Role tab appears not to honour the geo Region/Country/City filter ("Best city: Austin" persists); divergence unexplained. Noted as comprehension friction (spec-blind — cannot assert bug).

## Task 4 — "Compare cost of living, first open on phone" (id, 375 px)

1. **Land** — H1 "Kalkulator Tabungan Gaji" correctly localized; controls in Bahasa Indonesia; tab fusion present in DOM but partly off-viewport at mobile. Pass.
2. **Open mobile nav** — drawer shows a single "English Content >" item (English label even on id locale), no site nav → **UWT-011**.

**Verdict:** Core tool is data-rich and mostly usable, but a first-timer hits trust blockers within ~30 s:
H1/title mismatch (UWT-001), pre-populated red empty states (UWT-003/UWT-007), fused tab labels
(UWT-002), and the raw-key tools index (UWT-004).
