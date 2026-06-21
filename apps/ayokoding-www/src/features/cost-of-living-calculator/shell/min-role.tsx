"use client";

import { useState } from "react";
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@open-sharia-enterprise/web-ui";
import type { Dataset, Household, City } from "../core/data/cities";
import type { Area, SchoolType } from "../core/calc";
import type { RoleMeta, RoleMatrix } from "../core/data/roles";
import { rankLadder, minimumRole, orderForDisplay, resolveBaselineUsd, toDisplayCurrencies } from "../core/role-lookup";
import { fx } from "../core/data/fx";
import { fmtCurrencyTrailing } from "../core/format";
import { localeName } from "./geo-filters";
import { SegmentedControl } from "./controls";
import type { Locale } from "@/features/i18n/core/config";
import { t } from "@/features/i18n/core/translations";

type EngRole = RoleMeta["role"];

type Props = {
  dataset: Dataset;
  matrix: RoleMatrix;
  household: Household;
  schoolType: SchoolType;
  area: Area;
  cityScope: City[] | null;
  locale?: Locale;
};

const DISPLAY_CURRENCIES = ["USD", "EUR", "SGD", "IDR", "GBP", "JPY", "CAD", "AED"];

export function MinRoleTable({ dataset, matrix, household, schoolType, area, cityScope, locale = "en" }: Props) {
  const [baselineSource, setBaselineSource] = useState<"savings_target" | "reference_role" | "my_salary">(
    "savings_target",
  );
  const [targetAmount, setTargetAmount] = useState(0);
  const [targetCurrency, setTargetCurrency] = useState("USD");
  const [refCityId, setRefCityId] = useState(dataset.cities[0]?.id ?? "");
  const [refRole, setRefRole] = useState<EngRole>("senior_swe");
  const [myGrossMonthly, setMyGrossMonthly] = useState(0);
  const [mySalaryCityId, setMySalaryCityId] = useState(dataset.cities[0]?.id ?? "");
  const [displayCurrency, setDisplayCurrency] = useState("USD");

  const opts = { household, schoolType, area };
  const ranked = rankLadder(dataset, opts, matrix, cityScope);

  let baselineUsd = 0;
  let baselineReady = false;

  try {
    if (baselineSource === "savings_target" && targetAmount >= 0) {
      baselineUsd = resolveBaselineUsd(
        "savings_target",
        { amountLocal: targetAmount, displayCurrency: targetCurrency },
        opts,
        dataset,
        matrix,
      );
      baselineReady = true;
    } else if (baselineSource === "reference_role" && refCityId) {
      baselineUsd = resolveBaselineUsd(
        "reference_role",
        { role: refRole as EngRole, cityId: refCityId },
        opts,
        dataset,
        matrix,
      );
      baselineReady = true;
    } else if (baselineSource === "my_salary" && myGrossMonthly > 0 && mySalaryCityId) {
      baselineUsd = resolveBaselineUsd("my_salary", { grossMonthlyUsd: myGrossMonthly }, opts, dataset, matrix);
      baselineReady = true;
    }
  } catch {
    baselineReady = false;
  }

  const minRole = baselineReady ? minimumRole(baselineUsd, ranked) : null;
  const ordered = orderForDisplay(ranked, minRole);
  const qualifying = ordered.filter((e) => e.clears);
  const nonQualifying = ordered.filter((e) => !e.clears);
  const noQualifiers = baselineReady && minRole === null;

  // EWT-001: the qualifying divider anchors the qualifying group whenever a baseline is engaged and
  // at least one role qualifies — including the numeric zero-target case where EVERY role clears and
  // `nonQualifying` is empty. (Previously the divider required `nonQualifying.length > 0`, so at
  // target 0 it disappeared even though the qualifying group was non-empty.) Single source of truth
  // for both the desktop table and the mobile cards below.
  const showDivider = baselineReady && qualifying.length > 0;

  function DualCell({ usdVal, cityCurrency, className }: { usdVal: number; cityCurrency: string; className?: string }) {
    const conv = toDisplayCurrencies(fx, usdVal, cityCurrency, displayCurrency);
    return (
      <TableCell data-testid="dual-currency-cell" className={className}>
        <span data-line="display">{fmtCurrencyTrailing(conv.display, displayCurrency)}</span>
        <span data-line="local" className="block text-xs text-muted-foreground">
          {fmtCurrencyTrailing(conv.local, cityCurrency)}
        </span>
      </TableCell>
    );
  }

  function SavingsCell({ entry }: { entry: (typeof ordered)[0] }) {
    const conv = toDisplayCurrencies(fx, entry.bestEssentialSavingsUsd, entry.bestCity.currency, displayCurrency);
    return (
      <TableCell data-testid="savings-triple">
        <span data-line="usd">{fmtCurrencyTrailing(entry.bestEssentialSavingsUsd, "USD")}</span>
        {displayCurrency !== "USD" && (
          <span data-line="display" className="block text-xs">
            {fmtCurrencyTrailing(conv.display, displayCurrency)}
          </span>
        )}
        <span data-line="local" className="block text-xs text-muted-foreground">
          {fmtCurrencyTrailing(conv.local, entry.bestCity.currency)}
        </span>
      </TableCell>
    );
  }

  function RoleRow({ entry, isMin, dimmed }: { entry: (typeof ordered)[0]; isMin: boolean; dimmed: boolean }) {
    const rowLabel = matrix.ladder.find((r) => r.role === entry.role)?.label.en ?? entry.role;
    return (
      <TableRow
        key={entry.role}
        data-testid={dimmed ? "non-qualifying-row" : undefined}
        className={dimmed ? "opacity-50" : undefined}
      >
        <TableCell>
          {rowLabel}
          {isMin && (
            <span data-testid="minimum-marker" className="ml-1 text-xs font-bold">
              {t(locale, "minimumMarker")}
            </span>
          )}
        </TableCell>
        <TableCell className="hidden lg:table-cell">{entry.track}</TableCell>
        <TableCell data-testid="best-city-cell">
          {localeName(entry.bestCity.name, locale)}, {localeName(entry.bestCountry.name, locale)}
          {(entry.confidence === "proxy" || entry.confidence === "moderate") && (
            <span data-testid="confidence-flag" className="ml-1 text-xs text-muted-foreground">
              [{entry.confidence}]
            </span>
          )}
        </TableCell>
        <DualCell
          usdVal={entry.distributionUsd.p25}
          cityCurrency={entry.bestCity.currency}
          className="hidden lg:table-cell"
        />
        <DualCell usdVal={entry.distributionUsd.median} cityCurrency={entry.bestCity.currency} />
        <DualCell
          usdVal={entry.distributionUsd.p75}
          cityCurrency={entry.bestCity.currency}
          className="hidden lg:table-cell"
        />
        <SavingsCell entry={entry} />
        <TableCell className="hidden text-right lg:table-cell">
          {fmtCurrencyTrailing(entry.nonSalaryCompUsd, "USD")}
        </TableCell>
      </TableRow>
    );
  }

  function MobileRoleCard({ entry, isMin, dimmed }: { entry: (typeof ordered)[0]; isMin: boolean; dimmed: boolean }) {
    const rowLabel = matrix.ladder.find((r) => r.role === entry.role)?.label.en ?? entry.role;
    const med = toDisplayCurrencies(fx, entry.distributionUsd.median, entry.bestCity.currency, displayCurrency);
    const sav = toDisplayCurrencies(fx, entry.bestEssentialSavingsUsd, entry.bestCity.currency, displayCurrency);
    return (
      <div className={`overflow-hidden rounded-lg border bg-card shadow-sm ${dimmed ? "opacity-60" : ""}`}>
        <div className="flex flex-wrap items-center justify-between gap-2 bg-primary px-3 py-2 text-primary-foreground">
          <span className="font-semibold">
            {rowLabel}
            {isMin && <span className="ml-1 text-xs font-bold">{t(locale, "minimumMarker")}</span>}
          </span>
          <span className="text-xs text-primary-foreground/80">{entry.track}</span>
        </div>
        <div className="space-y-1 p-3 text-sm">
          <div className="flex items-baseline justify-between">
            <span className="text-muted-foreground">{t(locale, "colBestCity")}</span>
            <span>
              {localeName(entry.bestCity.name, locale)}, {localeName(entry.bestCountry.name, locale)}
            </span>
          </div>
          <div className="flex items-baseline justify-between">
            <span className="text-muted-foreground">{t(locale, "colMedian")}</span>
            <span className="tabular-nums">{fmtCurrencyTrailing(med.display, displayCurrency)}</span>
          </div>
          <div className="flex items-baseline justify-between border-t pt-1.5 font-medium">
            <span className="text-muted-foreground">{t(locale, "colEssentialSavings")}</span>
            <span className="tabular-nums">{fmtCurrencyTrailing(sav.display, displayCurrency)}</span>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div>
      {/* Baseline source */}
      <div>
        <p className="mb-1 text-sm font-medium text-foreground">{t(locale, "labelBaselineSource")}</p>
        <SegmentedControl
          label={t(locale, "labelBaselineSource")}
          value={baselineSource}
          onChange={(v) => setBaselineSource(v)}
          options={[
            { value: "savings_target" as const, label: t(locale, "optSavingsTarget") },
            { value: "reference_role" as const, label: t(locale, "optReferenceRole") },
            { value: "my_salary" as const, label: t(locale, "optMySalary") },
          ]}
        />
      </div>

      {/* Savings target inputs */}
      {baselineSource === "savings_target" && (
        <div>
          <label htmlFor="target-amount-input">{t(locale, "labelMonthlySavingsTarget")}</label>
          <input
            id="target-amount-input"
            type="number"
            aria-label={t(locale, "labelMonthlySavingsTarget")}
            value={targetAmount || ""}
            onChange={(e) => setTargetAmount(parseFloat(e.target.value) || 0)}
          />
          <select
            aria-label={t(locale, "labelTargetCurrency")}
            value={targetCurrency}
            onChange={(e) => setTargetCurrency(e.target.value)}
          >
            {DISPLAY_CURRENCIES.map((c) => (
              <option key={c} value={c}>
                {c}
              </option>
            ))}
          </select>
        </div>
      )}

      {/* Reference role inputs */}
      {baselineSource === "reference_role" && (
        <div>
          <label htmlFor="ref-city-select">{t(locale, "labelRefCity")}</label>
          <select
            id="ref-city-select"
            aria-label={t(locale, "labelRefCity")}
            value={refCityId}
            onChange={(e) => setRefCityId(e.target.value)}
          >
            {dataset.cities.map((c) => (
              <option key={c.id} value={c.id}>
                {localeName(c.name, locale)}
              </option>
            ))}
          </select>

          <label htmlFor="ref-role-select">{t(locale, "labelRefRole")}</label>
          <select
            id="ref-role-select"
            aria-label={t(locale, "labelRefRole")}
            value={refRole}
            onChange={(e) => setRefRole(e.target.value as EngRole)}
          >
            {matrix.ladder.map((r) => (
              <option key={r.role} value={r.role}>
                {r.label.en}
              </option>
            ))}
          </select>
        </div>
      )}

      {/* My salary inputs */}
      {baselineSource === "my_salary" && (
        <div>
          <label htmlFor="my-gross-input">{t(locale, "labelMyGrossMonthly")}</label>
          <input
            id="my-gross-input"
            type="number"
            aria-label={t(locale, "labelMyGrossMonthly")}
            value={myGrossMonthly || ""}
            onChange={(e) => setMyGrossMonthly(parseFloat(e.target.value) || 0)}
          />

          <label htmlFor="my-city-select">{t(locale, "labelMySalaryCity")}</label>
          <select
            id="my-city-select"
            aria-label={t(locale, "labelMySalaryCity")}
            value={mySalaryCityId}
            onChange={(e) => setMySalaryCityId(e.target.value)}
          >
            {dataset.cities.map((c) => (
              <option key={c.id} value={c.id}>
                {localeName(c.name, locale)}
              </option>
            ))}
          </select>
        </div>
      )}

      {/* Display currency */}
      <div>
        <label htmlFor="display-currency-select">{t(locale, "labelDisplayCurrency")}</label>
        <select
          id="display-currency-select"
          aria-label={t(locale, "labelDisplayCurrency")}
          value={displayCurrency}
          onChange={(e) => setDisplayCurrency(e.target.value)}
        >
          {DISPLAY_CURRENCIES.map((c) => (
            <option key={c} value={c}>
              {c}
            </option>
          ))}
        </select>
      </div>

      {/* Notes */}
      <p data-testid="rank-basis-note" className="text-xs">
        {t(locale, "rankBasisNote")}
      </p>
      <p data-testid="non-salary-rank-note" className="text-xs">
        {t(locale, "nonSalaryRankNote")}
      </p>

      {/* No qualifiers message */}
      {noQualifiers && <p data-testid="no-qualifier-message">{t(locale, "noQualifierMessage")}</p>}

      {/* Tablet + desktop (md+): table. Track / P25 / P75 / non-salary columns collapse on tablet. */}
      <div className="hidden overflow-x-auto md:block">
        <Table>
          <TableCaption data-testid="se-roles-caption">{t(locale, "seRolesCaption")}</TableCaption>
          <TableHeader>
            <TableRow>
              <TableHead>{t(locale, "colRole")}</TableHead>
              <TableHead className="hidden lg:table-cell">{t(locale, "colTrack")}</TableHead>
              <TableHead>{t(locale, "colBestCity")}</TableHead>
              <TableHead className="hidden lg:table-cell">{t(locale, "colP25")}</TableHead>
              <TableHead>{t(locale, "colMedian")}</TableHead>
              <TableHead className="hidden lg:table-cell">{t(locale, "colP75")}</TableHead>
              <TableHead>{t(locale, "colEssentialSavings")}</TableHead>
              <TableHead className="hidden lg:table-cell">{t(locale, "colNonSalaryCompInfo")}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {qualifying.map((entry) => (
              <RoleRow key={entry.role} entry={entry} isMin={entry.role === minRole} dimmed={false} />
            ))}

            {showDivider && (
              <TableRow data-testid="qualifying-divider">
                <TableCell colSpan={8} className="text-center text-xs text-muted-foreground">
                  {t(locale, "qualifyingDivider")}
                </TableCell>
              </TableRow>
            )}

            {nonQualifying.map((entry) => (
              <RoleRow key={entry.role} entry={entry} isMin={false} dimmed={true} />
            ))}
          </TableBody>
        </Table>
      </div>

      {/* Mobile (<md): stacked role cards (qualifying first, divider, then dimmed below-minimum) */}
      <div data-testid="mobile-role-cards" className="space-y-3 md:hidden">
        {qualifying.map((entry) => (
          <MobileRoleCard key={entry.role} entry={entry} isMin={entry.role === minRole} dimmed={false} />
        ))}

        {showDivider && (
          <p data-testid="qualifying-divider-mobile" className="text-center text-xs text-muted-foreground">
            {t(locale, "qualifyingDivider")}
          </p>
        )}

        {nonQualifying.map((entry) => (
          <MobileRoleCard key={entry.role} entry={entry} isMin={false} dimmed={true} />
        ))}
      </div>
    </div>
  );
}
