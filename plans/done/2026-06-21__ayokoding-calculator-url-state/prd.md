# Product Requirements — Calculator URL State Reflection

## Product Overview

The cost-of-living calculator gains **full URL state reflection**: all nine controls serialize to
the query string, the URL is the single source of truth, and selecting filters cascades (backfill
narrower→broader, clear broader→narrower) with sanitize + canonicalize on load. The locale stays a
path segment (`/en/...`, `/id/...`); only calculator state lives in the query string.

### The nine controls and their param keys

| Control        | Param key    | Valid values                                                                                                        | Default (omitted from URL) |
| -------------- | ------------ | ------------------------------------------------------------------------------------------------------------------- | -------------------------- |
| Tab            | `tab`        | `cost` \| `savings` \| `min-role`                                                                                   | `cost`                     |
| Region         | `region`     | one of the dataset regions (`asean`, `japan`, `europe`, `nordics`, `americas`, `mena`, `asia`, `oceania`, `africa`) | none (all regions)         |
| Country        | `country`    | a valid country `id` (e.g. `sg`, `id`, `jp`)                                                                        | none (all countries)       |
| City           | `city`       | a valid city `id` (e.g. `singapore`, `jakarta`)                                                                     | none (all cities)          |
| Adults         | `adults`     | `1` \| `2`                                                                                                          | `1`                        |
| Preschool kids | `preschool`  | `0` \| `1` \| `2` \| `3`                                                                                            | `0`                        |
| School kids    | `schoolkids` | `0` \| `1` \| `2` \| `3`                                                                                            | `0`                        |
| School type    | `schooltype` | `public` \| `private`                                                                                               | `public`                   |
| Area           | `area`       | `center` \| `rural`                                                                                                 | `center`                   |

`[Repo-grounded]`: region values from `core/data/cities.ts` `City.region` union; `adults`/kids
ranges from `controls.tsx` `adultOptions`/`kidOptions`; `schooltype`/`area` from `core/calc.ts`
`SchoolType`/`Area`. Existing keys `tab`/`country`/`city` are retained for deep-link back-compat.

## Personas

Solo-maintainer repo — personas are the hats worn and the agents consuming the artifacts.

- **Comparison shopper (end user)** — wants to configure the calculator for their household and
  share/bookmark the result.
- **Frontend engineer (maintainer hat)** — needs the state model to be a single, testable source of
  truth rather than three drifting copies.
- **Tester (maintainer hat + advocate agents)** — needs deep links and back-navigation to behave
  predictably and the absorbed findings to be closed.

## User Stories

- As a **comparison shopper**, I want every filter I change to appear in the URL, so that I can copy
  the link and have a friend see the exact same view.
- As a **comparison shopper**, I want the browser Back button to step through my filter changes, so
  that I can undo a selection without re-entering everything.
- As a **comparison shopper**, I want an obvious way back to the Tools index and Home, so that I am
  never trapped in a deep history stack.
- As a **comparison shopper**, I want picking a city to also show its country and region, so that
  the dropdowns and URL show a consistent full path.
- As a **comparison shopper**, I want picking a broad region to clear an incompatible city, so that
  I never see a contradictory filter (e.g. Singapore selected under Europe).
- As a **comparison shopper**, I want an old or mistyped link to still land me on a sensible page,
  so that a broken param does not break the tool.
- As a **frontend engineer**, I want all calculator state derived from the URL, so that there is one
  source of truth and no drift between component state, page state, and the URL.

## Acceptance Criteria (Gherkin)

> All scenarios obey the step-keyword cardinality rule (one primary `Given`/`When`/`Then`; extras
> chained with `And`/`But`). These scenarios are the source of the first failing tests in
> `delivery.md`. Param defaults are omitted from the URL per the clean-URL rule.

### Round-trip: write on change

```gherkin
Scenario: Changing the tab writes the tab to the URL
  Given I am on the calculator with no query string
  When I switch to the "Savings" tab
  Then the URL query string includes "tab=savings"
  And reloading the page keeps the "Savings" tab active

Scenario: Selecting a region writes the region to the URL
  Given I am on the calculator with no query string
  When I select the region "Europe"
  Then the URL query string includes "region=europe"
  And the URL query string does not include "country" or "city"

Scenario: Selecting a city writes only the city to the URL and backfills the dropdowns
  Given I am on the calculator with no query string
  When I select the city "Singapore"
  Then the URL query string includes "city=singapore"
  And the Country filter shows "Singapore" and the Region filter shows "ASEAN"

Scenario: Changing a cost-basis control writes it to the URL
  Given I am on the calculator with no query string
  When I change the Adults control to "2"
  Then the URL query string includes "adults=2"
  And the household preview updates without a page reload

Scenario: Setting school-age children and a private school type writes both params
  Given I am on the calculator with no query string
  When I set school-age children to "1" and the school type to "Private"
  Then the URL query string includes "schoolkids=1" and "schooltype=private"
  And the school-type toggle shows "Private" selected
```

### Round-trip: restore on load (deep link)

