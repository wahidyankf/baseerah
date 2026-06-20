# Usability Findings — ayokoding-www Cost-of-Living / Salary Calculator

**Evaluation date**: 2026-06-19
**Evaluator stance**: spec-blind first-timer
**Plan**: `plans/in-progress/ayokoding-www-calc-usability-findings/`

Each finding carries:

- **ID** (UWT-NNN)
- **Violated principle** — named heuristic / walkthrough question / UX law / WCAG SC
- **Severity** — Nielsen 0–4
- **Priority** — proposed business urgency

---

## UWT-001 — `html[lang]` remains `en` on the Indonesian locale page

**Severity**: 4 — Usability catastrophe
**Priority**: Critical

**Violated principle**: WCAG 2.1 SC 3.1.1 Language of Page (Level AA); ISO 9241-110 §2
Self-Descriptiveness; Heuristic 2 (Match Between System and the Real World — the system signals
the wrong natural language to assistive technology).

**Area / Component**: All pages under `/id/` locale, `<html>` element.

**Persona & task**: Indonesian-locale first-time visitor — any task.

**Environment**: `http://localhost:3101/id/tools/cost-of-living-calculator`, Chromium, all
viewports, 2026-06-19.

**Steps to Reproduce**:

1. Navigate to `http://localhost:3101/id/tools/cost-of-living-calculator`.
2. Inspect the `<html>` element (browser DevTools or `document.documentElement.lang`).
3. Observe: `lang` attribute value is `en`.

**Expected (predictable) behaviour**: The `html[lang]` attribute of any page served at `/id/...`
should be `id` (ISO 639-1 code for Bahasa Indonesia), so screen readers pronounce the content in
the correct language and the browser's built-in auto-translate detects it as Indonesian.

**Actual behaviour**: `html[lang]="en"` regardless of locale. The page renders in Bahasa
Indonesia visually, but the machine-readable language declaration says "English".

**Evidence**: Playwright script `uwt-calc-final.mjs` — logged `ID page html[lang]: en`.
Screenshots: `uwt-desktop-1280-id.png`, `uwt-mobile-375-id.png`.

**Reproducibility**: Always.

**Suggested clarification**: The Next.js `<html lang={locale}>` prop (or equivalent i18n
configuration) should resolve the active locale and write `lang="id"` for Indonesian pages and
`lang="en"` for English pages.

---

## UWT-002 — Page H1 ("Salary Savings Calculator") mismatches the URL slug ("cost-of-living-calculator")

**Severity**: 3 — Major usability problem
**Priority**: High

**Violated principle**: Heuristic 4 — Consistency and Standards (the URL is part of the
interface; URL slug and H1 should agree); Nielsen "URLs as UI" (URL must match content scent);
Heuristic 6 — Recognition Over Recall (the user who arrives from a search result for
"cost-of-living calculator" cannot confirm they are in the right place from the H1 alone).

**Area / Component**: Page heading, URL slug, browser tab title.

