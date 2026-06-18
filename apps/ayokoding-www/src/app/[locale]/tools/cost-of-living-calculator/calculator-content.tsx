"use client";

import { useState } from "react";
import { useSearchParams, useRouter } from "next/navigation";
import { dataset } from "@/features/cost-of-living-calculator/core/data/cities";
import { roleMatrix } from "@/features/cost-of-living-calculator/core/data/roles";
import type { Household } from "@/features/cost-of-living-calculator/core/data/cities";
import type { Area, SchoolType } from "@/features/cost-of-living-calculator/core/calc";
import type { GeoScope } from "@/features/cost-of-living-calculator/shell/geo-filters";
import { CostOfLivingTable } from "@/features/cost-of-living-calculator/shell/cost-of-living";
import { SavingsTable } from "@/features/cost-of-living-calculator/shell/savings";
import { MinRoleTable } from "@/features/cost-of-living-calculator/shell/min-role";
import { CityDetail } from "@/features/cost-of-living-calculator/shell/city-detail";
import { GeoFilters } from "@/features/cost-of-living-calculator/shell/geo-filters";
import { Controls } from "@/features/cost-of-living-calculator/shell/controls";
import { useLocale } from "@/features/i18n/shell/use-locale";
import { t } from "@/features/i18n/core/translations";

type Tab = "cost" | "savings" | "min-role";

function parseTab(val: string | null): Tab {
  if (val === "savings" || val === "min-role") return val;
  return "cost";
}

