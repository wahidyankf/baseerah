"use client";

import type { Dataset, Household } from "../core/data/cities";
import { subLinear, perCapita, AREA_MULTIPLIERS } from "../core/data/cities";
import type { Area, SchoolType } from "../core/calc";
import { childcareLocal, schoolLocal } from "../core/calc";
import { fmtNum } from "../core/format";
import type { Locale } from "@/features/i18n/core/config";
import { t } from "@/features/i18n/core/translations";
import { cn } from "@/lib/utils";

/** Accessible 2+-option segmented control (radiogroup) matching the hi-fi mockups. */
export function SegmentedControl<T extends string>({
  label,
  value,
  options,
  onChange,
  disabled = false,
}: {
  label: string;
  value: T;
  options: Array<{ value: T; label: string }>;
  onChange: (v: T) => void;
  disabled?: boolean;
}) {
  return (
    <div
      role="radiogroup"
      aria-label={label}
      aria-disabled={disabled || undefined}
      className={cn(
        // min-h-[44px] + items-center keeps the control the same height as the sibling 44px
        // inputs/selects so it bottom-aligns cleanly in `items-end` field rows (and meets the
        // WCAG 44px touch target), instead of sitting low because it was shorter.
        "inline-flex min-h-[44px] items-center rounded-lg border border-border bg-muted p-[3px]",
        disabled && "opacity-50",
      )}
    >
      {options.map((opt) => {
        const selected = opt.value === value;
        return (
          <button
            key={opt.value}
            type="button"
            role="radio"
            aria-checked={selected}
            aria-label={opt.label}
            disabled={disabled}
            onClick={() => onChange(opt.value)}
            className={cn(
              "rounded-md px-3 py-1 text-sm font-medium transition-colors focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none",
              selected ? "bg-primary text-primary-foreground shadow-sm" : "text-foreground/60 hover:text-foreground",
              disabled && "cursor-not-allowed",
            )}
          >
            {opt.label}
          </button>
        );
      })}
    </div>
  );
}

