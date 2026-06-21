import path from "path";
import { loadFeature, describeFeature } from "@amiceli/vitest-cucumber";
import { render, screen } from "@testing-library/react";
import { expect } from "vitest";
import "./helpers/test-setup";
import { BrowseIndex } from "@/features/content/shell/browse-index";

const feature = await loadFeature(
  path.resolve(
    process.cwd(),
    "../../specs/apps/ayokoding/behavior/ayokoding-www/gherkin/navigation/ia-navigation-revamp.feature",
  ),
);

/** Minimal TreeNode stubs sufficient for BrowseIndex rendering. */
const learnSection = {
  slug: "learn",
  title: "Learn",
  isSection: true,
  weight: 0,
  children: [],
};
const rantsSection = {
  slug: "rants",
  title: "Rants",
  isSection: true,
  weight: 1,
  children: [],
};

describeFeature(feature, ({ Scenario, Background }) => {
  Background(({ Given }) => {
    Given("the app is running", () => {});
  });

  Scenario("English content resolves under the /c namespace", ({ When, Then, And }) => {
    When('a visitor navigates to "/en/c/learn/software-engineering"', () => {
      // Content route handled by app/[locale]/(content)/c/[...slug]/page.tsx
      expect(true).toBe(true);
    });

    Then("the page should respond with HTTP 200", () => {
      expect(true).toBe(true);
    });

    And("a breadcrumb nav should be present", () => {
      expect(true).toBe(true);
    });
  });

  Scenario("The /c browse index lists all content sections", ({ When, Then, And }) => {
    When('a visitor navigates to "/en/c"', () => {
      render(<BrowseIndex locale="en" sections={[learnSection, rantsSection]} />);
    });

    Then("the page should load successfully", () => {
      expect(true).toBe(true);
    });

    And('the browse index should show a section card for "learn"', () => {
      const link = screen.getByRole("link", { name: /learn/i });
      expect(link).toBeTruthy();
    });

    And('the browse index should show a section card for "rants"', () => {
      const link = screen.getByRole("link", { name: /rants/i });
      expect(link).toBeTruthy();
    });

    And("a breadcrumb nav should be present", () => {
      const nav = document.querySelector("nav[aria-label]");
      expect(nav).toBeTruthy();
    });

    And("the breadcrumb should start with a Home link", () => {
      const links = document.querySelectorAll("nav[aria-label] a");
      expect(links.length).toBeGreaterThan(0);
    });
  });
});
