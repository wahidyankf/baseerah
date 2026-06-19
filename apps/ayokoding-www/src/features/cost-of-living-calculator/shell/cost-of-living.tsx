import { Badge, Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@open-sharia-enterprise/web-ui";
import { healthcareBadgeHue } from "../core/format";
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
import { fmtNum } from "../core/format";
import type { Locale } from "@/features/i18n/core/config";
import { t } from "@/features/i18n/core/translations";

type Props = {
  dataset: Dataset;
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

export function CostOfLivingTable({ dataset, household, schoolType, area, locale = "en" }: Props) {
  const countryById = Object.fromEntries(dataset.countries.map((c) => [c.id, c]));

  return (
    <>
      <p data-testid="oop-legend" className="mb-1 text-xs text-muted-foreground">
        {t(locale, "oopLegend")}
      </p>
      <div className="-mx-4 overflow-x-auto px-4 sm:mx-0 sm:px-0">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>{t(locale, "colCountry")}</TableHead>
              <TableHead>{t(locale, "colCity")}</TableHead>
              <TableHead>{t(locale, "colHealthcareScheme")}</TableHead>
              <TableHead>{t(locale, "colHousing")}</TableHead>
              <TableHead>{t(locale, "colFood")}</TableHead>
              <TableHead>{t(locale, "colTransport")}</TableHead>
              <TableHead>{t(locale, "colUtilities")}</TableHead>
              <TableHead>{t(locale, "colHealthcareOOP")}</TableHead>
              <TableHead>{t(locale, "colChildcare")}</TableHead>
              <TableHead>{t(locale, "colSchool")}</TableHead>
              <TableHead>{t(locale, "colEssentials")}</TableHead>
              <TableHead>{t(locale, "colTotal")}</TableHead>
              <TableHead>{t(locale, "colRelocationSunk")}</TableHead>
              <TableHead>{t(locale, "colLiquidityReserve")}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {dataset.cities.map((city) => {
              const country = countryById[city.countryId];
              const e = city.expenses;
              const housing = e.housing.amount;
              const food = e.food.amount;
              const transport = e.transport.amount;
              const utilities = e.utilities.amount;
              const healthcare = e.healthcare.amount;
              const childcare = childcareLocal(city, household);
              const school = schoolLocal(city, household, schoolType);
              const essentials = essentialsLocal(city, household, schoolType, area);
              const total = expensesLocal(city, household, schoolType, area);
              const relocation = relocationSunkLocal(city);
              const liquidity = liquidityReserveLocal(city);

              return (
                <TableRow key={city.id}>
                  <TableCell>
                    <a href={`?tab=cost&country=${city.countryId}`}>{country?.name.en ?? city.countryId}</a>
                  </TableCell>
                  <TableCell>
                    <a href={`?tab=cost&city=${city.id}`}>{city.name.en}</a>
                  </TableCell>
                  <TableCell>
                    {country ? (
                      <Badge
                        data-testid="healthcare-badge"
                        variant="outline"
                        hue={healthcareBadgeHue(country.healthcareModelType)}
                      >
                        {healthcareBadgeLabel(country.healthcareModelType, locale)}
                      </Badge>
                    ) : (
                      <span data-testid="healthcare-badge">—</span>
                    )}
                  </TableCell>
                  <TableCell className="text-right">{fmtNum(housing)}</TableCell>
                  <TableCell className="text-right">{fmtNum(food)}</TableCell>
                  <TableCell className="text-right">{fmtNum(transport)}</TableCell>
                  <TableCell className="text-right">{fmtNum(utilities)}</TableCell>
                  <TableCell className="text-right">{fmtNum(healthcare)}</TableCell>
                  <TableCell className="text-right">{fmtNum(childcare)}</TableCell>
                  <TableCell className="text-right">{fmtNum(school)}</TableCell>
                  <TableCell className="text-right font-medium">{fmtNum(essentials)}</TableCell>
                  <TableCell className="text-right font-medium">{fmtNum(total)}</TableCell>
                  <TableCell className="text-right">{fmtNum(relocation)}</TableCell>
                  <TableCell className="text-right">{fmtNum(liquidity)}</TableCell>
                </TableRow>
              );
            })}
          </TableBody>
        </Table>
      </div>
    </>
  );
}
