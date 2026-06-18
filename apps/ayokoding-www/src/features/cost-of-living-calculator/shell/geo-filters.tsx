"use client";

import { useState } from "react";
import type { Dataset } from "../core/data/cities";
import { countriesForRegion, citiesForCountry, scopedCities } from "../core/geo-filter";
import type { Locale } from "@/features/i18n/core/config";
import { t } from "@/features/i18n/core/translations";

type Region = "asean" | "japan" | "europe" | "nordics" | "americas" | "mena" | "asia" | "oceania" | "africa";

export type GeoScope = {
  region: Region | null;
  countryId: string | null;
  cityId: string | null;
};

type Props = {
  dataset: Dataset;
  locale?: Locale;
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

export function GeoFilters({ dataset, locale = "en", onScopeChange }: Props) {
  const [region, setRegion] = useState<Region | null>(null);
  const [countryId, setCountryId] = useState<string | null>(null);
  const [cityId, setCityId] = useState<string | null>(null);

  const availableRegions = [...new Set(dataset.cities.map((c) => c.region as Region))].sort();

  const availableCountries = region !== null ? countriesForRegion(dataset, region) : dataset.countries;

  const availableCities =
    countryId !== null ? citiesForCountry(dataset, countryId) : scopedCities(dataset, region, null, null);

  function handleRegionChange(val: string) {
    const newRegion = val === "" ? null : (val as Region);
    setRegion(newRegion);
    setCountryId(null);
    setCityId(null);
    onScopeChange({ region: newRegion, countryId: null, cityId: null });
  }

  function handleCountryChange(val: string) {
    const newCountryId = val === "" ? null : val;
    setCountryId(newCountryId);
    setCityId(null);
    onScopeChange({ region, countryId: newCountryId, cityId: null });
  }

  function handleCityChange(val: string) {
    const newCityId = val === "" ? null : val;
    setCityId(newCityId);
    onScopeChange({ region, countryId, cityId: newCityId });
  }

  function clearRegion() {
    setRegion(null);
    setCountryId(null);
    setCityId(null);
    onScopeChange({ region: null, countryId: null, cityId: null });
  }

  return (
    <div className="flex flex-wrap items-center gap-3">
      <div className="flex items-center gap-2">
        <label htmlFor="geo-region-select" className="text-sm font-medium">
          {t(locale, "labelRegion")}
        </label>
        <select
          id="geo-region-select"
          aria-label={t(locale, "labelRegion")}
          value={region ?? ""}
          onChange={(e) => handleRegionChange(e.target.value)}
          className="rounded border px-2 py-1 text-sm"
        >
          <option value="">{t(locale, "optAllRegions")}</option>
          {availableRegions.map((r) => (
            <option key={r} value={r}>
              {REGION_LABELS[r]}
            </option>
          ))}
        </select>
        {region !== null && (
          <button
            type="button"
            aria-label="Clear region"
            onClick={clearRegion}
            className="text-xs text-muted-foreground underline"
          >
            {t(locale, "clearRegion")}
          </button>
        )}
      </div>

      <div className="flex items-center gap-2">
        <label htmlFor="geo-country-select" className="text-sm font-medium">
          {t(locale, "labelCountry")}
        </label>
        <select
          id="geo-country-select"
          aria-label={t(locale, "labelCountry")}
          value={countryId ?? ""}
          onChange={(e) => handleCountryChange(e.target.value)}
          className="rounded border px-2 py-1 text-sm"
        >
          <option value="">{t(locale, "optAllCountries")}</option>
          {availableCountries.map((c) => (
            <option key={c.id} value={c.id}>
              {c.name.en}
            </option>
          ))}
        </select>
      </div>

      <div className="flex items-center gap-2">
        <label htmlFor="geo-city-select" className="text-sm font-medium">
          {t(locale, "labelCity")}
        </label>
        <select
          id="geo-city-select"
          aria-label={t(locale, "labelCity")}
          value={cityId ?? ""}
          onChange={(e) => handleCityChange(e.target.value)}
          className="rounded border px-2 py-1 text-sm"
        >
          <option value="">{t(locale, "optAllCities")}</option>
          {availableCities.map((c) => (
            <option key={c.id} value={c.id}>
              {c.name.en}
            </option>
          ))}
        </select>
      </div>
    </div>
  );
}
