# Product Requirements Document

## Plan

Usability Evaluation: ayokoding-www Cost-of-Living / Salary Calculator
`plans/in-progress/ayokoding-www-calc-usability-findings/`

## Personas

### Primary: First-Time Relocation Researcher (English)

A software professional in Asia or Europe who is considering a move to a new country. They have
no previous AyoKoding account. They land on the calculator via a search result or a shared link.
They want to quickly understand what cities they can afford on their current salary and which
cities offer the best savings rate. They may be using a laptop (1280 px) or a phone (375 px).
They are not familiar with terms like "liquidity reserve" or "RSU/equity".

### Secondary: Returning Bahasa Indonesia Reader

A developer in Indonesia who prefers reading in Bahasa Indonesia. They switch the site language
to "Bahasa Indonesia" and expect the page to be fully in Indonesian, including for assistive
technology (screen reader announces page as Indonesian).

### Secondary: Mobile Power User

A user who primarily accesses AyoKoding on a 375 px phone. They use the filter controls to
explore multiple cities and switch between the Savings and Minimum Role tabs.

## User Stories

### Story 1: Page Orientation

As a first-time visitor, when I land on the calculator page, I can understand within seconds what
this tool does and what each of its three sections (tabs) offers, without reading any help text.

### Story 2: Cost-of-Living Comparison

As a relocation researcher, when I select a region, country, and city in the filters, the
comparison table and summary card immediately show cost-of-living figures I can interpret — with
currency units, understandable column labels, and consistent values between the summary card and
the table.

### Story 3: Salary Savings Analysis

As a professional exploring affordability, when I enter my gross monthly salary in the Savings
tab, I can see which cities let me save the most, with clear labels explaining what "Net
(monthly)", "Savings after essentials", and "Savings after lifestyle" each mean.

### Story 4: Minimum Role Discovery

As a job-seeker, when I open the Minimum Role tab and choose a baseline (savings target, reference
role, or my own salary), I can see the minimum seniority level I need in each city to meet my
goal — with the control labels explained so I do not need to guess what "Baseline source" means.

### Story 5: Indonesian Locale Equivalence

As a Bahasa Indonesia reader, when I switch the page language to "Bahasa Indonesia", the page is
fully in Indonesian — including the `html[lang]` attribute that tells my screen reader and browser
which language it is reading.

### Story 6: Shareable URL State Restoration

As a user who wants to share a specific city view, when I copy the URL that contains filter
parameters (e.g. `?tab=cost&country=sg`) and send it to a colleague, my colleague's browser opens
the page with the same filters pre-selected and visible in the dropdown controls.

### Story 7: Mobile Filter Usability

As a mobile user on a 375 px phone, when I use the filter controls (Region, Country, City, area
toggle), I can tap them precisely without repeated mis-taps, because every interactive target
meets the minimum touch size.

## Acceptance Criteria (Gherkin)

### AC-01: Page Title Scent — H1 matches URL slug theme

```gherkin
Scenario: Page H1 matches the URL slug concept
  Given a user navigates to /en/tools/cost-of-living-calculator
  When the page loads
  Then the H1 heading contains language that reflects both "cost of living" and "salary/savings"
  And the page title (browser tab) contains a recognisable name for the tool
```

### AC-02: Indonesian locale `html[lang]` attribute

```gherkin
Scenario: Indonesian locale has correct language attribute
  Given a user navigates to /id/tools/cost-of-living-calculator
  When the browser renders the page
  Then the html element has lang="id"
```

### AC-03: URL state parameters restore filter dropdowns

```gherkin
Scenario: Country URL parameter pre-selects the Country dropdown
  Given a URL /en/tools/cost-of-living-calculator?tab=cost&country=sg
  When a user navigates to that URL
  Then the Country dropdown displays "Singapore"
  And the City dropdown displays the first city for Singapore
  And the summary card reflects Singapore cost data
```

### AC-04: Cost-of-living table numbers include currency context

