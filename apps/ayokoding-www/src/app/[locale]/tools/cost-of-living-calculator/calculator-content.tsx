"use client";

import { useEffect } from "react";
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
import { CalculatorBreadcrumb } from "@/features/cost-of-living-calculator/shell/calculator-breadcrumb";
import { useLocale } from "@/features/i18n/shell/use-locale";
import { t } from "@/features/i18n/core/translations";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@open-sharia-enterprise/web-ui";
import {
  decodeState,
  encodeState,
  applyCountryChange,
  applyCityChange,
  parentScopeParams,
} from "@/features/cost-of-living-calculator/core/url-state";
import type { CalculatorState } from "@/features/cost-of-living-calculator/core/url-state";

export function CostOfLivingCalculatorContent() {
  const searchParams = useSearchParams();
  const router = useRouter();
  const locale = useLocale();

  // URL is the single source of truth. Derive ALL state from decoded URL params.
  const currentState = decodeState(new URLSearchParams(searchParams.toString()), dataset);

  const { tab: activeTab, region, countryId, cityId, household, schoolType, area } = currentState;

  // Canonicalize on mount: if decoded state differs from raw URL, replace with canonical form.
  useEffect(() => {
    const rawParams = new URLSearchParams(searchParams.toString());
    const canonicalParams = encodeState(currentState);
    if (rawParams.toString() !== canonicalParams.toString()) {
      const qs = canonicalParams.toString();
      router.replace(qs ? `?${qs}` : "?");
    }
    // Run only on mount — exhaustive deps would cause loops; eslint-disable is intentional.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // Helper: encode new state and push to URL history.
  function pushState(next: CalculatorState) {
    const params = encodeState(next);
    const qs = params.toString();
    router.push(qs ? `?${qs}` : "?");
  }

  // Build scoped dataset for filtered table views.
  // City selection is the narrowest scope and must win — a city-only filter
  // (Country = "All countries") still scopes candidates to that single city.
  const scopedCities = (() => {
    if (cityId) return dataset.cities.filter((c) => c.id === cityId);
    if (countryId) return dataset.cities.filter((c) => c.countryId === countryId);
    if (region) return dataset.cities.filter((c) => c.region === region);
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
      const newCityId = params.get("city")!;
      const next = applyCityChange({ ...currentState, tab: "cost" }, newCityId, dataset);
      pushState(next);
    } else if (params.has("country")) {
      e.preventDefault();
      const newCountryId = params.get("country")!;
      const next = applyCountryChange({ ...currentState, tab: "cost" }, newCountryId, dataset);
      pushState(next);
    }
  }

  const firstCity = dataset.cities[0]!;

  function handleTabChange(value: string) {
    const next = value as CalculatorState["tab"];
    // Tab change: clear cityId when moving away from cost tab
    const nextState: CalculatorState = {
      ...currentState,
      tab: next,
      cityId: next === "cost" ? cityId : null,
    };
    pushState(nextState);
  }

  function handleScopeChange(scope: GeoScope) {
    const nextState: CalculatorState = {
      ...currentState,
      region: scope.region,
      countryId: scope.countryId,
      cityId: scope.cityId,
    };
    pushState(nextState);
  }

  function handleHouseholdChange(h: Household) {
    pushState({ ...currentState, household: h });
  }

  function handleSchoolTypeChange(s: SchoolType) {
    pushState({ ...currentState, schoolType: s });
  }

  function handleAreaChange(a: Area) {
    pushState({ ...currentState, area: a });
  }

  // Show city detail view when a city is selected on the cost tab
  const detailCityId = activeTab === "cost" ? cityId : null;

  // Back link for city detail: encode parent geo scope (region+country, no city).
  // Falls back to ?tab=cost when no geo scope is set.
  const cityDetailBackHref = (() => {
    const p = parentScopeParams(currentState);
    const qs = p.toString();
    return qs ? `?${qs}` : "?tab=cost";
  })();

  return (
    <main data-testid="calc-page" className="mx-auto max-w-6xl space-y-4 px-4 py-6">
      <CalculatorBreadcrumb />
      <h1 className="text-2xl font-bold tracking-tight">{t(locale, "calcTitle")}</h1>
      <p data-testid="calc-subtitle" className="text-sm text-muted-foreground">
        {t(locale, "calcSubtitle")}
      </p>

      <Tabs value={activeTab} onValueChange={handleTabChange}>
        {/* Colored segmented tab control — active tab uses ayokoding brand primary (blue) */}
        <TabsList aria-label={t(locale, "ariaTabsNav")} className="overflow-x-auto">
          <TabsTrigger
            value="cost"
            className="data-[state=active]:bg-primary data-[state=active]:text-primary-foreground dark:data-[state=active]:!bg-primary dark:data-[state=active]:!text-primary-foreground"
          >
            {t(locale, "tabCostOfLiving")}
          </TabsTrigger>
          <TabsTrigger
            value="savings"
            aria-describedby="tab-desc-savings"
            className="data-[state=active]:bg-primary data-[state=active]:text-primary-foreground dark:data-[state=active]:!bg-primary dark:data-[state=active]:!text-primary-foreground"
          >
            {t(locale, "tabSavings")}
          </TabsTrigger>
          <TabsTrigger
            value="min-role"
            aria-describedby="tab-desc-min-role"
            className="data-[state=active]:bg-primary data-[state=active]:text-primary-foreground dark:data-[state=active]:!bg-primary dark:data-[state=active]:!text-primary-foreground"
          >
            {t(locale, "tabMinRole")}
          </TabsTrigger>
        </TabsList>
        <span id="tab-desc-savings" data-testid="tab-desc-savings" className="sr-only">
          {t(locale, "tabSavingsDesc")}
        </span>
        <span id="tab-desc-min-role" data-testid="tab-desc-min-role" className="sr-only">
          {t(locale, "tabMinRoleDesc")}
        </span>
        {activeTab !== "cost" && (
          <p className="mt-1 text-sm text-muted-foreground" aria-hidden="true">
            {activeTab === "savings" ? t(locale, "tabSavingsDesc") : t(locale, "tabMinRoleDesc")}
          </p>
        )}

        {/* Shared geo filters — fully controlled, reads from URL-derived state */}
        <GeoFilters
          dataset={dataset}
          locale={locale}
          region={region}
          countryId={countryId}
          cityId={cityId}
          onScopeChange={handleScopeChange}
        />

        {/* Shared cost-basis controls */}
        <Controls
          dataset={dataset}
          previewCityId={detailCityId ?? firstCity.id}
          household={household}
          schoolType={schoolType}
          area={area}
          locale={locale}
          onHouseholdChange={handleHouseholdChange}
          onSchoolTypeChange={handleSchoolTypeChange}
          onAreaChange={handleAreaChange}
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
          <TabsContent value="cost">
            {detailCityId ? (
              <div data-testid="city-detail">
                <CityDetail
                  dataset={dataset}
                  cityId={detailCityId}
                  household={household}
                  schoolType={schoolType}
                  area={area}
                  locale={locale}
                  backHref={cityDetailBackHref}
                />
              </div>
            ) : (
              <CostOfLivingTable
                dataset={scopedDataset}
                household={household}
                schoolType={schoolType}
                area={area}
                locale={locale}
              />
            )}
          </TabsContent>

          <TabsContent value="savings">
            <SavingsTable
              dataset={dataset}
              matrix={roleMatrix}
              household={household}
              schoolType={schoolType}
              area={area}
              locale={locale}
            />
          </TabsContent>

          <TabsContent value="min-role">
            <MinRoleTable
              dataset={dataset}
              matrix={roleMatrix}
              household={household}
              schoolType={schoolType}
              area={area}
              cityScope={cityScope}
              locale={locale}
            />
          </TabsContent>
        </div>
      </Tabs>

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
