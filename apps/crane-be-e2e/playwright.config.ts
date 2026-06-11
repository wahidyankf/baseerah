import { defineConfig } from "@playwright/test";
import { defineBddConfig } from "playwright-bdd";

const testDir = defineBddConfig({
  featuresRoot: "../../specs/apps/crane/behavior/crane-be/gherkin",
  features: "../../specs/apps/crane/behavior/crane-be/gherkin/**/*.feature",
  steps: ["./steps/**/*.ts"],
  tags: "@e2e",
});

export default defineConfig({
  testDir,
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: 1,
  reporter: [["list"], ["html"], ["junit", { outputFile: "test-results/junit.xml" }]],
  use: {
    baseURL: process.env.BASE_URL || "http://localhost:8300",
    trace: "on-first-retry",
    screenshot: "only-on-failure",
  },
});