```gherkin
Scenario: Table rows provide currency context without horizontal scroll
  Given a user views the Cost of Living tab on a 375-px viewport
  When the cost-of-living table is visible
  Then each row's numeric figures are unambiguously associated with a currency unit
  And the currency context is visible without horizontal scrolling
```

### AC-05: "Total" in summary card and "Total" in table are reconciled

```gherkin
Scenario: Summary card Total and table Total are consistent or labelled distinctly
  Given a user views the Cost of Living tab with Singapore selected
  When they compare the "Total" figure in the summary card with the "Total" column in the table
  Then both figures either match or carry distinct labels that explain the difference
  And the difference is explained by visible copy near the relevant element
```

### AC-06: Touch targets meet WCAG 2.5.8 minimum on mobile

```gherkin
Scenario: Filter controls meet minimum touch target size on 375-px viewport
  Given a user views the page on a 375-px viewport
  When measuring the height and width of Region, Country, City dropdowns and area toggle buttons
  Then each interactive element is at least 24 CSS px in both height and width
  And the preferred size of 44 CSS px in both dimensions is met for primary controls
```

### AC-07: "Savings after lifestyle" is defined inline

```gherkin
Scenario: Lifestyle savings column has an accessible explanation
  Given a user views the Savings tab
  When the table header "Savings after lifestyle" is visible
  Then an inline definition or tooltip explains what "lifestyle spending" represents
  And the explanation is accessible without clicking away from the table
```

### AC-08: "Baseline source" control is labelled descriptively

```gherkin
Scenario: Minimum role Baseline source label includes a description
  Given a user opens the Minimum Role tab
  When they see the "Baseline source" dropdown
  Then helper text or a visible description explains what each option (Monthly savings target / Reference role / My salary) does
  And the description is visible without user interaction
```

### AC-09: "Relocation (sunk)" and "Liquidity reserve" have accessible definitions

```gherkin
Scenario: Jargon table columns carry accessible definitions
  Given a user views the cost-of-living comparison table
  When they encounter the column headers "Relocation (sunk)" and "Liquidity reserve"
  Then a tooltip, caption, or footnote defines each term in plain language
  And the definition is reachable by keyboard and visible on hover or focus
```

### AC-10: Parent URL (/en/tools/) returns a usable page

```gherkin
Scenario: Removing the tool slug from the path lands on a usable parent
  Given a user edits the URL from /en/tools/cost-of-living-calculator to /en/tools/
  When the browser navigates to that URL
  Then the server returns a 200 response with a page listing available tools
  Or redirects to a sensible parent page rather than returning a 404
```

### AC-11: Area toggle (City center / Rural) conveys selected state visually

```gherkin
Scenario: Area toggle selected state is visually distinct
  Given a user views the Cost of Living tab
  When they click the "Rural" toggle button
  Then the "Rural" button has a visually distinct active state (e.g. filled background, border, or aria-pressed="true")
  And the "City center" button appears visually inactive
  And the change is also announced via aria-pressed or equivalent to assistive technology
```

### AC-12: Mobile savings table card layout preserves key data

```gherkin
Scenario: Savings tab on 375-px shows all essential columns in card layout
  Given a user views the Savings tab on a 375-px viewport with a salary entered
  When the card layout replaces the table
  Then each card shows Country, City, Net (monthly), Savings after essentials, and Savings after lifestyle
  And currency context is visible on each card
```

## In-Scope / Out-of-Scope

### In Scope

- Usability friction for first-time users: comprehension, labelling, information scent
- WCAG Understandable overlap: `html[lang]`, opaque labels, unclear controls
- URL naturalness and state restoration
- Touch target sizing (Fitts's Law / WCAG 2.5.8)
- Responsive usability at 320, 375, 768, 1280, 1440 px
- English and Indonesian locales
- All three calculator tabs: Cost of Living, Savings, Minimum Role

### Out of Scope

- Correctness of the underlying cost data (data accuracy is `exploratory-web-tester` territory)
- Full POUR accessibility audit (contrast ratios, keyboard traps, ARIA wiring completeness)
- Performance optimisation (load time, bundle size)
- Backend API testing
- Authentication or account flows