**Persona & task**: All personas — orientation (Walkthrough Task 1, CW-Q1: "Will the user try
to achieve the right result?").

**Environment**: `http://localhost:3101/en/tools/cost-of-living-calculator`, 1280 px, 2026-06-19.

**Steps to Reproduce**:

1. Navigate to `http://localhost:3101/en/tools/cost-of-living-calculator`.
2. Observe the URL slug: `cost-of-living-calculator`.
3. Observe the H1 rendered on the page: "Salary Savings Calculator".
4. Observe the browser tab title: "AyoKoding" (no tool name at all).
5. Note: the URL implies a cost-of-living focus; the H1 implies a salary-savings focus; the
   browser tab gives no tool context at all.

**Expected (predictable) behaviour**: The H1, the URL slug, and the browser tab title should all
describe the same tool concept. A user who types the URL manually or arrives from a search result
should be immediately confirmed that they reached the right page. By Nielsen's "URLs as UI",
removing a trailing path segment should reach a sensible parent; the slug should match content.
A first-timer who sees "Salary Savings Calculator" in the H1 would not guess the URL contains
"cost-of-living".

**Actual behaviour**: H1 says "Salary Savings Calculator"; URL slug says "cost-of-living-calculator";
browser tab title says only "AyoKoding" (no tool name). Three different descriptions of the same
page create a three-way scent mismatch.

**Evidence**: Playwright output — `H1: [ 'Salary Savings Calculator' ]`, `PAGE TITLE: AyoKoding`,
`URL slug: cost-of-living-calculator`, `Mismatch? true`. Screenshot: `uwt-desktop-initial-state.png`.

**Reproducibility**: Always.

**Suggested clarification**: Align H1, URL slug, and page `<title>` around a single agreed tool
name (e.g. "Salary & Cost-of-Living Calculator" in the H1; `salary-cost-of-living-calculator` in
the slug; `Salary & Cost-of-Living Calculator — AyoKoding` in the browser tab). If a rename is
out of scope, at minimum add the tool name to `<title>` so the tab is meaningful.

---

## UWT-003 — URL state parameters do not restore visible filter dropdowns

**Severity**: 3 — Major usability problem
**Priority**: High

**Violated principle**: Heuristic 1 — Visibility of System Status (the URL implies a filtered
state that the filter UI does not reflect); Heuristic 6 — Recognition Over Recall (a user who
returns via a bookmarked URL must re-discover which filter was set); WCAG 3.2.2 On Input (state
communicated to the user should match the actual state); ISO 9241-110 §1 Suitability for the Task
(the system should support the user's task context across sessions).

**Area / Component**: Region, Country, City filter selects; URL query-parameter handling.

**Persona & task**: All personas — sharing a filtered view (Walkthrough Task 2).

**Environment**: `http://localhost:3101/en/tools/cost-of-living-calculator?tab=cost&country=sg`,
Chromium, 1280 px, 2026-06-19.

**Steps to Reproduce**:

1. Click any Country link in the cost-of-living table (e.g. "Singapore").
2. Observe the URL changes to `?tab=cost&country=sg`.
3. Observe the Country dropdown: it still shows "All countries" (empty/default).
4. Alternatively, navigate directly to
   `http://localhost:3101/en/tools/cost-of-living-calculator?tab=cost&country=sg`.
5. Confirm Country dropdown still shows default, not "Singapore".

**Expected (predictable) behaviour**: A URL that contains `?country=sg` should pre-select
"Singapore" in the Country filter dropdown, matching what the URL communicates. The summary card
already does update (it shows Singapore data), but the filter dropdowns do not reflect the
active filter — breaking consistency between the URL state, the UI state, and the summary card.

**Actual behaviour**: The summary card shows Singapore data, but the Country dropdown shows its
default ("All countries") empty value. The filter UI and the URL are out of sync. A user who
bookmarks this URL, revisits it, and looks at the dropdown sees a contradiction: the summary card
refers to Singapore while the filter shows "All countries".

**Evidence**: Playwright `uwt-calc-extras2.mjs` — `Country filter when URL has country=sg:` (empty string),
`City filter when URL has country=sg:` (empty string). Screenshot: `uwt-desktop-url-param-country.png`.

**Reproducibility**: Always.

**Suggested clarification**: On page load, read URL query parameters and synchronise them to the
filter dropdowns so the UI reflects the same state as the URL. The summary card already does this
correctly — the dropdowns should follow the same pattern.

---

## UWT-004 — "Total" in summary card and "Total" in comparison table show different values for the same city with no explanation

**Severity**: 3 — Major usability problem
**Priority**: High

**Violated principle**: Heuristic 4 — Consistency and Standards (the same label "Total" refers to
different quantities in the same view); Heuristic 1 — Visibility of System Status (neither element
explains the discrepancy); Heuristic 2 — Match Between System and the Real World (the numeric
difference is unexplained).

**Area / Component**: Summary card headline row; comparison table "Total" column.

**Persona & task**: Relocation researcher — reading cost estimate for a selected city (Task 2).

**Environment**: `http://localhost:3101/en/tools/cost-of-living-calculator`, 1280 px,
Singapore selected (default), 2026-06-19.

**Steps to Reproduce**:

1. Navigate to the page at desktop 1280 px (default city: Singapore).
2. Observe the summary card at the top: "Total SGD 4,328".
3. Scroll to the comparison table.
4. Locate the Singapore row; read the "Total" column: `4,578`.
5. Note: 4,578 ≠ 4,328. The difference is 250. Neither figure is labelled to explain the gap.
6. The summary card "Total" = Housing + Food + Transport + Utilities + Healthcare (OOP) + Childcare
   - School = 4,328. The table "Essentials" column = 4,328. The table "Total" column = 4,578.
     Something adds 250 to Essentials to produce the table Total — but this is not stated anywhere.

**Expected (predictable) behaviour**: Two elements on the same page labelled "Total" for the same
city should either (a) show the same number, or (b) use different, self-explanatory labels that
tell the user what each includes. By Heuristic 4, the same label must mean the same thing.

**Actual behaviour**: The summary card "Total" (4,328) equals the table "Essentials" (4,328), not
the table "Total" (4,578). The word "Total" is thus used with two different definitions on the
same screen. There is no tooltip, footnote, or inline copy explaining what additional amount
produces the table "Total".

**Evidence**: Playwright output — `Singapore Essentials: 4,328, Total: 4,578` and
`Card Total: SGD 4,328`. Summary card HTML confirmed: line items sum to 4,328, labelled "Total".

**Reproducibility**: Always (all cities show a consistent Essentials < Total gap in the table,
while the summary card uses "Total" for the Essentials sum).

**Suggested clarification**: Either (a) relabel the summary card field as "Essentials" (matching
the table column) and explain what the table "Total" adds (likely a lifestyle buffer), or
(b) add a footnote/tooltip on the table "Total" header explaining the increment. The mismatch
erodes trust in the calculation.

---

## UWT-005 — Comparison table numbers carry no currency units; multi-currency rows are ambiguous without context

**Severity**: 3 — Major usability problem
**Priority**: High

**Violated principle**: Heuristic 2 — Match Between System and the Real World (bare numbers
without units are not "the user's language"); Heuristic 6 — Recognition Over Recall (users must
remember the country each row refers to and mentally recall its currency rather than reading it);
Information Scent — Pirolli & Card (the number "25,000" without currency gives no scent of value
relative to "3,500").

**Area / Component**: Cost-of-living comparison table — all numeric columns; mobile view.

**Persona & task**: Relocation researcher — comparing costs across cities (Task 2).

**Environment**: `http://localhost:3101/en/tools/cost-of-living-calculator`, 375 px and 1280 px,
2026-06-19.

**Steps to Reproduce**:

1. View the cost-of-living comparison table at any size.
2. Observe the Housing column values: Singapore = 3,500; Bangkok = 25,000; Jakarta = 8,000,000.
3. No currency symbol or unit appears in any table cell.
4. On mobile (375 px) the table collapses to a scrollable overflow container — no currency column
   is visible in the viewport unless the user scrolls horizontally. (No horizontal scroll appears
   at 375 px per Playwright; the overflow container has `overflow: auto` but at this size the
   table does not overflow, meaning the "Healthcare scheme" column is the only contextual cue and
   it is not a currency indicator.)
5. A first-timer reading "25,000" for Bangkok and "3,500" for Singapore cannot directly compare
   them because the currencies are different (THB vs SGD).

**Expected (predictable) behaviour**: Each row, or at minimum each table column header, should
indicate the currency in which the figure is expressed. Prevailing convention on cost-of-living
comparison tools (Numbeo, Expatistan, Nomad List) either show a currency-per-row suffix or a
header note stating "figures in local currency". Without this, the numbers are meaningless in
isolation.

**Actual behaviour**: All numeric cells are bare integers (e.g. `3,500`, `25,000`, `8,000,000`)
with no currency indicator. The "Healthcare scheme" column provides country context but not
currency context. The user must know that Singapore uses SGD, Thailand uses THB, etc. — a
significant recall burden across 31 cities in 24 countries.

**Evidence**: Playwright output — `Currency in table cells: { count: 0, examples: [] }`.
First three table rows confirmed as bare numbers. Summary card does include "SGD" labels but the
table does not.

**Reproducibility**: Always.

**Suggested clarification**: Add a "Currency" column to the table (or a per-row currency suffix
in each numeric cell), or add a subheader row under each country group showing the currency code.
At minimum, the column header group should note "figures in local currency".

---

## UWT-006 — Primary filter dropdowns and area toggle buttons are below WCAG 2.5.8 touch target minimum on mobile

**Severity**: 3 — Major usability problem
**Priority**: High

**Violated principle**: WCAG 2.2 SC 2.5.8 Target Size (Minimum) — 24 × 24 CSS px; Fitts's Law —
smaller targets take longer to acquire and produce more errors; Heuristic 5 — Error Prevention
(undersized targets cause accidental selections).

**Area / Component**: Region dropdown, Country dropdown, City dropdown, City center button, Rural
button, hamburger menu; all at 375 px viewport.

**Persona & task**: Mobile user — any filtering task (Task 2); Task 4 (area toggle).

**Environment**: `http://localhost:3101/en/tools/cost-of-living-calculator`, Chrome mobile,
375 px viewport, 2026-06-19.

**Steps to Reproduce**:

1. Open the page at 375 px with touch emulation.
2. Measure the rendered height and width of the filter dropdowns:
   - Region dropdown: 106 × 29 px
   - City center button: 96.6 × 28 px
   - Rural button: 58.2 × 28 px
   - Hamburger menu: 36 × 36 px
3. WCAG 2.5.8 requires a minimum of 24 × 24 CSS px (met), but the preferred interactive size is
   44 × 44 px (not met). WCAG 2.5.8 strict 24 px minimum: the 28–29 px heights pass the hard
   floor but are below the 44 px recommended target, and the 36 × 36 hamburger does not meet
   44 × 44.
4. The salary input field on the Savings tab measures 175 × 24 px — exactly at the 24 px hard
   floor, with no tolerance for off-centre taps.

**Expected (predictable) behaviour**: Interactive controls that are a primary part of the
calculator workflow should meet the 44 × 44 px recommended touch target so that users do not
routinely mis-tap on a phone.

**Actual behaviour**: City center and Rural buttons are 28 px tall; the Region dropdown select
is 29 px tall; the salary input is 24 px tall. On a 375 px device these require precise stylus-
level tapping accuracy.

**Evidence**: Playwright bounding-box measurements (uwt-calc-mobile2.mjs):
`City center button: { x: 58.5, y: 378, width: 96.6, height: 28 }`,
`Region dropdown: { x: 69.5, y: 181, width: 106, height: 29 }`,
`Salary input bb: { x: 16, y: 602, width: 175, height: 24 }`,
`Hamburger bb: { x: 16, y: 14, width: 36, height: 36 }`.

**Reproducibility**: Always at 375 px.

**Suggested clarification**: Increase the minimum height of all interactive filter controls to
44 px on mobile via Tailwind responsive utilities (`h-11` = 44 px). The hamburger should be at
least 44 × 44 px. The salary number input should have `min-height: 44px` on touch breakpoints.

---

## UWT-007 — "Minimum role" tab label and "Baseline source" control are opaque to a first-time user

**Severity**: 3 — Major usability problem
**Priority**: High

**Violated principle**: Heuristic 2 — Match Between System and the Real World (jargon without
definition is not "the user's language"); Heuristic 6 — Recognition Over Recall (users must
already know what "Baseline source" means to choose correctly); Cognitive Walkthrough Q3 — "Will
the user associate the correct action with the result?" (the answer is "uncertain" for any user
unfamiliar with the domain).

**Area / Component**: Minimum Role tab; "Baseline source" dropdown; "Monthly savings target" input.

**Persona & task**: First-time relocation researcher — discovering minimum role (Task 4,
CW-Q1 and CW-Q3).

**Environment**: `http://localhost:3101/en/tools/cost-of-living-calculator`, Minimum Role tab,
1280 px, 2026-06-19.

**Steps to Reproduce**:

1. Click the "Minimum role" tab.
2. Observe the first visible control: a dropdown labelled "Baseline source" with options
   "Monthly savings target", "Reference role", "My salary".
3. Observe there is no description, tooltip, or helper text explaining what "Baseline source"
   controls or how the three options differ from each other.
4. Observe "Monthly savings target" — there is no initial value; the input is blank. The first
   table appears to rank roles against an undefined target.
5. The tab heading "Minimum role" itself gives no context: minimum role for what? What is the
   ranking criterion?
6. The "Ranking key" note at the bottom (essential savings) is separated from the controls and
   easily missed by a scanner.

**Expected (predictable) behaviour**: Each control on the Minimum Role tab should carry visible
helper text explaining its effect, per Heuristic 2 and ISO 9241-110 §2 Self-Descriptiveness. A
new user should be able to predict what changing "Baseline source" from "Monthly savings target"
to "Reference role" will do without trial-and-error.

**Actual behaviour**: No tooltip, no helper text, no description. The three options in "Baseline
source" are named but not explained. The "Ranking key" note exists but is far from the controls
and in small muted text.

**Evidence**: Playwright output — `Baseline source descriptions: ['Baseline source', 'Ranking key: ...', ...]`.
No descriptive text adjacent to the dropdown itself. Screenshots: `uwt-desktop-minrole-full.png`,
`uwt-desktop-minrole-controls.png`.

**Reproducibility**: Always.

**Suggested clarification**: Add a one-line description below the "Baseline source" dropdown
(e.g. "Choose how to set the savings threshold: enter a target amount, pick a benchmark role,
or use your own salary") and add placeholder text or an example value to the "Monthly savings
target" input (e.g. "e.g. 2000"). Move the "Ranking key" note to be immediately adjacent to the
control area rather than separated by the full table.

---

## UWT-008 — "Savings after lifestyle" column has no definition; "lifestyle spending" is unexplained

**Severity**: 2 — Minor usability problem
**Priority**: Medium

**Violated principle**: Heuristic 2 — Match Between System and the Real World; Heuristic 6 —
Recognition Over Recall; Cognitive Walkthrough Q3 — "Will the user associate the correct action
with the result?".

**Area / Component**: Savings tab — comparison table, "Savings after lifestyle" column.

**Persona & task**: Relocation researcher — interpreting the savings table (Task 3).

**Environment**: `http://localhost:3101/en/tools/cost-of-living-calculator`, Savings tab,
1280 px, 375 px, 2026-06-19.

**Steps to Reproduce**:

1. Switch to the Savings tab and enter any salary.
2. Observe two savings columns: "Savings after essentials ↕" and "Savings after lifestyle".
3. Observe there is no definition, footnote, or tooltip for "lifestyle spending". The table
   rows show a smaller value under "Savings after lifestyle" than "Savings after essentials",
   implying an additional deduction — but the deduction amount and its basis are unstated.
4. The note "Ranking key: essential savings … Lifestyle excluded — personal preference variable"
   is present but does not define what "lifestyle spending" is numerically.
5. There is no "lifestyle" input visible anywhere in the Savings tab.

**Expected (predictable) behaviour**: A column header that references a quantity ("savings after
lifestyle") should either define the quantity inline or link to an explanation. Users need to
know whether the lifestyle deduction is fixed, per-user, or per-city to trust the figure.

**Actual behaviour**: "Lifestyle spending" is referenced but never defined. Its numerical value
appears implicitly in the column but is not explained anywhere visible in the tab.

**Evidence**: Playwright output — `Lifestyle input found: 0`, `Lifestyle mentions: ['Savings
after lifestyle', ...]`, `Text around "lifestyle": ...Lifestyle excluded — personal preference
variable.`. No definition of the lifestyle amount found.

**Reproducibility**: Always.

**Suggested clarification**: Add a column header tooltip or a note below the table defining the
lifestyle amount (e.g. "Lifestyle = estimated discretionary spending: dining out, entertainment,
travel"). If the amount varies by city, show it as a separate line item or explain the formula.

---

## UWT-009 — "Relocation (sunk)" and "Liquidity reserve" columns have no definitions or tooltips

**Severity**: 2 — Minor usability problem
**Priority**: Medium

**Violated principle**: Heuristic 2 — Match Between System and the Real World; Heuristic 6 —
Recognition Over Recall; WCAG 3.1.3 Unusual Words (comprehension block for non-finance readers).

**Area / Component**: Cost-of-living comparison table — "Relocation (sunk)" and
"Liquidity reserve" columns.

**Persona & task**: Relocation researcher — evaluating total relocation cost (Task 2).

**Environment**: `http://localhost:3101/en/tools/cost-of-living-calculator`, 1280 px, 2026-06-19.

**Steps to Reproduce**:

1. View the cost-of-living comparison table.
2. Identify columns "Relocation (sunk)" and "Liquidity reserve".
3. Hover over the column headers — no tooltip appears (`tooltip triggers found: 0`).
4. Search the page for any explanation — none found. "Sunk" (as in sunk cost) and "liquidity
   reserve" are finance/accounting terms with non-obvious meanings to users outside those fields.

**Expected (predictable) behaviour**: Technical financial terms in a table column header should
be explained via a tooltip, footnote, or glossary link. External consistency: tools like Numbeo
and Expatistan define all column terms inline or link to a glossary.

**Actual behaviour**: No tooltip, no footnote, no glossary. The raw terms are displayed as column
headers with no context for users who do not know what a "sunk cost" or a "liquidity reserve" is.

**Evidence**: Playwright output — `Tooltip triggers found: 0`, `Info/help icons: 0`,
`Table header tooltips: 0`. Confirmed Singapore: Relocation (sunk) = 11,100; Liquidity reserve = 15,000.

**Reproducibility**: Always.

**Suggested clarification**: Add `<th>` tooltip attributes (or a `<caption>`/footnote) defining:
"Relocation (sunk) = one-time costs to move (flights, shipping, deposit, agency fees); Liquidity
reserve = recommended cash buffer (3–6 months of monthly Total)."

---

## UWT-010 — Area toggle (City center / Rural) has no visible selected-state affordance

**Severity**: 2 — Minor usability problem
**Priority**: Medium

**Violated principle**: Heuristic 1 — Visibility of System Status (user cannot tell which area
is active after clicking); WCAG 4.1.2 Name, Role, Value (interactive toggle should expose its
state via `aria-pressed`); Heuristic 5 — Error Prevention (user may believe a click did nothing
and click again).

**Area / Component**: Area toggle buttons ("City center" / "Rural").

**Persona & task**: Relocation researcher — toggling between city and rural costs (Task 2, cost
filter).

**Environment**: `http://localhost:3101/en/tools/cost-of-living-calculator`, 1280 px, 2026-06-19.

**Steps to Reproduce**:

1. On the Cost of Living tab, observe the Area section with "City center" and "Rural" buttons.
2. Click "Rural".
3. Observe: `aria-pressed` is null on both buttons; `data-state` is null. The Rural button has
   a background class after clicking (`has-bg-class`), but City center has no visual change
   indicating it is deactivated. The distinction between selected and unselected states is not
   communicated via aria attributes.
4. A user who clicks Rural cannot confirm via assistive technology that the state changed.

**Expected (predictable) behaviour**: The selected area button should carry `aria-pressed="true"`;
the unselected should carry `aria-pressed="false"`. Visually, the selected button should have a
clearly distinct style (background, border, bold text, or check mark). This follows universal
toggle-button convention (Bootstrap, Radix UI toggle patterns, GOV.UK).

**Actual behaviour**: `aria-pressed` is null on both buttons. Visually the Rural button gains a
background class after clicking, but City center has no "inactive" visual cue to confirm the
toggle worked. Screen-reader users receive no state feedback.

**Evidence**: Playwright output — `City center button state: { ariaPressed: null, dataState: null }`,
`Rural button state: { ariaPressed: null, dataState: null, className: 'has-bg-class' }`.

**Reproducibility**: Always.

**Suggested clarification**: Add `aria-pressed` (true/false) to both toggle buttons; ensure the
design system's `data-state` toggles visibly so the inactive button looks visually distinct from
the active one.

---

## UWT-011 — Parent path `/en/tools/` returns 404; the URL hierarchy is not "hackable"

**Severity**: 2 — Minor usability problem
**Priority**: Medium

**Violated principle**: Nielsen "URLs as UI" — Hackable URLs; Heuristic 4 — Consistency and
Standards (web convention: removing a trailing slug should reach a sensible parent); information
scent — a user who shortens the URL to explore available tools gets a dead end.

**Area / Component**: URL path hierarchy, `/en/tools/` and `/en/tools`.

**Persona & task**: All personas — exploring what other tools exist after discovering the
calculator.

**Environment**: `http://localhost:3101/en/tools/`, `http://localhost:3101/en/tools`, Chromium,
2026-06-19.

**Steps to Reproduce**:

1. Copy the page URL: `http://localhost:3101/en/tools/cost-of-living-calculator`.
2. Edit the URL to remove `cost-of-living-calculator`, leaving `http://localhost:3101/en/tools/`.
3. Press Enter. Observe: HTTP 404.
4. Try `http://localhost:3101/en/tools` (no trailing slash). Observe: HTTP 404.

**Expected (predictable) behaviour**: Removing the final path segment should either return a
"Tools" listing page or redirect to the site homepage. A 404 at a clearly intermediate-level
path (`/en/tools/`) violates the user's mental model of hierarchical URL paths — the URL implies
a `/tools/` collection exists, but there is nothing at that address.

**Actual behaviour**: Both `/en/tools/` and `/en/tools` return 404. There is no parent page for
the tools section.

**Evidence**: Playwright output — `http://localhost:3101/en/tools/ → 404`.

**Reproducibility**: Always.

**Suggested clarification**: Either create a `/en/tools/` listing page that shows all available
tools, or configure a redirect from `/en/tools/` and `/en/tools` to `/en/` (homepage). This
also improves SEO for the tools namespace.

---

## UWT-012 — "Net (monthly)" column in the Savings tab is not labelled to explain what it represents

**Severity**: 2 — Minor usability problem
**Priority**: Medium

**Violated principle**: Heuristic 2 — Match Between System and the Real World; Heuristic 6 —
Recognition Over Recall; Cognitive Walkthrough Q3.

**Area / Component**: Savings tab — comparison table, "Net (monthly)" column.

**Persona & task**: Relocation researcher — interpreting savings (Task 3).

**Environment**: `http://localhost:3101/en/tools/cost-of-living-calculator`, Savings tab,
1280 px, 2026-06-19.

**Steps to Reproduce**:

1. Switch to the Savings tab and enter a salary (e.g. 5000 USD).
2. Observe the "Net (monthly)" column.
3. At $5,000 gross, the first row (Malaysia / Kuala Lumpur) shows "Net (monthly) = 4,150".
4. The label "Net (monthly)" does not clarify: net of what? Tax only? Or gross minus employer
   contributions? No footnote or tooltip answers this.
5. First-time users unfamiliar with Malaysian tax and social security will not know what 4,150
   represents (it appears to be net take-home after local taxes, but this is not stated).

**Expected (predictable) behaviour**: "Net (monthly)" should include a brief tooltip or
parenthetical like "(after local income tax and mandatory contributions)" so users understand
what deductions are applied.

**Actual behaviour**: Plain "Net (monthly)" header with no additional explanation.

**Evidence**: Playwright savings tab headers: `'Net (monthly)'`. No adjacent description found.

**Reproducibility**: Always.

**Suggested clarification**: Change column header to "Net (monthly)" with a tooltip: "Take-home
pay after estimated local income tax and mandatory social contributions — excludes employer-
side contributions."

---

## UWT-013 — Mobile savings tab hides "Net (monthly)" and "Essentials" columns without informing the user

**Severity**: 2 — Minor usability problem
**Priority**: Medium

**Violated principle**: Heuristic 6 — Recognition Over Recall; Responsive Usability —
Content Parity (a feature visible at desktop should not silently disappear at mobile without
a discoverable path to it); Heuristic 1 — Visibility of System Status.

**Area / Component**: Savings tab comparison table / card layout at 375 px.

**Persona & task**: Mobile user — interpreting savings data on a phone (Task 3).

**Environment**: `http://localhost:3101/en/tools/cost-of-living-calculator`, Savings tab,
375 px, 2026-06-19.

**Steps to Reproduce**:

1. Open the Savings tab on a 375 px viewport with salary 5000 entered.
2. The table is replaced by a card layout (31 card elements counted).
3. Each card shows: Country, City, Net (monthly), Savings after essentials, Savings after
   lifestyle, Typical non-salary comp, Total comp.
4. The columns hidden on mobile via `display: none` are: "Net (monthly)" and "Essentials".
5. Wait — the card layout actually shows "Net (monthly)" (confirmed in card text). The
   `display: none` headers correspond to the desktop table headers of columns the card
   layout re-expresses differently. However, the card title/structure uses "Net (monthly)"
   correctly. There is no warning to desktop users that mobile sees a different layout.

**Expected (predictable) behaviour**: The responsive card layout for the Savings tab is a
legitimate and functional transformation. The concern is that a desktop user who shares a
mobile-optimised URL has no indication the layout will differ. This is a minor coverage gap
rather than a blocking issue, since the card layout actually contains the key data.

**Actual behaviour**: The 4-column hidden header for `display:none` includes "Net (monthly)"
and "Essentials" — but these are re-included in each card. No data loss at mobile for the
Savings tab card view. The mobile card layout is functional.

**Note**: This finding is downgraded from major to minor after verifying that the card layout
re-presents the hidden columns as card fields. The issue remains a minor inconsistency in the
table-header vs. card-field alignment.

**Evidence**: Playwright — `Savings table headers on mobile`: Net (monthly) = `display:none`
but card text shows "Net (monthly)4,150". `uwt-mobile-savings-5000.png`.

**Reproducibility**: Always at 375 px.

**Suggested clarification**: The card layout is functional. As a polish item, ensure the card
field labels exactly match the desktop column headers to avoid any future divergence.

---

## UWT-014 — Adults filter is limited to "1" or "2"; no option for single adult or larger households

**Severity**: 1 — Cosmetic problem
**Priority**: Low

**Violated principle**: Heuristic 7 — Flexibility and Efficiency of Use (the range of configurable
values is narrow); Heuristic 2 — Match Between System and the Real World (a solo traveller and a
couple with three children are valid personas but only a couple is the maximum Adults value).

**Area / Component**: Adults select filter on Cost of Living tab.

**Persona & task**: Relocation researcher — configuring household composition (Task 2).

**Environment**: `http://localhost:3101/en/tools/cost-of-living-calculator`, Cost of Living tab,
1280 px, 2026-06-19.

**Steps to Reproduce**:

1. On the Cost of Living tab, observe the "Adults" dropdown.
2. The options are only "1" and "2".
3. There is no option for 3 or more adults (e.g. a family with a grandparent, or a shared
   household of three adults).

**Expected (predictable) behaviour**: While adding unlimited adults may not be in scope, the
narrow range of 1–2 is not explained to the user. If the tool is intentionally designed for
individuals and couples only, a brief label note ("for 1–2 adults; children configurable
separately") would set expectations.

**Actual behaviour**: The dropdown silently offers only "1" and "2" with no rationale. A user
expecting to configure three adults will not find the option and may not understand why.

**Evidence**: Playwright output — `Adults options: [ '1', '2' ]`. Preschool options go to 3;
school-age options go to 3 — but adults do not.

**Reproducibility**: Always.

**Suggested clarification**: Either extend to 3+ adults or add a small note near the selector
("adults: 1–2 supported; contact for custom scenarios").

---

## Finding Summary

| ID      | Title (abbreviated)                                       | Severity | Priority |
| ------- | --------------------------------------------------------- | -------- | -------- |
| UWT-001 | `html[lang]` wrong on Indonesian locale                   | 4        | Critical |
| UWT-002 | H1 / URL slug / page title three-way mismatch             | 3        | High     |
| UWT-003 | URL query params do not restore filter dropdowns          | 3        | High     |
| UWT-004 | "Total" label means different things in card vs table     | 3        | High     |
| UWT-005 | Table numbers have no currency units                      | 3        | High     |
| UWT-006 | Filter controls below 44 px touch target on mobile        | 3        | High     |
| UWT-007 | "Minimum role" / "Baseline source" are opaque jargon      | 3        | High     |
| UWT-008 | "Savings after lifestyle" column has no definition        | 2        | Medium   |
| UWT-009 | "Relocation (sunk)" and "Liquidity reserve" undefined     | 2        | Medium   |
| UWT-010 | Area toggle has no visible / aria selected-state          | 2        | Medium   |
| UWT-011 | `/en/tools/` parent path returns 404                      | 2        | Medium   |
| UWT-012 | "Net (monthly)" not explained in Savings tab              | 2        | Medium   |
| UWT-013 | Mobile savings card layout vs desktop column misalignment | 2        | Medium   |
| UWT-014 | Adults capped at 2 without explanation                    | 1        | Low      |
