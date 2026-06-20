# Spec Suggestions — AyoKoding Cost-of-Living Calculator

> **Spec-blind caveat (applies to every entry)**: This agent did not read `specs/**`. Each
> suggestion below proposes a _desired_ behaviour identified from first-principles usability
> evaluation. A spec-aware reviewer MUST confirm whether each behaviour is already covered in the
> existing `specs/**` before adding it. If it is already covered, the entry is redundant and should
> be discarded. If it is not covered, it is a candidate for a new Gherkin scenario in `specs/**`.
>
> These are **not** spec gaps (which require reading the existing specs to identify missing
> coverage). They are **usability-grounded behaviour proposals**, flagged for reconciliation.

---

## USS-001 — Savings tab must show guidance when salary input is empty

**Paired finding**: UWT-001 (Severity 3)
**Violated principle**: Heuristic 1 (Visibility of system status); Heuristic 6 (Recognition over
recall)

### Desired behaviour

When a user opens the Savings tab and has not yet entered a salary, the interface shows a clear
prompt (placeholder text on the input and/or an explanatory note) so that the user understands
what input is needed to produce meaningful savings figures. The table should either be hidden or
accompanied by a message when the salary field is empty.

### Proposed Gherkin scenario

```gherkin
Scenario: Savings tab displays guidance when no salary has been entered
  Given a user navigates to the Savings tab on the cost-of-living calculator
  When the gross-salary input is empty
  Then the gross-salary input displays placeholder text indicating an example value
  And a visible guidance message is shown explaining that salary entry is required
  And the message text is displayed in the active locale language
```

---

## USS-002 — Minimum Role mode selector must have a visible group label

**Paired finding**: UWT-002 (Severity 3)
**Violated principle**: Heuristic 6 (Recognition over recall); ISO 9241-110 §2
(Self-descriptiveness)

### Desired behaviour

When a user opens the Minimum Role tab, a visible label above the three mode radio buttons
explains what the group controls (e.g. "Ranking mode:" or "Find minimum role by:"), so that users
can predict the effect of their selection without experimenting.

### Proposed Gherkin scenario

```gherkin
Scenario: Minimum Role tab shows a visible label for the mode radio group
  Given a user opens the Minimum Role tab
  When the mode selector radio buttons are rendered
  Then a visible text label identifying the radio group's purpose is displayed above the buttons
  And the label text is readable without assistive technology
  And the label text is translated into the active locale language
```

---

## USS-003 — All three tabs must show visible sub-descriptions to sighted users

**Paired finding**: UWT-003 (Severity 3)
**Violated principle**: Heuristic 6 (Recognition over recall); Information scent (Pirolli & Card)

### Desired behaviour

All three calculator tabs display a brief visible sub-description (not screen-reader-only) beneath
or alongside the tab label, so that first-time sighted users can self-route to the correct tab
without having to click each one to discover its purpose.

### Proposed Gherkin scenario

```gherkin
Scenario: All three tab labels include visible sub-descriptions
  Given a user views the cost-of-living calculator page
  When the tab bar is rendered
  Then the Cost of living tab displays a visible sub-description explaining it compares expenses
  And the Savings tab displays a visible sub-description explaining it shows salary savings
  And the Minimum role tab displays a visible sub-description explaining it finds required seniority
  And all sub-descriptions are rendered in the active locale language
  And all sub-descriptions are visible to sighted users without assistive technology
```

---

## USS-004 — Savings tab must provide a salary currency selector

**Paired finding**: UWT-004 (Severity 2)
**Violated principle**: Heuristic 4 (Consistency and standards); Jakob's Law

### Desired behaviour

The Savings tab provides a currency selector adjacent to the salary input, consistent with the
Minimum Role tab's "Target currency" selector, so that users with non-USD salaries can enter their
salary in their own currency.

### Proposed Gherkin scenario

```gherkin
Scenario: Savings tab provides a salary currency selector
  Given a user opens the Savings tab
  When the salary input is visible
  Then a currency selector is displayed adjacent to or within the salary input field
  And the currency options include at minimum USD, EUR, SGD, IDR, GBP, JPY, CAD, AED
  And selecting a currency updates the salary input label or placeholder to reflect the selection
```

---

## USS-005 — Active tab must be reflected in the URL

**Paired finding**: UWT-005 (Severity 2)
**Violated principle**: Nielsen "URLs as UI"; Heuristic 3 (User control and freedom)

### Desired behaviour

When a user navigates to a non-default tab (Savings or Minimum role), the URL updates to include
a tab query parameter, so that the tab state is bookmarkable and shareable. When a URL with a
tab parameter is opened, the page loads with the corresponding tab active.

### Proposed Gherkin scenario

```gherkin
Scenario: Switching to the Savings tab updates the URL
  Given a user is on the Cost of living tab
  When the user clicks the Savings tab
  Then the page URL contains a tab query parameter indicating the Savings tab is active
```

```gherkin
Scenario: Loading a URL with the Savings tab parameter activates the Savings tab
  Given a URL is constructed with the Savings tab query parameter
  When a user navigates to that URL
  Then the Savings tab is active on page load
  And the Cost of living tab is not active
```

---

## USS-006 — Minimum Role tab must show empty-state guidance when no target is set

**Paired finding**: UWT-006 (Severity 2)
**Violated principle**: Heuristic 1 (Visibility of system status); Heuristic 6 (Recognition over
recall)

### Desired behaviour

When the Minimum Role tab is in "Monthly savings target" mode and no target has been entered, a
visible note or empty-state message explains that a target is required to filter the ranking to
roles that meet the user's goal.

### Proposed Gherkin scenario

```gherkin
Scenario: Minimum Role tab shows guidance when savings target is empty
  Given a user opens the Minimum Role tab in Monthly savings target mode
  When no savings target has been entered in the target input
  Then a visible guidance message is displayed
  And the message explains that entering a target will filter roles that meet that savings goal
  And the message is displayed in the active locale language
```

---

## USS-007 — Back-to-all-cities link must restore the previous filter state

**Paired finding**: UWT-010 (Severity 1)
**Violated principle**: Heuristic 3 (User control and freedom); Heuristic 5 (Error prevention)

### Desired behaviour

When a user has active geo filters (region, country) and drills into a city detail, the
"Back to all cities" link restores those filters rather than resetting them to "All regions / All
countries".

### Proposed Gherkin scenario

```gherkin
Scenario: Back to all cities restores prior filter state
  Given a user has set the Region filter to "ASEAN" and Country filter to "Singapore"
  And the user has selected City "Singapore" entering the city detail view
  When the user clicks the "Back to all cities" link
  Then the Region filter is restored to "ASEAN"
  And the Country filter is restored to "Singapore"
  And the City filter is reset to "All cities"
```
