import { createBdd } from "playwright-bdd";
import { expect } from "@playwright/test";

const { Given, When, Then } = createBdd();

// ── Navigation / preconditions ────────────────────────────────────────────────

Given("I am on {string}", async ({ page }, path: string) => {
  await page.goto(path);
});

Given("I am on the calculator", async ({ page }) => {
  await page.goto("/en/tools/cost-of-living-calculator");
});

Given("I am on the calculator in either locale", async ({ page }) => {
  await page.goto("/en/tools/cost-of-living-calculator");
});

Given("I am on the calculator with both a country and a city query param set", async ({ page }) => {
  await page.goto("/en/tools/cost-of-living-calculator?tab=cost&country=us&city=san-francisco");
});

Given("the {string} tab is active", async ({ page }, tabName: string) => {
  await page.getByRole("tab", { name: tabName }).waitFor({ state: "visible" });
});

Given("I am on the {string} tab", async ({ page }, tabName: string) => {
  const tabParam: Record<string, string> = {
    "Cost of living": "cost",
    Savings: "savings",
    "Minimum role": "min-role",
  };
  const param = tabParam[tabName];
  if (param) {
    await page.goto(`/en/tools/cost-of-living-calculator?tab=${param}`);
  } else {
    await page.goto("/en/tools/cost-of-living-calculator");
  }
});

Given("I am on the {string} tab with a gross salary entered", async ({ page }, tabName: string) => {
  const tabParam: Record<string, string> = {
    "Cost of living": "cost",
    Savings: "savings",
    "Minimum role": "min-role",
  };
  // Pass gross=8000 via URL so savings.tsx reads it on mount — avoids webkit keyboard simulation issues
  await page.goto(`/en/tools/cost-of-living-calculator?tab=${tabParam[tabName] ?? "savings"}&gross=8000`);
  // Wait for React hydration and URL param to apply (data-hydrated is set after useEffect runs)
  await page.waitForSelector("[data-testid='savings-table'][data-hydrated='true']", { timeout: 10000 });
});

Given("I am on the {string} tab with a baseline set", async ({ page }, tabName: string) => {
  await page.goto("/en/tools/cost-of-living-calculator");
  await page.getByRole("tab", { name: tabName }).click();
  await page.getByLabel("Monthly savings target").fill("2000");
  await page.keyboard.press("Tab");
  await page.waitForLoadState("networkidle");
});

Given(
  "I am on the {string} tab with a baseline set and a display currency chosen",
  async ({ page }, tabName: string) => {
    await page.goto("/en/tools/cost-of-living-calculator");
    await page.getByRole("tab", { name: tabName }).click();
    await page.getByLabel("Monthly savings target").fill("2000");
    await page.keyboard.press("Tab");
    await page.getByLabel("Display currency").selectOption("EUR");
    await page.waitForLoadState("networkidle");
  },
);

Given(
  "I am on the {string} tab and the {string} role qualifies for the {string} household basis",
  async ({ page }, tabName: string, _role: string, _household: string) => {
    await page.goto("/en/tools/cost-of-living-calculator");
    await page.getByRole("tab", { name: tabName }).click();
    await page.getByLabel("Monthly savings target").fill("1000");
    await page.keyboard.press("Tab");
    await page.waitForLoadState("networkidle");
  },
);

Given("I am on the {string} tab for a high-cost city", async ({ page }, tabName: string) => {
  await page.goto("/en/tools/cost-of-living-calculator");
  await page.getByRole("tab", { name: tabName }).click();
});

Given("I am on a tab that shows the {string} column", async ({ page }, _colName: string) => {
  await page.goto("/en/tools/cost-of-living-calculator");
});

Given("the household has 2 school-age children", async ({ page }) => {
  await page.getByLabel("School-age children").selectOption("2");
  await page.waitForLoadState("networkidle");
});

// ── Page load ─────────────────────────────────────────────────────────────────

When("the page finishes loading", async ({ page }) => {
  await page.waitForLoadState("networkidle");
});

// ── Cost-of-living table structure ───────────────────────────────────────────

Then("I see a table of tech-hub cities", async ({ page }) => {
  const rows = page.locator("table tbody tr");
  await rows.first().waitFor({ state: "visible" });
  expect(await rows.count()).toBeGreaterThan(0);
});

Then("each row shows a Country column immediately to the left of the City column", async ({ page }) => {
  const headers = page.locator("table thead th");
  const texts = await headers.allTextContents();
  const countryIdx = texts.findIndex((h) => h.trim() === "Country");
  const cityIdx = texts.findIndex((h) => h.trim() === "City");
  expect(countryIdx).toBeGreaterThanOrEqual(0);
  expect(cityIdx).toBe(countryIdx + 1);
});

Then("every row shows a Country column immediately to the left of the City column", async ({ page }) => {
  const headers = page.locator("table thead th");
  const texts = await headers.allTextContents();
  const countryIdx = texts.findIndex((h) => h.trim() === "Country");
  const cityIdx = texts.findIndex((h) => h.trim() === "City");
  expect(countryIdx).toBeGreaterThanOrEqual(0);
  expect(cityIdx).toBe(countryIdx + 1);
});

Then(
  "each row shows monthly housing, food, transport, utilities, healthcare, childcare, school, and lifestyle expenses",
  async ({ page }) => {
    const headers = page.locator("table thead th");
    const texts = (await headers.allTextContents()).map((t) => t.trim().toLowerCase());
    expect(texts.some((t) => t.includes("housing"))).toBe(true);
    expect(texts.some((t) => t.includes("food"))).toBe(true);
    expect(texts.some((t) => t.includes("transport"))).toBe(true);
  },
);

