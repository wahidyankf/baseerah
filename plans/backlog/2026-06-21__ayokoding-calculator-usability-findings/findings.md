# Usability Findings — AyoKoding Cost-of-Living Calculator

Findings sorted by severity (4 → 0), then by area. All findings are spec-blind: judgements are
grounded in established usability principles and prevailing web conventions, not in any knowledge
of the product's intended behaviour.

**Severity scale (Nielsen 0–4)**

| Rating | Label                   |
| ------ | ----------------------- |
| 4      | Usability catastrophe   |
| 3      | Major usability problem |
| 2      | Minor usability problem |
| 1      | Cosmetic problem        |
| 0      | Not a problem           |

---

## UWT-001 — Savings tab shows all-negative values on first open with no explanation

**Severity**: 3 — Major usability problem
**Priority**: High
**Area**: Savings tab — empty / zero-salary state
**Persona & task**: First-time visitor, Task 3 (estimate savings)
**Environment**: `/en/tools/cost-of-living-calculator`, Chromium, 1280 px, `en`, 2026-06-21
**Reproducibility**: Always

### Steps to Reproduce

1. Open `http://localhost:3101/en/tools/cost-of-living-calculator`.
2. Click the "Savings" tab.
3. Observe the table without touching any input.

### Expected (predictable) behaviour

A first-time user arriving on a "Savings" tab expects either an invitation to enter their salary
(with a clear placeholder or prompt), or a blank / zero-row table. When no salary has been entered
yet, the table showing deeply negative savings for every city (e.g. "INR -54,500 / $-578") is not
self-explanatory. The user's question — "am I really losing money in every city?" — has no visible
answer on screen.

### Actual behaviour

The salary input (`#gross-salary-input`) has no placeholder text (confirmed: `placeholder: ""`).
The table renders immediately with "Net (monthly): INR 0 / $0" and "Savings after essentials:
INR -54,500 / $-578 (—)" for every row. The label "Gross monthly salary (before tax) USD" is
visible above the empty box, but a first-timer's attention goes to the table, where all values are
negative. There is no explanatory note near the table that says "Enter your salary above to see
personalised savings."

### Violated principle

Heuristic 1 (Visibility of system status) — the system does not tell the user why values are
negative or what action to take to produce meaningful output. Heuristic 6 (Recognition over recall)
— the user must independently reason that a zero-salary causes the negatives; this should be made
explicit.

### Evidence

`./evidence/phase-1-savings-tab-en-1280px.png` (all-negative table, empty input)
`./evidence/phase-5-savings-with-salary-en-1280px.png` (after $5,000 entered — table becomes
positive)

### Suggested clarification

