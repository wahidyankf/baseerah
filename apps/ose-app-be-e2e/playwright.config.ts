import { defineConfig } from "@playwright/test";
import { defineBddConfig } from "playwright-bdd";

const testDir = defineBddConfig({
  featuresRoot: "../../specs/apps/ose/behavior/app-be/gherkin",
  features: "../../specs/apps/ose/behavior/app-be/gherkin/**/*.feature",
  steps: ["./steps/**/*.ts"],
  // Exclude @unit scenarios — those are covered by Rust unit tests (cargo test).
  // All other scenarios (including @e2e and untagged) run here via Playwright.
  tags: "not @unit",
});

export default defineConfig({
  testDir,
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: 1,
  reporter: [["list"], ["html"], ["junit", { outputFile: "test-results/junit.xml" }]],
  use: {
    baseURL: process.env.BASE_URL || "http://localhost:8302",
    trace: "on-first-retry",
    screenshot: "only-on-failure",
  },
});
