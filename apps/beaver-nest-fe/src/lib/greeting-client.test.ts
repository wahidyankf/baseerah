import { describe, expect, test, vi } from "vitest";

vi.mock("@/generated-contracts", () => ({
  getHello: vi.fn(),
}));

vi.mock("@/env", () => ({
  env: { BEAVER_NEST_FE_API_BASE_URL: "http://localhost:19320" },
}));

describe("fetchGreeting", () => {
  test("returns the greeting message when beaver-nest-be responds successfully", async () => {
    const { getHello } = await import("@/generated-contracts");
    vi.mocked(getHello).mockResolvedValue({
      data: { message: "Hello from BeaverNest" },
      error: undefined,
      request: new Request("http://localhost"),
      response: new Response(),
    } as never);

    const { fetchGreeting } = await import("./greeting-client");
    await expect(fetchGreeting()).resolves.toBe("Hello from BeaverNest");
  });

  test("throws when beaver-nest-be returns an error", async () => {
    const { getHello } = await import("@/generated-contracts");
    vi.mocked(getHello).mockResolvedValue({
      data: undefined,
      error: { error: "not found" },
      request: new Request("http://localhost"),
      response: new Response(),
    } as never);

    const { fetchGreeting } = await import("./greeting-client");
    await expect(fetchGreeting()).rejects.toThrow("beaver-nest-be did not return a greeting");
  });
});
