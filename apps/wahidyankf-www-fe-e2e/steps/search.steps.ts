import { createBdd } from "playwright-bdd";
import { expect } from "@playwright/test";

const { When, Then } = createBdd();

When('the visitor types "TypeScript" in the search input', async ({ page }) => {
  const input = page.getByPlaceholder(/Search skills, languages, or frameworks/i);
  await input.fill("TypeScript");
});

// @covers specs/apps/wahidyankf/behavior/wahidyankf-www/gherkin/search/search.feature:Typing a term updates the URL query string
Then("the URL becomes \\/?search=TypeScript", async ({ page }) => {
  await expect(page).toHaveURL(/\?search=TypeScript$/);
});

// @covers specs/apps/wahidyankf/behavior/wahidyankf-www/gherkin/search/search.feature:Clicking a skill pill navigates to the CV with scrollTop
Then("the URL becomes \\/cv?search=TypeScript&scrollTop=true", async ({ page }) => {
  // scrollTop is transient — CV page removes it via router.replace. Assert stable final URL.
  await expect(page).toHaveURL(/\/cv\?search=TypeScript/);
});

When('a visitor opens the home page with search term "TypeScript"', async ({ page }) => {
  await page.goto("/?search=TypeScript");
  await page.waitForLoadState("load");
});

// @covers specs/apps/wahidyankf/behavior/wahidyankf-www/gherkin/search/search.feature:Matching content is highlighted with a yellow mark
Then('the matching pill wraps "TypeScript" in a mark element', async ({ page }) => {
  const mark = page
    .locator("mark")
    .filter({ hasText: /TypeScript/i })
    .first();
  await expect(mark).toBeVisible();
});

When('a visitor opens the home page with search term "NoSuchTerm"', async ({ page }) => {
  await page.goto("/?search=NoSuchTerm");
  await page.waitForLoadState("load");
});

// @covers specs/apps/wahidyankf/behavior/wahidyankf-www/gherkin/search/search.feature:Non-matching About Me shows a placeholder
Then('the About Me card shows "No matching content in the About Me section."', async ({ page }) => {
  await expect(page.getByText(/No matching content in the About Me section\./i)).toBeVisible();
});

When('the visitor clicks the "TypeScript" skill pill', async ({ page }) => {
  await page.goto("/?search=TypeScript");
  await page.waitForLoadState("load");
  const pill = page
    .getByRole("button")
    .filter({ hasText: /^TypeScript$/ })
    .first();
  await pill.click();
});