type Props = {
  dataset: Dataset;
  previewCityId: string;
  household: Household;
  schoolType: SchoolType;
  area: Area;
  locale?: Locale;
  /** Whether to render the per-city monthly expense breakdown preview. The cost + savings
      tabs already surface these figures in their tables, so the preview only adds value on
      the min-role tab where no per-category breakdown is otherwise shown. Defaults to true. */
  showPreview?: boolean;
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
  showPreview = true,
  onHouseholdChange,
  onSchoolTypeChange,
  onAreaChange,
}: Props) {
  const city = dataset.cities.find((c) => c.id === previewCityId) ?? dataset.cities[0]!;
  const country = dataset.countries.find((c) => c.id === city.countryId) ?? dataset.countries[0]!;
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
  const school = schoolLocal(city, country, household, schoolType);
  const total = housing + food + transport + utilities + healthcare + childcare + school;

  const adultOptions: Array<1 | 2> = [1, 2];
  const kidOptions: Array<0 | 1 | 2 | 3> = [0, 1, 2, 3];

  return (
    <div className="space-y-2">
      {/* Household selectors */}
      <div className="flex flex-wrap items-center gap-x-3 gap-y-2 [&_label]:text-sm [&_label]:font-medium [&_select]:rounded-md [&_select]:border [&_select]:border-border [&_select]:bg-background [&_select]:px-2 [&_select]:py-1 [&_select]:text-sm">
        <div className="flex items-center gap-1">
          <label htmlFor="controls-adults">{t(locale, "labelAdults")}</label>
          <select
            id="controls-adults"
            aria-label={t(locale, "labelAdults")}
            value={household.adults}
            className="min-h-[44px]"
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
        </div>

        <div className="flex items-center gap-1">
          <label htmlFor="controls-preschool">{t(locale, "labelPreschoolKids")}</label>
          <select
            id="controls-preschool"
            aria-label={t(locale, "labelPreschoolKids")}
            value={household.preschoolKids}
            className="min-h-[44px]"
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
        </div>

        <div className="flex items-center gap-1">
          <label htmlFor="controls-schoolkids">{t(locale, "labelSchoolKids")}</label>
          <select
            id="controls-schoolkids"
            aria-label={t(locale, "labelSchoolKids")}
            value={household.schoolKids}
            className="min-h-[44px]"
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
      </div>

      {/* School-type segmented control — always shown so the option is discoverable on every
          tab; disabled with a hint until the household has school-age children (no schooling
          cost to vary otherwise). */}
      <div className="mt-2 flex flex-wrap items-center gap-2">
        <span className="text-sm font-medium">{t(locale, "labelSchoolType")}</span>
        <SegmentedControl<SchoolType>
          label={t(locale, "labelSchoolType")}
          value={schoolType}
          onChange={onSchoolTypeChange}
          disabled={household.schoolKids === 0}
          options={[
            { value: "public", label: t(locale, "optPublic") },
            { value: "private", label: t(locale, "optPrivate") },
          ]}
        />
        {household.schoolKids === 0 && (
          <span className="text-xs text-muted-foreground">{t(locale, "schoolTypeHint")}</span>
        )}
        {/* When "public" is selected and the household has school-age children, explain that
            public schooling is not open to foreign residents everywhere — where it isn't, the
            private figure is charged (effectiveSchoolType in calc.ts). basis-full puts the note
            on its own line inside the flex-wrap row. */}
        {household.schoolKids > 0 && schoolType === "public" && (
          <span data-testid="foreigner-public-school-note" className="basis-full text-xs text-muted-foreground">
            {t(locale, "foreignerPublicSchoolNote")}
          </span>
        )}
      </div>

      {/* Area segmented control */}
      <div className="mt-2 flex items-center gap-2">
        <span className="text-sm font-medium whitespace-nowrap">{t(locale, "labelArea")}</span>
        <SegmentedControl<Area>
          label={t(locale, "labelArea")}
          value={area}
          onChange={onAreaChange}
          options={[
            { value: "center", label: t(locale, "optCenter") },
            { value: "rural", label: t(locale, "optRural") },
          ]}
        />
      </div>

      {/* Expense preview — labelled monthly breakdown for the preview city.
          Only shown when showPreview is set (min-role tab); the cost + savings tabs
          already surface these figures in their own tables. */}
      {showPreview && (
        <div className="space-y-1">
          <p className="text-xs font-medium text-muted-foreground">
            {city.name[locale] ?? city.name.en} — {t(locale, "previewMonthlyEstimate")}
          </p>
          <div className="flex flex-wrap gap-x-2 gap-y-1 text-sm [&>span]:inline-flex [&>span]:items-baseline [&>span]:gap-1 [&>span]:rounded [&>span]:bg-muted [&>span]:px-2 [&>span]:py-0.5">
            <span data-testid="preview-housing" data-local={String(housing)}>
              <span className="text-xs text-muted-foreground">{t(locale, "colHousing")}</span>
              {city.currency} {fmtNum(housing)}
            </span>
            <span data-testid="preview-food" data-local={String(food)}>
              <span className="text-xs text-muted-foreground">{t(locale, "colFood")}</span>
              {city.currency} {fmtNum(food)}
            </span>
            <span data-testid="preview-transport" data-local={String(transport)}>
              <span className="text-xs text-muted-foreground">{t(locale, "colTransport")}</span>
              {city.currency} {fmtNum(transport)}
            </span>
            <span data-testid="preview-utilities" data-local={String(utilities)}>
              <span className="text-xs text-muted-foreground">{t(locale, "colUtilities")}</span>
              {city.currency} {fmtNum(utilities)}
            </span>
            <span data-testid="preview-healthcare" data-local={String(healthcare)}>
              <span className="text-xs text-muted-foreground">
                {t(locale, "colHealthcareOOPPrefix")} (<abbr title="out-of-pocket">OOP</abbr>)
              </span>
              {city.currency} {fmtNum(healthcare)}
            </span>
            <span data-testid="preview-childcare" data-local={String(childcare)}>
              <span className="text-xs text-muted-foreground">{t(locale, "colChildcare")}</span>
              {city.currency} {fmtNum(childcare)}
            </span>
            <span data-testid="preview-schooling" data-local={String(school)}>
              <span className="text-xs text-muted-foreground">{t(locale, "colSchool")}</span>
              {city.currency} {fmtNum(school)}
            </span>
            <span
              data-testid="preview-total"
              data-local={String(total)}
              className="!bg-primary font-semibold !text-primary-foreground"
            >
              <span className="text-xs text-primary-foreground/80">{t(locale, "colTotal")}</span>
              {city.currency} {fmtNum(total)}
            </span>
          </div>
        </div>
      )}
    </div>
  );
}
