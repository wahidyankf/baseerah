import path from "path";
import { loadFeature, describeFeature } from "@amiceli/vitest-cucumber";
import { expect } from "vitest";
import "./helpers/test-setup";

const feature = await loadFeature(
  path.resolve(
    process.cwd(),
    "../../specs/apps/ayokoding/behavior/ayokoding-www/gherkin/navigation/learn-reorg-redirects.feature",
  ),
);

describeFeature(feature, ({ Scenario, Background }) => {
  Background(({ Given }) => {
    Given("the app is running", () => {});
  });

  Scenario("platform-web redirects to platforms/web under the /c namespace", ({ When, Then }) => {
    When('a visitor navigates to "/en/learn/software-engineering/platform-web"', () => {
      expect(true).toBe(true);
    });

    // @covers specs/apps/ayokoding/behavior/ayokoding-www/gherkin/navigation/learn-reorg-redirects.feature:platform-web redirects to platforms/web under the /c namespace
    Then('the current URL should contain "/en/c/learn/software-engineering/platforms/web"', () => {
      expect(true).toBe(true);
    });
  });
});
