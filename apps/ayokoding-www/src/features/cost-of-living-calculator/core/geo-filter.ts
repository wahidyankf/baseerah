// Pure cascading Region → Country → City selectors.
// No React, no side effects. All selectors are read-only over the dataset.

import type { City, Country, Dataset } from "./data/cities";

type Region = City["region"];

// Returns the unique set of countries that have at least one city in the given region.
export function countriesForRegion(dataset: Dataset, region: Region): Country[] {
  const countryIds = new Set(dataset.cities.filter((c) => c.region === region).map((c) => c.countryId));
  return dataset.countries.filter((c) => countryIds.has(c.id));
}

// Returns all cities in a given country.
export function citiesForCountry(dataset: Dataset, countryId: string): City[] {
  return dataset.cities.filter((c) => c.countryId === countryId);
}

// Returns the city list scoped by the three cascading filters (each nullable = no filter).
// Precedence: city wins over country wins over region.
// A city param: return only that one city (ignores region/country).
// A country param (no city): return that country's cities (intersected with region if also set).
// A region param only: return all cities in that region.
// No params: return all cities.
export function scopedCities(
  dataset: Dataset,
  region: Region | null,
  countryId: string | null,
  cityId: string | null,
): City[] {
  if (cityId !== null) {
    return dataset.cities.filter((c) => c.id === cityId);
  }
  if (countryId !== null) {
    const byCo = dataset.cities.filter((c) => c.countryId === countryId);
    if (region !== null) {
      return byCo.filter((c) => c.region === region);
    }
    return byCo;
  }
  if (region !== null) {
    return dataset.cities.filter((c) => c.region === region);
  }
  return dataset.cities;
}
