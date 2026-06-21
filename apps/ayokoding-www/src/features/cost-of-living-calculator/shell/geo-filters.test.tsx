import { cleanup, render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it, vi } from "vitest";
import { useState } from "react";
import { dataset } from "../core/data/cities";
import { countriesForRegion, citiesForCountry } from "../core/geo-filter";
import { GeoFilters } from "./geo-filters";
import type { GeoScope } from "./geo-filters";
import { CostOfLivingTable } from "./cost-of-living";

afterEach(cleanup);

type Region = "asean" | "japan" | "europe" | "nordics" | "americas" | "mena" | "asia" | "oceania" | "africa";

// Stateful wrapper for testing the controlled GeoFilters component.
// Mirrors onScopeChange back to the controlled props so the component reflects changes.
function ControlledGeoFilters({
  onScopeChange,
  initialRegion = null,
  initialCountryId = null,
  initialCityId = null,
  locale,
}: {
  onScopeChange?: (scope: GeoScope) => void;
  initialRegion?: Region | null;
  initialCountryId?: string | null;
  initialCityId?: string | null;
  locale?: "en" | "id";
}) {
  const [scope, setScope] = useState<GeoScope>({
    region: initialRegion,
    countryId: initialCountryId,
    cityId: initialCityId,
  });

  function handleScopeChange(newScope: GeoScope) {
    setScope(newScope);
    onScopeChange?.(newScope);
  }

  return (
    <GeoFilters
      dataset={dataset}
      locale={locale}
      region={scope.region}
      countryId={scope.countryId}
      cityId={scope.cityId}
      onScopeChange={handleScopeChange}
    />
  );
}

