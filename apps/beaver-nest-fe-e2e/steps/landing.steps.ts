/**
 * Step definitions for the beaver-nest-fe landing-page feature's first scenario
 * (heading + backend-sourced greeting), plus the shared Background/navigation
 * steps reused by ../steps/accessibility.steps.ts.
 *
 * Covers: specs/apps/beaver-nest/behavior/beaver-nest-fe/gherkin/hello/landing-page.feature
 */
import { expect } from "@playwright/test";
import { createBdd } from "playwright-bdd";

const { Given, When, Then } = createBdd();

const BE_BASE_URL = process.env.BE_BASE_URL || "http://localhost:19320";

Given("the beaver-nest-fe app is running on port {int} against a live {word}", async () => {
  // No-op: the test suite assumes the docker-compose stack (beaver-nest-fe + beaver-nest-be) is
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

// @covers specs/apps/beaver-nest/behavior/beaver-nest-fe/gherkin/hello/landing-page.feature:The landing page names the product and shows the backend greeting
Then("the page shows a level-one heading containing {string}", async ({ page }, text: string) => {
  await expect(page.getByRole("heading", { level: 1 })).toContainText(text);
});

// `page.tsx` is an async Server Component (`dynamic = "force-dynamic"`), so its fetch to
// beaver-nest-be runs inside the Next.js server process during SSR — it never crosses the
// browser's own network stack, so page.route()/page.on("request") cannot observe it.
// Instead, this step independently queries the live /api/v1/hello endpoint and asserts the
// rendered page shows that exact value. The stronger "not hardcoded" proof — that stopping
// beaver-nest-be breaks the page rather than falling back to a static string — was verified
// manually in the Phase 8 Gate (see delivery.md) since it requires stopping a container mid-run.
Then("the page shows the text {string} sourced from the backend", async ({ page, request }, text: string) => {
  const response = await request.get(`${BE_BASE_URL}/api/v1/hello`);
  expect(response.ok()).toBe(true);
  const body = (await response.json()) as { message: string };
  expect(body.message).toBe(text);
  await expect(page.getByText(text)).toBeVisible();
});

// @covers specs/apps/beaver-nest/behavior/beaver-nest-fe/gherkin/hello/landing-page.feature:The homepage tells a first-time visitor what BeaverNest is
Given("a first-time visitor with no prior context navigates to {string}", async ({ page }, path: string) => {
  await page.context().clearCookies();
  await page.goto(path);
});

When("the page finishes loading", async ({ page }) => {
  await page.waitForLoadState("networkidle");
});

Then("a one-line description of what BeaverNest does is visible without scrolling", async ({ page }) => {
  await expect(page.getByText(/BeaverNest is a personal operating layer/i)).toBeVisible();
});

// @covers specs/apps/beaver-nest/behavior/beaver-nest-fe/gherkin/hello/landing-page.feature:The homepage no longer renders a brand-chip etymology gloss
Given("a first-time visitor viewing the rendered homepage", async ({ page }) => {
  await page.goto("/");
});

When("they inspect the page for a hoverable multilingual term chip", async () => {
  // No-op: the assertion below directly queries for the chip's absence.
});

Then("no بصيرة\\/wawasan-style etymology chip is present", async ({ page }) => {
  await expect(page.getByTitle(/insight/i)).toHaveCount(0);
});

Then("no automated test or Gherkin scenario asserts one exists", async () => {
  // No-op: the scenario's own presence as a negative check satisfies this clause.
});

// @covers specs/apps/beaver-nest/behavior/beaver-nest-fe/gherkin/hello/landing-page.feature:A visitor to a non-existent path can recover
Given("a visitor navigates to a non-existent path on beaver-nest-fe", async ({ page }) => {
  await page.goto("/this-route-does-not-exist");
});

When("the 404 page renders", async ({ page }) => {
  await page.waitForLoadState("networkidle");
});

Then("it shows BeaverNest branding", async ({ page }) => {
  await expect(page.getByRole("heading", { level: 1 })).toContainText("BeaverNest");
});

Then("it offers a link back to the homepage", async ({ page }) => {
  await expect(page.getByRole("link", { name: /back to home/i })).toHaveAttribute("href", "/");
});

// @covers specs/apps/beaver-nest/behavior/beaver-nest-fe/gherkin/hello/landing-page.feature:External GitHub link announces it opens in a new tab
When("they encounter the {string} link", async () => {
  // No-op: the assertion below directly queries the link by its accessible name.
});

Then("its accessible name indicates it opens in a new browser tab", async ({ page }) => {
  await expect(page.getByRole("link", { name: /view on github.*opens in new tab/i })).toBeVisible();
});
