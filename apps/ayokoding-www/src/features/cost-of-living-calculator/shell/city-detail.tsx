"use client";

import type { Dataset, Household } from "../core/data/cities";
import type { Area, SchoolType } from "../core/calc";
import {
  childcareLocal,
  essentialsLocal,
  expensesLocal,
  liquidityReserveLocal,
  relocationSunkLocal,
  schoolLocal,
} from "../core/calc";
import { fmtCurrency, healthcareBadgeHue } from "../core/format";
import type { Locale } from "@/features/i18n/core/config";
import { t } from "@/features/i18n/core/translations";
import { Badge } from "@open-sharia-enterprise/web-ui";

function Row({
  label,
  value,
  testId,
  emphasis,
}: {
  label: string;
  value: string;
  testId: string;
  emphasis?: "subtotal" | "total";
}) {
  const cls =
    emphasis === "total"
      ? "flex items-baseline justify-between border-t pt-2 text-base font-semibold"
      : emphasis === "subtotal"
        ? "flex items-baseline justify-between border-t pt-2 font-medium"
        : "flex items-baseline justify-between";
  return (
    <div className={cls}>
      <span className="text-muted-foreground">{label}</span>
      <span data-testid={testId} className="tabular-nums">
        {value}
      </span>
    </div>
  );
}

type Props = {
  dataset: Dataset;
  cityId: string;
  household: Household;
  schoolType: SchoolType;
  area: Area;
  locale?: Locale;
};

function healthcareBadgeLabel(type: "oop" | "tax-funded" | "mixed", locale: Locale): string {
  if (type === "oop") return t(locale, "healthcareOutOfPocket");
  if (type === "tax-funded") return t(locale, "healthcareTaxFunded");
  return t(locale, "healthcareMandatoryPayroll");
}

export function CityDetail({ dataset, cityId, household, schoolType, area, locale = "en" }: Props) {
  const city = dataset.cities.find((c) => c.id === cityId);
  if (!city) return <p>City not found.</p>;

  const country = dataset.countries.find((c) => c.id === city.countryId);
  const cur = city.currency;

  const e = city.expenses;
  const housingAmt = e.housing.amount;
  const foodAmt = e.food.amount;
  const transportAmt = e.transport.amount;
  const utilitiesAmt = e.utilities.amount;
  const healthcareAmt = e.healthcare.amount;
  const childcareAmt = childcareLocal(city, household);
  const schoolAmt = schoolLocal(city, household, schoolType);
  const essentials = essentialsLocal(city, household, schoolType, area);
  const monthlyTotal = expensesLocal(city, household, schoolType, area);
  const relocationSunk = relocationSunkLocal(city);
  const liquidityReserve = liquidityReserveLocal(city);

  return (
    <div className="mx-auto max-w-xl overflow-hidden rounded-lg border bg-card shadow-sm">
      {/* Colored header */}
      <div className="flex flex-wrap items-center justify-between gap-2 bg-primary px-4 py-3 text-primary-foreground">
        <div className="flex flex-col">
          <h2 className="text-lg leading-tight font-semibold">
            {city.name[locale] ?? city.name.en}
            {country ? `, ${country.name[locale] ?? country.name.en}` : ""}
          </h2>
          <a href="?tab=cost" className="text-sm text-primary-foreground/80 underline hover:text-primary-foreground">
            {t(locale, "backToAllCities")}
          </a>
        </div>
        {country && (
          <Badge
            data-testid="healthcare-badge"
            variant="outline"
            hue={healthcareBadgeHue(country.healthcareModelType)}
            className="border-white/40 bg-white/15 text-white"
          >
            {healthcareBadgeLabel(country.healthcareModelType, locale)}
          </Badge>
        )}
      </div>

      {/* Body */}
      <div className="space-y-4 p-4">
        <section aria-label={t(locale, "sectionMonthlyExpenses")} className="space-y-1.5 text-sm">
          <Row label={t(locale, "labelHousing")} value={fmtCurrency(housingAmt, cur)} testId="expense-housing" />
          <Row label={t(locale, "labelFood")} value={fmtCurrency(foodAmt, cur)} testId="expense-food" />
          <Row label={t(locale, "labelTransport")} value={fmtCurrency(transportAmt, cur)} testId="expense-transport" />
          <Row label={t(locale, "labelUtilities")} value={fmtCurrency(utilitiesAmt, cur)} testId="expense-utilities" />
          <Row
            label={t(locale, "labelHealthcareOOP")}
            value={fmtCurrency(healthcareAmt, cur)}
            testId="expense-healthcare"
          />
          <Row label={t(locale, "labelChildcare")} value={fmtCurrency(childcareAmt, cur)} testId="expense-childcare" />
          <Row label={t(locale, "labelSchool")} value={fmtCurrency(schoolAmt, cur)} testId="expense-school" />
          <Row
            label={t(locale, "labelEssentialsSubtotal")}
            value={fmtCurrency(essentials, cur)}
            testId="essentials-subtotal"
            emphasis="subtotal"
          />
          <Row
            label={t(locale, "labelMonthlyTotal")}
            value={fmtCurrency(monthlyTotal, cur)}
            testId="monthly-total"
            emphasis="total"
          />
        </section>

        <section aria-label={t(locale, "sectionRelocationCosts")} className="space-y-1.5 border-t pt-3 text-sm">
          <Row
            label={t(locale, "labelRelocationSunkCost")}
            value={fmtCurrency(relocationSunk, cur)}
            testId="relocation-sunk"
          />
          <Row
            label={t(locale, "labelLiquidityReserve")}
            value={fmtCurrency(liquidityReserve, cur)}
            testId="liquidity-reserve"
          />
        </section>
      </div>
    </div>
  );
}
