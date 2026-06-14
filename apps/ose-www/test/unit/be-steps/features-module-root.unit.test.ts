/**
 * Phase 7b structure guard: `src/features/` is the module root for ose-www
 * (matching the wahidyankf-www pattern), and the tRPC content/feed pipeline
 * remains reachable through the new layout.
 *
 * RED: fails before the contexts -> features reshape (import paths unresolved).
 * GREEN: passes once src/features/ exists and the tRPC router/feed builder
 * resolve from it.
 */
import { describe, it, expect } from "vitest";
import { existsSync } from "node:fs";
import path from "node:path";

describe("ose-www features module root", () => {
  it("exposes src/features/ as the module root (contexts/ removed)", () => {
    const root = path.resolve(__dirname, "../../../src");
    expect(existsSync(path.join(root, "features"))).toBe(true);
    expect(existsSync(path.join(root, "contexts"))).toBe(false);
  });

  it("keeps the tRPC app router reachable from features/", async () => {
    const mod = await import("@/features/app-shell/application/root-router");
    expect(mod.appRouter).toBeDefined();
  });

  it("keeps the content service (feed source) reachable from features/", async () => {
    const mod = await import("@/features/content/application/service");
    expect(mod.ContentService).toBeDefined();
  });

  it("keeps the rss feed builder under features/ (server-only module)", () => {
    // feed-builder pulls the Next.js `server-only` tRPC server caller, so it is
    // asserted by file location rather than imported into the node test env.
    const root = path.resolve(__dirname, "../../../src");
    expect(existsSync(path.join(root, "features/rss-feed/application/feed-builder.ts"))).toBe(true);
  });
});
