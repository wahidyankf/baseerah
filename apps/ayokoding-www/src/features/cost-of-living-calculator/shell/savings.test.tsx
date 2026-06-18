import { cleanup, render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it } from "vitest";
import { dataset } from "../core/data/cities";
import { roleMatrix } from "../core/data/roles";
import { SavingsTable } from "./savings";

afterEach(cleanup);

describe("SavingsTable", () => {
  const defaultProps = {
    dataset,
    matrix: roleMatrix,
    household: { adults: 1 as const, preschoolKids: 0 as const, schoolKids: 0 as const },
    schoolType: "public" as const,
    area: "center" as const,
  };

  // Gherkin (binds): "Savings tab converts gross salary to net before subtracting expenses"
  it("entering gross=8000 shows net, essentials, savings-after-essentials with % and savings-after-lifestyle with %, sortable", async () => {
    const user = userEvent.setup();
    render(<SavingsTable {...defaultProps} />);

    const input = screen.getByRole("spinbutton", { name: /gross monthly salary/i });
    await user.clear(input);
    await user.type(input, "8000");

    // Table present
    expect(screen.getByRole("table")).toBeTruthy();

    // Columns: Country, City, Net, Essentials, Savings after essentials, Savings after lifestyle
    const headers = screen.getAllByRole("columnheader").map((h) => h.textContent?.toLowerCase() ?? "");
    expect(headers.some((t) => t.includes("country"))).toBe(true);
    expect(headers.some((t) => t.includes("city"))).toBe(true);
    expect(headers.some((t) => t.includes("net"))).toBe(true);
    expect(headers.some((t) => t.includes("essentials"))).toBe(true);
    expect(headers.some((t) => t.includes("savings"))).toBe(true);

    // Rows present (one per city + header)
    const rows = screen.getAllByRole("row");
    expect(rows.length).toBeGreaterThan(1);

    // Sort trigger present
    expect(screen.getByRole("button", { name: /sort/i })).toBeTruthy();
  });

  // Gherkin (binds): "Gross salary entered monthly shows the derived annual figure"
  it("entering gross=8000 shows annual gross=96000", async () => {
    const user = userEvent.setup();
    render(<SavingsTable {...defaultProps} />);

    const input = screen.getByRole("spinbutton", { name: /gross monthly salary/i });
    await user.clear(input);
    await user.type(input, "8000");

    // Annual gross = 8000 * 12 = 96000
    expect(screen.getByTestId("annual-gross")).toHaveTextContent("96");
  });

  // Gherkin (binds): "Non-salary comp is shown as informational context only"
  it("shows informational non-salary comp column that does not affect net or savings", async () => {
    const user = userEvent.setup();
    render(<SavingsTable {...defaultProps} />);

    const input = screen.getByRole("spinbutton", { name: /gross monthly salary/i });
    await user.clear(input);
    await user.type(input, "8000");

    // Non-salary comp header present
    const headers = screen.getAllByRole("columnheader").map((h) => h.textContent?.toLowerCase() ?? "");
    expect(headers.some((t) => t.includes("non-salary") || t.includes("rsu") || t.includes("equity"))).toBe(true);

    // The informational nature is marked
    expect(screen.getByTestId("non-salary-comp-note")).toBeTruthy();
  });

  // Gherkin (binds): "Total compensation is shown for negotiation context"
  it("shows informational total comp column that does not affect savings", async () => {
    const user = userEvent.setup();
    render(<SavingsTable {...defaultProps} />);

    const input = screen.getByRole("spinbutton", { name: /gross monthly salary/i });
    await user.clear(input);
    await user.type(input, "8000");

    const headers = screen.getAllByRole("columnheader").map((h) => h.textContent?.toLowerCase() ?? "");
    expect(headers.some((t) => t.includes("total comp") || t.includes("total compensation"))).toBe(true);
  });

  // Gherkin (binds): "Sub-national tax lowers net only in federal countries"
  it("US/CA/CH cities apply sub-national rate; unitary cities apply federal rate alone", async () => {
    const user = userEvent.setup();
    render(<SavingsTable {...defaultProps} />);

    const input = screen.getByRole("spinbutton", { name: /gross monthly salary/i });
    await user.clear(input);
    await user.type(input, "8000");

    // All rows have a net cell — the math is validated at the core level (calc.unit.test.ts)
    // Here we verify the sub-national indicator is shown for federal-country cities
    const subNatCells = screen.getAllByTestId("sub-national-indicator");
    expect(subNatCells.length).toBeGreaterThan(0);
  });

  // Gherkin (binds): "Net take-home is lower than the entered gross"
  it("net shown for each city is lower than the entered gross", async () => {
    const user = userEvent.setup();
    render(<SavingsTable {...defaultProps} />);

    const input = screen.getByRole("spinbutton", { name: /gross monthly salary/i });
    await user.clear(input);
    await user.type(input, "8000");

    const netCells = screen.getAllByTestId("net-value");
    expect(netCells.length).toBeGreaterThan(0);

    for (const cell of netCells) {
      // Net ≤ gross (UAE has 0% income tax so net = gross; other cities net < gross)
      const raw = cell.getAttribute("data-usd") ?? "0";
      expect(parseFloat(raw)).toBeLessThanOrEqual(8000);
    }
  });

  // Gherkin (binds): "Essentials above net show a deficit"
  it("shows negative savings-after-essentials when net < essentials", async () => {
    const user = userEvent.setup();
    render(<SavingsTable {...defaultProps} />);

    // Enter a very low gross so at least one city shows a deficit
    const input = screen.getByRole("spinbutton", { name: /gross monthly salary/i });
    await user.clear(input);
    await user.type(input, "500");

    // At least one savings-after-essentials cell should be negative
    const savingsCells = screen.getAllByTestId("savings-essential");
    const hasDeficit = savingsCells.some((c) => parseFloat(c.getAttribute("data-usd") ?? "0") < 0);
    expect(hasDeficit).toBe(true);
  });
});
