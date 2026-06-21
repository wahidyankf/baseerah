import Link from "next/link";
import { ArrowRight } from "lucide-react";
import { Button } from "@open-sharia-enterprise/web-ui";
import type { Locale } from "@/features/i18n/core/config";
import { t } from "@/features/i18n/core/translations";

interface ToolsTeaserProps {
  locale: Locale;
}

/**
 * Landing tools teaser — a single highlighted card promoting the cost-of-living
 * calculator. Kicker/title/description/CTA all resolve through {@link t}; the
 * CTA links straight to the calculator. Composes the existing `bg-accent`
 * token surface (no net-new primitive).
 */
export function ToolsTeaser({ locale }: ToolsTeaserProps) {
  return (
    <section className="px-6 py-10 lg:px-8">
      <div className="mx-auto max-w-6xl">
        <div className="flex flex-col gap-6 rounded-xl border border-primary/15 bg-primary/5 p-8 sm:flex-row sm:items-center sm:justify-between">
          <div className="min-w-0">
            <p className="text-sm font-semibold tracking-wide text-primary uppercase">
              {t(locale, "toolsTeaserKicker")}
            </p>
            <h2 className="mt-1 text-2xl font-bold tracking-tight">{t(locale, "toolsTeaserTitle")}</h2>
            <p className="mt-2 max-w-xl text-muted-foreground">{t(locale, "toolsTeaserDesc")}</p>
          </div>
          <Button asChild size="lg" className="shrink-0">
            <Link href={`/${locale}/tools/cost-of-living-calculator`}>
              {t(locale, "toolsTeaserCta")}
              <ArrowRight className="h-4 w-4" aria-hidden="true" />
            </Link>
          </Button>
        </div>
      </div>
    </section>
  );
}
