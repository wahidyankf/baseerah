# Spec Gaps — AyoKoding Calculator URL-State Exploratory

These are **proposals** for the maintainer to confirm. Each describes a behaviour the live target
exhibits correctly that the existing `specs/**` Gherkin does not yet describe. Proposed Gherkin
targets `specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`.

---

## SG-U-001 — All nine controls individually update the URL

**Observed behaviour**: Changing any of the nine controls (region, country, city, tab, adults,
preschool, schoolkids, schooltype, area) in isolation immediately writes the corresponding query
parameter to the URL. Returning a control to its default removes the parameter.

**Why spec-worthy**: The existing URL-state scenarios (URL-003 through URL-013) cover combinations
and edge cases but do not enumerate each control in isolation. A regression on any single control
would not be caught by the current scenario set.

**Proposed Gherkin** (extend existing feature file):

```gherkin
# SG-U-001 — Each control writes its parameter in isolation
Scenario Outline: Changing a control in isolation writes its query parameter
  Given I am on the calculator with no query string
  When I change the "<control>" control to "<value>"
  Then the URL query string includes "<param>=<encoded_value>"
  And no other query parameters are added

  Examples:
    | control   | value   | param      | encoded_value |
    | Region    | ASEAN   | region     | asean         |
    | Tab       | Savings | tab        | savings       |
    | Adults    | 2       | adults     | 2             |
    | Preschool | 1       | preschool  | 1             |
    | Schoolkids| 1       | schoolkids | 1             |
    | Area      | Rural   | area       | rural         |
```

**Target file**:
`specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`

---

## SG-U-002 — Default-value controls omit their parameter from the URL

**Observed behaviour**: After setting adults=2 then switching back to adults=1 (the default), the
`adults` parameter is removed entirely from the URL. The same is true for all nine controls — the
URL stays clean when every control is at its default value.

**Why spec-worthy**: Protects the "clean URL" invariant for the default state. Regression would
cause unnecessarily long URLs and break bookmark hygiene.

**Proposed Gherkin**:

```gherkin
# SG-U-002 — Returning a control to its default removes its parameter
Scenario: Returning area to city center removes the area parameter
  Given I am on the calculator with query string "area=rural"
  When I change the Area control back to "City center"
  Then the URL query string does not include "area"
  And the URL query string is empty if all other controls are at their defaults
```

**Target file**:
`specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`

---

## SG-U-003 — Country deep link restores country and backfills region (no city detail)

**Observed behaviour**: Opening `?country=sg` loads the calculator with Country = Singapore,
Region = ASEAN backfilled, City = "All cities" (no single-city detail view shown). This is
distinct from a city deep link, which shows a city detail.

**Why spec-worthy**: The existing URL-011 covers city deep links; country-only deep links are a
separate code path with different UX (filtered list, not single-city detail).

**Proposed Gherkin**:

```gherkin
# SG-U-003 — Country-only deep link restores country filter without showing city detail
Scenario: A country deep link restores the country filter and shows the filtered city list
  Given a deep link with query string "country=sg"
  When the page resolves the deep link
  Then the Country filter shows "Singapore" and the Region filter shows "ASEAN"
  And the cost-of-living table shows Singapore cities as a filtered list
  And no single-city detail view is shown
```

**Target file**:
`specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`

---

## SG-U-004 — Schooltype toggle appears when schoolkids >= 1 and is absent when schoolkids = 0

**Observed behaviour**: With schoolkids = 0 (the default), no "Public" / "Private" school-type
toggle is visible. Changing schoolkids to 1 makes the toggle appear and immediately allows
selecting "Private", which writes `schooltype=private` to the URL.

**Why spec-worthy**: The existing spec scenario "School type toggle is hidden without school-age
children" covers the hidden state, and "School type toggle appears when school-age children is set
to one or more" (SG-005) covers the visible state. However, the URL serialization of the schooltype
control (that clicking "Private" writes `schooltype=private` to the URL) is not covered by any
existing scenario. This gap means a regression on that write could go undetected.

**Proposed Gherkin**:

```gherkin
# SG-U-004 — School type change writes schooltype to URL
Scenario: Switching to private school writes schooltype to the URL
  Given I am on the calculator with 1 school-age child
  And the school type toggle is visible
  When I click the "Private" school type option
  Then the URL query string includes "schooltype=private"
  And switching back to "Public" removes the "schooltype" parameter from the URL
```

**Target file**:
`specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`
