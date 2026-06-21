"use client";

import { Breadcrumb } from "@/features/navigation/shell/breadcrumb";
import { t } from "@/features/i18n/core/translations";
import { useLocale } from "@/features/i18n/shell/use-locale";

export function CalculatorBreadcrumb() {
  const locale = useLocale();

  // Delegate to the shared Breadcrumb primitive. showCurrent renders the final
  // segment as a non-link aria-current="page" crumb whose label matches the
  // page H1 (calcTitle), and chevron separators replace the legacy slash glyph.
  const segments = [
    { label: t(locale, "breadcrumbHome"), slug: "" },
    { label: t(locale, "toolsPageTitle"), slug: "tools" },
    { label: t(locale, "calcTitle"), slug: "tools/cost-of-living-calculator" },
  ];

  return <Breadcrumb locale={locale} slug="tools/cost-of-living-calculator" segments={segments} showCurrent />;
}
