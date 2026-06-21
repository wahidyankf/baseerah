import { describe, expect, it, vi } from "vitest";

vi.mock("@/lib/trpc/server", () => ({
  serverCaller: {
    content: {
      getBySlug: vi.fn().mockResolvedValue({
        title: "Software Engineering",
        description: "Learn SE",
        html: "<p>content</p>",
        headings: [],
        date: null,
        prev: null,
        next: null,
      }),
    },
  },
}));

vi.mock("@/features/app-shell/shell/trpc-init", () => ({
  createTRPCContext: () => ({
    contentService: { getIndex: async () => ({ contentMap: new Map() }) },
  }),
}));

// eslint-disable-next-line import/first
import { generateMetadata } from "./page";

describe("generateMetadata", () => {
  it("sets canonical to the /c/ URL", async () => {
    const meta = await generateMetadata({
      params: Promise.resolve({ locale: "en", slug: ["learn", "software-engineering"] }),
    });
    expect(meta.alternates?.canonical).toBe("/en/c/learn/software-engineering");
  });

  it("includes alternates.languages with en and x-default", async () => {
    const meta = await generateMetadata({
      params: Promise.resolve({ locale: "en", slug: ["learn", "software-engineering"] }),
    });
    const langs = meta.alternates?.languages as Record<string, string> | undefined;
    expect(langs).toBeDefined();
    expect(langs?.["en"]).toBeDefined();
    expect(langs?.["x-default"]).toBeDefined();
  });
});
