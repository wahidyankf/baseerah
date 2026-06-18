import { cleanup, render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it, vi } from "vitest";
import { dataset } from "../core/data/cities";
import { countriesForRegion, citiesForCountry } from "../core/geo-filter";
import { GeoFilters } from "./geo-filters";

afterEach(cleanup);

// Gherkin (binds): "Region narrows the country filter and country narrows the city filter"
describe("GeoFilters", () => {
  it("selecting a region narrows country options to that region", async () => {
    const user = userEvent.setup();
    const onScopeChange = vi.fn();

    render(<GeoFilters dataset={dataset} onScopeChange={onScopeChange} />);

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

    render(<GeoFilters dataset={dataset} onScopeChange={onScopeChange} />);

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

    render(<GeoFilters dataset={dataset} onScopeChange={onScopeChange} />);

    const regionSelect = screen.getByRole("combobox", { name: /region/i });
    await user.selectOptions(regionSelect, "asean");

    const idCountry = countriesForRegion(dataset, "asean").find((c) => c.id === "id")!;
    const countrySelect = screen.getByRole("combobox", { name: /country/i });
    await user.selectOptions(countrySelect, idCountry.id);

    // Clear region via button
    const clearRegion = screen.getByRole("button", { name: /clear region/i });
    await user.click(clearRegion);

    // Country and city selects should reset to empty
    expect(screen.getByRole("combobox", { name: /country/i })).toHaveValue("");
    expect(screen.getByRole("combobox", { name: /city/i })).toHaveValue("");
  });

  it("reports selected scope to parent via onScopeChange", async () => {
    const user = userEvent.setup();
    const onScopeChange = vi.fn();

    render(<GeoFilters dataset={dataset} onScopeChange={onScopeChange} />);

    const regionSelect = screen.getByRole("combobox", { name: /region/i });
    await user.selectOptions(regionSelect, "asean");

    expect(onScopeChange).toHaveBeenCalledWith(expect.objectContaining({ region: "asean" }));
  });
});
