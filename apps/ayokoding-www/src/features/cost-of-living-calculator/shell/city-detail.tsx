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
import { fmtCurrency } from "../core/format";
import type { Locale } from "@/features/i18n/core/config";
import { t } from "@/features/i18n/core/translations";

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
    <div>
      <h2>
        {city.name.en}
        {country ? `, ${country.name.en}` : ""}
      </h2>

      <a href="?tab=cost">{t(locale, "backToAllCities")}</a>

      {country && (
        <span data-testid="healthcare-badge">{healthcareBadgeLabel(country.healthcareModelType, locale)}</span>
      )}

      <section aria-label={t(locale, "sectionMonthlyExpenses")}>
        <dl>
          <dt>{t(locale, "labelHousing")}</dt>
          <dd data-testid="expense-housing">{fmtCurrency(housingAmt, cur)}</dd>

          <dt>{t(locale, "labelFood")}</dt>
          <dd data-testid="expense-food">{fmtCurrency(foodAmt, cur)}</dd>

          <dt>{t(locale, "labelTransport")}</dt>
          <dd data-testid="expense-transport">{fmtCurrency(transportAmt, cur)}</dd>

          <dt>{t(locale, "labelUtilities")}</dt>
          <dd data-testid="expense-utilities">{fmtCurrency(utilitiesAmt, cur)}</dd>

          <dt>{t(locale, "labelHealthcareOOP")}</dt>
          <dd data-testid="expense-healthcare">{fmtCurrency(healthcareAmt, cur)}</dd>

          <dt>{t(locale, "labelChildcare")}</dt>
          <dd data-testid="expense-childcare">{fmtCurrency(childcareAmt, cur)}</dd>

          <dt>{t(locale, "labelSchool")}</dt>
          <dd data-testid="expense-school">{fmtCurrency(schoolAmt, cur)}</dd>

          <dt>{t(locale, "labelEssentialsSubtotal")}</dt>
          <dd data-testid="essentials-subtotal">{fmtCurrency(essentials, cur)}</dd>

          <dt>{t(locale, "labelMonthlyTotal")}</dt>
          <dd data-testid="monthly-total">{fmtCurrency(monthlyTotal, cur)}</dd>
        </dl>
      </section>

      <section aria-label={t(locale, "sectionRelocationCosts")}>
        <dl>
          <dt>{t(locale, "labelRelocationSunkCost")}</dt>
          <dd data-testid="relocation-sunk">{fmtCurrency(relocationSunk, cur)}</dd>

          <dt>{t(locale, "labelLiquidityReserve")}</dt>
          <dd data-testid="liquidity-reserve">{fmtCurrency(liquidityReserve, cur)}</dd>
        </dl>
      </section>
    </div>
  );
}
