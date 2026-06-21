/**
 * Permanent (308) redirects that move the old bare content URLs
 * (`/{locale}/{section}/...`) into the `/c/` content namespace
 * (`/{locale}/c/{section}/...`).
 *
 * One rule per locale + moved section, scoped with a `:path*` wildcard so that
 * loose top-level pages (`about-ayokoding`, `terms-and-conditions`, …), the
 * `/{locale}/tools` route, and the locale home are NOT matched — a blanket
 * `/{locale}/:path*` rule would wrongly swallow them.
 *
 * Section slugs are per-locale (the `id` library lives under `belajar`, not
 * `learn`); see the Locale Slug Asymmetry table in the plan tech-docs.
 *
 * `permanent: true` yields a method-preserving 308 that clients and search
 * engines cache. Spread into `next.config.ts` `redirects()` AFTER
 * `learnReorgRedirects` — the learn-reorg renames stay within `/en/learn/...`
 * and 308 first; this rule then 308s the result into `/c`. That chain is
 * acceptable and there is no exact-source duplication between the two modules.
 */
export const contentNamespaceRedirects: Array<{
  source: string;
  destination: string;
  permanent: boolean;
}> = [
  // en — moved sections: learn, rants
  { source: "/en/learn/:path*", destination: "/en/c/learn/:path*", permanent: true },
  { source: "/en/rants/:path*", destination: "/en/c/rants/:path*", permanent: true },
  // id — moved sections: belajar, celoteh, konten-video
  { source: "/id/belajar/:path*", destination: "/id/c/belajar/:path*", permanent: true },
  { source: "/id/celoteh/:path*", destination: "/id/c/celoteh/:path*", permanent: true },
  { source: "/id/konten-video/:path*", destination: "/id/c/konten-video/:path*", permanent: true },
];
