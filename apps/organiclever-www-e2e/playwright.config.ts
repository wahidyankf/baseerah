import { defineConfig, devices } from "@playwright/test";
import { defineBddConfig } from "playwright-bdd";

const testDir = defineBddConfig({
  featuresRoot: "../../specs/apps/organiclever/behavior/organiclever-www/gherkin",
  steps: "./steps/**/*.ts",
});

export default defineConfig({
  testDir,
  timeout: 60000,
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 1 : 0,
  workers: 1,
  reporter: process.env.CI ? [["list"], ["html"]] : "list",
  use: {
    baseURL: process.env.BASE_URL || "http://localhost:3200",
    trace: "on-first-retry",
    screenshot: "only-on-failure",
  },
  webServer: {
    command: "npx nx run organiclever-www:start",
    url: "http://localhost:3200",
    reuseExistingServer: !process.env.CI,
    timeout: 120000,
    cwd: "../..",
  },
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
  ],
});
