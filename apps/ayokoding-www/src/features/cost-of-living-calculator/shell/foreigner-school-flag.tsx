import { Badge } from "@open-sharia-enterprise/web-ui";
import type { Locale } from "@/features/i18n/core/config";
import { t } from "@/features/i18n/core/translations";

type Props = {
  /** City id — drives the data-testid that table + city-detail views share. */
  cityId: string;
  locale: Locale;
  /** Optional testid suffix so a second render of the same city (e.g. the mobile card view
      alongside the desktop table) gets a unique testid and does not collide with getByTestId. */
  testIdSuffix?: string;
};

// Shared warning-tone flag rendered when "public" school is selected but the country does not
// open public school to foreign residents, so the shown figure is actually the private rate.
// Factored into one component so the cost-of-living table (cost-of-living.tsx) and the
// city-detail view (city-detail.tsx) render identical wording + hierarchy and cannot drift
// (UWT-002 wording, DWT-006 hierarchy, EWT-003 parity).
//
// Styling: design-system `Badge`, SOLID (default/filled) variant with the `terracotta` warning
// hue token — NOT `text-muted-foreground`, so the flag reads as an override annotation rather than
// ordinary caption text.
//
// DWT-008: the healthcare-scheme badge in the same row uses `honey` for the common
// mixed/"mandatory payroll insurance" model (healthcareBadgeHue), so a previously honey + outline
// foreigner flag was indistinguishable from it. The flag now uses a DISTINCT register — terracotta
// (a stronger red alert tone) AND the solid filled variant (vs the healthcare badges' outline wash)
// — so the override flag stands out in both hue and weight. terracotta is a real design-system hue
// token; no raw hex. `normal-case` overrides the Badge's default uppercase so the localized
// sentence-case wording renders as written.
export function ForeignerSchoolFlag({ cityId, locale, testIdSuffix = "" }: Props) {
  return (
    <Badge
      data-testid={`school-foreigner-flag-${cityId}${testIdSuffix}`}
      variant="default"
      hue="terracotta"
      className="mt-1 max-w-full whitespace-normal normal-case"
    >
      {t(locale, "publicSchoolForeignerFlagBadge")}
    </Badge>
  );
}
