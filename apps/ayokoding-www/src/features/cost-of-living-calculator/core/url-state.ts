// Pure URL state encode/decode/sanitize functions for the cost-of-living calculator.
// No React, no router, no side effects. All functions are pure.
// URL is the single source of truth: 9 controls serialized, defaults omitted.

import type { City, Dataset } from "./data/cities";
import type { SchoolType, Area } from "./calc";

// ─── Types ───────────────────────────────────────────────────────────────────

type Region = City["region"];

export type CalculatorState = {
  tab: "cost" | "savings" | "min-role";
  region: Region | null;
  countryId: string | null;
  cityId: string | null;
  household: {
    adults: 1 | 2;
    preschoolKids: 0 | 1 | 2 | 3;
    schoolKids: 0 | 1 | 2 | 3;
  };
  schoolType: SchoolType;
  area: Area;
};

// ─── Constants ───────────────────────────────────────────────────────────────

export const PARAM_KEYS = {
  tab: "tab",
  region: "region",
  country: "country",
  city: "city",
  adults: "adults",
  preschool: "preschool",
  schoolkids: "schoolkids",
  schooltype: "schooltype",
  area: "area",
} as const;

const VALID_TABS = ["cost", "savings", "min-role"] as const;
const VALID_REGIONS: readonly Region[] = [
  "asean",
  "japan",
  "europe",
  "nordics",
  "americas",
  "mena",
  "asia",
  "oceania",
  "africa",
];
const VALID_SCHOOL_TYPES: readonly SchoolType[] = ["public", "private"];
const VALID_AREAS: readonly Area[] = ["center", "rural"];

const VALID_ADULTS = [1, 2] as const;
const VALID_PRESCHOOL_KIDS = [0, 1, 2, 3] as const;
const VALID_SCHOOL_KIDS = [0, 1, 2, 3] as const;

// ─── Default State ───────────────────────────────────────────────────────────

export const DEFAULT_STATE: CalculatorState = {
  tab: "cost",
  region: null,
  countryId: null,
  cityId: null,
  household: {
    adults: 1,
    preschoolKids: 0,
    schoolKids: 0,
  },
  schoolType: "public",
  area: "center",
};

// ─── Validation Helpers ───────────────────────────────────────────────────────

function isValidTab(v: string): v is CalculatorState["tab"] {
  return (VALID_TABS as readonly string[]).includes(v);
}

function isValidRegion(v: string): v is Region {
  return (VALID_REGIONS as readonly string[]).includes(v);
}

function isValidSchoolType(v: string): v is SchoolType {
  return (VALID_SCHOOL_TYPES as readonly string[]).includes(v);
}

function isValidArea(v: string): v is Area {
  return (VALID_AREAS as readonly string[]).includes(v);
}

function parseAdults(v: string | null): 1 | 2 {
  if (v === null) return DEFAULT_STATE.household.adults;
  const n = parseInt(v, 10);
  if ((VALID_ADULTS as readonly number[]).includes(n)) return n as 1 | 2;
  return DEFAULT_STATE.household.adults;
}

function parsePreschoolKids(v: string | null): 0 | 1 | 2 | 3 {
  if (v === null) return DEFAULT_STATE.household.preschoolKids;
  const n = parseInt(v, 10);
  if ((VALID_PRESCHOOL_KIDS as readonly number[]).includes(n)) return n as 0 | 1 | 2 | 3;
  return DEFAULT_STATE.household.preschoolKids;
}

function parseSchoolKids(v: string | null): 0 | 1 | 2 | 3 {
  if (v === null) return DEFAULT_STATE.household.schoolKids;
  const n = parseInt(v, 10);
  if ((VALID_SCHOOL_KIDS as readonly number[]).includes(n)) return n as 0 | 1 | 2 | 3;
  return DEFAULT_STATE.household.schoolKids;
}

function parseCityId(v: string | null, dataset: Dataset): string | null {
  if (v === null) return null;
  const found = dataset.cities.find((c) => c.id === v);
  return found ? v : null;
}

function parseCountryId(v: string | null, dataset: Dataset): string | null {
  if (v === null) return null;
  const found = dataset.countries.find((c) => c.id === v);
  return found ? v : null;
}

function parseRegion(v: string | null): Region | null {
  if (v === null) return null;
  return isValidRegion(v) ? v : null;
}

// ─── Geo Backfill Helpers ────────────────────────────────────────────────────

function cityById(dataset: Dataset, cityId: string): (typeof dataset.cities)[number] | undefined {
  return dataset.cities.find((c) => c.id === cityId);
}

