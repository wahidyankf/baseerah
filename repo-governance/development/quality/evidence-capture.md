---
title: "Evidence Capture Convention"
description: Standards for capturing and organizing testing evidence (screenshots, curl outputs, console logs) in plan folders and delivery.md during plan execution
category: explanation
subcategory: development
tags:
  - evidence
  - testing
  - screenshots
  - plans
  - verification
  - locale
  - manual-testing
created: 2026-06-20
---

# Evidence Capture Convention

Manual verification is only as useful as the record it leaves. A screenshot taken but not referenced,
a curl response examined but not recorded — these leave no audit trail and cannot prove the behavior
was actually tested. This convention defines where evidence lives, what form it takes, and how to
reference it from `delivery.md` so every verification step is auditable by the next reader.

## Principles Implemented/Respected

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: Evidence
  makes verification visible and checkable. "I tested it" is implicit; a screenshot and a curl response
  in the delivery notes are explicit.
- **[Root Cause Orientation](../../principles/general/root-cause-orientation.md)**: When a defect surfaces
  post-archival, evidence lets the investigator reconstruct what the state was at delivery time — a root
  cause investigation tool, not just a bureaucratic artifact.
- **[Deliberate Problem-Solving](../../principles/general/deliberate-problem-solving.md)**: Capturing
  evidence forces the tester to actually observe the system, not just trust that tests passed. The act of
  taking a screenshot and recording the curl output is itself the deliberate observation step.

## Conventions Implemented/Respected

- **[Manual Behavioral Verification](./manual-behavioral-verification.md)**: Evidence capture is the
  persistent record of manual verification. The two conventions are complementary: this one defines the
  storage structure; the other defines the verification actions.
- **[Plans Organization Convention](../../conventions/structure/plans.md)**: The `evidence/` subfolder
  sits inside the plan folder and moves with it through the lifecycle (`backlog/` → `in-progress/` →
  `done/`).
- **[Temporary Files Convention](../infra/temporary-files.md)**: `local-temp/` is for ephemeral scratch
  work. Evidence that should survive across sessions and be committed belongs in the plan's `evidence/`
  folder, not in `local-temp/`.

## The Rule

**Every manual verification step in a plan MUST produce a committed evidence artifact — inline in
`delivery.md` for text evidence, and in the plan's `evidence/` subfolder for file-based evidence
(screenshots, exported reports). Implementation notes that say "verified manually" without a record
of WHAT was observed are incomplete.**

## Evidence Folder Location

Every plan folder MAY contain an `evidence/` subfolder:

```
plans/
├── in-progress/
│   └── my-feature/
│       ├── README.md
│       ├── brd.md
│       ├── prd.md
│       ├── tech-docs.md
│       ├── delivery.md
│       └── evidence/              ← evidence goes here
│           ├── phase-1-homepage-en.png
│           ├── phase-1-homepage-id.png
│           ├── phase-2-api-health.txt
│           └── phase-3-mobile-375px.png
└── done/
    └── 2026-06-20__my-feature/
        ├── delivery.md
        └── evidence/              ← moves with the plan on archival
```

The `evidence/` folder is committed to git and moves with the plan folder when it is archived to
`done/`. It is part of the permanent historical record.

## What Goes Where

### Inline in `delivery.md` (under the implementation-notes block)

Short text evidence that fits naturally in the notes:

- **curl responses** — paste the JSON response as a fenced code block:

  ````markdown
  - [x] [AI] Verify `/api/health` returns 200 — acceptance: status 200, `{"status":"ok"}`
    > **Evidence** (2026-06-20): `curl http://localhost:8202/api/health`
    >
    > ```json
    > { "status": "ok", "version": "1.2.3" }
    > ```
  ````

- **Console output** — relevant lines from `browser_console_messages`:

  ```markdown
  > **Evidence** (2026-06-20): No JS errors. Console clean on `/en/tools` and `/id/tools`.
  ```

- **Network summary** — which endpoints were hit, what status codes returned.

- **Screenshot reference** — path to the file in `evidence/`:

  ```markdown
  > **Evidence** (2026-06-20): `![Desktop EN homepage](./evidence/phase-2-desktop-en.png)`,
  > `![Mobile 375px EN](./evidence/phase-2-mobile-en.png)`,
  > `![Mobile 375px ID](./evidence/phase-2-mobile-id.png)`
  ```

### In `evidence/` subfolder

File-based artifacts that would bloat `delivery.md` if inlined:

