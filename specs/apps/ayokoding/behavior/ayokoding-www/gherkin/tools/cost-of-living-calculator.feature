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
    And the minimum marker appears on the lowest-ranked role in the ladder
    And all roles appear above the divider because every role clears a zero target

  Scenario: Expense preview updates in real time when household controls change
    Given I am on the cost-of-living calculator
    And the default household is 1 adult with no children in city center
    When I change the Adults control to 2
    Then the Housing preview amount increases to base times subLinear 2 adults
    And the Childcare and School preview amounts remain zero
    And the Total preview updates immediately without a page reload

  Scenario: Selecting filters updates the URL with query parameters
    Given a user is on the cost-of-living calculator page
    When the user selects Country "Indonesia" and City "Jakarta"
    Then the URL updates to include query parameters reflecting those selections
    And copying the URL and opening it in a new tab restores the same filter state

  Scenario: Page title includes tool name on load
    Given a user navigates to the cost-of-living calculator
    When the page finishes loading with default filter state
    Then the browser tab title includes the name of the tool
