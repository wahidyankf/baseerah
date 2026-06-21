import { describe, expect, it, vi } from "vitest";

const contentMap = new Map([
  [
    "en:learn/software-engineering",
    {
      locale: "en",
      slug: "learn/software-engineering",
      isSection: false,
      date: "2024-01-01",
      title: "SE",
      description: "Software engineering article",
    },
  ],
  [
    "id:belajar/rekayasa",
    {
      locale: "id",
      slug: "belajar/rekayasa",
      isSection: false,
      date: "2024-01-01",
      title: "Rekayasa",
      description: null,
    },
  ],
]);

vi.mock("@/features/app-shell/shell/trpc-init", () => ({
  createTRPCContext: () => ({
    contentService: {
      getIndex: async () => ({ contentMap }),
    },
  }),
}));

// eslint-disable-next-line import/first
import { GET } from "./route";

describe("feed GET", () => {
  it("emits /c/ prefixed URL for English content items", async () => {
    const response = await GET();
    const text = await response.text();
    expect(text).toContain("/c/learn/software-engineering");
  });

  it("does not include non-English entries", async () => {
    const response = await GET();
    const text = await response.text();
    expect(text).not.toContain("belajar/rekayasa");
  });
});
