import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { dataset } from "../core/data/cities";
import { CityDetail } from "./city-detail";

afterEach(cleanup);

// Gherkin (binds): "Relocation reserve is shown separately from sunk costs"
describe("CityDetail", () => {
  const firstCity = dataset.cities[0]!;
  const defaultProps = {
    dataset,
    cityId: firstCity.id,
    household: { adults: 1 as const, preschoolKids: 0 as const, schoolKids: 0 as const },
    schoolType: "public" as const,
    area: "center" as const,
  };

  it("shows relocation sunk-cost total distinct from monthly total", () => {
    render(<CityDetail {...defaultProps} />);

    // Monthly total
    expect(screen.getByTestId("monthly-total")).toBeTruthy();

    // Relocation sunk-cost total — distinct section
    expect(screen.getByTestId("relocation-sunk")).toBeTruthy();

    // They should be different elements (not folded together)
    const monthly = screen.getByTestId("monthly-total");
    const sunk = screen.getByTestId("relocation-sunk");
    expect(monthly).not.toBe(sunk);
  });

  it("shows liquidity-reserve cash cushion in its own labelled figure", () => {
    render(<CityDetail {...defaultProps} />);

    const liquidity = screen.getByTestId("liquidity-reserve");
    expect(liquidity).toBeTruthy();

    // Must NOT be folded into relocation sunk
    const sunk = screen.getByTestId("relocation-sunk");
    expect(liquidity).not.toBe(sunk);

    // Should have non-zero value for any city
    const text = liquidity.textContent ?? "";
    expect(text.length).toBeGreaterThan(0);
  });

  it("shows per-category expense breakdown", () => {
    render(<CityDetail {...defaultProps} />);

    expect(screen.getByTestId("expense-housing")).toBeTruthy();
    expect(screen.getByTestId("expense-food")).toBeTruthy();
    expect(screen.getByTestId("expense-transport")).toBeTruthy();
    expect(screen.getByTestId("expense-utilities")).toBeTruthy();
    expect(screen.getByTestId("expense-healthcare")).toBeTruthy();
  });

  it("shows healthcare funding-scheme badge", () => {
    render(<CityDetail {...defaultProps} />);

    const badge = screen.getByTestId("healthcare-badge");
    const validTexts = ["tax-funded", "mandatory payroll insurance", "out-of-pocket"];
    expect(validTexts).toContain(badge.textContent?.trim());
  });
});
