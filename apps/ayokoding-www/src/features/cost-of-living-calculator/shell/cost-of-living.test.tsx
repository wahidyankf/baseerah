import { cleanup, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { dataset } from "../core/data/cities";
import { CostOfLivingTable } from "./cost-of-living";

afterEach(cleanup);

// Gherkin (binds): "Cost-of-living breakdown lists category expenses per city"
describe("CostOfLivingTable", () => {
  const defaultProps = {
    dataset,
    household: { adults: 1 as const, preschoolKids: 0 as const, schoolKids: 0 as const },
    schoolType: "public" as const,
    area: "center" as const,
  };

  it("renders a table of tech-hub cities with Country column left of City column", () => {
    render(<CostOfLivingTable {...defaultProps} />);

    // Table present
    expect(screen.getByRole("table")).toBeTruthy();

    // Headers present — Country before City
    const columnHeaders = screen.getAllByRole("columnheader");
    const headerTexts = columnHeaders.map((h) => h.textContent ?? "");
    const countryIdx = headerTexts.findIndex((t) => /country/i.test(t));
    const cityIdx = headerTexts.findIndex((t) => /city/i.test(t));
    expect(countryIdx).toBeGreaterThanOrEqual(0);
    expect(cityIdx).toBeGreaterThan(countryIdx);
  });

  it("each row shows all 7 expense categories plus school, essentials subtotal, total, relocation sunk-cost and liquidity reserve", () => {
    render(<CostOfLivingTable {...defaultProps} />);

    const columnHeaders = screen.getAllByRole("columnheader");
    const headerTexts = columnHeaders.map((h) => h.textContent?.toLowerCase() ?? "");

    // 7 expense categories
    expect(headerTexts.some((t) => t.includes("housing"))).toBe(true);
    expect(headerTexts.some((t) => t.includes("food"))).toBe(true);
    expect(headerTexts.some((t) => t.includes("transport"))).toBe(true);
    expect(headerTexts.some((t) => t.includes("utilities"))).toBe(true);
    expect(headerTexts.some((t) => t.includes("healthcare"))).toBe(true);
    expect(headerTexts.some((t) => t.includes("childcare"))).toBe(true);
    expect(headerTexts.some((t) => t.includes("school"))).toBe(true);

    // Essentials subtotal
    expect(headerTexts.some((t) => t.includes("essentials"))).toBe(true);

    // Total (monthly)
    expect(headerTexts.some((t) => t.includes("total"))).toBe(true);

    // Separate one-time relocation sunk-cost total
    expect(headerTexts.some((t) => t.includes("relocation") || t.includes("sunk"))).toBe(true);

    // Separately labelled liquidity reserve
    expect(headerTexts.some((t) => t.includes("liquidity") || t.includes("reserve"))).toBe(true);
  });

  it("renders a row for each city in the dataset", () => {
    render(<CostOfLivingTable {...defaultProps} />);

    const rows = screen.getAllByRole("row");
    // rows = 1 header row + N city rows
    expect(rows.length).toBe(dataset.cities.length + 1);
  });

  // Gherkin (binds): "Country and city are always shown together on every tab"
  it("every data row shows Country cell immediately to the left of City cell", () => {
    render(<CostOfLivingTable {...defaultProps} />);

    const rows = screen.getAllByRole("row").slice(1); // skip header
    expect(rows.length).toBeGreaterThan(0);

    for (const city of dataset.cities.slice(0, 5)) {
      const country = dataset.countries.find((c) => c.id === city.countryId)!;
      const cityRow = rows.find((r) => r.textContent?.includes(city.name.en));
      expect(cityRow).toBeTruthy();
      const cells = within(cityRow!).getAllByRole("cell");
      // Country in cell[0], City in cell[1]
      expect(cells[0]!.textContent).toContain(country.name.en);
      expect(cells[1]!.textContent).toContain(city.name.en);
    }
  });

  // Gherkin (binds): "Healthcare funding scheme is always shown"
  it("each row shows a healthcare funding-scheme badge", () => {
    render(<CostOfLivingTable {...defaultProps} />);

    const badges = screen.getAllByTestId("healthcare-badge");
    expect(badges.length).toBe(dataset.cities.length);

    const validTexts = ["tax-funded", "mandatory payroll insurance", "out-of-pocket"];
    for (const badge of badges) {
      expect(validTexts).toContain(badge.textContent?.trim());
    }
  });

  // Gherkin (binds): "Clicking a city name opens its single-city cost-of-living detail"
  it("each city name is a link to the single-city detail", () => {
    render(<CostOfLivingTable {...defaultProps} />);

    for (const city of dataset.cities.slice(0, 5)) {
      // Some cities share a name with their country (e.g. Singapore) — filter by href
      const links = screen.getAllByRole("link", { name: city.name.en });
      const cityLink = links.find((l) => l.getAttribute("href") === `?tab=cost&city=${city.id}`);
      expect(cityLink).toBeDefined();
      expect(cityLink).toHaveAttribute("href", `?tab=cost&city=${city.id}`);
    }
  });

  // Gherkin (binds): "Clicking a country opens Cost-of-living filtered to that country"
  it("each country name is a link to the country-filtered view", () => {
    render(<CostOfLivingTable {...defaultProps} />);

    const countriesInDataset = dataset.countries.filter((c) => dataset.cities.some((city) => city.countryId === c.id));

    for (const country of countriesInDataset.slice(0, 5)) {
      const links = screen.getAllByRole("link", { name: country.name.en });
      expect(links.length).toBeGreaterThan(0);
      expect(links[0]).toHaveAttribute("href", `?tab=cost&country=${country.id}`);
    }
  });

  // Responsive parity: mobile stacked-card view exists alongside the table (toggled by CSS),
  // one card per city, each card a city link. (Desktop/tablet use the table.)
  it("renders a mobile city-card view with one card per city", () => {
    render(<CostOfLivingTable {...defaultProps} />);
    const cards = screen.getByTestId("mobile-city-cards");
    expect(cards).toBeTruthy();
    const cityLinks = cards.querySelectorAll('a[href^="?tab=cost&city="]');
    expect(cityLinks.length).toBe(dataset.cities.length);
  });

  // UWT-005: definition tooltips on relocation column headers
  it("UWT-005: Relocation (sunk) column header has a tooltip explaining it is a one-time cost", () => {
    render(<CostOfLivingTable {...defaultProps} />);

    const columnHeaders = screen.getAllByRole("columnheader");
    const relocationHeader = columnHeaders.find((h) => /relocation/i.test(h.textContent ?? ""));
    expect(relocationHeader).toBeDefined();
    // Tooltip: either title attribute, aria-label on abbr, or data-tooltip
    const title = relocationHeader!.querySelector("[title]") ?? relocationHeader!.closest("[title]");
    const abbr = relocationHeader!.querySelector("abbr");
    const hasTooltip = title !== null || (abbr !== null && abbr.hasAttribute("title"));
    expect(hasTooltip).toBe(true);
  });

  it("UWT-005: Liquidity reserve column header has a tooltip explaining it is a cash cushion kept not spent", () => {
    render(<CostOfLivingTable {...defaultProps} />);

    const columnHeaders = screen.getAllByRole("columnheader");
    const liquidityHeader = columnHeaders.find((h) => /liquidity/i.test(h.textContent ?? ""));
    expect(liquidityHeader).toBeDefined();
    const title = liquidityHeader!.querySelector("[title]") ?? liquidityHeader!.closest("[title]");
    const abbr = liquidityHeader!.querySelector("abbr");
    const hasTooltip = title !== null || (abbr !== null && abbr.hasAttribute("title"));
    expect(hasTooltip).toBe(true);
  });

  // Phase 5 — Cycle 1c: Right-edge scroll affordance indicator
  it("Phase5-1c: a scroll affordance element with data-testid='scroll-affordance' is rendered", () => {
    render(<CostOfLivingTable {...defaultProps} />);
    const affordance = screen.getByTestId("scroll-affordance");
    expect(affordance).toBeTruthy();
  });

  // Phase 5 — Cycle 1a: Summary columns (Total, Essentials) appear immediately after City
  it("Phase5-1a: Total column header appears before Housing column header", () => {
    render(<CostOfLivingTable {...defaultProps} />);

    const columnHeaders = screen.getAllByRole("columnheader");
    const headerTexts = columnHeaders.map((h) => h.textContent?.toLowerCase() ?? "");

    const totalIdx = headerTexts.findIndex((t) => /^total$/i.test(t.trim()));
    const housingIdx = headerTexts.findIndex((t) => /housing/i.test(t));

    expect(totalIdx).toBeGreaterThanOrEqual(0);
    expect(housingIdx).toBeGreaterThanOrEqual(0);
    // Total must come BEFORE Housing (summary-first ordering)
    expect(totalIdx).toBeLessThan(housingIdx);
  });

  // Phase 5 — Cycle 1b: Total/Essentials in DOM and table wrapper has overflow-x-auto
  it("Phase5-1b: Total and Essentials column headers are in the DOM and table wrapper has overflow-x-auto class", () => {
    const { container } = render(<CostOfLivingTable {...defaultProps} />);

    // Both summary columns must be present in the DOM
    const columnHeaders = screen.getAllByRole("columnheader");
    const headerTexts = columnHeaders.map((h) => h.textContent?.toLowerCase() ?? "");
    expect(headerTexts.some((t) => /^total$/i.test(t.trim()))).toBe(true);
    expect(headerTexts.some((t) => /essentials/i.test(t))).toBe(true);

    // Table wrapper must have overflow-x-auto for horizontal scrollability
    const tableWrapper = container.querySelector(".overflow-x-auto");
    expect(tableWrapper).not.toBeNull();
  });

  // Phase 5 — Cycle 1a: Essentials also appears before Housing
  it("Phase5-1a: Essentials column header appears before Housing column header", () => {
    render(<CostOfLivingTable {...defaultProps} />);

    const columnHeaders = screen.getAllByRole("columnheader");
    const headerTexts = columnHeaders.map((h) => h.textContent?.toLowerCase() ?? "");

    const essentialsIdx = headerTexts.findIndex((t) => /essentials/i.test(t));
    const housingIdx = headerTexts.findIndex((t) => /housing/i.test(t));

    expect(essentialsIdx).toBeGreaterThanOrEqual(0);
    expect(housingIdx).toBeGreaterThanOrEqual(0);
    // Essentials must come BEFORE Housing (summary-first ordering)
    expect(essentialsIdx).toBeLessThan(housingIdx);
  });

  // UWT-014: "OOP" must be wrapped in an <abbr> with title="out-of-pocket"
  it("UWT-014: the text 'OOP' is inside an abbr element with title='out-of-pocket'", () => {
    const { container } = render(<CostOfLivingTable {...defaultProps} />);

    const abbrElements = Array.from(container.querySelectorAll("abbr"));
    const oopAbbr = abbrElements.find(
      (el) => el.textContent?.trim() === "OOP" && el.getAttribute("title") === "out-of-pocket",
    );
    expect(oopAbbr).toBeDefined();
  });

  // UWT-011: healthcare scheme badge should be sentence-cased (not ALL-CAPS)
  it("UWT-011: healthcare scheme badge text is sentence-cased, not all-caps", () => {
    render(<CostOfLivingTable {...defaultProps} />);

    const badges = screen.getAllByTestId("healthcare-badge");
    expect(badges.length).toBeGreaterThan(0);

    for (const badge of badges) {
      const text = badge.textContent?.trim() ?? "";
      if (text === "—") continue;
      // Must NOT be all-caps (i.e., text !== text.toUpperCase())
      expect(text).not.toBe(text.toUpperCase());
    }
  });

  // UWT-011: healthcare scheme column header should have a tooltip
  it("UWT-011: Healthcare scheme column header has a tooltip (title attribute)", () => {
    render(<CostOfLivingTable {...defaultProps} />);

    const columnHeaders = screen.getAllByRole("columnheader");
    const healthcareSchemeHeader = columnHeaders.find((h) =>
      /healthcare scheme|skema kesehatan/i.test(h.textContent ?? ""),
    );
    expect(healthcareSchemeHeader).toBeDefined();

    const abbr = healthcareSchemeHeader!.querySelector("abbr");
    const titleEl = healthcareSchemeHeader!.querySelector("[title]");
    const hasTooltip = abbr !== null || titleEl !== null || healthcareSchemeHeader!.hasAttribute("title");
    expect(hasTooltip).toBe(true);
  });

  // EWT-006: per-category column values must scale for household size so their sum
  // equals the Essentials subtotal shown in the same row.
  it("EWT-006: for a 2-adult household, per-category column amounts sum to the Essentials subtotal for each city row", () => {
    const twoAdultProps = {
      ...defaultProps,
      household: { adults: 2 as const, preschoolKids: 0 as const, schoolKids: 0 as const },
    };
    render(<CostOfLivingTable {...twoAdultProps} />);

    // Read all city rows via data-testid on individual cells
    for (const city of dataset.cities) {
      const housingCell = screen.getByTestId(`col-housing-${city.id}`);
      const foodCell = screen.getByTestId(`col-food-${city.id}`);
      const transportCell = screen.getByTestId(`col-transport-${city.id}`);
      const utilitiesCell = screen.getByTestId(`col-utilities-${city.id}`);
      const healthcareCell = screen.getByTestId(`col-healthcare-${city.id}`);
      const childcareCell = screen.getByTestId(`col-childcare-${city.id}`);
      const schoolCell = screen.getByTestId(`col-school-${city.id}`);
      const essentialsCell = screen.getByTestId(`col-essentials-${city.id}`);

      const housing = parseFloat(housingCell.getAttribute("data-raw") ?? "NaN");
      const food = parseFloat(foodCell.getAttribute("data-raw") ?? "NaN");
      const transport = parseFloat(transportCell.getAttribute("data-raw") ?? "NaN");
      const utilities = parseFloat(utilitiesCell.getAttribute("data-raw") ?? "NaN");
      const healthcare = parseFloat(healthcareCell.getAttribute("data-raw") ?? "NaN");
      const childcare = parseFloat(childcareCell.getAttribute("data-raw") ?? "NaN");
      const school = parseFloat(schoolCell.getAttribute("data-raw") ?? "NaN");
      const essentials = parseFloat(essentialsCell.getAttribute("data-raw") ?? "NaN");

      const categorySum = housing + food + transport + utilities + healthcare + childcare + school;
      expect(Math.abs(categorySum - essentials)).toBeLessThan(0.01);
    }
  });
});
