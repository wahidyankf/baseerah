import { describe, expect, it } from "vitest";
import { contentNamespaceRedirects } from "./content-namespace";

describe("contentNamespaceRedirects", () => {
  it("declares at least one rule per moved section", () => {
    // en: learn, rants ; id: belajar, celoteh, konten-video
    expect(contentNamespaceRedirects.length).toBeGreaterThanOrEqual(5);
  });

  it("every rule is a permanent (308) redirect with non-empty source/destination", () => {
    for (const rule of contentNamespaceRedirects) {
      expect(rule.permanent).toBe(true);
      expect(rule.source.length).toBeGreaterThan(0);
      expect(rule.destination.length).toBeGreaterThan(0);
    }
  });

  it("each rule keeps the section and only swaps in the /c/ namespace", () => {
    for (const rule of contentNamespaceRedirects) {
      // source: /{locale}/{section}/:path*  ->  destination: /{locale}/c/{section}/:path*
      const match = rule.source.match(/^\/(en|id)\/([^/]+)\/:path\*$/);
      expect(match, `source not in expected shape: ${rule.source}`).not.toBeNull();
      const [, locale, section] = match as RegExpMatchArray;
      expect(rule.destination).toBe(`/${locale}/c/${section}/:path*`);
    }
  });

  it("covers the expected en sections", () => {
    const enSources = contentNamespaceRedirects.filter((r) => r.source.startsWith("/en/")).map((r) => r.source);
    expect(enSources).toContain("/en/learn/:path*");
    expect(enSources).toContain("/en/rants/:path*");
  });

  it("covers the expected id sections", () => {
    const idSources = contentNamespaceRedirects.filter((r) => r.source.startsWith("/id/")).map((r) => r.source);
    expect(idSources).toContain("/id/belajar/:path*");
    expect(idSources).toContain("/id/celoteh/:path*");
    expect(idSources).toContain("/id/konten-video/:path*");
  });

  it("does NOT use a blanket /{locale}/:path* rule that would swallow about/terms/tools", () => {
    const blanket = contentNamespaceRedirects.find((r) => /^\/(en|id)\/:path\*$/.test(r.source));
    expect(blanket).toBeUndefined();
  });
});
