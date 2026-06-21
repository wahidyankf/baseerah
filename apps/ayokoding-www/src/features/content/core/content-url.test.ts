import { describe, expect, it } from "vitest";
import { contentUrl, isLoosePage, LOOSE_PAGE_ALLOWLIST } from "./content-url";

describe("contentUrl", () => {
  it("prefixes /c/ for en content-tree slugs", () => {
    expect(contentUrl("en", "learn/software-engineering")).toBe("/en/c/learn/software-engineering");
  });

  it("prefixes /c/ for id content-tree slugs", () => {
    expect(contentUrl("id", "belajar/ikhtisar")).toBe("/id/c/belajar/ikhtisar");
  });

  it("leaves en loose top-level pages bare (no /c/)", () => {
    expect(contentUrl("en", "about-ayokoding")).toBe("/en/about-ayokoding");
    expect(contentUrl("en", "terms-and-conditions")).toBe("/en/terms-and-conditions");
  });

  it("leaves id loose top-level pages bare (no /c/)", () => {
    expect(contentUrl("id", "tentang-ayokoding")).toBe("/id/tentang-ayokoding");
    expect(contentUrl("id", "syarat-dan-ketentuan")).toBe("/id/syarat-dan-ketentuan");
  });

  it("maps empty/root slug to the locale root", () => {
    expect(contentUrl("en", "")).toBe("/en");
    expect(contentUrl("id", "")).toBe("/id");
  });

  it("maps the _index slug to the locale root", () => {
    expect(contentUrl("en", "_index")).toBe("/en");
    expect(contentUrl("id", "_index")).toBe("/id");
  });

  it("normalizes leading and trailing slashes on content slugs", () => {
    expect(contentUrl("en", "/learn/software-engineering/")).toBe("/en/c/learn/software-engineering");
  });
});

describe("isLoosePage", () => {
  it("is true for allowlisted loose pages, false for content sections", () => {
    expect(isLoosePage("en", "about-ayokoding")).toBe(true);
    expect(isLoosePage("id", "tentang-ayokoding")).toBe(true);
    expect(isLoosePage("en", "learn")).toBe(false);
    expect(isLoosePage("id", "belajar")).toBe(false);
  });
});

describe("LOOSE_PAGE_ALLOWLIST", () => {
  it("declares the per-locale loose pages", () => {
    expect(LOOSE_PAGE_ALLOWLIST.en).toContain("about-ayokoding");
    expect(LOOSE_PAGE_ALLOWLIST.en).toContain("terms-and-conditions");
    expect(LOOSE_PAGE_ALLOWLIST.id).toContain("tentang-ayokoding");
    expect(LOOSE_PAGE_ALLOWLIST.id).toContain("syarat-dan-ketentuan");
  });
});
