/**
 * Step definitions for the baseerah-fe landing-page feature's first scenario
 * (heading + backend-sourced greeting), plus the shared Background/navigation
 * steps reused by ../steps/accessibility.steps.ts.
 *
 * Covers: specs/apps/baseerah/behavior/baseerah-fe/gherkin/hello/landing-page.feature
 */
import { expect } from "@playwright/test";
import { createBdd } from "playwright-bdd";

const { Given, When, Then } = createBdd();

const BE_BASE_URL = process.env.BE_BASE_URL || "http://localhost:19320";

Given("the baseerah-fe app is running on port {int} against a live {word}", async () => {
  // No-op: the test suite assumes the docker-compose stack (baseerah-fe + baseerah-be) is
  // already running at baseURL / BE_BASE_URL.
});

Given("I have not visited the site before", async ({ page }) => {
  await page.context().clearCookies();
});

Given("I am on {string}", async ({ page }, path: string) => {
  await page.goto(path);
});

When("I navigate to {string}", async ({ page }, path: string) => {
  await page.goto(path);
});

// @covers specs/apps/baseerah/behavior/baseerah-fe/gherkin/hello/landing-page.feature:The landing page names the product and shows the backend greeting
Then("the page shows a level-one heading containing {string}", async ({ page }, text: string) => {
  await expect(page.getByRole("heading", { level: 1 })).toContainText(text);
});

// `page.tsx` is an async Server Component (`dynamic = "force-dynamic"`), so its fetch to
// baseerah-be runs inside the Next.js server process during SSR — it never crosses the
// browser's own network stack, so page.route()/page.on("request") cannot observe it.
// Instead, this step independently queries the live /api/v1/hello endpoint and asserts the
// rendered page shows that exact value. The stronger "not hardcoded" proof — that stopping
// baseerah-be breaks the page rather than falling back to a static string — was verified
// manually in the Phase 8 Gate (see delivery.md) since it requires stopping a container mid-run.
Then("the page shows the text {string} sourced from the backend", async ({ page, request }, text: string) => {
  const response = await request.get(`${BE_BASE_URL}/api/v1/hello`);
  expect(response.ok()).toBe(true);
  const body = (await response.json()) as { message: string };
  expect(body.message).toBe(text);
  await expect(page.getByText(text)).toBeVisible();
});
