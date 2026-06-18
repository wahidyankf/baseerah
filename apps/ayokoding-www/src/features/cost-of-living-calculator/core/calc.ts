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

// ─── Expense Components (local currency) ────────────────────────────────────

// Monthly childcare in local currency: per pre-school child, no OECD damping.
export function childcareLocal(city: City, household: Household): number {
  return city.childcareMedianLocal.amount * household.preschoolKids;
}

// Monthly school cost in local currency: per school-age child.
export function schoolLocal(city: City, household: Household, schoolType: SchoolType): number {
  return city.schoolMedianLocal[schoolType].amount * household.schoolKids;
}

// Monthly essential expenses total in local currency.
// Essentials = housing + food + transport + utilities + healthcare + childcare + school.
export function essentialsLocal(city: City, household: Household, schoolType: SchoolType, area: Area): number {
  const e = city.expenses;
  const s = subLinear(household);
  const p = perCapita(household);
  const areaMultiplier = AREA_MULTIPLIERS[area];

  const housing = e.housing.amount * s * areaMultiplier;
  const food = e.food.amount * p;
  const transport = e.transport.amount; // flat per earner (1 transit pass assumed)
  const utilities = e.utilities.amount * s;
  const healthcare = e.healthcare.amount * p;
  const childcare = childcareLocal(city, household);
  const school = schoolLocal(city, household, schoolType);

  return housing + food + transport + utilities + healthcare + childcare + school;
}

// Monthly total expenses in local currency (essentials + lifestyle).
// Lifestyle stays flat (personal/discretionary — not household-scaled in v1).
export function expensesLocal(city: City, household: Household, schoolType: SchoolType, area: Area): number {
  return essentialsLocal(city, household, schoolType, area) + city.expenses.lifestyle.amount;
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
  const essLocal = essentialsLocal(city, household, schoolType, area);
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