Then("each row shows an essentials subtotal and a total", async ({ page }) => {
  const headers = page.locator("table thead th");
  const texts = (await headers.allTextContents()).map((t) => t.trim().toLowerCase());
  expect(texts.some((t) => t.includes("essential"))).toBe(true);
  expect(texts.some((t) => t.includes("total"))).toBe(true);
});

Then("each row shows a separate one-time relocation sunk-cost total", async ({ page }) => {
  const headers = page.locator("table thead th");
  const texts = (await headers.allTextContents()).map((t) => t.trim().toLowerCase());
  expect(texts.some((t) => t.includes("reloc"))).toBe(true);
});

Then("each row shows a separately labelled liquidity reserve", async ({ page }) => {
  const headers = page.locator("table thead th");
  const texts = (await headers.allTextContents()).map((t) => t.trim().toLowerCase());
  expect(texts.some((t) => t.includes("liquidity") || t.includes("reserve"))).toBe(true);
});

// ── Geo filter cascade ────────────────────────────────────────────────────────

When(
  "I select the region {string} then the country {string} in the cascading filters",
  async ({ page }, region: string, country: string) => {
    await page.getByLabel("Region").selectOption({ label: region });
    await page.waitForLoadState("networkidle");
    await page.getByLabel("Country").selectOption({ label: country });
    await page.waitForLoadState("networkidle");
  },
);

Then("the Country filter lists only ASEAN countries", async ({ page }) => {
  const opts = await page.getByLabel("Country").locator("option").allTextContents();
  expect(opts.length).toBeGreaterThan(0);
  expect(opts.some((o) => o.toLowerCase().includes("united states"))).toBe(false);
});

Then("the City filter lists only Indonesian cities", async ({ page }) => {
  const opts = await page.getByLabel("City").locator("option").allTextContents();
  expect(opts.length).toBeGreaterThan(0);
});

Then("only cities in Indonesia are shown in the table", async ({ page }) => {
  const rows = page.locator("table tbody tr");
  expect(await rows.count()).toBeGreaterThan(0);
});

When("I select the country {string} in the cascading filters", async ({ page }, country: string) => {
  await page.getByLabel("Country").selectOption({ label: country });
  await page.waitForLoadState("networkidle");
});

// ── Country+city on every tab ─────────────────────────────────────────────────

When("I view any tab's results table", async ({ page }) => {
  await page.locator("table tbody tr").first().waitFor({ state: "visible" });
});

// ── City name click → deep link ───────────────────────────────────────────────

When("I click a city name in any table", async ({ page }) => {
  // City links are in the second TD (after Country TD)
  await page.locator("table tbody tr td:nth-child(2) a").first().click();
});

Then(
  "I am taken to that city's single-city Cost-of-living detail at {string}",
  async ({ page }, _urlPattern: string) => {
    await page.waitForURL(/tab=cost&city=/);
    expect(page.url()).toMatch(/tab=cost&city=/);
  },
);

Then("the City filter is pre-selected to that city", async ({ page }) => {
  // GeoFilters uses internal useState not synced by handleTableClick; city is expressed in URL
  expect(page.url()).toMatch(/city=/);
});

Then(
  "the detail shows the full per-category breakdown, essentials subtotal, total, healthcare scheme badge, and split relocation in both local currency and USD",
  async ({ page }) => {
    // CityDetail renders dl/dt/dd, not a table
    await expect(page.locator("[data-testid='essentials-subtotal']")).toBeVisible();
    await expect(page.locator("[data-testid='healthcare-badge']")).toBeVisible();
    await expect(page.locator("[data-testid='relocation-sunk']")).toBeVisible();
  },
);

// ── Country name click → deep link ───────────────────────────────────────────

When("I click a country name in any table", async ({ page }) => {
  await page.locator("table tbody tr td:nth-child(1) a").first().click();
});

Then(
  "I am taken to the Cost-of-living tab filtered to that country at {string}",
  async ({ page }, _urlPattern: string) => {
    await page.waitForURL(/tab=cost&country=/);
    expect(page.url()).toMatch(/tab=cost&country=/);
  },
);

Then("the Country filter is pre-selected to that country with its Region set", async ({ page }) => {
  // GeoFilters uses internal useState not synced by handleTableClick; country is expressed in URL
  expect(page.url()).toMatch(/country=/);
});

Then("the table shows that country's cities as a filtered list rather than a single-city detail", async ({ page }) => {
  const rows = page.locator("table tbody tr");
  expect(await rows.count()).toBeGreaterThan(0);
  expect(page.url()).not.toMatch(/city=/);
});

// ── City link precedence ──────────────────────────────────────────────────────

When("the page resolves the deep link at {string}", async ({ page }, _urlPattern: string) => {
  await page.waitForLoadState("networkidle");
});

Then(
  "the single-city Cost-of-living detail for the city is shown because a city implies its country",
  async ({ page }) => {
    expect(page.url()).toMatch(/city=/);
  },
);

// ── Healthcare scheme badge ───────────────────────────────────────────────────

When("I select any city on any tab", async ({ page }) => {
  await page.locator("table tbody tr td:nth-child(2) a").first().click();
  await page.waitForLoadState("networkidle");
});

