/**
 * Step definitions for the baseerah-fe landing-page feature's accessibility scenario.
 *
 * Covers: specs/apps/baseerah/behavior/baseerah-fe/gherkin/hello/landing-page.feature
 */
import AxeBuilder from "@axe-core/playwright";
import { createBdd } from "playwright-bdd";

const { When, Then } = createBdd();

type AxeResults = Awaited<ReturnType<AxeBuilder["analyze"]>>;

let lastScan: AxeResults | undefined;

function violationsOfImpact(results: AxeResults, impact: string): AxeResults["violations"] {
  return results.violations.filter((violation) => violation.impact === impact);
}

// @covers specs/apps/baseerah/behavior/baseerah-fe/gherkin/hello/landing-page.feature:The landing page meets the baseline accessibility bar
When("an automated accessibility scan runs against the rendered page", async ({ page }) => {
  lastScan = await new AxeBuilder({ page }).analyze();
});

// oxlint-disable-next-line no-empty-pattern
Then("it reports zero serious violations", async ({}) => {
  if (!lastScan) {
    throw new Error("No accessibility scan recorded. The scan step must run first.");
  }
  const serious = violationsOfImpact(lastScan, "serious");
  if (serious.length > 0) {
    throw new Error(`Expected zero serious violations, found: ${serious.map((v) => v.id).join(", ")}`);
  }
});

// oxlint-disable-next-line no-empty-pattern
Then("it reports zero critical violations", async ({}) => {
  if (!lastScan) {
    throw new Error("No accessibility scan recorded. The scan step must run first.");
  }
  const critical = violationsOfImpact(lastScan, "critical");
  if (critical.length > 0) {
    throw new Error(`Expected zero critical violations, found: ${critical.map((v) => v.id).join(", ")}`);
  }
});
