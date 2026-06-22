import { cleanup, render, screen, within } from "@testing-library/react";
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
  // UWT-010: Area label must not wrap (Indonesian locale has longer text)
  it("UWT-010: the Area label element has whitespace-nowrap class", () => {
    const { container } = render(<ControlsWithState />);

    // Find the span/label for "Area"
    const areaLabel = Array.from(container.querySelectorAll("span")).find((el) =>
      el.textContent?.match(/area|wilayah/i),
    );
    expect(areaLabel).toBeDefined();
    expect(areaLabel!.classList.contains("whitespace-nowrap")).toBe(true);
  });

  // UWT-009: interactive controls must have 44px minimum touch target
  it("UWT-009: interactive select controls have min-h-[44px] class or data-min-touch attribute", () => {
    const { container } = render(<ControlsWithState />);

    const selects = container.querySelectorAll("select");
    expect(selects.length).toBeGreaterThan(0);

    for (const select of Array.from(selects)) {
      const hasMinHeight = select.classList.contains("min-h-[44px]");
      const hasDataAttr = select.getAttribute("data-min-touch") === "true";
      const wrapper = select.closest("[data-min-touch='true']") ?? select.closest(".min-h-\\[44px\\]");
      expect(hasMinHeight || hasDataAttr || wrapper !== null).toBe(true);
    }
  });

  // Phase 9 Cluster J — id Area label must be short enough not to wrap at 375px
  it("Phase9J: id locale Area label text is no longer than 10 characters", () => {
    const { container } = render(
      <Controls
        dataset={dataset}
        previewCityId={firstCity.id}
        household={{ adults: 1, preschoolKids: 0, schoolKids: 0 }}
        schoolType="public"
        area="center"
        locale="id"
        onHouseholdChange={() => {}}
        onSchoolTypeChange={() => {}}
        onAreaChange={() => {}}
      />,
    );
    const areaLabel = Array.from(container.querySelectorAll("span")).find((el) =>
      el.className.includes("whitespace-nowrap"),
    );
    expect(areaLabel).toBeTruthy();
    // "Wilayah tempat tinggal" = 22 chars; must be shortened
    expect((areaLabel!.textContent ?? "").length).toBeLessThanOrEqual(10);
  });

  // Phase 9 Cluster I — each label+select pair must not mid-pair wrap at 320px
  it("Phase9I: each label+select pair has its own flex wrapper (not siblings of other pairs)", () => {
    const { container } = render(<ControlsWithState />);
    const adultsLabel = container.querySelector('label[for="controls-adults"]');
    const adultsSelect = container.querySelector("#controls-adults");
    expect(adultsLabel).toBeTruthy();
    expect(adultsSelect).toBeTruthy();
    // Each pair must be isolated in its own wrapper (parent has exactly 2 children)
    expect(adultsLabel!.parentElement!.children.length).toBe(2);
  });

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

  // Gherkin (binds): "School type toggle is shown but disabled without school-age children"
  it("school-type toggle always shown — disabled without school-age children, enabled when > 0", async () => {
    const user = userEvent.setup();
    render(<ControlsWithState />);

    // Always present, but disabled at 0 school-age kids
    const group = screen.getByRole("radiogroup", { name: /school type/i });
    expect(group).toBeTruthy();
    expect(group.getAttribute("aria-disabled")).toBe("true");
    expect(
      within(group)
        .getAllByRole("radio")
        .every((b) => (b as HTMLButtonElement).disabled),
    ).toBe(true);
    // A hint explains why it is disabled
    expect(screen.getByText(/add school-age children to choose/i)).toBeTruthy();

    // Add 1 school-age child → toggle becomes enabled and the hint disappears
    await user.selectOptions(screen.getByRole("combobox", { name: /school-age children/i }), "1");

    const enabled = screen.getByRole("radiogroup", { name: /school type/i });
    expect(enabled.getAttribute("aria-disabled")).toBeNull();
    expect(
      within(enabled)
        .getAllByRole("radio")
        .every((b) => !(b as HTMLButtonElement).disabled),
    ).toBe(true);
    expect(screen.queryByText(/add school-age children to choose/i)).toBeNull();
  });

  // Gherkin (binds): "Private school raises expenses more than public"
  it("switching to private school increases schooling portion", async () => {
    const user = userEvent.setup();
    render(<ControlsWithState schoolKids={2} />);

    const schoolingPublic = parseFloat(screen.getByTestId("preview-schooling").getAttribute("data-local") ?? "0");

    await user.click(screen.getByRole("radio", { name: /private/i }));

    const schoolingPrivate = parseFloat(screen.getByTestId("preview-schooling").getAttribute("data-local") ?? "0");

    expect(schoolingPrivate).toBeGreaterThan(schoolingPublic);
  });

  // Gherkin (binds): "Rural area lowers housing versus city center"
  it("switching to rural reduces modeled housing and city total", async () => {
    const user = userEvent.setup();
    render(<ControlsWithState />);

    const housingCenter = parseFloat(screen.getByTestId("preview-housing").getAttribute("data-local") ?? "0");
    const totalCenter = parseFloat(screen.getByTestId("preview-total").getAttribute("data-local") ?? "0");

    await user.click(screen.getByRole("radio", { name: /rural/i }));

    const housingRural = parseFloat(screen.getByTestId("preview-housing").getAttribute("data-local") ?? "0");
    const totalRural = parseFloat(screen.getByTestId("preview-total").getAttribute("data-local") ?? "0");

    expect(housingRural).toBeLessThan(housingCenter);
    expect(totalRural).toBeLessThan(totalCenter);
  });
});
