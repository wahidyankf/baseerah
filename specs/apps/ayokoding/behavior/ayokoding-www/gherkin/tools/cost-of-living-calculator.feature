Feature: Salary savings calculator

  Scenario: Cost-of-living breakdown lists category expenses per city
    Given I am on "/en/tools/cost-of-living-calculator"
    And the "Cost of living" tab is active
    When the page finishes loading
    Then I see a table of tech-hub cities
    And each row shows a Country column immediately to the left of the City column
    And each row shows monthly housing, food, transport, utilities, healthcare, childcare, school, and lifestyle expenses
    And each row shows an essentials subtotal and a total
    And each row shows a separate one-time relocation sunk-cost total
    And each row shows a separately labelled liquidity reserve

  Scenario: Region narrows the country filter and country narrows the city filter
    Given I am on "/en/tools/cost-of-living-calculator"
    And the "Cost of living" tab is active
    When I select the region "ASEAN" then the country "Indonesia" in the cascading filters
    Then the Country filter lists only ASEAN countries
    And the City filter lists only Indonesian cities
    And only cities in Indonesia are shown in the table

  Scenario: Country and city are always shown together on every tab
    Given I am on "/en/tools/cost-of-living-calculator"
    When I view any tab's results table
    Then every row shows a Country column immediately to the left of the City column

  Scenario: Clicking a city name opens its single-city cost-of-living detail
    Given I am on "/en/tools/cost-of-living-calculator"
    When I click a city name in any table
    Then I am taken to that city's single-city Cost-of-living detail at "?tab=cost&city=<id>"
    And the City filter is pre-selected to that city
    And the detail shows the full per-category breakdown, essentials subtotal, total, healthcare scheme badge, and split relocation in both local currency and USD

  Scenario: Clicking a country opens Cost-of-living filtered to that country
    Given I am on "/en/tools/cost-of-living-calculator"
    When I click a country name in any table
    Then I am taken to the Cost-of-living tab filtered to that country at "?tab=cost&country=<id>"
    And the Country filter is pre-selected to that country with its Region set
    And the table shows that country's cities as a filtered list rather than a single-city detail

  Scenario: A city link takes precedence over a country link when both params are present
    Given I am on the calculator with both a country and a city query param set
    When the page resolves the deep link at "?tab=cost&country=<id>&city=<id>"
    Then the single-city Cost-of-living detail for the city is shown because a city implies its country

  Scenario: Healthcare funding scheme is always shown
    Given I am on "/en/tools/cost-of-living-calculator"
    When I select any city on any tab
    Then a healthcare funding-scheme badge is shown for that city's country
    And the badge reads "tax-funded", "mandatory payroll insurance", or "out-of-pocket"

  Scenario: The OOP abbreviation is explained on screen
    Given I am on a tab that shows the "Healthcare (OOP)" column
    When I read the legend near the table
    Then an on-screen explanation states that "OOP = out-of-pocket"
    And the explanation says it is the healthcare you pay yourself on top of any tax-funded or insurance coverage
    And every "OOP" acronym is wrapped in an abbr element titled "out-of-pocket"

  Scenario: Relocation reserve is shown separately from sunk costs
    Given I am on the "Cost of living" tab
    When I read a city row
    Then the one-time relocation sunk-cost total is shown distinct from the monthly total
    And the liquidity-reserve cash cushion is shown in its own labelled figure, not folded into the sunk-cost total

  Scenario: Savings tab converts gross salary to net before subtracting expenses
    Given I am on "/en/tools/cost-of-living-calculator"
    And I switch to the "Savings" tab
    When I enter a gross monthly salary of "8000" USD
    Then each city row shows a net take-home after the country's federal and sub-national effective tax
    And each row shows the essentials, the savings after essentials, and the savings after lifestyle with percentages
    And the table can be sorted by savings

  Scenario: Gross salary entered monthly shows the derived annual figure
    Given I am on the "Savings" tab
    When I enter a gross monthly salary of "8000" USD
    Then the annual gross is shown as "96000" USD
    And the annual figure equals twelve times the monthly figure

  Scenario: Non-salary comp is shown as informational context only
    Given I am on the "Savings" tab with a gross salary entered
    When I read a city row
    Then a typical non-salary compensation (RSU/equity + bonus) figure is shown as a separate informational column
    But it is not added into the net, the essential savings, or the after-lifestyle savings

  Scenario: Total compensation is shown for negotiation context
    Given I am on the "Savings" tab with a gross salary entered
    When I read a city row
    Then a total compensation figure equal to the base annual gross plus the typical non-salary comp is shown as informational context
    And the total compensation is not added into the net, the essential savings, or the after-lifestyle savings

  Scenario: Sub-national tax lowers net only in federal countries
    Given I am on the "Savings" tab with a gross salary entered
    When I compare a US, Canadian, or Swiss city against a unitary-country city
    Then the federal-country city applies its city sub-national rate on top of the federal rate
    But the unitary-country city applies the federal rate alone

  Scenario: Net take-home is lower than the entered gross
    Given I am on the "Savings" tab
    When I enter a gross monthly salary above a city's tax band threshold
    Then the net take-home shown for that city is lower than the entered gross

  Scenario: Essentials above net show a deficit
    Given I am on the "Savings" tab for a high-cost city
    When I enter a gross salary whose net is lower than that city's modeled essentials
    Then the savings-after-essentials amount and percentage are shown as negative

  Scenario: Indonesian locale is fully translated
    Given I am on "/id/tools/cost-of-living-calculator"
    When the page finishes loading
    Then all labels, category names, tax wording, healthcare-scheme labels, relocation labels, and the disclaimer are in Indonesian

  Scenario: No Israeli cities are listed
    Given I am on the calculator in either locale
    When the page finishes loading
    Then no Israeli city appears in the dataset or any table

  Scenario: Data snapshot date is clearly shown
    Given I am on the calculator
    When the page finishes loading
    Then I see a prominent "Data last updated" label with the dataset snapshot date
    And I see an "estimates only" disclaimer

  Scenario: Every monetary figure converts to USD via the in-repo FX table
    Given I am on the calculator
    When I read any USD figure derived from a local-currency value
    Then the conversion uses the rate for that currency stored in the in-repo fx.ts table
    And every currency referenced by a city, country, role, or display-currency selector has an fx.ts entry

  Scenario: Adding adults and children changes the modeled expenses
    Given I am on the "Cost of living" tab
    When I change the household from "single" to married with 2 school-age children
    Then the modeled housing and utilities increase sub-linearly
    And the modeled food and healthcare increase near per-capita
    And schooling is added for the two school-age children

  Scenario: Pre-school children incur childcare, not schooling
    Given I am on the "Cost of living" tab
    When I set the household to 1 pre-school child and 0 school-age children
    Then the childcare expense is added for the one pre-school child
    But no schooling cost is added

  Scenario: School type toggle is hidden without school-age children
    Given I am on "/en/tools/cost-of-living-calculator"
    When the household has no school-age children
    Then no school-type toggle is shown

  Scenario: Private school raises expenses more than public
    Given I am on "/en/tools/cost-of-living-calculator"
    And the household has 2 school-age children
    When I switch the school type from "public" to "private"
    Then the schooling portion of the modeled expenses increases

  Scenario: Rural area lowers housing versus city center
    Given I am on the "Cost of living" tab
    When I switch the area from "city center" to "rural"
    Then the modeled housing expense decreases
    And the city total decreases accordingly

  Scenario: Minimum role for a savings target ranks on essential savings and is reordered
    Given I am on "/en/tools/cost-of-living-calculator"
    And I switch to the "Minimum role" tab
    And I set the baseline source to "savings target"
    When I enter a monthly savings target of "8000" USD
    Then I see the software-engineering role ladder with qualifying roles grouped above a divider and non-qualifying roles dimmed below it
    And the lowest role whose best city reaches at least 8000 USD essential savings is marked as the minimum
    And roles whose best city cannot reach 8000 USD essential savings are shown below the divider and de-emphasised

  Scenario: Roles are labelled as software-engineering roles
    Given I am on the "Minimum role" tab
    When the page finishes loading
    Then a caption states the ladder is software-engineering roles covering IC and management tracks

  Scenario: Each role shows its per-country salary distribution
    Given I am on the "Minimum role" tab with a baseline set
    When I read a role row
    Then the role shows its country's p25, median, and p75 salary distribution
    And the row's essential savings is computed from the median salary

  Scenario: Best city shows its country alongside the city name
    Given I am on the "Minimum role" tab with a baseline set
    When I read a qualifying role row
    Then the row shows the best city and its country

  Scenario: Geographic filter scopes the candidate cities
    Given I am on the "Minimum role" tab with a baseline set
    When I select the country "Indonesia" in the cascading filters
    Then each role's best city is chosen only from Indonesian cities

  Scenario: Non-salary comp does not change the minimum-role ranking
    Given I am on the "Minimum role" tab with a baseline set
    When I compare two roles whose non-salary comp differs but whose median salary is equal
    Then their essential-savings ranking is unchanged because non-salary comp is informational only

  Scenario: Lifestyle does not change the minimum-role ranking
    Given I am on the "Minimum role" tab with a baseline set
    When I change a city's lifestyle assumption
    Then the marked minimum role is unchanged because ranking is on essential savings only

  Scenario: Minimum role from a reference city and role
    Given I am on the "Minimum role" tab
    And I set the baseline source to "reference role"
    And I pick the city "Jakarta" and the role "Senior SWE"
    When I view the minimum role result
    Then the baseline savings bar equals that role's essential savings in Jakarta
    And the marked minimum role reaches at least that essential savings in absolute terms

  Scenario: Minimum role from my own salary
    Given I am on the "Minimum role" tab
    And I set the baseline source to "my salary"
    When I enter my gross salary and its city
    Then the baseline savings bar equals my computed essential savings
    And the ladder marks the lowest role that meets or beats it

  Scenario: Savings shown in USD, local, and display currency
    Given I am on the "Minimum role" tab with a baseline set
    When I choose a display currency
    Then each role row shows its essential savings in USD, the city's local currency, and the display currency

  Scenario: Every money column on the Minimum-role tab is dual currency
    Given I am on the "Minimum role" tab with a baseline set and a display currency chosen
    When I read a role row
    Then every money column (p25, median, p75, non-salary comp, total comp, and essential savings) shows the display currency on the first line and the city's local currency on the second line
    And no money column shows only a single currency

  Scenario: Household composition changes the minimum qualifying role
    Given I am on the "Minimum role" tab and the "SWE I" role qualifies for the "single" household basis
    When I change the household to "married with 2 children" and the area to "center"
    Then "SWE I" no longer qualifies because childcare, schooling, and central housing raise its essentials above its net
    And a more senior role becomes the marked minimum

  Scenario: No role can reach the bar
    Given I am on the "Minimum role" tab
    When I set a savings target higher than any role's essential savings in any city
    Then the tool states that no role clears the bar
    And no row is marked as the minimum

  Scenario: Cost-basis controls affect role candidates
    Given I am on the "Minimum role" tab with a baseline set
    When I change the household type or area
    Then the role candidates' savings and the marked minimum role update accordingly

  Scenario: Low-confidence cells are flagged on the minimum-role tab
    Given I am on the "Minimum role" tab
    When the table renders
    Then cells with lower data confidence display a visual flag indicator

  Scenario: No Israeli city appears among role candidates
    Given I am on the "Minimum role" tab
    When the page finishes loading
    Then no Israeli city appears as a candidate city for any role

  Scenario: Zero or empty salary shows deficit with suppressed percentage
    Given I am on the "Savings" tab
    When the gross monthly salary field is empty or zero
    Then each city row shows a negative essential-savings amount equal to the negation of that city's essential expenses in USD
    And each percentage cell shows an em dash because there is no net income to compute a percentage from

  Scenario: Rural area and multi-adult household multiply the housing estimate sub-linearly
    Given I am on the "Cost of living" tab
    And I set the household to 2 adults with no children
    When I switch the area from "city center" to "rural"
    Then the housing estimate in the expense preview decreases to base times subLinear 2 adults times 0.75
    And the essentials total in the preview decreases accordingly

  Scenario: Selecting a city from the City filter opens its detail view
    Given I am on the "Cost of living" tab
    When I select a city from the City dropdown filter
    Then the single-city cost-of-living detail for that city is shown
    And the detail is identical to the one shown when clicking the city name in the table

  Scenario: Income exactly at the low-to-mid threshold uses the mid band
    Given I am on the "Savings" tab
    When I enter a gross monthly salary at exactly the low-to-mid band threshold for a city
    Then that city's net take-home uses the mid band effective tax rate

  Scenario: Mobile city cards show the country name alongside the city
    Given I am viewing the "Cost of living" tab on a viewport narrower than 768 px
    When the mobile city cards render
    Then each card header shows both the city name and its country name

  Scenario: Zero savings target marks the lowest role as the minimum
    Given I am on the "Minimum role" tab
    And I set the baseline source to "savings target"
    When I enter a monthly savings target of zero USD
    Then the qualifying divider is shown
    And the qualifying divider element is rendered in the role ladder
    And the minimum marker appears on the lowest-ranked role in the ladder
    And all roles appear above the divider because every role clears a zero target

  Scenario: Expense preview updates in real time when household controls change
    Given I am on the cost-of-living calculator
    And the default household is 1 adult with no children in city center
    When I change the Adults control to 2
    Then the Housing preview amount increases to base times subLinear 2 adults
    And the Childcare and School preview amounts remain zero
    And the Total preview updates immediately without a page reload

  # Reconciled 2026-06-21: all 9 controls now serialized (region/country/city/tab/adults/
  # preschool/schoolkids/schooltype/area); selecting country alone encodes only "country=id",
  # not "tab=cost" (default tab is omitted per encodeState default-stripping).
  Scenario: Selecting filters updates the URL with all active query parameters
    Given a user is on the cost-of-living calculator page
    When the user selects Country "Indonesia" and City "Jakarta"
    Then the URL updates to include query parameters reflecting those selections
    And copying the URL and opening it in a new tab restores the same filter state

  Scenario: Page title includes tool name on load
    Given a user navigates to the cost-of-living calculator
    When the page finishes loading with default filter state
    Then the browser tab title includes the name of the tool

  # ── Accepted proposals: SG-001…006, USS-001…005, SG-D-001…004 (Phase 4 grill 2026-06-20) ──

  # SG-001 — Negative salary input is clamped to zero
  Scenario: Negative salary input is clamped to zero
    Given I am on the "Savings" tab
    When I enter a gross monthly salary of "-1000"
    Then the annual gross displayed is "0 USD"
    And each city row shows the same deficit as for a zero salary entry

  # SG-002 — Decimal salary computes annual gross correctly
  Scenario: Decimal monthly salary produces correct annual gross
    Given I am on the "Savings" tab
    When I enter a gross monthly salary of "8000.5"
    Then the annual gross is shown as "96,006 USD"
    And the annual figure equals twelve times the monthly figure

  # SG-003 — Very large salary does not produce NaN or Infinity
  Scenario: Very large salary produces valid savings figures
    Given I am on the "Savings" tab
    When I enter a gross monthly salary of "99999999"
    Then no city row shows "NaN" or "Infinity" in any column
    And each city row shows a positive net take-home

  # SG-004 — Selecting only a country updates the URL
  # Reconciled 2026-06-21: default tab ("cost") is omitted from the URL; only "country=id" is
  # encoded. The assertion that "tab=cost" appears in the URL was stale.
  Scenario: Selecting only a country updates the URL country parameter
    Given a user is on the cost-of-living calculator page
    When the user selects Country "Indonesia" without selecting a city
    Then the URL query string includes "country=id"
    And opening that URL in a new tab shows only Indonesian cities in the table
    And the Country filter is pre-selected to "Indonesia"

  # SG-005 — School type toggle appears when school-age children >= 1
  Scenario: School type toggle appears when school-age children is set to one or more
    Given I am on "/en/tools/cost-of-living-calculator"
    And the household has no school-age children
    When I set the household to 1 school-age child
    Then the school type toggle is shown with "Public" and "Private" options
    And the default selection is "Public"

  # SG-006 — Housing scales sub-linearly (1.25x) for a 2-adult household
  Scenario: Housing preview scales sub-linearly for 2-adult household
    Given I am on the cost-of-living calculator
    And the default household is 1 adult with no children in city center
    When I change the Adults control to 2
    Then the Housing preview amount is exactly 1.25 times the 1-adult amount
    And the Utilities preview amount is exactly 1.25 times the 1-adult amount
    And the Food preview amount is exactly 1.5 times the 1-adult amount
    And the Transport preview amount is unchanged from the 1-adult amount

  # USS-001 — Savings tab empty-state when no salary entered
  Scenario: Savings tab shows empty-state guidance when no salary entered
    Given a user has opened the Cost of Living Calculator
    When they click the Savings tab
    And the gross monthly salary field contains no value or zero
    Then the savings comparison table is not shown
    And an instructional message is shown
    And no negative savings figures are visible

  Scenario: Savings tab shows results after salary is entered
    Given a user is on the Savings tab with the empty-state message displayed
    When they enter a positive gross monthly salary value
    Then the instructional message disappears
    And the savings comparison table is shown with computed savings figures

  # USS-002 — Minimum Role tab empty-state when no target entered
  Scenario: Minimum Role tab shows empty-state when no target amount entered
    Given a user has opened the Cost of Living Calculator
    When they click the Minimum Role tab
    And the Monthly savings target field contains no value or zero
    Then the role comparison table is not shown
    And an instructional message is shown
    And no role salary data is visible

  # USS-003 — Area toggle confirms data update
  Scenario: Area toggle shows selected state and confirms data update
    Given a user is on the Cost of Living tab
    And "City center" is the currently active area selection
    When the user clicks "Rural"
    Then the "Rural" button displays as the active/selected state
    And a visible signal confirms the table data has been recalculated for rural estimates

  # USS-004 — Tab name and sub-label are visually/aria distinct
  Scenario: Tab sub-labels are visually separated from tab names
    Given a user views the Cost of Living Calculator tab bar
    When any tab is in the inactive state
    Then the tab primary name and its descriptive sub-label are visually distinct
    And the two pieces of text do not run together without a visual separator
    And a screen reader announces them as separate text nodes

  # USS-005 — Tools index renders localized text
  Scenario: Tools index page renders all text in the active locale
    Given a user navigates to /en/tools
    When the page renders
    Then the page heading and the calculator link display readable English labels
    And no raw i18n key strings are visible

  Scenario: Tools index page renders in Indonesian on /id/tools
    Given a user navigates to /id/tools
    When the page renders
    Then the heading and link labels are in Indonesian
    And no raw i18n key strings are visible

  # SG-D-001 — Dual-currency display in cost-of-living and savings tables
  Scenario: Cost-of-living table shows local currency and USD for each expense cell
    Given the user is on the Cost of living tab at desktop width
    When the table renders with at least one city row
    Then every monetary cell shows the local currency amount and the USD equivalent
    And no money cell shows a bare integer without a currency label

  Scenario: Savings table shows local currency and USD for net and savings columns
    Given the user is on the Savings tab with a gross salary entered
    When the table renders
    Then the Net, Essentials, Essential-savings, and After-lifestyle-savings columns show both local and USD amounts

  # SG-D-002 — covered by existing "Mobile city cards show the country name alongside the city"

  # SG-D-003 — Page heading matches tool identity
  Scenario Outline: H1 matches the tool's official name in each locale
    Given the user opens "/<locale>/tools/cost-of-living-calculator"
    When the page renders
    Then the H1 reads "<expected_h1>"
    And the browser title starts with "Cost of Living Calculator"

    Examples:
      | locale | expected_h1               |
      | en     | Cost of Living Calculator |
      | id     | Kalkulator Biaya Hidup    |

  # SG-D-004 — id locale uses Indonesian city/country names in all table views
  Scenario: Id locale cost-of-living table uses Indonesian translations
    Given the user is on "/id/tools/cost-of-living-calculator" at desktop width
    When the cost-of-living table renders
    Then the Country column shows Indonesian country names where translations exist
    And the City column shows Indonesian city names where translations exist

  Scenario: Id locale minimum-role table uses Indonesian best-city names
    Given the user is on "/id/tools/cost-of-living-calculator" at desktop width
    And the Minimum role tab is active
    When the ladder table renders
    Then the Best city column shows Indonesian city and country names where translations exist

  # prd.md acceptance criteria — design-system controls, locale URL redirect, mobile nav
  Scenario: Gross-salary input uses the design-system Input primitive
    Given the user is on the "Savings" tab
    When the tab renders
    Then the gross-salary field renders with a visible border, design-token radius, and padding
    And it is paired with a Label primitive

  Scenario: Baseline selector is a segmented control
    Given the user is on the "Minimum role" tab
    When the tab renders
    Then the baseline-source control renders as a styled segmented button group, not a plain select

  Scenario: Tab labels are clean single phrases
    Given the user views the tab bar at any breakpoint
    When the tab bar renders
    Then each tab trigger's visible text is its label only, with the description not fused into it

  Scenario: Each tab has a visible description associated with its trigger
    Given the user views the calculator tab bar
    When the tab bar renders
    Then each of the three tabs has a visibly rendered description element associated with its trigger via aria-describedby
    And no tab description text is duplicated elsewhere on screen

  Scenario: Uppercase locale URL redirects to canonical lowercase
    Given the user requests "/EN/tools/cost-of-living-calculator"
    When the middleware processes the request
    Then the server redirects to "/en/tools/cost-of-living-calculator"

  Scenario: Mobile nav drawer shows localized site navigation
    Given the user opens the mobile nav drawer at 375px on the "/id/" locale
    When the drawer renders
    Then it shows the site's top-level navigation links
    And every drawer label is localized

  # ── URL state Phase 4 scenarios (added 2026-06-21) ──────────────────────────

  # URL-001 — Out-of-range numeric param is reset to its default on load
  Scenario: An out-of-range numeric param is reset to its default on load
    Given a deep link with query string "adults=4"
    When the page resolves the deep link
    Then the Adults control shows "1"
    And the URL is rewritten to have no "adults" param

  # URL-002 — Full country name is dropped (only ISO id is valid)
  Scenario: A full-country-name param is dropped on load
    Given a deep link with query string "country=Indonesia"
    When the page resolves the deep link
    Then the Country filter returns to "All countries"
    And the URL is rewritten to have no "country" param

  # URL-003 — Selecting a city backfills country and region
  Scenario: Selecting a city under no prior filter backfills country and region
    Given I am on the calculator with no query string
    When I select the city "Jakarta"
    Then the URL query string includes "city=jakarta"
    And the Country filter shows "Indonesia" and the Region filter shows "ASEAN"

  # URL-004 — Selecting a broader region clears incompatible narrower filters
  Scenario: Selecting a broader region clears an incompatible country and city
    Given I am on the calculator with query string "city=singapore"
    When I select the region "Europe"
    Then the URL query string includes "region=europe"
    But the URL query string does not include "country" or "city"

  # URL-005 — Contradictory region+city deep link resolves with narrower filter winning
  Scenario: A contradictory region-and-city deep link resolves with the narrower filter winning
    Given a deep link with query string "region=europe&city=singapore"
    When the page resolves the deep link
    Then the single-city detail for Singapore is shown
    And the URL is rewritten to canonical form with "city=singapore" and "region" backfilled to "asean"

  # URL-006 — City-detail back link preserves parent geo scope
  Scenario: The city-detail back link preserves the parent geo scope
    Given I am on the single-city detail with query string "city=singapore"
    When I activate the "Back to all cities" link
    Then the URL query string includes "region=asean" and "country=sg"
    But the URL query string does not include "city"

  # URL-007 — Tab change is written to the URL
  Scenario: Changing the tab writes the tab to the URL
    Given I am on the calculator with no query string
    When I switch to the "Savings" tab
    Then the URL query string includes "tab=savings"
    And reloading the page keeps the "Savings" tab active

  # URL-008 — Cost-basis control change is written to the URL
  Scenario: Changing a cost-basis control writes it to the URL
    Given I am on the calculator with no query string
    When I change the Adults control to "2"
    Then the URL query string includes "adults=2"
    And the household preview updates without a page reload

  # URL-009 — Breadcrumb offers Home and Tools escape links
  Scenario: The breadcrumb offers an escape to the Tools index and Home
    Given I am on the calculator with query string "city=singapore"
    When I read the breadcrumb above the page title
    Then a "Home" link to "/en" is shown
    And a "Tools" link to "/en/tools" is shown

  # AC-2 (DWT-B-003/DWT-B-004) — Breadcrumb uses the shared primitive with chevron separators
  Scenario: The breadcrumb separates crumbs with chevrons, not a literal slash
    Given I am on the calculator with query string "city=singapore"
    When I read the breadcrumb above the page title
    Then the crumbs are separated by chevron icons
    And no literal "/" separator is shown between crumbs

  # AC-3 (UWT-013) — Final breadcrumb crumb equals the page H1 in each locale
  Scenario Outline: The final breadcrumb crumb matches the page title in each locale
    Given the user opens "/<locale>/tools/cost-of-living-calculator"
    When the breadcrumb renders
    Then the current-page crumb text reads "<expected_title>"
    And the current-page crumb is marked aria-current="page"

    Examples:
      | locale | expected_title            |
      | en     | Cost of Living Calculator |
      | id     | Kalkulator Biaya Hidup    |

  # URL-010 — Region selection writes region to the URL
  Scenario: Selecting a region writes the region to the URL
    Given I am on the calculator with no query string
    When I select the region "Europe"
    Then the URL query string includes "region=europe"
    And the URL query string does not include "country" or "city"

  # URL-011 — City deep link restores city and backfills country and region
  Scenario: A city deep link restores the city and backfills country and region
    Given a deep link with query string "city=singapore"
    When I open that link in a fresh tab
    Then the single-city Cost-of-living detail for Singapore is shown
    And the Country filter shows "Singapore" and the Region filter shows "ASEAN"

  # URL-012 — Unknown city param is dropped on load
  Scenario: An unknown city param is dropped on load
    Given a deep link with query string "city=atlantis"
    When the page resolves the deep link
    Then the City filter returns to "All cities"
    And the URL is rewritten to have no "city" param

  # URL-013 — Canonicalization uses replace so Back button skips the dirty URL
  Scenario: Canonicalization does not add a browser history entry
    Given a deep link with query string "city=atlantis"
    When the page rewrites the URL to canonical form
    Then pressing the browser Back button does not return to the "city=atlantis" URL

  # AC-4 (UWT-016/DWT-005) — Geo-filter selects meet the 44px minimum touch target
  Scenario: Geo-filter selects meet the minimum touch-target height on mobile
    Given I am on the calculator at a 375px-wide viewport
    When the geo-filter selects render
    Then each geo-filter select is at least 44 pixels tall

  # AC-5 (UWT-008) — Calculator page does not overflow horizontally at 320px
  Scenario: The calculator page has no horizontal overflow at 320px
    Given I am on the calculator at a 320px-wide viewport
    When the calculator page renders
    Then the document does not scroll horizontally

  # AC-8 (UWT-004) — Savings gross-salary field surfaces the active currency, not a hardcoded label
  Scenario: The Savings gross-salary field shows the active currency as a separate indicator
    Given I am on the "Savings" tab
    When the gross-salary field renders
    Then the gross-salary label does not contain the literal currency code "USD"
    And an active-currency indicator next to the field shows "USD"

  # AC-9 (UWT-006) — Minimum-role tab shows empty-state guidance for a BLANK savings target only
  Scenario: A blank savings target shows empty-state guidance instead of the role ladder
    Given I am on the "Minimum role" tab with the savings-target baseline and a blank target
    When the tab renders
    Then a minimum-role empty-state guidance message is shown
    But entering an explicit zero target replaces the guidance with the role ladder and its divider

  # AC-10 (UWT-007) — Region selector lists exactly the nine intended regions
  Scenario: The region selector lists exactly the nine intended regions
    Given I am on the calculator with no query string
    When the region filter renders
    Then the region selector offers exactly the nine regions africa, americas, asean, asia, europe, japan, mena, nordics, and oceania

  # AC-11 (UWT-014) — A country change that auto-changes the region surfaces an advisory
  Scenario: Selecting a country that changes the region shows a visible advisory
    Given I am on the calculator with no region selected
    When I select a country whose region differs from the current selection
    Then a visible region-auto-advisory message is shown

  # AC-12 (UWT-015) — A city-only deep link returns to the bare calculator
  Scenario: A city-only deep link back link omits the auto-derived region and country
    Given a deep link with query string "city=london"
    When I read the single-city detail back link
    Then the back link points to the bare calculator "?tab=cost" with no region or country