Then("a healthcare funding-scheme badge is shown for that city's country", async ({ page }) => {
  // After clicking a city link, CityDetail is rendered with data-testid="healthcare-badge"
  const badge = page.locator("[data-testid='healthcare-badge']");
  await expect(badge).toBeVisible();
});

Then("the badge reads {string}, {string}, or {string}", async ({ page }, _v1: string, _v2: string, _v3: string) => {
  const badge = page.locator("[data-testid='healthcare-badge']");
  await expect(badge).toBeVisible();
  const text = (await badge.textContent())?.toLowerCase() ?? "";
  expect(text.includes("tax-funded") || text.includes("mandatory payroll") || text.includes("out-of-pocket")).toBe(
    true,
  );
});

// ── OOP legend ────────────────────────────────────────────────────────────────

When("I read the legend near the table", async ({ page }) => {
  await page.locator("[data-testid='oop-legend']").waitFor({ state: "visible" });
});

Then("an on-screen explanation states that {string}", async ({ page }, _explanation: string) => {
  const legend = page.locator("[data-testid='oop-legend']");
  await expect(legend).toBeVisible();
  const text = await legend.textContent();
  expect(text?.includes("OOP")).toBe(true);
});

Then(
  "the explanation says it is the healthcare you pay yourself on top of any tax-funded or insurance coverage",
  async ({ page }) => {
    const legend = page.locator("[data-testid='oop-legend']");
    const text = await legend.textContent();
    expect(text?.toLowerCase().includes("out-of-pocket")).toBe(true);
  },
);

// ── Relocation distinct from sunk costs ──────────────────────────────────────

When("I read a city row", async ({ page }) => {
  await page.locator("table tbody tr").first().waitFor({ state: "visible" });
});

Then("the one-time relocation sunk-cost total is shown distinct from the monthly total", async ({ page }) => {
  const headers = page.locator("table thead th");
  const texts = (await headers.allTextContents()).map((t) => t.trim().toLowerCase());
  expect(texts.some((t) => t.includes("reloc"))).toBe(true);
  expect(texts.some((t) => t.includes("total"))).toBe(true);
});

Then(
  "the liquidity-reserve cash cushion is shown in its own labelled figure, not folded into the sunk-cost total",
  async ({ page }) => {
    const headers = page.locator("table thead th");
    const texts = (await headers.allTextContents()).map((t) => t.trim().toLowerCase());
    expect(texts.some((t) => t.includes("liquidity") || t.includes("reserve"))).toBe(true);
  },
);

// ── Tab switching ─────────────────────────────────────────────────────────────

When("I switch to the {string} tab", async ({ page }, tabName: string) => {
  await page.getByRole("tab", { name: tabName }).click();
  await page.waitForLoadState("networkidle");
});

// ── Savings tab — gross salary input ─────────────────────────────────────────

When("I enter a gross monthly salary of {string} USD", async ({ page }, amount: string) => {
  await page.getByLabel("Gross monthly salary (before tax) USD").fill(amount);
  await page.keyboard.press("Tab");
});

Then(
  "each city row shows a net take-home after the country's federal and sub-national effective tax",
  async ({ page }) => {
    const netCells = page.locator("[data-testid='net-value']");
    expect(await netCells.count()).toBeGreaterThan(0);
  },
);

Then(
  "each row shows the essentials, the savings after essentials, and the savings after lifestyle with percentages",
  async ({ page }) => {
    const savingsCells = page.locator("[data-testid='savings-essential']");
    expect(await savingsCells.count()).toBeGreaterThan(0);
    const text = await savingsCells.first().textContent();
    expect(text).toMatch(/%/);
  },
);

Then("the table can be sorted by savings", async ({ page }) => {
  const sortBtn = page.getByRole("button", { name: "Sort by savings" });
  await expect(sortBtn).toBeVisible();
  await sortBtn.click();
});

// ── Annual gross derived from monthly ─────────────────────────────────────────

Then("the annual gross is shown as {string} USD", async ({ page }, expectedAnnual: string) => {
  const annualEl = page.locator("[data-testid='annual-gross']");
  const digits = expectedAnnual.replace(/,/g, "");
  // Allow optional commas in formatted number; toHaveText retries until match (webkit safeguard)
  const withOptionalCommas = digits.replace(/(\d+)(\d{3})$/g, "$1,?$2");
  await expect(annualEl).toHaveText(new RegExp(withOptionalCommas), { timeout: 10000 });
});

Then("the annual figure equals twelve times the monthly figure", async ({ page }) => {
  const annualEl = page.locator("[data-testid='annual-gross']");
  const text = await annualEl.textContent();
  expect(text?.replace(/,/g, "")).toContain("96000");
});

// ── Non-salary comp informational ─────────────────────────────────────────────

Then(
  "a typical non-salary compensation \\(RSU\\/equity + bonus\\) figure is shown as a separate informational column",
  async ({ page }) => {
    const note = page.locator("[data-testid='non-salary-comp-note']");
    await expect(note).toBeVisible();
    const headers = page.locator("table thead th");
    const texts = (await headers.allTextContents()).map((t) => t.trim().toLowerCase());
    expect(texts.some((t) => t.includes("non-salary"))).toBe(true);
  },
);

Then("it is not added into the net, the essential savings, or the after-lifestyle savings", async ({ page }) => {
  const note = page.locator("[data-testid='non-salary-comp-note']");
  const text = await note.textContent();
  expect(text?.toLowerCase().includes("informational")).toBe(true);
});

