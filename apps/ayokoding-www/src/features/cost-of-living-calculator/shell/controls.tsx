"use client";

import type { Dataset, Household } from "../core/data/cities";
import { subLinear, perCapita, AREA_MULTIPLIERS } from "../core/data/cities";
import type { Area, SchoolType } from "../core/calc";
import { childcareLocal, schoolLocal } from "../core/calc";
import { fmtNum } from "../core/format";
import type { Locale } from "@/features/i18n/core/config";
import { t } from "@/features/i18n/core/translations";

type Props = {
  dataset: Dataset;
  previewCityId: string;
  household: Household;
  schoolType: SchoolType;
  area: Area;
  locale?: Locale;
  onHouseholdChange: (h: Household) => void;
  onSchoolTypeChange: (s: SchoolType) => void;
  onAreaChange: (a: Area) => void;
};

export function Controls({
  dataset,
  previewCityId,
  household,
  schoolType,
  area,
  locale = "en",
  onHouseholdChange,
  onSchoolTypeChange,
  onAreaChange,
}: Props) {
  const city = dataset.cities.find((c) => c.id === previewCityId) ?? dataset.cities[0]!;
  const e = city.expenses;
  const s = subLinear(household);
  const p = perCapita(household);
  const areaMultiplier = AREA_MULTIPLIERS[area];

  const housing = e.housing.amount * s * areaMultiplier;
  const food = e.food.amount * p;
  const transport = e.transport.amount;
  const utilities = e.utilities.amount * s;
  const healthcare = e.healthcare.amount * p;
  const childcare = childcareLocal(city, household);
  const school = schoolLocal(city, household, schoolType);
  const total = housing + food + transport + utilities + healthcare + childcare + school;

  const adultOptions: Array<1 | 2> = [1, 2];
  const kidOptions: Array<0 | 1 | 2 | 3> = [0, 1, 2, 3];

  return (
    <div>
      {/* Household selectors */}
      <div>
        <label htmlFor="controls-adults">{t(locale, "labelAdults")}</label>
        <select
          id="controls-adults"
          aria-label={t(locale, "labelAdults")}
          value={household.adults}
          onChange={(e) =>
            onHouseholdChange({
              ...household,
              adults: parseInt(e.target.value, 10) as 1 | 2,
            })
          }
        >
          {adultOptions.map((n) => (
            <option key={n} value={n}>
              {n}
            </option>
          ))}
        </select>

        <label htmlFor="controls-preschool">{t(locale, "labelPreschoolKids")}</label>
        <select
          id="controls-preschool"
          aria-label={t(locale, "labelPreschoolKids")}
          value={household.preschoolKids}
          onChange={(e) =>
            onHouseholdChange({
              ...household,
              preschoolKids: parseInt(e.target.value, 10) as 0 | 1 | 2 | 3,
            })
          }
        >
          {kidOptions.map((n) => (
            <option key={n} value={n}>
              {n}
            </option>
          ))}
        </select>

        <label htmlFor="controls-schoolkids">{t(locale, "labelSchoolKids")}</label>
        <select
          id="controls-schoolkids"
          aria-label={t(locale, "labelSchoolKids")}
          value={household.schoolKids}
          onChange={(e) =>
            onHouseholdChange({
              ...household,
              schoolKids: parseInt(e.target.value, 10) as 0 | 1 | 2 | 3,
            })
          }
        >
          {kidOptions.map((n) => (
            <option key={n} value={n}>
              {n}
            </option>
          ))}
        </select>
      </div>

      {/* School-type toggle — only when school-age kids > 0 */}
      {household.schoolKids > 0 && (
        <div>
          <label htmlFor="controls-school-type">{t(locale, "labelSchoolType")}</label>
          <select
            id="controls-school-type"
            aria-label={t(locale, "labelSchoolType")}
            value={schoolType}
            onChange={(e) => onSchoolTypeChange(e.target.value as SchoolType)}
          >
            <option value="public">{t(locale, "optPublic")}</option>
            <option value="private">{t(locale, "optPrivate")}</option>
          </select>
        </div>
      )}

      {/* Area selector */}
      <div>
        <label htmlFor="controls-area">{t(locale, "labelArea")}</label>
        <select
          id="controls-area"
          aria-label={t(locale, "labelArea")}
          value={area}
          onChange={(e) => onAreaChange(e.target.value as Area)}
        >
          <option value="center">{t(locale, "optCenter")}</option>
          <option value="rural">{t(locale, "optRural")}</option>
        </select>
      </div>

      {/* Expense preview */}
      <div>
        <span data-testid="preview-housing" data-local={String(housing)}>
          {city.currency} {fmtNum(housing)}
        </span>
        <span data-testid="preview-food" data-local={String(food)}>
          {city.currency} {fmtNum(food)}
        </span>
        <span data-testid="preview-transport" data-local={String(transport)}>
          {city.currency} {fmtNum(transport)}
        </span>
        <span data-testid="preview-utilities" data-local={String(utilities)}>
          {city.currency} {fmtNum(utilities)}
        </span>
        <span data-testid="preview-healthcare" data-local={String(healthcare)}>
          {city.currency} {fmtNum(healthcare)}
        </span>
        <span data-testid="preview-childcare" data-local={String(childcare)}>
          {city.currency} {fmtNum(childcare)}
        </span>
        <span data-testid="preview-schooling" data-local={String(school)}>
          {city.currency} {fmtNum(school)}
        </span>
        <span data-testid="preview-total" data-local={String(total)}>
          {city.currency} {fmtNum(total)}
        </span>
      </div>
    </div>
  );
}