- **Screenshots** — one per breakpoint per locale tested; filename encodes context:
  `phase-{N}-{description}-{locale}-{breakpoint}px.png`
  Example: `phase-2-tools-page-en-1280px.png`, `phase-2-tools-page-id-375px.png`
- **Long curl responses** — if a response exceeds ~20 lines, save to
  `evidence/phase-{N}-{endpoint-slug}.txt` and reference by path in `delivery.md`
- **Lighthouse reports** — `evidence/phase-{N}-lighthouse-{locale}.json`
- **Test coverage HTML** — `evidence/phase-{N}-coverage-report.html` (if exported)

### NOT in evidence/ (use local-temp/ for ephemeral work)

- Intermediate screenshots taken for the agent's own orientation that are not cited in delivery.md
- Scratch Playwright scripts used during testing
- Draft findings not committed to the plan

## Screenshot Conventions

### Naming Pattern

```
phase-{N}-{description}-{locale}-{breakpoint}.{ext}
```

Examples:

- `phase-1-homepage-en-1280px.png` — Phase 1 homepage, English, desktop
- `phase-1-homepage-id-375px.png` — Phase 1 homepage, Indonesian, mobile
- `phase-2-calculator-en-768px.png` — Phase 2 calculator, English, tablet
- `phase-3-error-state-en-1280px.png` — Phase 3 error state, English, desktop

### Required Coverage

For **every web-UI manual verification step**:

| Coverage axis | Minimum required                                                  |
| ------------- | ----------------------------------------------------------------- |
| Breakpoints   | Mobile (375 px), tablet (768 px), desktop (1280 px) — all three   |
| Locales       | Every locale the app supports (e.g., `en`, `id`) — all of them    |
| States        | Normal state; plus error/empty states when the step exercises one |

Zero screenshots for a UI verification step is a finding under
[Plan Execution Checker](../../../.claude/agents/plan-execution-checker.md) Step 7.

### How to Capture

Write a Playwright script to `local-temp/` and run it via `npx playwright`:

```bash
# Example: capture homepage at all breakpoints in all locales
npx playwright test local-temp/capture-evidence.spec.ts
```

Or use Playwright MCP `browser_take_screenshot` for interactive captures. Either way, save
screenshots to the plan's `evidence/` subfolder, not to `local-temp/`.

## curl / API Evidence Conventions

For every API endpoint verified:

1. Record the actual command run (so it is reproducible).
2. Record the full response (or the first 20 lines if very long, with "…truncated" noted).
3. Note the HTTP status code.
4. If the response is > 20 lines, save the full response to `evidence/phase-{N}-{endpoint}.txt`.

**Minimum coverage per endpoint**: happy path (valid input → expected 2xx) + at least one error
path (invalid input → expected 4xx with error body).

Example inline record:

````markdown
> **Evidence** (2026-06-20): API verification for `/api/tools`
>
> ```bash
> curl -s http://localhost:8202/api/tools | jq .
> ```
>
> ```json
> { "tools": [{ "id": "cost-of-living-calculator", "name": "Cost of Living Calculator" }] }
> ```
>
> HTTP 200. Error path: `curl -s -w "\n%{http_code}" http://localhost:8202/api/tools/nonexistent` → 404.
````

## Locale Testing Evidence Requirements

For any plan that touches a **multilingual / multi-locale web app**, every manual verification step
MUST cover ALL supported locales — not just the default.

### How to Discover Supported Locales

```bash
# For Next.js apps: read the locale config
grep -r "locales" apps/<app-name>/src/features/i18n/ --include="*.ts" | head -10
# Or read the Next.js config
cat apps/<app-name>/next.config.ts | grep -A 5 "i18n"
```

### Required Evidence Per Locale

For each locale `L` in the app's supported locales:

1. Navigate to the locale-specific URL (e.g., `http://localhost:3101/en/tools`,
   `http://localhost:3101/id/tools`).
2. Capture a screenshot: `evidence/phase-{N}-{feature}-{L}-{breakpoint}px.png`.
3. Verify locale-specific content: correct language text, correct locale-aware formatting
   (dates, numbers, currency symbols, units).
4. Verify the `html[lang]` attribute matches the locale.
5. Verify aria-labels, page title, and meta description are in the correct language.
6. Note any missing or untranslated strings in the implementation notes.

### Locale Evidence in `delivery.md`