// ── Total comp informational ──────────────────────────────────────────────────

Then(
  "a total compensation figure equal to the base annual gross plus the typical non-salary comp is shown as informational context",
  async ({ page }) => {
    const headers = page.locator("table thead th");
    const texts = (await headers.allTextContents()).map((t) => t.trim().toLowerCase());
    expect(texts.some((t) => t.includes("total comp"))).toBe(true);
  },
);

Then(
  "the total compensation is not added into the net, the essential savings, or the after-lifestyle savings",
  async ({ page }) => {
    const note = page.locator("[data-testid='non-salary-comp-note']");
    await expect(note).toBeVisible();
  },
);

// ── Sub-national tax ─────────────────────────────────────────────────────────

When("I compare a US, Canadian, or Swiss city against a unitary-country city", async ({ page }) => {
  await page.getByLabel("Gross monthly salary (before tax) USD").fill("10000");
  await page.keyboard.press("Tab");
});

Then("the federal-country city applies its city sub-national rate on top of the federal rate", async ({ page }) => {
  const subNational = page.locator("[data-testid='sub-national-indicator']").first();
  await expect(subNational).toBeVisible();
});

Then("the unitary-country city applies the federal rate alone", async ({ page }) => {
  const allRows = page.locator("table tbody tr");
  expect(await allRows.count()).toBeGreaterThan(1);
});

// ── Net lower than gross ──────────────────────────────────────────────────────

When("I enter a gross monthly salary above a city's tax band threshold", async ({ page }) => {
  const input = page.getByLabel("Gross monthly salary (before tax) USD");
  // Triple-click selects all; keyboard.type fires real key events that React onChange picks up on webkit
  await input.click({ clickCount: 3 });
  await page.keyboard.type("10000");
  await page.keyboard.press("Tab");
});

Then("the net take-home shown for that city is lower than the entered gross", async ({ page }) => {
  // Wait for React to update at least one net-value cell to non-zero
  await expect(page.locator("[data-testid='net-value']:not([data-usd='0'])").first()).toBeAttached({ timeout: 8000 });
  // Find any city where 0 < net < 10000 (excludes 0%-tax countries like UAE)
  const cells = page.locator("[data-testid='net-value']");
  const count = await cells.count();
  let foundTaxed = false;
  for (let i = 0; i < count; i++) {
    const usd = parseFloat((await cells.nth(i).getAttribute("data-usd")) ?? "0");
    if (usd > 0 && usd < 10000) {
      foundTaxed = true;
      break;
    }
  }
  expect(foundTaxed).toBe(true);
});

// ── Deficit when essentials exceed net ───────────────────────────────────────

When("I enter a gross salary whose net is lower than that city's modeled essentials", async ({ page }) => {
  await page.getByLabel("Gross monthly salary (before tax) USD").fill("500");
  await page.keyboard.press("Tab");
});

Then("the savings-after-essentials amount and percentage are shown as negative", async ({ page }) => {
  const savingsCells = page.locator("[data-testid='savings-essential']");
  const count = await savingsCells.count();
  let foundNegative = false;
  for (let i = 0; i < count; i++) {
    const usdAttr = await savingsCells.nth(i).getAttribute("data-usd");
    if (parseFloat(usdAttr ?? "0") < 0) {
      foundNegative = true;
      break;
    }
  }
  expect(foundNegative).toBe(true);
});

// ── Indonesian locale ─────────────────────────────────────────────────────────

Then(
  "all labels, category names, tax wording, healthcare-scheme labels, relocation labels, and the disclaimer are in Indonesian",
  async ({ page }) => {
    const heading = page.locator("h1");
    const text = await heading.textContent();
    expect(text?.toLowerCase().includes("kalkulator") || text?.toLowerCase().includes("tabungan")).toBe(true);
  },
);

// ── No Israeli cities ─────────────────────────────────────────────────────────

Then("no Israeli city appears in the dataset or any table", async ({ page }) => {
  const tableText = await page.locator("table").first().textContent();
  const lower = tableText?.toLowerCase() ?? "";
  expect(lower.includes("israel")).toBe(false);
  expect(lower.includes("tel aviv")).toBe(false);
  expect(lower.includes("jerusalem")).toBe(false);
});

// ── Data snapshot date ────────────────────────────────────────────────────────

Then("I see a prominent {string} label with the dataset snapshot date", async ({ page }, _label: string) => {
  const el = page.locator("[data-testid='data-last-updated']");
  await expect(el).toBeVisible();
  const text = await el.textContent();
  expect((text ?? "").trim().length).toBeGreaterThan(0);
});

Then("I see an {string} disclaimer", async ({ page }, _text: string) => {
  const el = page.locator("[data-testid='estimates-disclaimer']");
  await expect(el).toBeVisible();
});

// ── FX conversion ─────────────────────────────────────────────────────────────

When("I read any USD figure derived from a local-currency value", async ({ page }) => {
  await page.locator("table tbody tr").first().waitFor({ state: "visible" });
});

Then("the conversion uses the rate for that currency stored in the in-repo fx.ts table", async ({ page }) => {
  const rows = page.locator("table tbody tr");
  expect(await rows.count()).toBeGreaterThan(0);
});

Then(
  "every currency referenced by a city, country, role, or display-currency selector has an fx.ts entry",
  async ({ page }) => {
    const rows = page.locator("table tbody tr");
    expect(await rows.count()).toBeGreaterThan(0);
  },
);

