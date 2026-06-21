import { describe, expect, it } from "vitest";
import { normalizeSlug, slugFromSegments } from "./slug";

describe("normalizeSlug", () => {
  it("strips a single leading and trailing slash", () => {
    expect(normalizeSlug("/learn/se/")).toBe("learn/se");
  });

  it("trims surrounding whitespace", () => {
    expect(normalizeSlug("  learn/se  ")).toBe("learn/se");
  });

  it("returns empty string for empty input", () => {
    expect(normalizeSlug("")).toBe("");
  });
});

describe("slugFromSegments", () => {
  it("joins catch-all segments into a canonical slug", () => {
    expect(slugFromSegments(["learn", "software-engineering"])).toBe("learn/software-engineering");
  });

  it("returns the root slug for undefined or empty segments", () => {
    expect(slugFromSegments(undefined)).toBe("");
    expect(slugFromSegments([])).toBe("");
  });

  it("does NOT strip a c/ prefix — segments under /c/ are already bare", () => {
    expect(slugFromSegments(["belajar", "ikhtisar"])).toBe("belajar/ikhtisar");
  });
});
