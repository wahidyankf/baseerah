import Link from "next/link";
import { t } from "@/features/i18n/core/translations";
import type { Locale } from "@/features/i18n/core/config";

export default async function ToolsIndexPage({ params }: { params: Promise<{ locale: Locale }> }) {
  const { locale } = await params;
  return (
    <main className="mx-auto max-w-6xl space-y-4 px-4 py-6">
      <h1 className="text-2xl font-bold tracking-tight">{t(locale, "toolsPageTitle")}</h1>
      <ul className="space-y-2">
        <li>
          <Link href="./tools/cost-of-living-calculator" className="text-primary underline">
            {t(locale, "toolsPageCalcLink")}
          </Link>
        </li>
      </ul>
    </main>
  );
}