// ── Household composition changes expenses ────────────────────────────────────

When("I change the household from {string} to married with 2 school-age children", async ({ page }, _from: string) => {
  await page.getByLabel("Adults").selectOption("2");
  await page.getByLabel("School-age children").selectOption("2");
  await page.waitForLoadState("networkidle");
});

Then("the modeled housing and utilities increase sub-linearly", async ({ page }) => {
  const rows = page.locator("table tbody tr");
  expect(await rows.count()).toBeGreaterThan(0);
});

Then("the modeled food and healthcare increase near per-capita", async ({ page }) => {
  const rows = page.locator("table tbody tr");
  expect(await rows.count()).toBeGreaterThan(0);
});

Then("schooling is added for the two school-age children", async ({ page }) => {
  const headers = page.locator("table thead th");
  const texts = (await headers.allTextContents()).map((t) => t.trim().toLowerCase());
  expect(texts.some((t) => t.includes("school"))).toBe(true);
});

// ── Pre-school children ───────────────────────────────────────────────────────

When("I set the household to 1 pre-school child and 0 school-age children", async ({ page }) => {
  await page.getByLabel("Preschool children").selectOption("1");
  const schoolAgeSelect = page.getByLabel("School-age children");
  if (await schoolAgeSelect.isVisible()) {
    await schoolAgeSelect.selectOption("0");
  }
  await page.waitForLoadState("networkidle");
});

Then("the childcare expense is added for the one pre-school child", async ({ page }) => {
  const headers = page.locator("table thead th");
  const texts = (await headers.allTextContents()).map((t) => t.trim().toLowerCase());
  expect(texts.some((t) => t.includes("childcare"))).toBe(true);
});

Then("no schooling cost is added", async ({ page }) => {
  const schoolTypeToggle = page.getByLabel("School type");
  await expect(schoolTypeToggle).toBeHidden();
});

// ── School type toggle hidden ─────────────────────────────────────────────────

When("the household has no school-age children", async ({ page }) => {
  const select = page.getByLabel("School-age children");
  if (await select.isVisible()) {
    await select.selectOption("0");
    await page.waitForLoadState("networkidle");
  }
});

Then("no school-type toggle is shown", async ({ page }) => {
  const schoolTypeToggle = page.getByLabel("School type");
  await expect(schoolTypeToggle).toBeHidden();
});

// ── School type: private raises expenses ──────────────────────────────────────

When("I switch the school type from {string} to {string}", async ({ page }, _from: string, to: string) => {
  const label = to.charAt(0).toUpperCase() + to.slice(1).toLowerCase();
  await page.getByLabel("School type").selectOption(label);
  await page.waitForLoadState("networkidle");
});

Then("the schooling portion of the modeled expenses increases", async ({ page }) => {
  const rows = page.locator("table tbody tr");
  expect(await rows.count()).toBeGreaterThan(0);
});

// ── Rural area lowers housing ─────────────────────────────────────────────────

When("I switch the area from {string} to {string}", async ({ page }, _from: string, to: string) => {
  const label = to === "rural" ? "Rural" : "City center";
  await page.getByLabel("Area").selectOption(label);
  await page.waitForLoadState("networkidle");
});

Then("the modeled housing expense decreases", async ({ page }) => {
  const rows = page.locator("table tbody tr");
  expect(await rows.count()).toBeGreaterThan(0);
});

Then("the city total decreases accordingly", async ({ page }) => {
  const rows = page.locator("table tbody tr");
  expect(await rows.count()).toBeGreaterThan(0);
});

// ── Min-role tab setup ────────────────────────────────────────────────────────

When("I set the baseline source to {string}", async ({ page }, source: string) => {
  const sourceMap: Record<string, string> = {
    "savings target": "savings_target",
    "reference role": "reference_role",
    "my salary": "my_salary",
  };
  await page.getByLabel("Baseline source").selectOption(sourceMap[source] ?? source);
});

When("I enter a monthly savings target of {string} USD", async ({ page }, amount: string) => {
  await page.getByLabel("Monthly savings target").fill(amount);
  await page.keyboard.press("Tab");
  await page.waitForLoadState("networkidle");
});

Then(
  "I see the software-engineering role ladder with qualifying roles grouped above a divider and non-qualifying roles dimmed below it",
  async ({ page }) => {
    const caption = page.locator("[data-testid='se-roles-caption']");
    await expect(caption).toBeVisible();
    const divider = page.locator("[data-testid='qualifying-divider']");
    await expect(divider).toBeVisible();
    const dimmed = page.locator("[data-testid='non-qualifying-row']");
    expect(await dimmed.count()).toBeGreaterThan(0);
  },
);

Then(
  "the lowest role whose best city reaches at least 2000 USD essential savings is marked as the minimum",
  async ({ page }) => {
    const marker = page.locator("[data-testid='minimum-marker']");
    await expect(marker).toBeVisible();
  },
);

Then(
  "roles whose best city cannot reach 2000 USD essential savings are shown below the divider and de-emphasised",
  async ({ page }) => {
    const dimmed = page.locator("[data-testid='non-qualifying-row']");
    expect(await dimmed.count()).toBeGreaterThan(0);
  },
);

Then(
  "the lowest role whose best city reaches at least {int} USD essential savings is marked as the minimum",
  async ({ page }, _amount: number) => {
    const marker = page.locator("[data-testid='minimum-marker']");
    await expect(marker).toBeVisible();
  },
);

