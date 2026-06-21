"use client";

import Link from "next/link";
import { useLocale } from "@/features/i18n/shell/use-locale";

export function CalculatorBreadcrumb() {
  const locale = useLocale();

  return (
    <nav aria-label="Breadcrumb">
      <ol className="flex items-center gap-1 text-sm text-muted-foreground">
        <li>
          <Link href={`/${locale}`} className="hover:text-foreground">
            Home
          </Link>
        </li>
        <li aria-hidden="true" className="select-none">
          /
        </li>
        <li>
          <Link href={`/${locale}/tools`} className="hover:text-foreground">
            Tools
          </Link>
        </li>
        <li aria-hidden="true" className="select-none">
          /
        </li>
        <li aria-current="page" className="font-medium text-foreground">
          Calculator
        </li>
      </ol>
    </nav>
  );
}
