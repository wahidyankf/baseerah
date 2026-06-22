// Pure calculation functions for the Cost of Living Calculator.
// No React, no I/O, no side effects. All *Usd conversions route through fxToUsd(fx, currency).
// OECD modified equivalence scale drives per-category household scaling:
//   sub-linear (0.5 damping): housing, utilities
//   per-capita: food, healthcare
//   flat: transport, lifestyle
//   per-child: childcare (preschoolKids), school (schoolKids)

import type { FxTable } from "./data/fx";
import { fxToUsd } from "./data/fx";
import type { City, Country } from "./data/cities";
import { subLinear, perCapita, AREA_MULTIPLIERS } from "./data/cities";

type IncomeBand = "low" | "mid" | "high";
export type SchoolType = "public" | "private";
export type Area = "center" | "rural";
type Household = {
  adults: 1 | 2;
  preschoolKids: 0 | 1 | 2 | 3;
  schoolKids: 0 | 1 | 2 | 3;
};

export type SavingsRow = {
  cityId: string;
  currency: string;
  grossMonthlyUsd: number;
  netUsd: number;
  essentialsUsd: number;
  lifestyleUsd: number;
  essentialSavings: number;
  afterLifestyleSavings: number;
};

// ─── Gross ↔ Annual ─────────────────────────────────────────────────────────

export function grossMonthlyToAnnual(monthly: number): number {
  return monthly * 12;
}

export function grossAnnualToMonthly(annual: number): number {
  return annual / 12;
}

// Informational total comp — never alters net or savings figures.
export function totalCompAnnual(grossAnnual: number, nonSalaryCompAnnual: number): number {
  return grossAnnual + nonSalaryCompAnnual;
}

// ─── Tax ────────────────────────────────────────────────────────────────────

export function incomeBand(grossMonthlyUsd: number, country: Country): IncomeBand {
  if (grossMonthlyUsd < country.bandThresholdsUsd.lowToMid) return "low";
  if (grossMonthlyUsd < country.bandThresholdsUsd.midToHigh) return "mid";
  return "high";
}

// Combined federal + optional sub-national effective rate (0..1).
// net = gross × (1 − effectiveRateForCity)
export function effectiveRateForCity(grossMonthlyUsd: number, city: City, country: Country): number {
  const band = incomeBand(grossMonthlyUsd, country);
  const federal = country.effectiveRate[band].amount;
  const subNat = city.subNational?.effectiveRate[band]?.amount ?? 0;
  return federal + subNat;
}

// Net monthly income in USD after combined effective tax.
// grossMonthlyUsd is already in USD (caller converts from local if needed).
export function netUsd(grossMonthlyUsd: number, city: City, country: Country, _fx: FxTable): number {
  const rate = effectiveRateForCity(grossMonthlyUsd, city, country);
  return grossMonthlyUsd * (1 - rate);
}

// ─── Per-Category Household Scaling ──────────────────────────────────────────

// Expense categories that have distinct OECD scaling behaviours.
export type ExpenseCategory = "housing" | "food" | "transport" | "utilities" | "healthcare";

// Apply OECD household scaling to a single expense category amount (in local currency).
// This is the single authoritative path for per-category scaling used by both the
// comparison table and the city-detail view.
//   housing   → sub-linear × area
//   food      → per-capita
//   transport → flat (one transit pass per household)
//   utilities → sub-linear (no area factor)
//   healthcare→ per-capita
export function scaleAmount(amount: number, category: ExpenseCategory, household: Household, area: Area): number {
  const s = subLinear(household);
  const p = perCapita(household);
  const areaMultiplier = AREA_MULTIPLIERS[area];
  switch (category) {
    case "housing":
      return amount * s * areaMultiplier;
    case "food":
      return amount * p;
    case "transport":
      return amount;
    case "utilities":
      return amount * s;
    case "healthcare":
      return amount * p;
  }
}

// ─── Expense Components (local currency) ────────────────────────────────────

// Monthly childcare in local currency: per pre-school child, no OECD damping.
export function childcareLocal(city: City, household: Household): number {
  return city.childcareMedianLocal.amount * household.preschoolKids;
}

// Whether a FOREIGN resident worker can realistically budget for local-price PUBLIC school here.
// Conservative: only when access is fully "open". "limited" (non-resident fees, local-language-only,
// low admission priority) and "nationals-only" (legally barred) both fall back to private, matching
// the research finding that most expat families use private/international in those countries.
export function foreignerCanUsePublicSchool(country: Country): boolean {
  return country.foreignerPublicSchool.access === "open";
}