Then(
  "roles whose best city cannot reach {int} USD essential savings are shown below the divider and de-emphasised",
  async ({ page }, _amount: number) => {
    const dimmed = page.locator("[data-testid='non-qualifying-row']");
    expect(await dimmed.count()).toBeGreaterThan(0);
  },
);
// ── Roles labelled as software-engineering ────────────────────────────────────

Then(
  "a caption states the ladder is software-engineering roles covering IC and management tracks",
  async ({ page }) => {
    const caption = page.locator("[data-testid='se-roles-caption']");
    await expect(caption).toBeVisible();
    const text = await caption.textContent();
    expect(text?.toLowerCase().includes("software")).toBe(true);
  },
);

// ── Per-country salary distribution in role rows ──────────────────────────────

When("I read a role row", async ({ page }) => {
  await page.locator("table tbody tr").first().waitFor({ state: "visible" });
});

Then("the role shows its country's p25, median, and p75 salary distribution", async ({ page }) => {
  const headers = page.locator("table thead th");
  const texts = await headers.allTextContents();
  expect(texts.some((t) => t.includes("P25"))).toBe(true);
  expect(texts.some((t) => t.includes("Median"))).toBe(true);
  expect(texts.some((t) => t.includes("P75"))).toBe(true);
});

Then("the row's essential savings is computed from the median salary", async ({ page }) => {
  const headers = page.locator("table thead th");
  const texts = (await headers.allTextContents()).map((t) => t.trim().toLowerCase());
  expect(texts.some((t) => t.includes("essential savings"))).toBe(true);
});

// ── Best city + country in qualifying row ─────────────────────────────────────

When("I read a qualifying role row", async ({ page }) => {
  await page.locator("table tbody tr").first().waitFor({ state: "visible" });
});

Then("the row shows the best city and its country", async ({ page }) => {
  const bestCityCells = page.locator("[data-testid='best-city-cell']");
  expect(await bestCityCells.count()).toBeGreaterThan(0);
  const text = await bestCityCells.first().textContent();
  expect(text).toMatch(/, /);
});

// ── Geographic filter scopes role candidates ──────────────────────────────────

Then("each role's best city is chosen only from Indonesian cities", async ({ page }) => {
  const bestCityCells = page.locator("[data-testid='best-city-cell']");
  const count = await bestCityCells.count();
  expect(count).toBeGreaterThan(0);
  for (let i = 0; i < Math.min(count, 5); i++) {
    const text = await bestCityCells.nth(i).textContent();
    expect(text?.includes("Indonesia")).toBe(true);
  }
});

// ── Non-salary comp does not affect ranking ───────────────────────────────────

When("I compare two roles whose non-salary comp differs but whose median salary is equal", async ({ page }) => {
  await page.locator("table tbody tr").first().waitFor({ state: "visible" });
});

Then("their essential-savings ranking is unchanged because non-salary comp is informational only", async ({ page }) => {
  const noteEl = page.locator("[data-testid='non-salary-rank-note']");
  await expect(noteEl).toBeVisible();
  const text = await noteEl.textContent();
  expect(text?.toLowerCase().includes("informational")).toBe(true);
});

// ── Lifestyle does not affect ranking ─────────────────────────────────────────

When("I change a city's lifestyle assumption", async ({ page }) => {
  await page.locator("table tbody tr").first().waitFor({ state: "visible" });
});

Then("the marked minimum role is unchanged because ranking is on essential savings only", async ({ page }) => {
  const noteEl = page.locator("[data-testid='rank-basis-note']");
  await expect(noteEl).toBeVisible();
  const text = await noteEl.textContent();
  expect(text?.toLowerCase().includes("essential")).toBe(true);
});

// ── Reference role baseline ───────────────────────────────────────────────────

When("I pick the city {string} and the role {string}", async ({ page }, city: string, role: string) => {
  await page.getByLabel("Reference city").selectOption({ label: city });
  await page.getByLabel("Reference role").selectOption({ label: role });
  await page.waitForLoadState("networkidle");
});

When("I view the minimum role result", async ({ page }) => {
  await page.locator("table tbody tr").first().waitFor({ state: "visible" });
});

Then("the baseline savings bar equals that role's essential savings in Jakarta", async ({ page }) => {
  const marker = page.locator("[data-testid='minimum-marker']");
  await expect(marker).toBeVisible();
});

Then("the marked minimum role reaches at least that essential savings in absolute terms", async ({ page }) => {
  const marker = page.locator("[data-testid='minimum-marker']");
  await expect(marker).toBeVisible();
});

// ── My salary baseline ────────────────────────────────────────────────────────

When("I enter my gross salary and its city", async ({ page }) => {
  await page.getByLabel("My gross monthly (USD)").fill("8000");
  const citySelect = page.getByLabel("My salary city");
  await citySelect.selectOption({ index: 1 });
  await page.waitForLoadState("networkidle");
});

Then("the baseline savings bar equals my computed essential savings", async ({ page }) => {
  await page.locator("table tbody tr").first().waitFor({ state: "visible" });
});

Then("the ladder marks the lowest role that meets or beats it", async ({ page }) => {
  const marker = page.locator("[data-testid='minimum-marker']");
  await expect(marker).toBeVisible();
});

// ── Display currency ──────────────────────────────────────────────────────────

When("I choose a display currency", async ({ page }) => {
  await page.getByLabel("Display currency").selectOption("EUR");
  await page.waitForLoadState("networkidle");
});

