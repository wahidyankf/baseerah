import { describe, expect, it, vi } from "vitest";

const mockRedirectFn = vi.fn((url: URL) => ({ status: 308, redirectUrl: url.href }));
const mockNextFn = vi.fn(() => ({ headers: { set: vi.fn() } }));

vi.mock("next/server", () => ({
  NextResponse: {
    next: () => mockNextFn(),
    redirect: (url: URL) => mockRedirectFn(url),
  },
}));

// Import after mock registration
// eslint-disable-next-line import/first
import { middleware } from "./middleware";

function makeRequest(pathname: string) {
  return {
    nextUrl: { pathname },
    url: `http://localhost${pathname}`,
  } as Parameters<typeof middleware>[0];
}

describe("Phase 9L — locale URL casing redirect", () => {
  it("Phase9L: /EN/tools redirects to /en/tools", () => {
    mockRedirectFn.mockClear();
    middleware(makeRequest("/EN/tools/cost-of-living-calculator"));
    expect(mockRedirectFn).toHaveBeenCalledTimes(1);
    const redirectUrl: URL = mockRedirectFn.mock.calls[0]![0];
    expect(redirectUrl.pathname).toBe("/en/tools/cost-of-living-calculator");
  });

  it("Phase9L: /en/tools passes through without redirect", () => {
    mockRedirectFn.mockClear();
    middleware(makeRequest("/en/tools/cost-of-living-calculator"));
    expect(mockRedirectFn).not.toHaveBeenCalled();
  });
});
