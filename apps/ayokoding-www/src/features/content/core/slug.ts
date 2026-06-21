/**
 * Slug normalization shared by the two content catch-all routes
 * (`[...slug]` and `c/[...slug]`) and by {@link import("./content-url").contentUrl}.
 *
 * Pure functions — no IO.
 */

/**
 * Normalize a content slug string: trim, collapse internal whitespace away by
 * stripping a single leading and trailing slash. Returns a canonical
 * `a/b/c` (or `""`) form. The empty string represents the locale root.
 */
export function normalizeSlug(slug: string): string {
  return slug.trim().replace(/^\/+/, "").replace(/\/+$/, "");
}

/**
 * Join a Next.js catch-all `slug` array segment into the canonical bare content
 * slug. Empty / undefined arrays collapse to the root slug `""`.
 *
 * Under `/c/[...slug]` the captured segments are ALREADY the bare content slug
 * (no `c/` prefix to strip), so this is a plain normalized join.
 */
export function slugFromSegments(segments: string[] | undefined): string {
  if (!segments || segments.length === 0) return "";
  return normalizeSlug(segments.join("/"));
}
