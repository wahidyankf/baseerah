// Pure role-lookup functions: baseline resolution + minimum-role search.
// All absolute comparisons in USD. Uses median salary for ranking.
// Non-salary comp is informational only — never affects savings math.
// No React, no side effects.

import type { FxTable } from "./data/fx";
import { fxToUsd, usdToDisplay } from "./data/fx";
import type { City, Country, Dataset } from "./data/cities";
import type { RoleMatrix } from "./data/roles";
import type { EngRole } from "./data/roles";
import { savingsRow } from "./calc";

type Area = "center" | "rural";
type SchoolType = "public" | "private";
type Household = {
  adults: 1 | 2;
  preschoolKids: 0 | 1 | 2 | 3;
  schoolKids: 0 | 1 | 2 | 3;
};
type Opts = { household: Household; area: Area; schoolType: SchoolType };
type BaselineSource = "my_salary" | "reference_role" | "savings_target";
type Confidence = "high" | "moderate" | "proxy";

export type LadderEntry = {
  role: EngRole;
  rank: number;
  track: "ic" | "mgmt";
  bestCity: City;
  bestCountry: Country;
  bestEssentialSavingsUsd: number;
  distributionUsd: { p25: number; median: number; p75: number };
  nonSalaryCompUsd: number;
  totalCompUsd: number;
  confidence: Confidence;
  clears: boolean;
};

// ─── Salary helpers ─────────────────────────────────────────────────────────

// Gross monthly USD using the MEDIAN of the role × country distribution.
export function roleMedianGrossUsd(fx: FxTable, matrix: RoleMatrix, city: City, role: EngRole): number {
  const dist = matrix.salaries[city.countryId]?.[role];
  if (!dist) return 0;
  return dist.median.monthlyGrossLocal * fxToUsd(fx, city.currency);
}

// p25 / median / p75 each converted to USD for display.
export function roleSalaryDistributionUsd(
  fx: FxTable,
  matrix: RoleMatrix,
  city: City,
  role: EngRole,
): { p25: number; median: number; p75: number } {
  const dist = matrix.salaries[city.countryId]?.[role];
  const rate = fxToUsd(fx, city.currency);
  return {
    p25: (dist?.p25.monthlyGrossLocal ?? 0) * rate,
    median: (dist?.median.monthlyGrossLocal ?? 0) * rate,
    p75: (dist?.p75.monthlyGrossLocal ?? 0) * rate,
  };
}

// Annual non-salary comp in USD (informational only — not in savings math).
export function roleNonSalaryCompUsd(fx: FxTable, matrix: RoleMatrix, city: City, role: EngRole): number {
  const dist = matrix.salaries[city.countryId]?.[role];
  return (dist?.nonSalaryComp.annualLocal ?? 0) * fxToUsd(fx, city.currency);
}

// Informational total comp: annual base + non-salary comp. NOT used in ranking.
export function roleTotalCompUsd(fx: FxTable, matrix: RoleMatrix, city: City, role: EngRole): number {
  const medUsd = roleMedianGrossUsd(fx, matrix, city, role);
  const nonSalUsd = roleNonSalaryCompUsd(fx, matrix, city, role);
  return medUsd * 12 + nonSalUsd;
}

// Essential savings for this city + role combo. Uses median salary.
// Lifestyle is EXCLUDED from the ranking key (personal preference variable).
export function candidateEssentialSavingsUsd(
  fx: FxTable,
  country: Country,
  city: City,
  role: EngRole,
  opts: Opts,
  matrix: RoleMatrix,
): number {
  const grossUsd = roleMedianGrossUsd(fx, matrix, city, role);
  const row = savingsRow(grossUsd, city, country, fx, opts.household, opts.schoolType, opts.area);
  return row.essentialSavings;
}

// ─── Best city for role ───────────────────────────────────────────────────────

// Returns the city (within the optional scope) that maximises essentialSavingsUsd for
// the given role. `cityScope = null` means all cities are candidates.
export function bestCityForRole(
  dataset: Dataset,
  role: EngRole,
  opts: Opts,
  matrix: RoleMatrix,
  cityScope: City[] | null,
): { city: City; essentialSavingsUsd: number; confidence: Confidence } {
  const candidates = cityScope ?? dataset.cities;
  let best: City | null = null;
  let bestSavings = -Infinity;
  let bestConf: Confidence = "proxy";
  for (const city of candidates) {
    const country = dataset.countries.find((c) => c.id === city.countryId);
    if (!country) continue;
    const savings = candidateEssentialSavingsUsd(dataset.fx, country, city, role, opts, matrix);
    if (savings > bestSavings) {
      bestSavings = savings;
      best = city;
      // Carry confidence from the median salary point
      bestConf = matrix.salaries[city.countryId]?.[role]?.median.confidence ?? "high";
    }
  }
  if (!best) {
    throw new Error(`No candidate city found for role ${role}`);
  }
  return { city: best, essentialSavingsUsd: bestSavings, confidence: bestConf };
}

// ─── Baseline resolution ──────────────────────────────────────────────────────

type BaselineInput =
  | { grossMonthlyUsd: number } // my_salary
  | { role: EngRole; cityId: string } // reference_role
  | { amountLocal: number; displayCurrency: string }; // savings_target