```gherkin
Scenario: A deep link restores the tab on load
  Given a deep link with query string "tab=min-role"
  When I open that link in a fresh tab
  Then the "Minimum role" tab is active
  And no other filter is applied

Scenario: A city deep link restores the city and backfills country and region
  Given a deep link with query string "city=singapore"
  When I open that link in a fresh tab
  Then the single-city Cost-of-living detail for Singapore is shown
  And the Country filter shows "Singapore" and the Region filter shows "ASEAN"

Scenario: A cost-basis deep link restores every control
  Given a deep link with query string "adults=2&preschool=1&schoolkids=2&schooltype=private&area=rural"
  When I open that link in a fresh tab
  Then the household controls show 2 adults, 1 preschool child, and 2 school-age children
  And the school type shows "Private" and the area shows "Rural"
```

### Cascade-clear (broader clears narrower)

```gherkin
Scenario: Selecting a broader region clears an incompatible country and city
  Given I am on the calculator with query string "city=singapore"
  When I select the region "Europe"
  Then the URL query string includes "region=europe"
  But the URL query string does not include "country" or "city"

Scenario: Selecting a different country clears an incompatible city
  Given I am on the calculator with query string "city=singapore"
  When I select the country "Japan"
  Then the URL query string includes "country=jp"
  But the URL query string does not include "city"

Scenario: Clearing the region clears the country and city
  Given I am on the calculator with query string "city=singapore"
  When I clear the Region filter
  Then the URL query string is empty
  And all geo dropdowns return to their "All" state
```

### Backfill (narrower fills broader)

```gherkin
Scenario: Selecting a country backfills its region
  Given I am on the calculator with no query string
  When I select the country "Indonesia"
  Then the URL query string includes "country=id"
  And the Region filter shows "ASEAN"

Scenario: Selecting a city under no prior filter backfills country and region
  Given I am on the calculator with no query string
  When I select the city "Jakarta"
  Then the URL query string includes "city=jakarta"
  And the Country filter shows "Indonesia" and the Region filter shows "ASEAN"
```

### Sanitize + canonicalize on load

```gherkin
Scenario: An unknown city param is dropped on load
  Given a deep link with query string "city=atlantis"
  When the page resolves the deep link
  Then the City filter returns to "All cities"
  And the URL is rewritten to have no "city" param

Scenario: A full-country-name param is dropped on load
  Given a deep link with query string "country=Indonesia"
  When the page resolves the deep link
  Then the Country filter returns to "All countries"
  And the URL is rewritten to have no "country" param

Scenario: An out-of-range numeric param is reset to its default on load
  Given a deep link with query string "adults=4"
  When the page resolves the deep link
  Then the Adults control shows "1"
  And the URL is rewritten to have no "adults" param

Scenario: A contradictory region-and-city deep link resolves with the narrower filter winning
  Given a deep link with query string "region=europe&city=singapore"
  When the page resolves the deep link
  Then the single-city detail for Singapore is shown
  And the URL is rewritten to canonical form with "city=singapore" and "region" backfilled to "asean"

Scenario: A default-valued param is stripped to a clean URL on load
  Given a deep link with query string "tab=cost&adults=1&area=center"
  When the page resolves the deep link
  Then the calculator shows its pristine default state
  And the URL is rewritten to have no query string

Scenario: Canonicalization does not add a browser history entry
  Given a deep link with query string "city=atlantis"
  When the page rewrites the URL to canonical form
  Then pressing the browser Back button does not return to the "city=atlantis" URL
```

### Back-button stepping & nav escape

```gherkin
Scenario: The browser Back button steps through filter changes
  Given I am on the calculator with no query string
  When I select the region "Europe" and then the country "Germany" and then press Back
  Then the URL query string includes "region=europe"
  But the URL query string does not include "country"

Scenario: The breadcrumb offers an escape to the Tools index and Home
  Given I am on the calculator with query string "city=singapore"
  When I read the breadcrumb above the page title
  Then a "Home" link to "/en" is shown
  And a "Tools" link to "/en/tools" is shown

Scenario: The city-detail back link preserves the parent geo scope
  Given I am on the single-city detail with query string "city=singapore"
  When I activate the "Back to all cities" link
  Then the URL query string includes "region=asean" and "country=sg"
  But the URL query string does not include "city"
```

### Locale parity

```gherkin
Scenario: URL state behavior is identical in the Indonesian locale
  Given a deep link with query string "city=jakarta" under the "/id" locale
  When I open that link in a fresh tab
  Then the single-city detail for Jakarta is shown in Indonesian
  And the URL keeps the "/id" locale path segment with the calculator state in the query string
```

## Product Scope

### In scope (product features)

- All nine controls serialized to the query string with the keys above.
- URL-as-source-of-truth derivation; backfill; cascade-clear; sanitize; canonicalize; clean-URL
  defaults omission.
- Breadcrumb nav escape links (Home / Tools / Calculator).
- City-detail back link preserves parent geo scope.
- Full `en` + `id` locale parity.

### Out of scope (product features)

- Any visual/design change (tab-bar overflow, dark-mode) — separate design-findings plan.
- Savings-tab salary input, new cities/data, sorting persistence, or any non-URL usability finding.

## Product-Level Risks

- **Hydration mismatch** — Next.js App Router client components reading `useSearchParams` must
  render consistently between server and client; mitigated by keeping the calculator a client
  component (`"use client"` already present) and deriving state purely from the params.
- **Param-key collision with future controls** — mitigated by centralizing all key names as
  constants in `core/url-state.ts`.
- **Canonicalize flicker** — a visible flash if canonicalization runs late; mitigated by running it
  once on mount via `router.replace` and keeping `sanitize` pure and idempotent.
