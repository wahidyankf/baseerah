"use client";

import type { Locale } from "@/features/i18n/core/config";
import { t } from "@/features/i18n/core/translations";

type Props = {
  locale: Locale;
};

export function SkipLink({ locale }: Props) {
  return (
    <a
      href="#main-content"
      className="sr-only focus:not-sr-only focus:absolute focus:top-4 focus:left-4 focus:z-50 focus:rounded focus:bg-primary focus:px-4 focus:py-2 focus:text-primary-foreground"
    >
      {t(locale, "skipToContent")}
    </a>
  );
}
