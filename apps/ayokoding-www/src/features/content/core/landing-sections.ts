import type { Locale } from "@/features/i18n/core/config";
import type { TreeNode } from "./types";

/**
 * A curated override for a single landing-page section card.
 *
 * Every field is optional: a section with no override still renders, taking its
 * title from the content tree and its blurb from {@link mergeLandingSections}'s
 * fallback string.
 */
export interface LandingSectionOverride {
  /** Lower numbers sort earlier; sections without an order keep tree order behind ordered ones. */
  readonly order?: number;
  /** A lucide icon name (looked up by the shell), e.g. `"code"`. */
  readonly icon?: string;
  /** When `true`, the section produces no card. */
  readonly hide?: boolean;
  /** Overrides the section blurb; otherwise the fallback string is used. */
  readonly blurb?: string;
}

/** Per-locale curated override map, keyed by the locale's own section slug. */
export type LandingSectionOverrides = Record<string, LandingSectionOverride>;

/**
 * A resolved landing-section descriptor — the merge output the shell renders as
 * a section card. Pure data: no JSX, no IO.
 */
export interface LandingSectionDescriptor {
  readonly slug: string;
  readonly title: string;
  readonly blurb: string;
  readonly icon: string | undefined;
}

/**
 * Curated overrides keyed per-locale section slug. The asymmetry is intentional
 * — `id` uses Indonesian slugs (`belajar`/`celoteh`), so its keys differ from
 * `en`'s (`learn`/`rants`). Blurbs are localized placeholder copy refined by a
 * maintainer before archival; icons name lucide glyphs resolved in the shell.
 *
 * Sections present on disk but absent here still render (title from the tree,
 * fallback blurb). Sections marked `hide` produce no card.
 */
export const LANDING_SECTION_OVERRIDES: Record<Locale, LandingSectionOverrides> = {
  en: {
    learn: { order: 1, icon: "code", blurb: "Languages, architecture, system design — by example." },
    rants: { order: 2, icon: "message-square", blurb: "Opinionated takes — a first-class section." },
  },
  id: {
    belajar: { order: 1, icon: "code", blurb: "Bahasa, arsitektur, dan desain sistem — lewat contoh." },
    celoteh: { order: 2, icon: "message-square", blurb: "Opini lugas — bagian kelas satu." },
  },
};

/**
 * Derive the ordered, visible landing-section descriptors from a locale's
 * top-level content sections and its curated override map.
 *
 * Pure function — no IO. For each input section:
 * - `hide: true` overrides drop the section entirely.
 * - `title` comes from the content tree (`_index.md` title).
 * - `blurb` is the override blurb, else `fallbackBlurb`.
 * - `icon` is the override icon, else `undefined`.
 *
 * Ordering: sections with an override `order` sort first (ascending by `order`);
 * remaining sections keep their tree order behind them. Among equal orders the
 * original tree order is preserved (stable sort).
 *
 * @param sections Top-level content sections from `getTree(locale)`.
 * @param overrides Curated overrides for this locale (e.g. `LANDING_SECTION_OVERRIDES[locale]`).
 * @param fallbackBlurb Blurb used when no override blurb exists (e.g. `t(locale, "sectionBlurbFallback")`).
 */
export function mergeLandingSections(
  sections: readonly TreeNode[],
  overrides: LandingSectionOverrides,
  fallbackBlurb: string,
): LandingSectionDescriptor[] {
  const ORDER_FALLBACK = Number.MAX_SAFE_INTEGER;

  return sections
    .map((section, index) => {
      const override = overrides[section.slug];
      return { section, override, index };
    })
    .filter(({ override }) => override?.hide !== true)
    .sort((a, b) => {
      const orderA = a.override?.order ?? ORDER_FALLBACK;
      const orderB = b.override?.order ?? ORDER_FALLBACK;
      if (orderA !== orderB) return orderA - orderB;
      return a.index - b.index;
    })
    .map(({ section, override }) => ({
      slug: section.slug,
      title: section.title,
      blurb: override?.blurb ?? fallbackBlurb,
      icon: override?.icon,
    }));
}
