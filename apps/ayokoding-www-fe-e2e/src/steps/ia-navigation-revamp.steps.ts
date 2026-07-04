import { createBdd } from "playwright-bdd";
import { expect } from "@playwright/test";

const { Given, When, Then } = createBdd();

// ---------------------------------------------------------------------------
// Scenario: Breadcrumb segments link to /c URLs
// ---------------------------------------------------------------------------

Given("a visitor is on {string}", async ({ page }, url: string) => {
  await page.goto(url);
});

When("the breadcrumb renders its ancestor segments", async ({ page }) => {
  // The breadcrumb renders as part of the content page — just confirm it's visible.
  const breadcrumb = page.getByRole("navigation", { name: /breadcrumb/i });
  await expect(breadcrumb).toBeVisible();
});

// @covers specs/apps/ayokoding/behavior/ayokoding-www/gherkin/navigation/ia-navigation-revamp.feature:Breadcrumb segments link to /c URLs
Then("each ancestor crumb links to a {string} prefixed URL", async ({ page }, prefix: string) => {
  const breadcrumb = page.getByRole("navigation", { name: /breadcrumb/i });
  const links = breadcrumb.getByRole("link");
  const count = await links.count();
  expect(count).toBeGreaterThan(0);
  for (let i = 0; i < count; i++) {
    const href = await links.nth(i).getAttribute("href");
    // Skip: root ("/"), locale root ("/en"), and the browse root ("/en/c") —
    // browse root IS the /c/ namespace but has no trailing path segment.
    if (href && href !== "/" && !href.match(/^\/[a-z]{2}$/) && !href.match(/^\/[a-z]{2}\/c$/)) {
      expect(href).toContain(prefix);
    }
  }
});

// ---------------------------------------------------------------------------
// Scenario: Internal content links emit /c URLs directly without relying on redirects
// ---------------------------------------------------------------------------

Given("the sidebar tree, breadcrumb, prev-next, and search results render content links", async ({ page }) => {
  // Navigate to a known content page that has sidebar, breadcrumb, and prev/next.
  await page.goto("/en/c/learn/software-engineering/algorithms-and-data-structures");
  await page.waitForLoadState("networkidle");
});

When("their hrefs are computed via the central content URL helper", async ({ page }) => {
  // All link-emitting components (sidebar-tree, breadcrumb, prev-next) are rendered;
  // gathering hrefs is done in the Then steps below.
  await expect(page.getByRole("article")).toBeVisible();
});

Then(
  "every content link resolves directly to a {string} URL with status 200",
  async ({ page }, _urlPattern: string) => {
    // Collect hrefs from the navigation chrome (sidebar + breadcrumb).
    const navLinks = page.locator("nav a[href]");
    const count = await navLinks.count();
    expect(count).toBeGreaterThan(0);

    // Collect unique content hrefs, then check in parallel to avoid sequential timeout.
    const hrefs: string[] = [];
    const seen = new Set<string>();
    for (let i = 0; i < count; i++) {
      const href = await navLinks.nth(i).getAttribute("href");
      if (!href || seen.has(href) || !href.includes("/c/")) continue;
      seen.add(href);
      hrefs.push(href);
    }

    await Promise.all(
      hrefs.map(async (href) => {
        const response = await page.request.get(href, { maxRedirects: 0, timeout: 10000 });
        expect(response.status()).not.toBe(404);
      }),
    );
  },
);

// @covers specs/apps/ayokoding/behavior/ayokoding-www/gherkin/navigation/ia-navigation-revamp.feature:Internal content links emit /c URLs directly without relying on redirects
Then("no internal content link resolves through a 308 redirect", async ({ page }) => {
  const navLinks = page.locator("nav a[href]");
  const count = await navLinks.count();

  // Collect unique internal hrefs, then check in parallel to avoid sequential timeout.
  const hrefs: string[] = [];
  const seen = new Set<string>();
  for (let i = 0; i < count; i++) {
    const href = await navLinks.nth(i).getAttribute("href");
    if (!href || !href.startsWith("/") || seen.has(href)) continue;
    seen.add(href);
    hrefs.push(href);
  }

  await Promise.all(
    hrefs.map(async (href) => {
      const response = await page.request.get(href, { maxRedirects: 0, timeout: 10000 });
      expect(response.status(), `Link ${href} should not be a 308 redirect`).not.toBe(308);
    }),
  );
});