// Gherkin (binds): "Region narrows the country filter and country narrows the city filter"
describe("GeoFilters", () => {
  it("selecting a region narrows country options to that region", async () => {
    const user = userEvent.setup();
    const onScopeChange = vi.fn();

    render(<ControlledGeoFilters onScopeChange={onScopeChange} />);

    const aseanCountries = countriesForRegion(dataset, "asean");

    const regionSelect = screen.getByRole("combobox", { name: /region/i });
    await user.selectOptions(regionSelect, "asean");

    const countrySelect = screen.getByRole("combobox", { name: /country/i });
    const countryOptions = within(countrySelect).getAllByRole("option");
    // +1 for the "All countries" empty option
    expect(countryOptions.length).toBe(aseanCountries.length + 1);
    for (const c of aseanCountries) {
      expect(
        within(countrySelect).getByRole("option", {
          name: new RegExp(c.name.en, "i"),
        }),
      ).toBeTruthy();
    }
  });

  it("selecting a country narrows city options to that country", async () => {
    const user = userEvent.setup();
    const onScopeChange = vi.fn();

    render(<ControlledGeoFilters onScopeChange={onScopeChange} />);

    const regionSelect = screen.getByRole("combobox", { name: /region/i });
    await user.selectOptions(regionSelect, "asean");

    const idCountry = countriesForRegion(dataset, "asean").find((c) => c.id === "id")!;
    const countrySelect = screen.getByRole("combobox", { name: /country/i });
    await user.selectOptions(countrySelect, idCountry.id);

    const idCities = citiesForCountry(dataset, "id");

    const citySelect = screen.getByRole("combobox", { name: /city/i });
    const cityOptions = within(citySelect).getAllByRole("option");
    // +1 for the "All cities" empty option
    expect(cityOptions.length).toBe(idCities.length + 1);
    for (const c of idCities) {
      expect(
        within(citySelect).getByRole("option", {
          name: new RegExp(c.name.en, "i"),
        }),
      ).toBeTruthy();
    }
  });

  it("clearing a region resets country and city selections", async () => {
    const user = userEvent.setup();
    const onScopeChange = vi.fn();

    render(<ControlledGeoFilters onScopeChange={onScopeChange} />);

    const regionSelect = screen.getByRole("combobox", { name: /region/i });
    await user.selectOptions(regionSelect, "asean");

    const idCountry = countriesForRegion(dataset, "asean").find((c) => c.id === "id")!;
    const countrySelect = screen.getByRole("combobox", { name: /country/i });
    await user.selectOptions(countrySelect, idCountry.id);

    // Clear region via button (EN locale default: aria-label = t("en","clearRegion") = "Clear")
    const clearRegion = screen.getByRole("button", { name: /clear/i });
    await user.click(clearRegion);

    // Country and city selects should reset to empty
    expect(screen.getByRole("combobox", { name: /country/i })).toHaveValue("");
    expect(screen.getByRole("combobox", { name: /city/i })).toHaveValue("");
  });

  it("reports selected scope to parent via onScopeChange", async () => {
    const user = userEvent.setup();
    const onScopeChange = vi.fn();

    render(<ControlledGeoFilters onScopeChange={onScopeChange} />);

    const regionSelect = screen.getByRole("combobox", { name: /region/i });
    await user.selectOptions(regionSelect, "asean");

    expect(onScopeChange).toHaveBeenCalledWith(expect.objectContaining({ region: "asean" }));
  });

  it("renders region select with value from the region prop", () => {
    const onScopeChange = vi.fn();

    render(
      <GeoFilters
        dataset={dataset}
        region={"europe" as Region}
        countryId={null}
        cityId={null}
        onScopeChange={onScopeChange}
      />,
    );

    const regionSelect = screen.getByRole("combobox", { name: /region/i });
    expect(regionSelect).toHaveValue("europe");
  });

  it("renders country select with value from the countryId prop", () => {
    const onScopeChange = vi.fn();

    render(
      <GeoFilters
        dataset={dataset}
        region={"asean" as Region}
        countryId={"id"}
        cityId={null}
        onScopeChange={onScopeChange}
      />,
    );

    const countrySelect = screen.getByRole("combobox", { name: /country/i });
    expect(countrySelect).toHaveValue("id");
  });

  it("when region changes to incompatible region, onScopeChange receives cascaded scope with cleared country and city", async () => {
    const user = userEvent.setup();
    const onScopeChange = vi.fn();

    // Start with asean + id (Indonesia)
    render(
      <ControlledGeoFilters
        onScopeChange={onScopeChange}
        initialRegion={"asean" as Region}
        initialCountryId={"id"}
        initialCityId={null}
      />,
    );

    // Change to europe — Indonesia is not in Europe
    const regionSelect = screen.getByRole("combobox", { name: /region/i });
    await user.selectOptions(regionSelect, "europe");

    // The cascaded scope should clear country and city
    expect(onScopeChange).toHaveBeenCalledWith(
      expect.objectContaining({ region: "europe", countryId: null, cityId: null }),
    );
  });

  // Gherkin (binds): "Filter dropdowns show Indonesian country and city names in the ID locale"
  it("shows Indonesian country names for locale=id after selecting ASEAN region", async () => {
    const user = userEvent.setup();
    const onScopeChange = vi.fn();

    render(<ControlledGeoFilters onScopeChange={onScopeChange} locale="id" />);

    const regionSelect = screen.getByRole("combobox", { name: /wilayah/i });
    await user.selectOptions(regionSelect, "asean");

    const aseanCountries = countriesForRegion(dataset, "asean");
    const countrySelect = screen.getByRole("combobox", { name: /negara/i });
    for (const c of aseanCountries) {
      const expectedLabel = c.name.id ?? c.name.en;
      expect(
        within(countrySelect).getByRole("option", {
          name: new RegExp(expectedLabel, "i"),
        }),
      ).toBeTruthy();
    }
  });

  it("shows Indonesian city names for locale=id after selecting a country", async () => {
    const user = userEvent.setup();
    const onScopeChange = vi.fn();

    render(<ControlledGeoFilters onScopeChange={onScopeChange} locale="id" />);

    const regionSelect = screen.getByRole("combobox", { name: /wilayah/i });
    await user.selectOptions(regionSelect, "asean");

    const idCountry = countriesForRegion(dataset, "asean").find((c) => c.id === "id")!;
    const countrySelect = screen.getByRole("combobox", { name: /negara/i });
    await user.selectOptions(countrySelect, idCountry.id);

    const idCities = citiesForCountry(dataset, "id");
    const citySelect = screen.getByRole("combobox", { name: /kota/i });
    for (const c of idCities) {
      const expectedLabel = c.name.id ?? c.name.en;
      expect(
        within(citySelect).getByRole("option", {
          name: new RegExp(expectedLabel, "i"),
        }),
      ).toBeTruthy();
    }
  });

  // Gherkin (binds): "Relocation column header is fully translated in the ID locale"
  it("cost-of-living table shows a fully Indonesian relocation sunk-cost column header for locale=id", () => {
    const defaultProps = {
      dataset,
      household: { adults: 1 as const, preschoolKids: 0 as const, schoolKids: 0 as const },
      schoolType: "public" as const,
      area: "center" as const,
      locale: "id" as const,
    };
    render(<CostOfLivingTable {...defaultProps} />);

    const columnHeaders = screen.getAllByRole("columnheader");
    const headerTexts = columnHeaders.map((h) => h.textContent ?? "");
    // Must NOT contain the English word "sunk" (case-insensitive)
    const relocationHeader = headerTexts.find((t) => /reloka/i.test(t));
    expect(relocationHeader).toBeDefined();
    expect(relocationHeader).not.toMatch(/sunk/i);
  });

  // Gherkin (binds): "Clear-region control aria-label is translated in the ID locale"
  it("clear-region button has Indonesian aria-label for locale=id", async () => {
    const user = userEvent.setup();
    const onScopeChange = vi.fn();

    render(<ControlledGeoFilters onScopeChange={onScopeChange} locale="id" />);

    // Select a region to reveal the clear button
    const regionSelect = screen.getByRole("combobox", { name: /wilayah/i });
    await user.selectOptions(regionSelect, "asean");

    // The clear button accessible name must be the Indonesian translation "Hapus", not "Clear region"
    const clearBtn = screen.getByRole("button", { name: /hapus/i });
    expect(clearBtn).toBeTruthy();
    expect(clearBtn.getAttribute("aria-label")).toMatch(/hapus/i);
  });
});