function countryRegion(dataset: Dataset, countryId: string): Region | null {
  const firstCity = dataset.cities.find((c) => c.countryId === countryId);
  return firstCity ? firstCity.region : null;
}

// Reconcile geo state: city > country > region (narrower wins).
// Backfills broader fields from narrower; clears narrower that don't belong to broader.
function reconcileGeo(
  cityId: string | null,
  countryId: string | null,
  region: Region | null,
  dataset: Dataset,
): { cityId: string | null; countryId: string | null; region: Region | null } {
  // Narrower wins: start from city if set
  if (cityId !== null) {
    const city = cityById(dataset, cityId);
    if (city) {
      // City is authoritative — backfill country and region from city
      return { cityId, countryId: city.countryId, region: city.region };
    }
    // City ID is invalid
    cityId = null;
  }

  // Next: country if set
  if (countryId !== null) {
    const inferredRegion = countryRegion(dataset, countryId);
    if (inferredRegion !== null) {
      // Country is authoritative — backfill region from country
      return { cityId: null, countryId, region: inferredRegion };
    }
    // Country ID invalid
    countryId = null;
  }

  // Just region (or nothing)
  return { cityId: null, countryId: null, region };
}

// ─── Sanitize ────────────────────────────────────────────────────────────────

// Idempotent sanitize: sanitizeState(sanitizeState(s, d), d) deepEquals sanitizeState(s, d)
export function sanitizeState(state: CalculatorState, dataset: Dataset): CalculatorState {
  // Validate scalar fields
  const tab = isValidTab(state.tab) ? state.tab : DEFAULT_STATE.tab;
  const schoolType = isValidSchoolType(state.schoolType) ? state.schoolType : DEFAULT_STATE.schoolType;
  const area = isValidArea(state.area) ? state.area : DEFAULT_STATE.area;

  // Validate household
  const adults = (VALID_ADULTS as readonly number[]).includes(state.household.adults)
    ? state.household.adults
    : DEFAULT_STATE.household.adults;
  const preschoolKids = (VALID_PRESCHOOL_KIDS as readonly number[]).includes(state.household.preschoolKids)
    ? state.household.preschoolKids
    : DEFAULT_STATE.household.preschoolKids;
  const schoolKids = (VALID_SCHOOL_KIDS as readonly number[]).includes(state.household.schoolKids)
    ? state.household.schoolKids
    : DEFAULT_STATE.household.schoolKids;

  // Validate geo IDs against dataset
  const rawCityId = parseCityId(state.cityId, dataset);
  const rawCountryId = parseCountryId(state.countryId, dataset);
  const rawRegion = parseRegion(state.region);

  // Reconcile (narrower wins + backfill)
  const { cityId, countryId, region } = reconcileGeo(rawCityId, rawCountryId, rawRegion, dataset);

  return {
    tab,
    region,
    countryId,
    cityId,
    household: { adults, preschoolKids, schoolKids },
    schoolType,
    area,
  };
}

// ─── Decode ───────────────────────────────────────────────────────────────────

// Parse raw URLSearchParams → sanitized CalculatorState.
// Drops unknown values, clamps out-of-range numerics, narrower-wins on geo conflict.
export function decodeState(params: URLSearchParams, dataset: Dataset): CalculatorState {
  const tabRaw = params.get(PARAM_KEYS.tab);
  const tab = tabRaw !== null && isValidTab(tabRaw) ? tabRaw : DEFAULT_STATE.tab;

  const regionRaw = params.get(PARAM_KEYS.region);
  const countryRaw = params.get(PARAM_KEYS.country);
  const cityRaw = params.get(PARAM_KEYS.city);
  const adultsRaw = params.get(PARAM_KEYS.adults);
  const preschoolRaw = params.get(PARAM_KEYS.preschool);
  const schoolkidsRaw = params.get(PARAM_KEYS.schoolkids);
  const schooltypeRaw = params.get(PARAM_KEYS.schooltype);
  const areaRaw = params.get(PARAM_KEYS.area);

  const rawCityId = parseCityId(cityRaw, dataset);
  const rawCountryId = parseCountryId(countryRaw, dataset);
  const rawRegion = parseRegion(regionRaw);

  const { cityId, countryId, region } = reconcileGeo(rawCityId, rawCountryId, rawRegion, dataset);

  return {
    tab,
    region,
    countryId,
    cityId,
    household: {
      adults: parseAdults(adultsRaw),
      preschoolKids: parsePreschoolKids(preschoolRaw),
      schoolKids: parseSchoolKids(schoolkidsRaw),
    },
    schoolType: schooltypeRaw !== null && isValidSchoolType(schooltypeRaw) ? schooltypeRaw : DEFAULT_STATE.schoolType,
    area: areaRaw !== null && isValidArea(areaRaw) ? areaRaw : DEFAULT_STATE.area,
  };
}

