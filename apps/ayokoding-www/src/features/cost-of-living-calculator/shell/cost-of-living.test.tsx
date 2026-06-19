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
});