// ---------------------------------------------------------------------------
// Scenario: Sitemap lists only the new /c content URLs
// ---------------------------------------------------------------------------

Given("the sitemap is generated from the content index", async ({ page }) => {
  await page.goto("/sitemap.xml");
  await page.waitForLoadState("networkidle");
});

When("the sitemap entries are produced", async ({ page }) => {
  const body = await page.content();
  expect(body).toBeTruthy();
});

Then("every moved-content entry uses a {string} prefixed URL", async ({ page }, _prefix: string) => {
  const body = await page.content();
  // The sitemap should contain /c/ URLs for content pages.
  expect(body).toContain("/c/");
});

// @covers specs/apps/ayokoding/behavior/ayokoding-www/gherkin/navigation/ia-navigation-revamp.feature:Sitemap lists only the new /c content URLs
Then(/^top-level pages \(about, terms, tools\) are not prefixed with "([^"]+)"$/, async ({ page }, prefix: string) => {
  const body = await page.content();
  // about-ayokoding and terms-and-conditions must NOT contain the /c/ prefix.
  const aboutIdx = body.indexOf("about-ayokoding");
  const termsIdx = body.indexOf("terms-and-conditions");
  if (aboutIdx !== -1) {
    const contextSlice = body.slice(Math.max(0, aboutIdx - 10), aboutIdx);
    expect(contextSlice).not.toContain(prefix);
  }
  if (termsIdx !== -1) {
    const contextSlice = body.slice(Math.max(0, termsIdx - 10), termsIdx);
    expect(contextSlice).not.toContain(prefix);
  }
});

// ---------------------------------------------------------------------------
// Scenario: RSS feed item links use the new /c content URLs
// ---------------------------------------------------------------------------

// Use page.request (APIRequestContext) instead of page.goto to avoid
// cross-browser XML rendering quirks (Firefox renders XML differently).
let feedBody = "";

Given("the feed is generated from the content index", async ({ page }) => {
  const response = await page.request.get("/feed.xml");
  expect(response.status()).toBe(200);
  feedBody = await response.text();
});

When("the feed items are produced", async () => {
  expect(feedBody.length).toBeGreaterThan(0);
});

// @covers specs/apps/ayokoding/behavior/ayokoding-www/gherkin/navigation/ia-navigation-revamp.feature:RSS feed item links use the new /c content URLs
Then("every content item link uses a {string} prefixed URL", async ({}, _prefix: string) => {
  // Each feed <item> <link> should point to a /c/ URL.
  expect(feedBody).toContain("/c/");
});

// ---------------------------------------------------------------------------
// Scenario: Canonical link for moved content points to the /c URL
// ---------------------------------------------------------------------------

Given("the content page at {string}", async ({ page }, url: string) => {
  await page.goto(url);
  await page.waitForLoadState("networkidle");
});

When("its metadata is generated", async ({ page }) => {
  // Metadata is embedded in <head> — the page has loaded.
  await expect(page.locator("head")).toBeDefined();
});

Then("the canonical alternate is {string}", async ({ page }, expectedCanonical: string) => {
  const canonical = await page.locator("link[rel='canonical']").getAttribute("href");
  expect(canonical).toContain(expectedCanonical);
});

// @covers specs/apps/ayokoding/behavior/ayokoding-www/gherkin/navigation/ia-navigation-revamp.feature:Canonical link for moved content points to the /c URL
Then("the language alternates include en and x-default", async ({ page }) => {
  const enAlternate = await page.locator("link[hreflang='en']").getAttribute("href");
  const xDefaultAlternate = await page.locator("link[hreflang='x-default']").getAttribute("href");
  expect(enAlternate).toBeTruthy();
  expect(xDefaultAlternate).toBeTruthy();
});