export function resolveBaselineUsd(
  source: BaselineSource,
  input: BaselineInput,
  opts: Opts,
  dataset: Dataset,
  matrix: RoleMatrix,
): number {
  if (source === "my_salary") {
    const inp = input as { grossMonthlyUsd: number };
    // Return the best-city essential savings for this gross salary
    let best = -Infinity;
    for (const city of dataset.cities) {
      const country = dataset.countries.find((c) => c.id === city.countryId);
      if (!country) continue;
      const row = savingsRow(
        inp.grossMonthlyUsd,
        city,
        country,
        dataset.fx,
        opts.household,
        opts.schoolType,
        opts.area,
      );
      if (row.essentialSavings > best) best = row.essentialSavings;
    }
    return best;
  }
  if (source === "reference_role") {
    const inp = input as { role: EngRole; cityId: string };
    const city = dataset.cities.find((c) => c.id === inp.cityId);
    if (!city) throw new Error(`City not found: ${inp.cityId}`);
    const country = dataset.countries.find((c) => c.id === city.countryId);
    if (!country) throw new Error(`Country not found for city: ${inp.cityId}`);
    return candidateEssentialSavingsUsd(dataset.fx, country, city, inp.role, opts, matrix);
  }
  // savings_target
  const inp = input as { amountLocal: number; displayCurrency: string };
  return inp.amountLocal * fxToUsd(dataset.fx, inp.displayCurrency);
}

// ─── Rank ladder ─────────────────────────────────────────────────────────────

export function rankLadder(dataset: Dataset, opts: Opts, matrix: RoleMatrix, cityScope: City[] | null): LadderEntry[] {
  return matrix.ladder.map((rungMeta) => {
    const { city, essentialSavingsUsd, confidence } = bestCityForRole(dataset, rungMeta.role, opts, matrix, cityScope);
    const country = dataset.countries.find((c) => c.id === city.countryId)!;
    const distUsd = roleSalaryDistributionUsd(dataset.fx, matrix, city, rungMeta.role);
    const nonSalaryCompUsd = roleNonSalaryCompUsd(dataset.fx, matrix, city, rungMeta.role);
    const totalCompUsd = roleTotalCompUsd(dataset.fx, matrix, city, rungMeta.role);
    return {
      role: rungMeta.role,
      rank: rungMeta.rank,
      track: rungMeta.track,
      bestCity: city,
      bestCountry: country,
      bestEssentialSavingsUsd: essentialSavingsUsd,
      distributionUsd: distUsd,
      nonSalaryCompUsd,
      totalCompUsd,
      confidence,
      clears: false, // set by minimumRole pass; unknown until baseline is set
    };
  });
}

// ─── Minimum role ─────────────────────────────────────────────────────────────

// Returns the lowest-rank role whose bestEssentialSavingsUsd ≥ baselineUsd.
// Returns null if no role clears.
export function minimumRole(baselineUsd: number, rankedLadder: LadderEntry[]): EngRole | null {
  // Mark clears flags
  const withClears = rankedLadder.map((e) => ({
    ...e,
    clears: e.bestEssentialSavingsUsd >= baselineUsd,
  }));
  const qualifying = withClears.filter((e) => e.clears);
  if (qualifying.length === 0) return null;
  // Lowest rank = minimum seniority needed
  return qualifying.reduce((min, e) => (e.rank < min.rank ? e : min)).role;
}

// ─── Display order ────────────────────────────────────────────────────────────

// Reorder: qualifying (clears=true) roles first, sorted by rank HIGH→LOW down to the
// minimum qualifier; then non-qualifying (clears=false) roles, also ranked LOW→HIGH.
export function orderForDisplay(rankedLadder: LadderEntry[], minRole: EngRole | null): LadderEntry[] {
  // Recompute clears based on minRole presence
  const minRank = minRole ? (rankedLadder.find((e) => e.role === minRole)?.rank ?? Infinity) : Infinity;

  const withClears = rankedLadder.map((e) => ({
    ...e,
    clears:
      minRole !== null && e.rank <= minRank
        ? e.bestEssentialSavingsUsd >= (rankedLadder.find((r) => r.role === minRole)?.bestEssentialSavingsUsd ?? 0)
        : false,
  }));

  const qualifying = withClears.filter((e) => e.clears).sort((a, b) => b.rank - a.rank); // high→low seniority
  const nonQualifying = withClears.filter((e) => !e.clears).sort((a, b) => a.rank - b.rank); // low→high for non-qualifying

  return [...qualifying, ...nonQualifying];
}

// ─── Display currency conversion ─────────────────────────────────────────────

// Converts a USD savings figure to the three display forms: USD, local city currency,
// and the user's chosen display currency.
export function toDisplayCurrencies(
  fx: FxTable,
  savingsUsd: number,
  cityCurrency: string,
  displayCurrency: string,
): { usd: number; local: number; display: number } {
  const cityRate = fxToUsd(fx, cityCurrency);
  const local = cityRate > 0 ? savingsUsd / cityRate : 0;
  const display = usdToDisplay(fx, savingsUsd, displayCurrency);
  return { usd: savingsUsd, local, display };
}