Add a placeholder to `#gross-salary-input` (e.g. "e.g. 5000") and add a contextual note beneath
the input or above the table ("Enter your gross monthly salary to see how much you would save in
each city."). Alternatively, hide the table rows until a salary is entered and show a friendly
empty-state message instead.

---

## UWT-002 — Minimum Role tab: three modes (radio buttons) have no visible group label

**Severity**: 3 — Major usability problem
**Priority**: High
**Area**: Minimum Role tab — mode selector (radiogroup)
**Persona & task**: First-time visitor, Task 4 (find minimum role)
**Environment**: `/en/tools/cost-of-living-calculator`, Chromium, 1280 px, `en`, 2026-06-21
**Reproducibility**: Always

### Steps to Reproduce

1. Open the calculator and click "Minimum role" tab.
2. Observe the three buttons at the top of the tab: "Monthly savings target", "Reference role",
   "My salary".
3. Note there is no visible heading or label group above these three buttons explaining what the
   choice affects.

### Expected (predictable) behaviour

A first-time user expects radio-group controls to be preceded by a visible question or label that
explains what the choice selects. Without that group label, users must experiment to discover that
these three options switch between entirely different ranking methodologies. The three noun phrases
("Monthly savings target", "Reference role", "My salary") do not tell the user whether they are
selecting a display mode, a filter, or a data source.

### Actual behaviour

The three radio buttons sit at the top of the tab with no visible group label. The radiogroup's
`aria-label` is not surfaced as visible text on screen. The Cost-of-living tab's area selector
shows a visible "Area" SPAN label; the Minimum Role mode selector has no equivalent visible text.

### Violated principle

Heuristic 6 (Recognition over recall) — the user must recall or experiment to understand what the
radio group controls. ISO 9241-110 §2 (Self-descriptiveness) — an interface element should explain
its own purpose without additional documentation. Heuristic 4 (Consistency and standards) — the
area selector has a visible label; the mode selector does not.

### Evidence

`./evidence/phase-7-min-role-subtabs-en-1280px.png` (three radio buttons, no group label)
`./evidence/phase-5-min-role-default-en-1280px.png` (initial state)

### Suggested clarification

Add a visible label above the three radio buttons, e.g. "Ranking mode:" or "Find minimum role
by:". Mirror the pattern used for the "Area" label on the Cost-of-living tab.

---

## UWT-003 — Tab sub-descriptions are screen-reader-only; sighted users have no visible hint of what each tab requires

**Severity**: 3 — Major usability problem
**Priority**: High
**Area**: Tab bar — all three tabs
**Persona & task**: First-time visitor, Task 1 (orient)
**Environment**: `/en/tools/cost-of-living-calculator`, Chromium, 1280 px, `en`, 2026-06-21
**Reproducibility**: Always

### Steps to Reproduce

1. Open the calculator and observe the three tabs.
2. Only bare tab label text is visible: "Cost of living", "Savings", "Minimum role".
3. Inspect the DOM: `<span id="tab-desc-savings" class="sr-only">See how much you'd save</span>`
   and `<span id="tab-desc-min-role" class="sr-only">Find the min role you need</span>` exist but
   are clipped off-screen (`clip-path: inset(50%)`).
4. The "Cost of living" tab has no `aria-describedby` at all — no description exists for it.

### Expected (predictable) behaviour

A first-time user scanning three tab labels cannot reliably predict what input each tab requires or
what output each produces. Sub-descriptions visible beneath or alongside each tab label — even
small, muted text — would let users self-route without clicking each tab to discover its purpose.

### Actual behaviour

Sub-descriptions for "Savings" and "Minimum role" are clipped off-screen. The "Cost of living" tab
has no description at all. Sighted users see only the bare labels.

### Violated principle

Heuristic 6 (Recognition over recall) — users must click to learn, rather than reading visible
cues to decide. Information scent (Pirolli & Card) — the tab labels do not communicate the
required action. Heuristic 4 (Consistency) — two non-default tabs have sr-only descriptions; the
first tab has none.

### Evidence

`./evidence/phase-1-initial-en-1280px.png` (all three tabs visible, no sub-descriptions)

### Suggested clarification

Surface sub-descriptions as visible subtitle text (small, muted, below or alongside the tab
label). Add a matching description for the "Cost of living" tab. Example: "Cost of living: compare
monthly expenses" | "Savings: see how much you'd keep" | "Minimum role: find the seniority you
need".

---

## UWT-004 — Savings tab: salary label embeds "USD" as static text with no route to change currency

**Severity**: 2 — Minor usability problem
**Priority**: Medium
**Area**: Savings tab — salary input
**Persona & task**: First-time visitor with non-USD salary, Task 3
**Environment**: `/en/tools/cost-of-living-calculator`, Chromium, 1280 px, `en`, 2026-06-21
**Reproducibility**: Always

### Steps to Reproduce

1. Click the "Savings" tab.
2. Read the label: "Gross monthly salary (before tax) USD".
3. Notice there is no currency selector adjacent to this input.
4. Compare with the "Minimum role" tab, which has a "Target currency" selector next to its
   savings-target input.

### Expected (predictable) behaviour

A tool targeting 30+ cities across Asia, Europe, and the Americas would allow a user with a salary
in EUR, SGD, or IDR to enter their salary in their own currency. The "Minimum role" tab provides a
"Target currency" dropdown; the Savings tab does not.

### Actual behaviour

The label hardcodes USD. The Savings tabpanel has zero `<select>` elements (confirmed). The
"Minimum role" tab's `<select aria-label="Target currency">` covers the equivalent input on that
tab but is absent from the Savings tab.

### Violated principle

Heuristic 4 (Consistency and standards) — the "Minimum role" tab allows a target currency; the
"Savings" tab does not, despite accepting the same class of input (a monetary amount). Jakob's Law
— comparable global financial tools allow currency selection for salary inputs.

### Evidence

`./evidence/phase-1-savings-tab-en-1280px.png` (label shows USD, no currency selector)
`./evidence/phase-7-min-role-monthly-target-mode-en-1280px.png` (Min Role has currency selector)

### Suggested clarification

Add a currency selector adjacent to the salary input on the Savings tab, consistent with the
Minimum Role tab pattern. Alternatively, add a visible note that the salary must be entered in
USD.

---

## UWT-005 — Tab state is not reflected in the URL; back-navigation and bookmarking always reset to "Cost of living"

**Severity**: 2 — Minor usability problem
**Priority**: Medium
**Area**: URL naturalness — tab state
**Persona & task**: First-time visitor returning to a previous view, Task 3
**Environment**: `/en/tools/cost-of-living-calculator`, Chromium, 1280 px, `en`, 2026-06-21
**Reproducibility**: Always

### Steps to Reproduce

1. Open the calculator. Click "Savings" tab. URL stays:
   `http://localhost:3101/en/tools/cost-of-living-calculator` (unchanged).
2. Copy the URL, open a new tab, paste. The page opens on "Cost of living" tab.
3. Observe that selecting a city does update the URL (`?tab=cost&city=singapore`) — inconsistently
   with tab selection.

### Expected (predictable) behaviour

If city selection updates the URL, tab selection should too. Users who bookmark or share the page
expect the recipient to land on the same tab. The browser back button should navigate tab-to-tab.

### Actual behaviour

Switching tabs does not update the URL. A `?tab=savings` or `?tab=min-role` query param is never
added, even though the URL-state mechanism for city-select (`?tab=cost&city=singapore`) exists.

### Violated principle

Nielsen "URLs as UI" — the address bar should reflect navigable state. Heuristic 3 (User control
and freedom) — users cannot bookmark or share the active tab state. Heuristic 4 (Consistency) —
city selection updates the URL but tab selection does not.

### Evidence

Observed via Playwright: URL before and after tab clicks was unchanged; city-select produced
`?tab=cost&city=singapore`.

### Suggested clarification

Add `?tab=savings` / `?tab=min-role` / `?tab=cost` to the URL on tab change. Restore the correct
tab on load when the param is present.

---

## UWT-006 — Minimum Role tab shows highest-earning roles ranked when no savings target is entered

**Severity**: 2 — Minor usability problem
**Priority**: Medium
**Area**: Minimum Role tab — empty / zero-target state
**Persona & task**: First-time visitor, Task 4 (find minimum role)
**Environment**: `/en/tools/cost-of-living-calculator`, Chromium, 1280 px, `en`, 2026-06-21
**Reproducibility**: Always

### Steps to Reproduce

1. Click "Minimum role" tab without entering a savings target.
2. The table shows "CTO in Austin, $130,300 USD" as the top row.

### Expected (predictable) behaviour

When no savings target has been entered, the user has not yet declared their goal. A pre-ranked
list with no explanation of the ranking criterion (the small "Ranking key" note is below the
radiogroup) leaves the user asking "Is this telling me I need to be a CTO?" when in fact the
ranking is by highest essential savings — not by minimum required seniority.

### Actual behaviour

The table displays all roles ranked without communicating that a target is needed to make the list
filter to the user's goal. "Monthly savings target" appears selected as a radio button but the
input field is empty and the full unfiltered ranking renders.

### Violated principle

Heuristic 1 (Visibility of system status) — no indication that the ranking is unfiltered. Heuristic
6 (Recognition over recall) — small "Ranking key" note must be read to understand the ranking.

### Evidence

`./evidence/phase-5-min-role-default-en-1280px.png`

### Suggested clarification

When the target input is empty, either replace the table with an invitation ("Enter your monthly
savings target to see which roles meet it") or add a prominent inline note ("Showing all roles
ranked by essential savings — enter a target to filter").

---

## UWT-007 — "MENA", "ASEAN", and "Nordics" region labels are unexpanded acronyms or informal names

**Severity**: 2 — Minor usability problem
**Priority**: Low
**Area**: Cost of Living tab — Region filter
**Persona & task**: First-time visitor not from those regions, Task 2 (filter by region)
**Environment**: `/en/tools/cost-of-living-calculator`, Chromium, 1280 px, `en`, 2026-06-21
**Reproducibility**: Always

### Steps to Reproduce

1. Click the "Region" dropdown.
2. Read options: "Africa", "Americas", "ASEAN", "Asia", "Europe", "Japan", "MENA", "Nordics",
   "Oceania".
3. Note "MENA" and "ASEAN" are unexpanded acronyms; "Nordics" is informal; "Japan" is a single
   country listed among regions.

### Expected (predictable) behaviour

Region labels in a global tool should be unambiguous to any first-time user regardless of
background. "MENA" and "ASEAN" are unfamiliar to users outside those regions. "Japan" as a
region-level entry alongside "Asia" and "ASEAN" creates a granularity inconsistency.

### Actual behaviour

Acronyms appear as bare strings with no tooltip, expansion, or parenthetical.

### Violated principle

Heuristic 2 (Match between system and the real world) — acronyms not universally known create
friction. ISO 9241-110 §2 (Self-descriptiveness).

### Evidence

`./evidence/phase-1-initial-en-1280px.png`

### Suggested clarification

Expand acronyms in the dropdown text ("ASEAN (Southeast Asia)", "MENA (Middle East & North
Africa)") or add a `title` attribute for hover expansion. Consider whether "Japan" should appear
under "Asia" or whether "Nordics" should be renamed "Northern Europe".

---

## UWT-008 — Horizontal scroll at 320 px with no affordance

**Severity**: 2 — Minor usability problem
**Priority**: Medium
**Area**: Responsive usability — 320 px viewport
**Persona & task**: First-time visitor on narrow phone, Task 2 (mobile)
**Environment**: `/en/tools/cost-of-living-calculator`, Chromium, 320 px, `en`, 2026-06-21
**Reproducibility**: Always at 320 px

### Steps to Reproduce

1. Open the calculator at 320 px viewport width.
2. `document.body.scrollWidth > window.innerWidth` returns `true`.
3. No horizontal scroll shadow, arrow, or truncation cue is visible.

### Expected (predictable) behaviour

At 320 px (iPhone SE) content should either reflow or show an explicit scroll affordance. A user
does not know whether the clipped content is navigation controls or data.

### Actual behaviour

The page body overflows horizontally at 320 px with no visual indicator.

### Violated principle

Heuristic 1 (Visibility of system status) — user does not know content is hidden off-screen.
WCAG 1.4.10 Reflow — content should reflow at 320 CSS px without horizontal scrolling (AA).

### Evidence

`./evidence/phase-8-initial-en-320px.png`

### Suggested clarification

Audit the filter row and table for min-width constraints that prevent reflow. For the table, add a
horizontal-scroll container with a visible scroll shadow or "swipe to see more" label.

---

## UWT-009 — Tools index: the sole tool card has no description

**Severity**: 2 — Minor usability problem
**Priority**: Low
**Area**: Tools index — `/en/tools`
**Persona & task**: First-time visitor discovering tools, Task 1 (orientation)
**Environment**: `http://localhost:3101/en/tools`, Chromium, 1280 px, `en`, 2026-06-21
**Reproducibility**: Always

### Steps to Reproduce

1. Navigate to `http://localhost:3101/en/tools`.
2. H1 is "Tools". One link: "Cost of Living Calculator". No description.

### Expected (predictable) behaviour

A tools index page is an information-scent checkpoint. A bare link title is moderately good scent
but a one-line description would sharply raise first-click accuracy, especially as more tools are
added.

### Actual behaviour

One `<a>` with text "Cost of Living Calculator" inside its immediate parent, no sibling description
element (confirmed: `parentText === linkText`).

### Violated principle

Information scent (Pirolli & Card) — the label alone omits what the tool does, what data it covers,
and who it is for. Heuristic 6 (Recognition over recall).

### Evidence

`./evidence/phase-4-tools-index-en-1280px.png`
`./evidence/phase-4-tools-index-id-1280px.png`

### Suggested clarification

Add a brief description to each tool card, e.g. "Cost of Living Calculator — Compare monthly
essentials, savings potential, and minimum seniority across 30+ cities worldwide."

---

## UWT-010 — "Back to all cities" link does not restore prior filter state

**Severity**: 1 — Cosmetic problem
**Priority**: Low
**Area**: Cost of Living tab — city-detail back link
**Persona & task**: First-time visitor, Task 2 (browsing city details)
**Environment**: `/en/tools/cost-of-living-calculator`, Chromium, 1280 px, `en`, 2026-06-21
**Reproducibility**: Always when a city is selected

### Steps to Reproduce

1. Set Region to "ASEAN", Country to "Singapore", City to "Singapore".
2. Click "Back to all cities". URL becomes `?tab=cost` — Region and Country reset to "All".

### Expected (predictable) behaviour

Clicking "Back to all cities" should restore the user to the filter state before drilling in
(ASEAN region, Singapore country).

### Actual behaviour

`href="?tab=cost"` — no region, country, or other filter state is preserved in the back-link URL.

### Violated principle

Heuristic 3 (User control and freedom) — back action should restore a safe prior state. Heuristic
5 (Error prevention) — destroying filter context without warning forces users to redo work.

### Evidence

Observed via Playwright: `back link href: "…?tab=cost"`.

### Suggested clarification

Encode the active filter state in the back-link URL, e.g. `?tab=cost&region=asean&country=sg`.

---

## UWT-011 — "Cost of living" tab has no `aria-describedby` while sibling tabs do

**Severity**: 1 — Cosmetic problem
**Priority**: Low
**Area**: Tab bar — Cost of living tab
**Persona & task**: Any user; screen-reader path
**Environment**: `/en/tools/cost-of-living-calculator`, Chromium, all breakpoints
**Reproducibility**: Always

### Steps to Reproduce

1. Inspect DOM: the Cost of living tab button has no `aria-describedby`.
2. The Savings and Minimum role tabs each have `aria-describedby` pointing to an sr-only span.

### Expected (predictable) behaviour

Consistency requires all three tabs to follow the same ARIA pattern.

### Actual behaviour

First tab lacks `aria-describedby` and has no `<span id="tab-desc-cost">` sibling.

### Violated principle

Heuristic 4 (Consistency and standards). WCAG 3.2.4 (Consistent Identification).

### Suggested clarification

Add `<span id="tab-desc-cost" class="sr-only">Compare monthly living costs by city</span>` and
`aria-describedby="tab-desc-cost"` on the Cost of living tab button.

---

## UWT-012 — "OOP" abbreviation footnote is spatially separated from the data label

**Severity**: 1 — Cosmetic problem
**Priority**: Low
**Area**: Cost of Living tab — city detail / table legend
**Persona & task**: First-time visitor reading city expenses
**Environment**: `/en/tools/cost-of-living-calculator`, Chromium, 1280 px, `en`, 2026-06-21
**Reproducibility**: Always when "Healthcare (OOP)" is visible

### Steps to Reproduce

1. Select a city to enter city detail view.
2. Read "Healthcare (OOP)" in the expense breakdown.
3. The definition "OOP = out-of-pocket…" appears as a separate paragraph, not adjacent to the
   label.

### Expected (predictable) behaviour

Abbreviations embedded in data labels should have definitions immediately discoverable — ideally a
tooltip on the abbreviation or a footnote directly below the row.

### Actual behaviour

The OOP definition is a separate block-level paragraph, visually distant from the label.

### Violated principle

Heuristic 2 (Match between system and the real world). Law of Proximity — related items should be
grouped visually.

### Evidence

`./evidence/phase-5-col-singapore-selected-en-1280px.png`

### Suggested clarification

Wrap "OOP" in `<abbr title="out-of-pocket — healthcare you pay yourself">OOP</abbr>` wherever it
appears so the definition is one hover away.