```markdown
- [x] [AI] Verify /tools page renders correctly in all locales — acceptance: correct language text,
      html[lang] matches locale, no untranslated strings
  > **Evidence** (2026-06-20):
  >
  > - EN: `![/en/tools desktop](./evidence/phase-3-tools-en-1280px.png)` — html lang="en", all strings translated ✓
  > - ID: `![/id/tools desktop](./evidence/phase-3-tools-id-1280px.png)` — html lang="id", all strings translated ✓
  > - EN mobile: `![/en/tools mobile](./evidence/phase-3-tools-en-375px.png)` — layout intact ✓
  > - ID mobile: `![/id/tools mobile](./evidence/phase-3-tools-id-375px.png)` — layout intact ✓
```

## What plan-execution-checker Validates

The [plan-execution-checker](../../../.claude/agents/plan-execution-checker.md) validates evidence
capture as part of Step 7 (Manual Behavioral Assertions). It checks:

1. **Screenshots exist** — for each UI verification step, `evidence/` contains at least one
   screenshot per locale per breakpoint tested.
2. **Delivery.md references evidence** — implementation notes under ticked UI-verification
   checkboxes contain `![...]` references or explicit `evidence/` file paths.
3. **Locale coverage** — for multi-locale apps, evidence covers ALL supported locales.
4. **curl evidence** — for API verification steps, implementation notes contain the command,
   status code, and response body (inline or referenced).
5. **No "verified manually" without evidence** — a bare "verified manually" note with no
   screenshot and no curl response is a **HIGH** finding.

## Examples

### PASS: Complete evidence record

```markdown
- [x] [AI] Verify salary calculator computes correctly in EN and ID — acceptance: displayed
      value matches independent computation; no console errors; all 3 breakpoints tested
  > **Evidence** (2026-06-20): Computed gross salary IDR 25,000,000/month. Independent check:
  > 25000000 / 160h = 156,250/h ✓
  >
  > - `![EN desktop](./evidence/phase-4-calc-en-1280px.png)` — value correct, no console errors
  > - `![EN mobile](./evidence/phase-4-calc-en-375px.png)` — layout intact, value correct
  > - `![ID desktop](./evidence/phase-4-calc-id-1280px.png)` — value correct, thousands separator "." ✓
  > - `![ID mobile](./evidence/phase-4-calc-id-375px.png)` — layout intact
```

### FAIL: Evidence missing

```markdown
- [x] [AI] Verify salary calculator works — verified manually ✓
```

Missing: no screenshot, no locale coverage, no computation check, no console-error check.
`plan-execution-checker` would flag this as HIGH.

### FAIL: Locale coverage incomplete

```markdown
- [x] [AI] Verify tools page — `![EN desktop](./evidence/phase-3-tools-en-1280px.png)` ✓
```

Missing: ID locale not verified. `plan-execution-checker` would flag this as HIGH if the app
supports Indonesian.

## Relationship to Other Conventions

- **[Manual Behavioral Verification](./manual-behavioral-verification.md)** — defines WHAT to verify;
  this convention defines WHERE to record the verification evidence.
- **[User-Facing Delivery Hardening Convention](./user-facing-delivery-hardening.md)** — Rule 1
  (per-breakpoint, per-locale visual sign-off) and Rule 10 (production visual sign-off before archival)
  both require the evidence trail defined here.
- **[Plans Organization Convention](../../conventions/structure/plans.md)** — plan folder structure,
  lifecycle (in-progress → done), and the evidence/ subfolder naming.
- **[Temporary Files Convention](../infra/temporary-files.md)** — evidence/ in a plan folder is NOT a
  temporary file; it is committed and permanent. Use local-temp/ for scratch work only.

## Related Documentation

- [Plan Execution Workflow](../../workflows/plan/plan-execution.md) — Step 2d mandates evidence capture
  during manual behavioral assertions.
- [plan-execution-checker](../../../.claude/agents/plan-execution-checker.md) — validates evidence
  presence as part of Step 7.
- [plan-maker](../../../.claude/agents/plan-maker.md) — emits evidence-capture steps in delivery
  checklists for web-UI plans.
- [web-exploratory-tester](../../../.claude/agents/web-exploratory-tester.md) — saves screenshots to
  the backlog plan's evidence/ folder during exploratory testing.
- [web-usability-tester](../../../.claude/agents/web-usability-tester.md) — saves screenshots to the
  backlog plan's evidence/ folder during usability evaluation.
- [web-design-tester](../../../.claude/agents/web-design-tester.md) — saves screenshots to the
  backlog plan's evidence/ folder during design-fidelity evaluation.
