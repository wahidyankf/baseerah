"use client";

import { useParams } from "next/navigation";
import type { Locale } from "@/features/i18n/core/config";

export function useLocale(): Locale {
  const params = useParams();
  return (params.locale as Locale) ?? "en";
}
