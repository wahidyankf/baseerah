"use client";

import { useEffect, useState } from "react";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@open-sharia-enterprise/web-ui";
import type { Dataset, Household } from "../core/data/cities";
import type { Area, SchoolType } from "../core/calc";
import { grossMonthlyToAnnual, netUsd, essentialsLocal, expensesLocal } from "../core/calc";
import type { RoleMatrix } from "../core/data/roles";
import { roleNonSalaryCompUsd } from "../core/role-lookup";
import { fx } from "../core/data/fx";
import { fmtNum } from "../core/format";
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
        <label htmlFor="gross-salary-input">{t(locale, "grossMonthlySalaryLabel")}</label>
        <input
          id="gross-salary-input"
          type="number"
          aria-label={t(locale, "grossMonthlySalaryLabel")}
          value={grossMonthly || ""}
          onChange={(e) => setGrossMonthly(parseFloat(e.target.value) || 0)}
        />
        <span>
          {t(locale, "annualGrossLabel")}: <span data-testid="annual-gross">{fmtNum(annualGross)} USD</span>
        </span>
      </div>

      <p data-testid="non-salary-comp-note" className="text-xs text-muted-foreground">
        {t(locale, "nonSalaryCompNote")}
      </p>

      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>{t(locale, "colCountry")}</TableHead>
            <TableHead>{t(locale, "colCity")}</TableHead>
            <TableHead>{t(locale, "colNet")}</TableHead>
            <TableHead>{t(locale, "colEssentials")}</TableHead>
            <TableHead>
              <button type="button" onClick={() => setSortAsc((v) => !v)} aria-label={t(locale, "sortBySavings")}>
                {t(locale, "colSavingsEssential")}
              </button>
            </TableHead>
            <TableHead>{t(locale, "colSavingsLifestyle")}</TableHead>
            <TableHead>{t(locale, "colNonSalaryComp")}</TableHead>
            <TableHead>{t(locale, "colTotalComp")}</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {sorted.map(
            ({
              city,
              country,
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
                  <a href={`?tab=cost&country=${city.countryId}`}>{country.name.en}</a>
                </TableCell>
                <TableCell>
                  <a href={`?tab=cost&city=${city.id}`}>{city.name.en}</a>
                </TableCell>
                <TableCell data-testid="net-value" data-usd={net} className="text-right">
                  {fmtNum(net)}
                  {hasSubNational && (
                    <span data-testid="sub-national-indicator" className="ml-1 text-xs">
                      {t(locale, "subNationalIndicator")}
                    </span>
                  )}
                </TableCell>
                <TableCell className="text-right">{fmtNum(essUsd)}</TableCell>
                <TableCell
                  data-testid="savings-essential"
                  data-usd={essentialSavings}
                  className={`text-right ${essentialSavings < 0 ? "text-destructive" : ""}`}
                >
                  {fmtNum(essentialSavings)} ({pct(essentialSavings, net)})
                </TableCell>
                <TableCell className="text-right">
                  {fmtNum(afterLifestyle)} ({pct(afterLifestyle, net)})
                </TableCell>
                <TableCell className="text-right">{fmtNum(nonSalaryUsd)}</TableCell>
                <TableCell className="text-right">{fmtNum(totalCompUsd)}</TableCell>
              </TableRow>
            ),
          )}
        </TableBody>
      </Table>
    </div>
  );
}
