# Cognitive Walkthrough Transcript — AyoKoding Cost-of-Living Calculator

This file is the method-transparency artifact for the heuristic usability evaluation dated
2026-06-21. For each task, every step is walked with the four cognitive-walkthrough questions.

**The four questions (Spencer 2000 / NN/g)**

1. Will the user **try to achieve the right effect**? (Do they understand what to do here?)
2. Will the user **notice the correct action is available**? (Is it visible and findable?)
3. Will the user **associate the correct action with the result** they want? (Do labels read right?)
4. After acting, will the user **see that progress was made**? (Does the system confirm?)

**Verdict codes**: Pass / Friction (→ UWT-###) / Fail

---

## Task 1 — Land and orient: understand what the calculator does

**Persona**: First-time visitor, desktop 1280 px, English

### Step 1.1 — Arrive at the page

- Q1: Does the user know what to do? **Pass** — "Cost of Living Calculator" as H1 with subtitle
  "Compare cost of living and salary savings across cities" is immediately comprehensible.
- Q2: Is the correct action visible? **Pass** — three tabs are visually prominent.
- Q3: Do labels associate with the goal? **Friction → UWT-003** — "Cost of living", "Savings",
  "Minimum role" are informative nouns but do not tell the user what _input_ each tab requires or
  what output it produces. Sub-descriptions exist in the DOM (sr-only) but are invisible to sighted
  users.
- Q4: Does the system confirm progress? **Pass** — the Cost of living tab is pre-selected and
  immediately shows data (no blank state).

### Step 1.2 — Scan the default tab content

- Q1: Does the user know what the table is? **Pass** — column headings (Country, City, Total
  Essentials, Housing, Food, …) are self-explanatory.
- Q2: Are filters visible? **Pass** — Region, Country, City dropdowns are labelled and visible.
- Q3: Do labels read correctly? **Friction → UWT-007** — "ASEAN", "MENA", "Nordics" in the Region
  dropdown are not expanded; a non-specialist user in "Americas" or "Africa" may be uncertain which
  group their destination belongs to.
- Q4: Does the system confirm? **Pass** — table updates immediately when filters are changed.

---

## Task 2 — Find the cheapest city for a family of 2 adults, 1 school-age child

**Persona**: First-time visitor, desktop 1280 px, English

### Step 2.1 — Set household size

- Q1: Does the user know to change Adults / children selects? **Pass** — "Adults", "Preschool
  children", "School-age children" labels are clear; default "Adults: 1" is visible.
- Q2: Are the selects visible? **Pass** — all labelled with explicit `<label for="">` associations
  and min-height 44 px (within Fitts's Law target size).
- Q3: Do labels match expectation? **Pass** — the terms "Preschool children" and "School-age
  children" are sufficiently self-evident for a lay user.
- Q4: Does the system confirm? **Pass** — the table updates immediately; the city-level summary
  box shows "School SGD 0 / Childcare SGD 0" and then updates when children are added (the Childcare
  and School rows respond to the selector).

### Step 2.2 — Filter by region or country

- Q1: Does the user know to use Region / Country / City selects? **Pass** — the labels are clear.
- Q2: Are they visible? **Pass**.
- Q3: Do labels map to goal? **Friction → UWT-007** — "ASEAN" and "MENA" may not be self-evident
  to all users.
- Q4: After selecting Singapore country, does the view update? **Pass** — city dropdown cascades
  automatically to show Singapore cities.

### Step 2.3 — Read city costs

- Q1: Does the user understand the table's "Total" vs "Essentials" distinction? **Friction** — the
  column "Total Essentials" vs "Total" are both visible but not explained inline. (Severity 1;
  not filed as a separate UWT because the column heading "Essentials" is reasonably self-evident.)
- Q2: Is "OOP" in the Healthcare column findable/understandable? **Friction → UWT-012** — the
  footnote is spatially separated.
- Q3: Does "City center" / "Rural" toggle make sense? **Pass** — "City center" and "Rural" as
  radio labels are standard web vocabulary.
- Q4: Does switching City center → Rural update data? **Pass** — values update immediately.

### Step 2.4 — Drill into a single city

- Q1: Does the user know they can click a city row? **Friction** — no "click to expand" affordance
  is visible; city names are styled as links (`<a>`) in some contexts but there is no visual cue in
  the main table row that the city name is clickable. (Minor; omitted from findings due to
  reasonable link styling convention.)
- Q4: Does the back link work? **Friction → UWT-010** — "← Back to all cities" resets filters
  instead of restoring prior filter state.

**Task 2 verdict**: Completable with friction at UWT-007, UWT-010, UWT-012.

---

## Task 3 — Estimate monthly savings with a $5,000 salary

**Persona**: First-time visitor, desktop 1280 px, English

### Step 3.1 — Navigate to Savings tab

- Q1: Does the user know to click "Savings"? **Pass** — three tabs are visible and labelled.
- Q2: Is "Savings" visible? **Pass**.
- Q3: Does "Savings" label correctly predict a savings calculator? **Friction → UWT-003** — the
  tab label alone ("Savings") does not tell the user they need to enter a salary. No visible
  sub-description.
- Q4: Does the tab switch confirm? **Pass** — the tab activates, content changes.

### Step 3.2 — Observe the initial state of the Savings tab

- Q1: Does the user understand why all values are negative? **Fail → UWT-001** — no salary is
  entered, the table shows all-negative savings, and there is no explanatory note. The user's
  mental model is broken: "Is the calculator broken? Do I lose money everywhere?"
- Q2: Is the salary input visible? **Pass** — the label "Gross monthly salary (before tax) USD"
  is above the empty box.
- Q3: Does the label associate correctly? **Friction → UWT-004** — the "USD" embedded in the label
  implies the salary _must_ be in USD. A user with a salary in EUR or SGD has no path to enter it
  in their own currency. No currency selector exists on this tab.
- Q4: No salary entered = no positive feedback possible.

### Step 3.3 — Enter salary (5000)

- Q1: Does the user know to type in the input? **Pass**.
- Q2: Is the input clearly an editable field? **Pass** — standard number input styling.
- Q3: Does entering 5000 feel right? **Pass** — type: number, min: 0, no step constraint.
- Q4: Does the table update after entry? **Pass** — rows reorder, values become positive/negative
  based on the city. Annual gross updates to "0 USD" → "Annual gross: 60,000 USD" is shown.

### Step 3.4 — Interpret sorted results

- Q1: Does the user understand "Savings after essentials" vs "Savings after lifestyle"? **Friction**
  — both column names are present but the distinction between "essentials" and "lifestyle" is not
  explained inline.
- Q2: Is the sort control visible? **Friction** — only "Savings after essentials ↕ ↓" has a
  sort button; the other columns do not. The ↕ icon is a common sort indicator but is embedded in
  the column heading text with no visual distinction.
- Q4: After sorting, does the order change visibly? **Pass**.

**Task 3 verdict**: Step 3.2 is a major failure → UWT-001. Steps 3.3 and 3.4 pass with friction.

---

## Task 4 — Find the minimum seniority level to save $3,000/month in Austin

**Persona**: First-time visitor, desktop 1280 px, English

### Step 4.1 — Navigate to Minimum Role tab

- Q1: Does the user know to click "Minimum role"? **Pass** — tab is visible and labelled.
- Q3: Does "Minimum role" label predict this tab's purpose? **Friction → UWT-003** — "Minimum
  role" as a noun is ambiguous: is it the minimum job level, or the minimum role in a company? No
  visible sub-description to disambiguate.
- Q4: Tab switch confirms. **Pass**.

### Step 4.2 — Understand the three mode radio buttons

- Q1: Does the user know what "Monthly savings target", "Reference role", "My salary" choose?
  **Fail → UWT-002** — no group label explains what the radio group controls. A first-timer reads
  three labels and cannot determine whether they are changing a display format, a filter criterion,
  or something else.
- Q2: Are the radio buttons visible? **Pass** — styled as pill buttons.
- Q3: Do the labels associate with their effect? **Friction → UWT-002** — the labels describe the
  inputs for each mode, not the mode's _purpose_.
- Q4: Clicking a mode changes the visible inputs. **Pass** — "Monthly savings target" shows a
  target input; "Reference role" shows City and Role selectors; "My salary" shows a salary input.
  But this is discoverable only by clicking.

### Step 4.3 — Set savings target to $3,000

- Q1: Does the user know to type $3,000 in the "Monthly savings target" input? **Pass** after
  discovering the mode (see Step 4.2 friction).
- Q2: Is the target input visible? **Pass** — labelled "Monthly savings target".
- Q3: Does the label match the goal? **Pass** — "Monthly savings target" correctly implies
  "enter the amount I want to save each month".
- Q4: Does the table update? **Pass** — table rerenders. However, with no target entered, the
  ranked list still shows (UWT-006 friction — the pre-filled list does not communicate that a
  target is needed to filter to a goal).

### Step 4.4 — Find Austin in the results

- Q1: Does the user know how to filter to Austin? **Friction** — the main geo filters (Region /
  Country / City) are shared across tabs and are in the filter bar above the tab content, not inside
  the Minimum Role tab. A user may not realise the same filters apply.
- Q2: Is there a per-tab city filter? **Pass** — there is a "Reference city" selector within the
  Minimum Role tabpanel when "Reference role" mode is selected.
- Q3: Does "Reference city" label mean "filter to this city"? **Friction** — "Reference city"
  implies "city to compare against" not "city I am targeting". This is a labelling precision issue.
- Q4: After selecting Austin as reference city in "Reference role" mode, the role cards update.
  **Pass**.

**Task 4 verdict**: Steps 4.2 (UWT-002) and 4.3/4.4 carry friction; Step 4.2 is a major failure
for first-time task success.

---

## Task 5 — Complete Task 2 on mobile (375 px)

**Persona**: First-time visitor, mobile 375 px, English

### Step 5.1 — Land on calculator

- Q1: Does the user know what the tool does? **Pass** — H1 and subtitle are visible on mobile.
- Q2: Are tabs visible without scrolling? **Pass** — tab list is visible; all three tabs fit
  within 375 px (tab widths: Cost of living 104 px, Savings 70 px, Minimum role 107 px; total
  281 px, within 375 px).
- Q4: Does the initial state communicate a usable starting point? **Pass** — Cost of living table
  data is visible below fold.

### Step 5.2 — Use Region / Country / City dropdowns on mobile

- Q1: Does the user know to use the dropdowns? **Pass**.
- Q2: Are dropdowns visible and tappable? **Pass** — select elements are visible and not clipped
  at 375 px; min-height is 44 px for Adults/children selects.
- Q3: Do labels read correctly? **Pass** — same as desktop.
- Q4: Table updates after selection? **Pass** — same as desktop.

### Step 5.3 — Read the data table on mobile

- Q1: Does the user understand the table exists? **Pass** — table is visible below the filters.
- Q2: Is the table readable without horizontal scroll at 375 px? **Friction** — the full column
  table is wide; the container has a scrollable overflow. The data is accessible but the scroll
  affordance (shadow/arrow) is not visible. At 320 px the page body itself overflows (UWT-008).
- Q4: After scrolling right, does the user see more columns? **Pass** — the table scrolls; data
  is present but the affordance gap makes discovery non-obvious.

**Task 5 verdict**: Completable with mild friction at 375 px; UWT-008 applies at 320 px.

---

## First-Click Analysis

**Key question per task**: What is the correct first click, and does the page's visual hierarchy
make it the most compelling target?

| Task                      | Correct first click                    | Is it the most compelling target?                                                |
| ------------------------- | -------------------------------------- | -------------------------------------------------------------------------------- |
| 1 (orient)                | Already on the page; scan tab bar      | Yes — tabs are prominent                                                         |
| 2 (cheapest city, family) | "Adults" dropdown to change from 1 → 2 | Friction — "Adults" is below the tab bar; users may click region filter first    |
| 3 (savings with salary)   | "Savings" tab                          | Pass — tab is visible; but sub-description absent (UWT-003)                      |
| 4 (min role for $3k)      | "Minimum role" tab                     | Pass — tab is visible; but meaning of mode radios requires exploration (UWT-002) |

Overall: the tab bar is a strong first-click target for tasks 3 and 4. Household-filter tasks
have weaker information scent because "Adults" is visually subordinate to the geo filters.