// The school type actually charged to a FOREIGNER: a "public" choice collapses to "private" wherever
// public schooling is not fully open to foreign residents, because the realistic relocation budget
// there is private/international (see foreignerCanUsePublicSchool).
export function effectiveSchoolType(country: Country, schoolType: SchoolType): SchoolType {
  if (schoolType === "public" && !foreignerCanUsePublicSchool(country)) return "private";
  return schoolType;
}

// Monthly school cost in local currency: per school-age child. Uses the EFFECTIVE school type, so a
// foreigner in a nationals-only country pays the private figure even when "public" is chosen.
export function schoolLocal(city: City, country: Country, household: Household, schoolType: SchoolType): number {
  const effective = effectiveSchoolType(country, schoolType);
  return city.schoolMedianLocal[effective].amount * household.schoolKids;
}

// Monthly essential expenses total in local currency.
// Essentials = housing + food + transport + utilities + healthcare + childcare + school.
export function essentialsLocal(
  city: City,
  country: Country,
  household: Household,
  schoolType: SchoolType,
  area: Area,
): number {
  const e = city.expenses;

  const housing = scaleAmount(e.housing.amount, "housing", household, area);
  const food = scaleAmount(e.food.amount, "food", household, area);
  const transport = scaleAmount(e.transport.amount, "transport", household, area);
  const utilities = scaleAmount(e.utilities.amount, "utilities", household, area);
  const healthcare = scaleAmount(e.healthcare.amount, "healthcare", household, area);
  const childcare = childcareLocal(city, household);
  const school = schoolLocal(city, country, household, schoolType);

  return housing + food + transport + utilities + healthcare + childcare + school;
}

// Monthly total expenses in local currency (essentials + lifestyle).
// Lifestyle stays flat (personal/discretionary — not household-scaled in v1).
export function expensesLocal(
  city: City,
  country: Country,
  household: Household,
  schoolType: SchoolType,
  area: Area,
): number {
  return essentialsLocal(city, country, household, schoolType, area) + city.expenses.lifestyle.amount;
}

// ─── Relocation (local currency + USD) ──────────────────────────────────────

// One-time sunk costs: deposit + keyMoney + moving + visaAdmin (money actually spent).
export function relocationSunkLocal(city: City): number {
  const { deposit, keyMoney, moving, visaAdmin } = city.relocation.sunkCosts;
  return deposit.amount + keyMoney.amount + moving.amount + visaAdmin.amount;
}

export function relocationSunkUsd(city: City, fx: FxTable): number {
  return relocationSunkLocal(city) * fxToUsd(fx, city.currency);
}

// Liquidity reserve: cash cushion the user keeps (never folded into sunk costs).
export function liquidityReserveLocal(city: City): number {
  return city.relocation.liquidityReserve.cashCushion.amount;
}

export function liquidityReserveUsd(city: City, fx: FxTable): number {
  return liquidityReserveLocal(city) * fxToUsd(fx, city.currency);
}

// ─── Savings Row ─────────────────────────────────────────────────────────────

// Compute the full savings row for one city at a given gross salary and household basis.
// grossMonthlyUsd: caller's gross monthly income in USD.
// essentialSavings  = net − essentialsUsd
// afterLifestyleSavings = essentialSavings − lifestyleUsd
// Relocation figures are NOT folded into savings.
export function savingsRow(
  grossMonthlyUsd: number,
  city: City,
  country: Country,
  fx: FxTable,
  household: Household,
  schoolType: SchoolType,
  area: Area,
): SavingsRow {
  const rate = fxToUsd(fx, city.currency);
  const net = netUsd(grossMonthlyUsd, city, country, fx);
  const essLocal = essentialsLocal(city, country, household, schoolType, area);
  const essUsd = essLocal * rate;
  const lifestyleUsd = city.expenses.lifestyle.amount * rate;
  const essential = net - essUsd;
  const afterLifestyle = essential - lifestyleUsd;

  return {
    cityId: city.id,
    currency: city.currency,
    grossMonthlyUsd,
    netUsd: net,
    essentialsUsd: essUsd,
    lifestyleUsd,
    essentialSavings: essential,
    afterLifestyleSavings: afterLifestyle,
  };
}

// Sort rows descending by essentialSavings (highest savings first).
export function sortByEssentialSavings(rows: SavingsRow[]): SavingsRow[] {
  return [...rows].sort((a, b) => b.essentialSavings - a.essentialSavings);
}
