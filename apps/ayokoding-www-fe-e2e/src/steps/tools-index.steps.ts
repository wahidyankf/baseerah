import { createBdd } from "playwright-bdd";
import { expect } from "@playwright/test";

const { Given, When, Then } = createBdd();

// ── AC-13 (UWT-009): Tools index calculator entry has a description ───────────

Given("I am on the tools index page", async ({ page }) => {
  await page.goto("/en/tools");
  await page.waitForLoadState("networkidle");
});

When("the calculator entry renders", async ({ page }) => {
  await page.waitForLoadState("networkidle");
});

Then("the calculator entry shows a description distinct from its link text", async ({ page }) => {
  // The link text and the description paragraph must both be visible and distinct
  const calcLink = page.getByRole("link", { name: /cost of living/i });
  await expect(calcLink).toBeVisible();
  const linkText = (await calcLink.textContent())?.trim() ?? "";

  const descEl = page.locator("[data-testid='tool-desc-calculator']");
  await expect(descEl).toBeVisible();
  const descText = (await descEl.textContent())?.trim() ?? "";

  // The description must be non-empty and differ from the link text
  expect(descText.length).toBeGreaterThan(0);
  expect(descText).not.toBe(linkText);
});