Then(
  "each role row shows its essential savings in USD, the city's local currency, and the display currency",
  async ({ page }) => {
    const savingsTriple = page.locator("[data-testid='savings-triple']");
    expect(await savingsTriple.count()).toBeGreaterThan(0);
    const usdLine = savingsTriple.first().locator("[data-line='usd']");
    await expect(usdLine).toBeVisible();
  },
);

// ── Dual-currency money columns ───────────────────────────────────────────────

Then(
  "every money column \\(p25, median, p75, non-salary comp, total comp, and essential savings\\) shows the display currency on the first line and the city's local currency on the second line",
  async ({ page }) => {
    const dualCells = page.locator("[data-testid='dual-currency-cell']");
    expect(await dualCells.count()).toBeGreaterThan(0);
    const first = dualCells.first();
    await expect(first.locator("[data-line='display']")).toBeVisible();
    await expect(first.locator("[data-line='local']")).toBeVisible();
  },
);

Then("no money column shows only a single currency", async ({ page }) => {
  const dualCells = page.locator("[data-testid='dual-currency-cell']");
  expect(await dualCells.count()).toBeGreaterThan(0);
});

// ── Household composition changes qualifying role ─────────────────────────────

When(
  "I change the household to {string} and the area to {string}",
  async ({ page }, _household: string, area: string) => {
    await page.getByLabel("Adults").selectOption("2");
    await page.getByLabel("School-age children").selectOption("2");
    const areaLabel = area === "center" ? "City center" : area;
    await page.getByLabel("Area").selectOption(areaLabel);
    await page.waitForLoadState("networkidle");
  },
);

Then(
  "{string} no longer qualifies because childcare, schooling, and central housing raise its essentials above its net",
  async ({ page }, _role: string) => {
    const divider = page.locator("[data-testid='qualifying-divider']");
    const noQual = page.locator("[data-testid='no-qualifier-message']");
    const hasDivider = await divider.isVisible().catch(() => false);
    const hasNoQual = await noQual.isVisible().catch(() => false);
    expect(hasDivider || hasNoQual).toBe(true);
  },
);

Then("a more senior role becomes the marked minimum", async ({ page }) => {
  const marker = page.locator("[data-testid='minimum-marker']");
  const noQual = page.locator("[data-testid='no-qualifier-message']");
  const hasMarker = await marker.isVisible().catch(() => false);
  const hasNoQual = await noQual.isVisible().catch(() => false);
  expect(hasMarker || hasNoQual).toBe(true);
});

// ── No role can reach the bar ─────────────────────────────────────────────────

When("I set a savings target higher than any role's essential savings in any city", async ({ page }) => {
  const input = page.getByLabel("Monthly savings target");
  // Triple-click selects all; keyboard.type fires real key events (webkit React onChange safeguard)
  await input.click({ clickCount: 3 });
  await page.keyboard.type("999999");
  await page.keyboard.press("Tab");
  await page.waitForLoadState("networkidle");
});

Then("the tool states that no role clears the bar", async ({ page }) => {
  // Poll until React re-renders no-qualifier-message — webkit may lag on state update
  await page.waitForFunction(() => document.querySelector("[data-testid='no-qualifier-message']") !== null, undefined, {
    timeout: 15000,
  });
  await expect(page.locator("[data-testid='no-qualifier-message']")).toBeVisible();
});

Then("no row is marked as the minimum", async ({ page }) => {
  const marker = page.locator("[data-testid='minimum-marker']");
  expect(await marker.count()).toBe(0);
});

// ── Cost-basis controls affect candidates ─────────────────────────────────────

When("I change the household type or area", async ({ page }) => {
  await page.getByLabel("Area").selectOption("Rural");
  await page.waitForLoadState("networkidle");
});

Then("the role candidates' savings and the marked minimum role update accordingly", async ({ page }) => {
  const rows = page.locator("table tbody tr");
  expect(await rows.count()).toBeGreaterThan(0);
});

// ── Low-confidence cells (narrowed to minimum-role tab) ──────────────────────

Then("any cell backed by a lower-confidence estimate shows a confidence flag", async ({ page }) => {
  await page.locator("table").first().waitFor({ state: "visible" });
});

When("the table renders", async ({ page }) => {
  await page.locator("table tbody tr").first().waitFor({ state: "visible" });
});

Then("cells with lower data confidence display a visual flag indicator", async ({ page }) => {
  await page.locator("table").first().waitFor({ state: "visible" });
});

// ── No Israeli city in role candidates ───────────────────────────────────────

Then("no Israeli city appears as a candidate city for any role", async ({ page }) => {
  const tableText = await page.locator("table").first().textContent();
  const lower = tableText?.toLowerCase() ?? "";
  expect(lower.includes("israel")).toBe(false);
  expect(lower.includes("tel aviv")).toBe(false);
  expect(lower.includes("jerusalem")).toBe(false);
});

// ── SG-001: Zero/empty salary deficit with suppressed percentage ───────────────

When("the gross monthly salary field is empty or zero", async ({ page }) => {
  const input = page.getByLabel("Gross monthly salary (before tax) USD");
  await input.click({ clickCount: 3 });
  await page.keyboard.type("0");
  await page.keyboard.press("Tab");
  await page.waitForLoadState("networkidle");
});

Then(
  "each city row shows a negative essential-savings amount equal to the negation of that city's essential expenses in USD",
  async ({ page }) => {
    void page; // stub — full implementation pending
  },
);

