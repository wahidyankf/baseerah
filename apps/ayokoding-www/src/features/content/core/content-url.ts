import type { Locale } from "@/features/i18n/core/config";
import { normalizeSlug } from "./slug";

/**
 * Per-locale allowlist of loose top-level pages that live at the bare
 * `/{locale}/{slug}` URL (NOT under the `/c/` content namespace).
 *
 * Everything NOT in this list (and not the empty/root/`_index` slug) is treated
 * as content-tree material and is `/c/`-prefixed by {@link contentUrl}.
 *
 * The asymmetry between locales is intentional — `id` uses Indonesian slugs.
 */
export const LOOSE_PAGE_ALLOWLIST: Record<Locale, readonly string[]> = {
  en: ["about-ayokoding", "terms-and-conditions"],
  id: ["tentang-ayokoding", "syarat-dan-ketentuan"],
};

/**
 * True when `slug` is a loose top-level page for `locale` (one of the
 * {@link LOOSE_PAGE_ALLOWLIST} entries), and therefore must NOT be `/c/`-prefixed.
 */
export function isLoosePage(locale: Locale, slug: string): boolean {
  return LOOSE_PAGE_ALLOWLIST[locale].includes(normalizeSlug(slug));
}

/**
 * Map an on-disk content slug to its public URL — the single source of truth for
 * the content URL namespace.
 *
 * Rules:
 * - empty / root / `_index` slug → `/{locale}` (the locale home)
 * - loose top-level pages (see {@link LOOSE_PAGE_ALLOWLIST}) → `/{locale}/{slug}`
 * - everything else (content-tree slugs) → `/{locale}/c/{slug}`
 *
 * Pure function — no IO. Every URL emitter (content page, sidebar tree,
 * breadcrumb, prev/next, search results, sitemap, feed) imports it so the rule
 * lives in exactly one place.
 *
 * @example contentUrl("en", "learn/software-engineering") // "/en/c/learn/software-engineering"
 * @example contentUrl("en", "about-ayokoding")            // "/en/about-ayokoding"
 * @example contentUrl("en", "")                            // "/en"
 */
export function contentUrl(locale: Locale, slug: string): string {
  const normalized = normalizeSlug(slug);

  if (normalized === "" || normalized === "_index") {
    return `/${locale}`;
  }

  if (isLoosePage(locale, normalized)) {
    return `/${locale}/${normalized}`;
  }

  return `/${locale}/c/${normalized}`;
}
