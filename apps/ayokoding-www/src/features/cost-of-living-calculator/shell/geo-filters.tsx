"use client";

import { useState } from "react";
import type { Dataset } from "../core/data/cities";
import { countriesForRegion, citiesForCountry, scopedCities } from "../core/geo-filter";
import type { Locale } from "@/features/i18n/core/config";
import { t } from "@/features/i18n/core/translations";
import { applyRegionChange, applyCountryChange, applyCityChange } from "../core/url-state";
import type { CalculatorState } from "../core/url-state";

type Region = "asean" | "japan" | "europe" | "nordics" | "americas" | "mena" | "asia" | "oceania" | "africa";

export type GeoScope = {
  region: Region | null;
  countryId: string | null;
  cityId: string | null;
};

type Props = {
  dataset: Dataset;
  locale?: Locale;
  region: Region | null;
  countryId: string | null;
  cityId: string | null;
  onScopeChange: (scope: GeoScope) => void;
};

const REGION_LABELS: Record<Region, string> = {
  asean: "ASEAN",
  japan: "Japan",
  europe: "Europe",
  nordics: "Nordics",
  americas: "Americas",
  mena: "MENA",
  asia: "Asia",
  oceania: "Oceania",
  africa: "Africa",
};

/** Returns the locale-specific name, falling back to English. */
export function localeName(name: { en: string; id: string }, locale: Locale): string {
  return name[locale] ?? name.en;
}

// Build a minimal CalculatorState stub from the current geo props for apply helpers.
function toStateStub(region: Region | null, countryId: string | null, cityId: string | null): CalculatorState {
  return {
    tab: "cost",
    region,
    countryId,
    cityId,
    household: { adults: 1, preschoolKids: 0, schoolKids: 0 },
    schoolType: "public",
    area: "center",
  };
}

export function GeoFilters({ dataset, locale = "en", region, countryId, cityId, onScopeChange }: Props) {
  // Pre-resolve all translated strings to avoid repeated t() calls in JSX.
  const labels = {
    region: t(locale, "labelRegion"),
    country: t(locale, "labelCountry"),
    city: t(locale, "labelCity"),
    allRegions: t(locale, "optAllRegions"),
    allCountries: t(locale, "optAllCountries"),
    allCities: t(locale, "optAllCities"),
    clearRegion: t(locale, "clearRegion"),
    regionAutoAdvisory: t(locale, "regionAutoAdvisory"),
  };

  // UWT-014: when a country change silently auto-advances the region, surface a
  // visible advisory so the change is not invisible to the user.
  const [regionAutoChanged, setRegionAutoChanged] = useState(false);

  const availableRegions = [...new Set(dataset.cities.map((c) => c.region as Region))].sort();

  const availableCountries = region !== null ? countriesForRegion(dataset, region) : dataset.countries;

  const availableCities =
    countryId !== null ? citiesForCountry(dataset, countryId) : scopedCities(dataset, region, null, null);

  function handleRegionChange(val: string) {
    const newRegion = val === "" ? null : (val as Region);
    const currentState = toStateStub(region, countryId, cityId);
    const next = applyRegionChange(currentState, newRegion, dataset);
    setRegionAutoChanged(false);
    onScopeChange({ region: next.region, countryId: next.countryId, cityId: next.cityId });
  }

  function handleCountryChange(val: string) {
    const newCountryId = val === "" ? null : val;
    const currentState = toStateStub(region, countryId, cityId);
    const next = applyCountryChange(currentState, newCountryId, dataset);
    // Advise when the chosen country forced a different region than was selected.
    setRegionAutoChanged(next.countryId !== null && next.region !== region);
    onScopeChange({ region: next.region, countryId: next.countryId, cityId: next.cityId });
  }

  function handleCityChange(val: string) {
    const newCityId = val === "" ? null : val;
    const currentState = toStateStub(region, countryId, cityId);
    const next = applyCityChange(currentState, newCityId, dataset);
    setRegionAutoChanged(false);
    onScopeChange({ region: next.region, countryId: next.countryId, cityId: next.cityId });
  }

  function clearRegion() {
    setRegionAutoChanged(false);
    onScopeChange({ region: null, countryId: null, cityId: null });
  }

  return (
    <div className="space-y-2">
      <div className="flex flex-wrap items-center gap-3">
        <div className="flex min-w-0 basis-full items-center gap-2 sm:basis-auto">
          <label htmlFor="geo-region-select" className="text-sm font-medium">
            {labels.region}
          </label>
          <select
            id="geo-region-select"
            aria-label={labels.region}
            value={region ?? ""}
            onChange={(e) => handleRegionChange(e.target.value)}
            className="min-h-[44px] w-full max-w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs transition-[color,box-shadow] outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50"
          >
            <option value="">{labels.allRegions}</option>
            {availableRegions.map((r) => (
              <option key={r} value={r}>
                {REGION_LABELS[r]}
              </option>
            ))}
          </select>
          {region !== null && (
            <button
              type="button"
              aria-label={labels.clearRegion}
              onClick={clearRegion}
              className="text-xs text-muted-foreground underline"
            >
              {labels.clearRegion}
            </button>
          )}
        </div>

        <div className="flex min-w-0 basis-full items-center gap-2 sm:basis-auto">
          <label htmlFor="geo-country-select" className="text-sm font-medium">
            {labels.country}
          </label>
          <select
            id="geo-country-select"
            aria-label={labels.country}
            value={countryId ?? ""}
            onChange={(e) => handleCountryChange(e.target.value)}
            className="min-h-[44px] w-full max-w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs transition-[color,box-shadow] outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50"
          >
            <option value="">{labels.allCountries}</option>
            {availableCountries.map((c) => (
              <option key={c.id} value={c.id}>
                {localeName(c.name, locale)}
              </option>
            ))}
          </select>
        </div>

        <div className="flex min-w-0 basis-full items-center gap-2 sm:basis-auto">
          <label htmlFor="geo-city-select" className="text-sm font-medium">
            {labels.city}
          </label>
          <select
            id="geo-city-select"
            aria-label={labels.city}
            value={cityId ?? ""}
            onChange={(e) => handleCityChange(e.target.value)}
            className="min-h-[44px] w-full max-w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs transition-[color,box-shadow] outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50"
          >
            <option value="">{labels.allCities}</option>
            {availableCities.map((c) => (
              <option key={c.id} value={c.id}>
                {localeName(c.name, locale)}
              </option>
            ))}
          </select>
        </div>
      </div>
      {regionAutoChanged && (
        <output data-testid="region-auto-advisory" className="block text-xs text-muted-foreground">
          {labels.regionAutoAdvisory}
        </output>
      )}
    </div>
  );
}