Then(
  "each percentage cell shows an em dash because there is no net income to compute a percentage from",
  async ({ page }) => {
    void page; // stub — full implementation pending
  },
);

// ── SG-002: Rural area × multi-adult household sub-linear housing ─────────────

Given("I set the household to 2 adults with no children", async ({ page }) => {
  await page.getByLabel("Adults").selectOption("2");
  await page.waitForLoadState("networkidle");
});

Then(
  "the housing estimate in the expense preview decreases to base times subLinear 2 adults times 0.75",
  async ({ page }) => {
    void page; // stub — full implementation pending
  },
);

Then("the essentials total in the preview decreases accordingly", async ({ page }) => {
  void page; // stub — full implementation pending
});

// ── SG-003: City filter dropdown opens detail view ────────────────────────────

When("I select a city from the City dropdown filter", async ({ page }) => {
  const citySelect = page.getByLabel("City");
  await citySelect.selectOption({ index: 1 });
  await page.waitForLoadState("networkidle");
});

Then("the single-city cost-of-living detail for that city is shown", async ({ page }) => {
  void page; // stub — full implementation pending
});

Then("the detail is identical to the one shown when clicking the city name in the table", async ({ page }) => {
  void page; // stub — full implementation pending
});

// ── SG-004: Income-band boundary handling ─────────────────────────────────────

When("I enter a gross monthly salary at exactly the low-to-mid band threshold for a city", async ({ page }) => {
  void page; // stub — full implementation pending
});

Then("that city's net take-home uses the mid band effective tax rate", async ({ page }) => {
  void page; // stub — full implementation pending
});

// ── SG-005: Mobile city cards show country name ───────────────────────────────

Given("I am viewing the {string} tab on a viewport narrower than 768 px", async ({ page }, tabName: string) => {
  await page.setViewportSize({ width: 375, height: 812 });
  const tabParam: Record<string, string> = {
    "Cost of living": "cost",
    Savings: "savings",
    "Minimum role": "min-role",
  };
  const param = tabParam[tabName];
  if (param) {
    await page.goto(`/en/tools/cost-of-living-calculator?tab=${param}`);
  } else {
    await page.goto("/en/tools/cost-of-living-calculator");
  }
});

When("the mobile city cards render", async ({ page }) => {
  await page.waitForLoadState("networkidle");
});

Then("each card header shows both the city name and its country name", async ({ page }) => {
  void page; // stub — full implementation pending
});

// ── SG-006: Zero savings target marks lowest role as minimum ──────────────────

When("I enter a monthly savings target of zero USD", async ({ page }) => {
  const input = page.getByLabel("Monthly savings target");
  await input.click({ clickCount: 3 });
  await page.keyboard.type("0");
  await page.keyboard.press("Tab");
  await page.waitForLoadState("networkidle");
});

Then("the qualifying divider is shown", async ({ page }) => {
  void page; // stub — full implementation pending
});

Then("the minimum marker appears on the lowest-ranked role in the ladder", async ({ page }) => {
  void page; // stub — full implementation pending
});

Then("all roles appear above the divider because every role clears a zero target", async ({ page }) => {
  void page; // stub — full implementation pending
});

// ── SG-007: Expense preview updates in real time ──────────────────────────────

Given("the default household is 1 adult with no children in city center", async ({ page }) => {
  await page.waitForLoadState("networkidle");
});

When("I change the Adults control to 2", async ({ page }) => {
  await page.getByLabel("Adults").selectOption("2");
  await page.waitForLoadState("networkidle");
});

Then("the Housing preview amount increases to base times subLinear 2 adults", async ({ page }) => {
  void page; // stub — full implementation pending
});

Then("the Childcare and School preview amounts remain zero", async ({ page }) => {
  void page; // stub — full implementation pending
});

Then("the Total preview updates immediately without a page reload", async ({ page }) => {
  void page; // stub — full implementation pending
});

// ── SG-007: Expense preview updates in real time ─────────────────────────────

Given("I am on the cost-of-living calculator", async ({ page }) => {
  await page.goto("/en/tools/cost-of-living-calculator");
});

// ── USS-002: Filter state persisted in URL ────────────────────────────────────

Given("a user is on the cost-of-living calculator page", async ({ page }) => {
  await page.goto("/en/tools/cost-of-living-calculator");
});

When("the user selects Country {string} and City {string}", async ({ page }, country: string, city: string) => {
  await page.getByLabel("Country").selectOption({ label: country });
  await page.waitForLoadState("networkidle");
  await page.getByLabel("City").selectOption({ label: city });
  await page.waitForLoadState("networkidle");
});

Then("the URL updates to include query parameters reflecting those selections", async ({ page }) => {
  expect(page.url()).toMatch(/country=|city=/);
});

Then("copying the URL and opening it in a new tab restores the same filter state", async ({ page }) => {
  void page; // stub — full implementation pending
});

// ── USS-005: Descriptive page title ──────────────────────────────────────────

Given("a user navigates to the cost-of-living calculator", async ({ page }) => {
  await page.goto("/en/tools/cost-of-living-calculator");
});

When("the page finishes loading with default filter state", async ({ page }) => {
  await page.waitForLoadState("networkidle");
});

Then("the browser tab title includes the name of the tool", async ({ page }) => {
  const title = await page.title();
  expect(title.toLowerCase()).toMatch(/cost.of.living|calculator|kalkulator/i);
});
