import { cleanup, render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { useState } from "react";
import { afterEach, describe, expect, it } from "vitest";
import { dataset } from "../core/data/cities";
import type { Household } from "../core/data/cities";
import type { Area, SchoolType } from "../core/calc";
import { Controls } from "./controls";

afterEach(cleanup);

const firstCity = dataset.cities[0]!;

function ControlsWithState(
  overrides: Partial<{
    adults: 1 | 2;
    preschoolKids: 0 | 1 | 2 | 3;
    schoolKids: 0 | 1 | 2 | 3;
    schoolType: SchoolType;
    area: Area;
  }>,
) {
  const [household, setHousehold] = useState<Household>({
    adults: overrides.adults ?? 1,
    preschoolKids: overrides.preschoolKids ?? 0,
    schoolKids: overrides.schoolKids ?? 0,
  });
  const [schoolType, setSchoolType] = useState<SchoolType>(overrides.schoolType ?? "public");
  const [area, setArea] = useState<Area>(overrides.area ?? "center");

  return (
    <Controls
      dataset={dataset}
      previewCityId={firstCity.id}
      household={household}
      schoolType={schoolType}
      area={area}
      onHouseholdChange={setHousehold}
      onSchoolTypeChange={setSchoolType}
      onAreaChange={setArea}
    />
  );
}

describe("Controls", () => {
  // Gherkin (binds): "Adding adults and children changes the modeled expenses"
  it("changing from single to married+2-school-kids increases housing sub-linearly and adds schooling", async () => {
    const user = userEvent.setup();
    render(<ControlsWithState />);

    // Capture single baseline
    const housingBefore = parseFloat(screen.getByTestId("preview-housing").getAttribute("data-local") ?? "0");
    const foodBefore = parseFloat(screen.getByTestId("preview-food").getAttribute("data-local") ?? "0");
    const schoolingBefore = parseFloat(screen.getByTestId("preview-schooling").getAttribute("data-local") ?? "0");

    // Change to 2 adults
    await user.selectOptions(screen.getByRole("combobox", { name: /adults/i }), "2");
    // Change to 2 school-age kids
    await user.selectOptions(screen.getByRole("combobox", { name: /school-age children/i }), "2");

    const housingAfter = parseFloat(screen.getByTestId("preview-housing").getAttribute("data-local") ?? "0");
    const foodAfter = parseFloat(screen.getByTestId("preview-food").getAttribute("data-local") ?? "0");
    const schoolingAfter = parseFloat(screen.getByTestId("preview-schooling").getAttribute("data-local") ?? "0");

    // Housing increases but sub-linearly (less than 3× for 3 people vs 1)
    expect(housingAfter).toBeGreaterThan(housingBefore);
    expect(housingAfter).toBeLessThan(housingBefore * 3);

    // Food increases near per-capita (roughly 3× for 4 people → we just check increase)
    expect(foodAfter).toBeGreaterThan(foodBefore);

    // Schooling added for 2 school-age children
    expect(schoolingAfter).toBeGreaterThan(schoolingBefore);
  });

  // Gherkin (binds): "Pre-school children incur childcare, not schooling"
  it("1 preschool child adds childcare but no schooling", async () => {
    const user = userEvent.setup();
    render(<ControlsWithState />);

    const childcareBefore = parseFloat(screen.getByTestId("preview-childcare").getAttribute("data-local") ?? "0");
    const schoolingBefore = parseFloat(screen.getByTestId("preview-schooling").getAttribute("data-local") ?? "0");

    await user.selectOptions(screen.getByRole("combobox", { name: /preschool children/i }), "1");

    const childcareAfter = parseFloat(screen.getByTestId("preview-childcare").getAttribute("data-local") ?? "0");
    const schoolingAfter = parseFloat(screen.getByTestId("preview-schooling").getAttribute("data-local") ?? "0");

    expect(childcareAfter).toBeGreaterThan(childcareBefore);
    expect(schoolingAfter).toBe(schoolingBefore); // no schooling for preschool
  });

  // Gherkin (binds): "School type toggle is hidden without school-age children"
  it("school-type toggle hidden when no school-age children, shown when > 0", async () => {
    const user = userEvent.setup();
    render(<ControlsWithState />);

    // Initially hidden (0 school-age kids)
    expect(screen.queryByRole("combobox", { name: /school type/i })).toBeNull();

    // Add 1 school-age child
    await user.selectOptions(screen.getByRole("combobox", { name: /school-age children/i }), "1");

    // Now visible
    expect(screen.getByRole("combobox", { name: /school type/i })).toBeTruthy();
  });

  // Gherkin (binds): "Private school raises expenses more than public"
  it("switching to private school increases schooling portion", async () => {
    const user = userEvent.setup();
    render(<ControlsWithState schoolKids={2} />);

    const schoolingPublic = parseFloat(screen.getByTestId("preview-schooling").getAttribute("data-local") ?? "0");

    await user.selectOptions(screen.getByRole("combobox", { name: /school type/i }), "private");

    const schoolingPrivate = parseFloat(screen.getByTestId("preview-schooling").getAttribute("data-local") ?? "0");

    expect(schoolingPrivate).toBeGreaterThan(schoolingPublic);
  });

  // Gherkin (binds): "Rural area lowers housing versus city center"
  it("switching to rural reduces modeled housing and city total", async () => {
    const user = userEvent.setup();
    render(<ControlsWithState />);

    const housingCenter = parseFloat(screen.getByTestId("preview-housing").getAttribute("data-local") ?? "0");
    const totalCenter = parseFloat(screen.getByTestId("preview-total").getAttribute("data-local") ?? "0");

    await user.selectOptions(screen.getByRole("combobox", { name: /area/i }), "rural");

    const housingRural = parseFloat(screen.getByTestId("preview-housing").getAttribute("data-local") ?? "0");
    const totalRural = parseFloat(screen.getByTestId("preview-total").getAttribute("data-local") ?? "0");

    expect(housingRural).toBeLessThan(housingCenter);
    expect(totalRural).toBeLessThan(totalCenter);
  });
});
