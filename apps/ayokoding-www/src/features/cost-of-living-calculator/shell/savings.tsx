"use client";

import { useEffect, useState } from "react";
import {
  Input,
  Label,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@open-sharia-enterprise/web-ui";
import type { Dataset, Household } from "../core/data/cities";
import type { Area, SchoolType } from "../core/calc";
import { grossMonthlyToAnnual, netUsd, essentialsLocal, expensesLocal } from "../core/calc";
import type { RoleMatrix } from "../core/data/roles";
import { roleNonSalaryCompUsd } from "../core/role-lookup";
import { fx } from "../core/data/fx";
import { fmtNum, fmtDualCurrency } from "../core/format";
import { localeName } from "./geo-filters";
import type { Locale } from "@/features/i18n/core/config";
import { t } from "@/features/i18n/core/translations";

type Props = {
  dataset: Dataset;
  matrix: RoleMatrix;
  household: Household;
  schoolType: SchoolType;
  area: Area;
  locale?: Locale;
};

function pct(part: number, whole: number): string {
  if (whole <= 0) return "—";
  return `${Math.round((part / whole) * 100)}%`;
}

// One labelled row inside a mobile savings card.
function CardRow({ label, value, negative }: { label: string; value: string; negative?: boolean }) {
  return (
    <div className="flex items-baseline justify-between text-sm">
      <span className="text-muted-foreground">{label}</span>
      <span className={negative ? "text-destructive tabular-nums" : "tabular-nums"}>{value}</span>
    </div>
  );
}

// Use "senior_swe" as the canonical "typical" non-salary comp reference
const REFERENCE_ROLE = "senior_swe" as const;

export function SavingsTable({ dataset, matrix, household, schoolType, area, locale = "en" }: Props) {
  const [grossMonthly, setGrossMonthly] = useState(0);
  const [sortAsc, setSortAsc] = useState(false);
  const [hydrated, setHydrated] = useState(false);
  useEffect(() => {
    const params = new URLSearchParams(window.location.search);
    const urlGross = parseFloat(params.get("gross") ?? "0");
    if (urlGross > 0) setGrossMonthly(urlGross);
    setHydrated(true);
  }, []);

  const annualGross = grossMonthlyToAnnual(grossMonthly);
  const countryById = Object.fromEntries(dataset.countries.map((c) => [c.id, c]));

  const rows = dataset.cities.map((city) => {
    const country = countryById[city.countryId]!;
    const fxRate = fx.ratesUsdPerUnit[city.currency] ?? 1;
    const net = grossMonthly > 0 ? netUsd(grossMonthly, city, country, fx) : 0;
    const essLocal = essentialsLocal(city, household, schoolType, area);
    const essUsd = essLocal * fxRate;
    const totalExpUsd = expensesLocal(city, household, schoolType, area) * fxRate;
    const essentialSavings = net - essUsd;
    const lifestyleUsd = totalExpUsd - essUsd;
    const afterLifestyle = essentialSavings - lifestyleUsd;

    // Non-salary comp (informational): senior_swe at this country
    const nonSalaryUsd = roleNonSalaryCompUsd(fx, matrix, city, REFERENCE_ROLE);
    const totalCompUsd = annualGross + nonSalaryUsd;

    const hasSubNational = country && city.subNational !== undefined;

    return {
      city,
      country,
      fxRate,
      net,
      essUsd,
      essentialSavings,
      afterLifestyle,
      nonSalaryUsd,
      totalCompUsd,
      hasSubNational,
    };
  });

  const sorted = [...rows].sort((a, b) =>
    sortAsc ? a.essentialSavings - b.essentialSavings : b.essentialSavings - a.essentialSavings,
  );

  return (
    <div data-testid="savings-table" data-hydrated={hydrated ? "true" : undefined}>
      <div>
        <Label htmlFor="gross-salary-input">{t(locale, "grossMonthlySalaryLabel")}</Label>
        <div className="flex items-center gap-2">
          <Input
            id="gross-salary-input"
            type="number"
            min="0"
            aria-label={t(locale, "grossMonthlySalaryLabel")}
            value={grossMonthly || ""}
            onChange={(e) => setGrossMonthly(Math.max(0, parseFloat(e.target.value) || 0))}
          />
          {/* UWT-004: surface the active input currency explicitly instead of hardcoding "USD"
              into the field label. The savings model derives every figure in USD, so this is a
              fixed indicator (per Assumption A-6 — an indicator, not a selector). */}
          <span data-testid="salary-currency-indicator" className="shrink-0 text-sm text-muted-foreground">
            {t(locale, "salaryCurrencyIndicator")}
          </span>
        </div>
        {/* UWT-019: explain WHY USD is used for every city so the fixed indicator is not
            mistaken for a missing currency selector. */}
        <p data-testid="salary-currency-explanation" className="mt-1 text-xs text-muted-foreground">
          {t(locale, "salaryCurrencyExplanation")}
        </p>
        <span>
          {t(locale, "annualGrossLabel")}: <span data-testid="annual-gross">{fmtNum(annualGross)} USD</span>
        </span>
      </div>

      {grossMonthly === 0 && (
        <p data-testid="savings-empty-state" className="py-6 text-center text-sm text-muted-foreground">
          {t(locale, "savingsEmptyStateMessage")}
        </p>
      )}

      <p data-testid="non-salary-comp-note" className="text-xs text-muted-foreground">
        {t(locale, "nonSalaryCompNote")}
      </p>

      {/* Tablet + desktop (md+): table. Net / Essentials / comp columns collapse on tablet. */}
      {grossMonthly > 0 && (
        <div className="hidden overflow-x-auto md:block">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>{t(locale, "colCountry")}</TableHead>
                <TableHead>{t(locale, "colCity")}</TableHead>
                <TableHead className="hidden lg:table-cell">{t(locale, "colNet")}</TableHead>
                <TableHead className="hidden lg:table-cell">{t(locale, "colEssentials")}</TableHead>
                <TableHead>
                  <button
                    type="button"
                    onClick={() => setSortAsc((v) => !v)}
                    aria-label={t(locale, "sortBySavings")}
                    aria-pressed={sortAsc}
                  >
                    {t(locale, "colSavingsEssential")}
                  </button>
                </TableHead>
                <TableHead>{t(locale, "colSavingsLifestyle")}</TableHead>
                <TableHead className="hidden lg:table-cell">{t(locale, "colNonSalaryComp")}</TableHead>
                <TableHead className="hidden lg:table-cell">{t(locale, "colTotalComp")}</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {sorted.map(
                ({
                  city,
                  country,
                  fxRate,
                  net,
                  essUsd,
                  essentialSavings,
                  afterLifestyle,
                  nonSalaryUsd,
                  totalCompUsd,
                  hasSubNational,
                }) => (
                  <TableRow key={city.id}>
                    <TableCell>
                      <a href={`?tab=cost&country=${city.countryId}`}>{localeName(country.name, locale)}</a>
                    </TableCell>
                    <TableCell>
                      <a href={`?tab=cost&city=${city.id}`}>{localeName(city.name, locale)}</a>
                    </TableCell>
                    <TableCell data-testid="net-value" data-usd={net} className="hidden text-right lg:table-cell">
                      {fmtDualCurrency(net / fxRate, city.currency, net)}
                      {hasSubNational && (
                        <span data-testid="sub-national-indicator" className="ml-1 text-xs">
                          {t(locale, "subNationalIndicator")}
                        </span>
                      )}
                    </TableCell>
                    <TableCell className="hidden text-right lg:table-cell">
                      {fmtDualCurrency(essUsd / fxRate, city.currency, essUsd)}
                    </TableCell>
                    <TableCell
                      data-testid="savings-essential"
                      data-usd={essentialSavings}
                      className={`text-right ${essentialSavings < 0 ? "text-destructive" : ""}`}
                    >
                      {fmtDualCurrency(essentialSavings / fxRate, city.currency, essentialSavings)} (
                      {pct(essentialSavings, net)})
                    </TableCell>
                    <TableCell className="text-right">
                      {fmtDualCurrency(afterLifestyle / fxRate, city.currency, afterLifestyle)} (
                      {pct(afterLifestyle, net)})
                    </TableCell>
                    <TableCell className="hidden text-right lg:table-cell">
                      {fmtDualCurrency(nonSalaryUsd / fxRate, city.currency, nonSalaryUsd)}
                    </TableCell>
                    <TableCell className="hidden text-right lg:table-cell">
                      {fmtDualCurrency(totalCompUsd / fxRate, city.currency, totalCompUsd)}
                    </TableCell>
                  </TableRow>
                ),
              )}
            </TableBody>
          </Table>
        </div>
      )}

      {/* Mobile (<md): stacked savings cards — sort control */}
      {grossMonthly > 0 && (
        <div className="md:hidden">
          <button
            data-testid="sort-mobile"
            type="button"
            onClick={() => setSortAsc((v) => !v)}
            aria-pressed={sortAsc}
            className="mb-2 min-h-[44px] rounded-md border border-border px-3 py-2 text-sm font-medium"
          >
            {t(locale, "colSavingsEssential")} {sortAsc ? "↑" : "↓"}
          </button>
        </div>
      )}
      {grossMonthly > 0 && (
        <div data-testid="mobile-savings-cards" className="space-y-3 md:hidden">
          {sorted.map((r) => (
            <div key={r.city.id} className="overflow-hidden rounded-lg border bg-card shadow-sm">
              <div className="flex flex-wrap items-center justify-between gap-2 bg-primary px-3 py-2 text-primary-foreground">
                <a href={`?tab=cost&city=${r.city.id}`} className="font-semibold underline">
                  {r.city.name[locale] ?? r.city.name.en}
                </a>
                <span className="text-xs text-primary-foreground/80">
                  {r.country.name[locale] ?? r.country.name.en}
                </span>
              </div>
              <div className="space-y-1 p-3">
                <CardRow
                  label={t(locale, "colNet")}
                  value={fmtDualCurrency(r.net / r.fxRate, r.city.currency, r.net)}
                />
                <CardRow
                  label={t(locale, "colEssentials")}
                  value={fmtDualCurrency(r.essUsd / r.fxRate, r.city.currency, r.essUsd)}
                />
                <CardRow
                  label={t(locale, "colSavingsEssential")}
                  value={`${fmtDualCurrency(r.essentialSavings / r.fxRate, r.city.currency, r.essentialSavings)} (${pct(r.essentialSavings, r.net)})`}
                  negative={r.essentialSavings < 0}
                />
                <CardRow
                  label={t(locale, "colSavingsLifestyle")}
                  value={`${fmtDualCurrency(r.afterLifestyle / r.fxRate, r.city.currency, r.afterLifestyle)} (${pct(r.afterLifestyle, r.net)})`}
                  negative={r.afterLifestyle < 0}
                />
                <CardRow
                  label={t(locale, "colNonSalaryComp")}
                  value={fmtDualCurrency(r.nonSalaryUsd / r.fxRate, r.city.currency, r.nonSalaryUsd)}
                />
                <CardRow
                  label={t(locale, "colTotalComp")}
                  value={fmtDualCurrency(r.totalCompUsd / r.fxRate, r.city.currency, r.totalCompUsd)}
                />
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