// ─── Encode ───────────────────────────────────────────────────────────────────

// Encode state → URLSearchParams; defaults OMITTED (clean URL).
export function encodeState(state: CalculatorState): URLSearchParams {
  const params = new URLSearchParams();

  if (state.tab !== DEFAULT_STATE.tab) {
    params.set(PARAM_KEYS.tab, state.tab);
  }
  if (state.region !== DEFAULT_STATE.region) {
    params.set(PARAM_KEYS.region, state.region ?? "");
  }
  if (state.countryId !== DEFAULT_STATE.countryId) {
    params.set(PARAM_KEYS.country, state.countryId ?? "");
  }
  if (state.cityId !== DEFAULT_STATE.cityId) {
    params.set(PARAM_KEYS.city, state.cityId ?? "");
  }
  if (state.household.adults !== DEFAULT_STATE.household.adults) {
    params.set(PARAM_KEYS.adults, String(state.household.adults));
  }
  if (state.household.preschoolKids !== DEFAULT_STATE.household.preschoolKids) {
    params.set(PARAM_KEYS.preschool, String(state.household.preschoolKids));
  }
  if (state.household.schoolKids !== DEFAULT_STATE.household.schoolKids) {
    params.set(PARAM_KEYS.schoolkids, String(state.household.schoolKids));
  }
  if (state.schoolType !== DEFAULT_STATE.schoolType) {
    params.set(PARAM_KEYS.schooltype, state.schoolType);
  }
  if (state.area !== DEFAULT_STATE.area) {
    params.set(PARAM_KEYS.area, state.area);
  }

  return params;
}

// ─── Apply Helpers ────────────────────────────────────────────────────────────

// Cascade-clear + backfill for region change.
// If current city not in new region, clear city+country.
// If no city but country not in new region, clear country.
export function applyRegionChange(state: CalculatorState, region: Region | null, dataset: Dataset): CalculatorState {
  if (region === null) {
    return { ...state, region: null, countryId: null, cityId: null };
  }

  // Check if current city is in the new region
  const city = state.cityId !== null ? cityById(dataset, state.cityId) : undefined;
  if (city && city.region === region) {
    // City still belongs to new region — keep city+country
    return { ...state, region };
  }

  // City not in new region (or no city) — check country
  if (state.countryId !== null) {
    const inferredRegion = countryRegion(dataset, state.countryId);
    if (inferredRegion === region) {
      // Country is in new region — clear only city
      return { ...state, region, cityId: null };
    }
    // Country not in new region — clear both
    return { ...state, region, countryId: null, cityId: null };
  }

  return { ...state, region, countryId: null, cityId: null };
}

// Cascade-clear + backfill for country change.
// Backfills region from country, clears city if it doesn't belong to new country.
export function applyCountryChange(
  state: CalculatorState,
  countryId: string | null,
  dataset: Dataset,
): CalculatorState {
  if (countryId === null) {
    return { ...state, countryId: null, cityId: null };
  }

  const inferredRegion = countryRegion(dataset, countryId);

  // Check if current city belongs to the new country
  const city = state.cityId !== null ? cityById(dataset, state.cityId) : undefined;
  const cityBelongs = city && city.countryId === countryId;

  return {
    ...state,
    region: inferredRegion,
    countryId,
    cityId: cityBelongs ? state.cityId : null,
  };
}

// Backfill for city change.
// Backfills countryId and region from city.
export function applyCityChange(state: CalculatorState, cityId: string | null, dataset: Dataset): CalculatorState {
  if (cityId === null) {
    return { ...state, cityId: null };
  }

  const city = cityById(dataset, cityId);
  if (!city) {
    return { ...state, cityId: null };
  }

  return {
    ...state,
    region: city.region,
    countryId: city.countryId,
    cityId,
  };
}

// ─── Parent Scope Params ──────────────────────────────────────────────────────

// Encode region+country, omitting defaults, dropping city.
// Used for city-detail back link.
export function parentScopeParams(state: CalculatorState): URLSearchParams {
  const params = new URLSearchParams();

  if (state.region !== DEFAULT_STATE.region) {
    params.set(PARAM_KEYS.region, state.region ?? "");
  }
  if (state.countryId !== DEFAULT_STATE.countryId) {
    params.set(PARAM_KEYS.country, state.countryId ?? "");
  }
  // tab, household, schoolType, area — omit (not part of parent scope)
  // city — intentionally excluded

  return params;
}
