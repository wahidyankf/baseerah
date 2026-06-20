import type { Locale } from "./config";

/**
 * Returns the BCP-47 language tag for the given locale,
 * suitable for the HTML `lang` attribute.
 */
export function htmlLang(locale: Locale): string {
  return locale;
}