export function CostOfLivingCalculatorContent() {
  const searchParams = useSearchParams();
  const router = useRouter();
  const locale = useLocale();

  const initialCityId = searchParams.get("city");
  const initialCountryId = searchParams.get("country");

  const [activeTab, setActiveTab] = useState<Tab>(() => parseTab(searchParams.get("tab")));
  // city wins over country when both present
  const [detailCityId, setDetailCityId] = useState<string | null>(initialCityId);
  const [activeCountryId, setActiveCountryId] = useState<string | null>(initialCityId ? null : initialCountryId);

  const [household, setHousehold] = useState<Household>({
    adults: 1,
    preschoolKids: 0,
    schoolKids: 0,
  });
  const [schoolType, setSchoolType] = useState<SchoolType>("public");
  const [area, setArea] = useState<Area>("center");

  const [geoScope, setGeoScope] = useState<GeoScope>({
    region: null,
    countryId: initialCityId ? null : initialCountryId,
    cityId: initialCityId,
  });

  // Build scoped dataset for filtered table views
  const scopedCities = (() => {
    if (geoScope.countryId) return dataset.cities.filter((c) => c.countryId === geoScope.countryId);
    if (geoScope.region) return dataset.cities.filter((c) => c.region === geoScope.region);
    return dataset.cities;
  })();
  const scopedDataset = { ...dataset, cities: scopedCities };
  const cityScope = scopedCities === dataset.cities ? null : scopedCities;

  // Event delegation: intercept city/country <a> link clicks inside tab content
  function handleTableClick(e: React.MouseEvent) {
    const a = (e.target as HTMLElement).closest("a");
    if (!a) return;
    const href = a.getAttribute("href") ?? "";
    if (!href.startsWith("?")) return;
    const params = new URLSearchParams(href.slice(1));
    if (params.has("city")) {
      e.preventDefault();
      const cityId = params.get("city")!;
      setDetailCityId(cityId);
      setActiveCountryId(null);
      setActiveTab("cost");
      router.replace(`?tab=cost&city=${cityId}`);
    } else if (params.has("country")) {
      e.preventDefault();
      const countryId = params.get("country")!;
      setActiveCountryId(countryId);
      setDetailCityId(null);
      setGeoScope((prev) => ({ ...prev, countryId, cityId: null }));
      router.replace(`?tab=cost&country=${countryId}`);
    }
  }

  const firstCity = dataset.cities[0]!;

  // suppress unused variable warning — activeCountryId drives URL state via router.replace
  void activeCountryId;

  return (
    <main data-testid="calc-page">
      <h1>{t(locale, "calcTitle")}</h1>

      {/* Tab navigation */}
      <nav aria-label={t(locale, "ariaTabsNav")}>
        <button
          role="tab"
          aria-selected={activeTab === "cost"}
          onClick={() => {
            setActiveTab("cost");
            setDetailCityId(null);
          }}
        >
          {t(locale, "tabCostOfLiving")}
        </button>
        <button role="tab" aria-selected={activeTab === "savings"} onClick={() => setActiveTab("savings")}>
          {t(locale, "tabSavings")}
        </button>
        <button role="tab" aria-selected={activeTab === "min-role"} onClick={() => setActiveTab("min-role")}>
          {t(locale, "tabMinRole")}
        </button>
      </nav>

      {/* Shared geo filters */}
      <GeoFilters
        dataset={dataset}
        locale={locale}
        onScopeChange={(scope) => {
          setGeoScope(scope);
          setActiveCountryId(scope.countryId);
          if (scope.cityId) setDetailCityId(scope.cityId);
        }}
      />

      {/* Shared cost-basis controls */}
      <Controls
        dataset={dataset}
        previewCityId={detailCityId ?? firstCity.id}
        household={household}
        schoolType={schoolType}
        area={area}
        locale={locale}
        onHouseholdChange={setHousehold}
        onSchoolTypeChange={setSchoolType}
        onAreaChange={setArea}
      />

      {/* Data last updated + estimates disclaimer */}
      <p data-testid="data-last-updated" className="text-xs text-muted-foreground">
        {t(locale, "dataLastUpdated")}:{" "}
        {new Intl.DateTimeFormat(locale === "id" ? "id-ID" : "en-US", {
          year: "numeric",
          month: "long",
          day: "numeric",
        }).format(new Date(dataset.snapshotDate))}
        {" · "}
        <span data-testid="estimates-disclaimer">{t(locale, "estimatesOnly")}</span>
      </p>

      {/* Tab content — event delegation intercepts link clicks */}
      <div onClick={handleTableClick}>
        {activeTab === "cost" && detailCityId && (
          <div data-testid="city-detail">
            <CityDetail
              dataset={dataset}
              cityId={detailCityId}
              household={household}
              schoolType={schoolType}
              area={area}
              locale={locale}
            />
          </div>
        )}

        {activeTab === "cost" && !detailCityId && (
          <CostOfLivingTable
            dataset={scopedDataset}
            household={household}
            schoolType={schoolType}
            area={area}
            locale={locale}
          />
        )}

        {activeTab === "savings" && (
          <SavingsTable
            dataset={dataset}
            matrix={roleMatrix}
            household={household}
            schoolType={schoolType}
            area={area}
            locale={locale}
          />
        )}

        {activeTab === "min-role" && (
          <MinRoleTable
            dataset={dataset}
            matrix={roleMatrix}
            household={household}
            schoolType={schoolType}
            area={area}
            cityScope={cityScope}
            locale={locale}
          />
        )}
      </div>

      {/* Full disclaimer block */}
      <details data-testid="disclaimer-block">
        <summary className="cursor-pointer text-xs text-muted-foreground">
          {t(locale, "estimatesOnly")} — disclaimer
        </summary>
        <ul className="mt-1 space-y-1 text-xs text-muted-foreground">
          <li>{t(locale, "disclaimerPension")}</li>
          <li>{t(locale, "disclaimerClothing")}</li>
          <li>{t(locale, "disclaimerFx")}</li>
          <li>{t(locale, "disclaimerSnapshot")}</li>
          <li>{t(locale, "disclaimerTax")}</li>
          <li>{t(locale, "disclaimerHealthcare")}</li>
          <li>{t(locale, "disclaimerRelocation")}</li>
          <li>{t(locale, "disclaimerRoleSalary")}</li>
          <li>{t(locale, "disclaimerNonSalary")}</li>
        </ul>
      </details>
    </main>
  );
}
