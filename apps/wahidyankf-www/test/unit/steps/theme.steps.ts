import path from "node:path";
import React from "react";
import { loadFeature, describeFeature } from "@amiceli/vitest-cucumber";
import { render, screen, fireEvent } from "@testing-library/react";
import { expect } from "vitest";
import { ThemeToggle } from "@open-sharia-enterprise/web-ui";

// @amiceli/vitest-cucumber registers every Given/When/Then/And as its own
// vitest test, and this project's src/test/setup.ts runs
// @testing-library/react's cleanup() after every test — a render() done in one
// step does not survive into the next. ThemeToggle's persisted state
// (localStorage + the "light-theme" class on document.documentElement) is
// genuinely global, though, so each step below that needs a live button
// re-mounts ThemeToggle itself, while assertions that only need to observe
// the persisted global state read it directly.
const feature = await loadFeature(
  path.resolve(process.cwd(), "../../specs/apps/wahidyankf/behavior/wahidyankf-www/gherkin/app-shell/theme.feature"),
);

describeFeature(feature, ({ Scenario, Background }) => {
  Background(({ Given }) => {
    Given("the app is running", () => {
      // Every scenario starts from a clean slate: no persisted theme choice
      // and no "light-theme" class on the document root.
      window.localStorage.clear();
      document.documentElement.classList.remove("light-theme");
    });
  });

  Scenario("Default theme is dark", ({ When, Then, And }) => {
    When("a visitor opens the home page for the first time", () => {});

    Then('the html element has no "light-theme" class', () => {
      render(React.createElement(ThemeToggle));
      expect(document.documentElement.classList.contains("light-theme")).toBe(false);
    });

    // @covers specs/apps/wahidyankf/behavior/wahidyankf-www/gherkin/app-shell/theme.feature:Default theme is dark
    And('the theme toggle aria-label is "Switch to light theme"', () => {
      render(React.createElement(ThemeToggle));
      expect(screen.getByRole("button", { name: "Switch to light theme" })).toBeInTheDocument();
    });
  });

  Scenario("Clicking the toggle switches to light theme", ({ When, And, Then }) => {
    When("a visitor opens the home page", () => {});

    And("the visitor clicks the theme toggle", () => {
      render(React.createElement(ThemeToggle));
      fireEvent.click(screen.getByRole("button", { name: "Switch to light theme" }));
      expect(localStorage.getItem("theme")).toBe("light");
    });

    Then('the html element has the "light-theme" class', () => {
      // Global DOM state set by the previous step's click — survives the
      // per-step React unmount because it lives on document.documentElement,
      // not inside the (already torn down) ThemeToggle render tree.
      expect(document.documentElement.classList.contains("light-theme")).toBe(true);
    });

    // @covers specs/apps/wahidyankf/behavior/wahidyankf-www/gherkin/app-shell/theme.feature:Clicking the toggle switches to light theme
    And('the theme toggle aria-label is "Switch to dark theme"', () => {
      // Fresh mount reads the localStorage choice set by the earlier click.
      render(React.createElement(ThemeToggle));
      expect(screen.getByRole("button", { name: "Switch to dark theme" })).toBeInTheDocument();
    });
  });

  Scenario("Theme persists across navigation", ({ When, And, Then }) => {
    When("a visitor opens the home page", () => {});

    And("the visitor clicks the theme toggle", () => {
      render(React.createElement(ThemeToggle));
      fireEvent.click(screen.getByRole("button", { name: "Switch to light theme" }));
      expect(localStorage.getItem("theme")).toBe("light");
    });

    And("the visitor navigates to the CV page", () => {
      // No-op: the real assertion is that a freshly-mounted ThemeToggle on
      // the "new page" still reflects the persisted choice (checked below).
    });

    // @covers specs/apps/wahidyankf/behavior/wahidyankf-www/gherkin/app-shell/theme.feature:Theme persists across navigation
    Then('the html element still has the "light-theme" class', () => {
      // Simulates arriving at the CV page: a brand new ThemeToggle instance
      // mounts and must read the persisted localStorage choice back.
      render(React.createElement(ThemeToggle));
      expect(document.documentElement.classList.contains("light-theme")).toBe(true);
    });
  });

  Scenario("Theme choice persists across reloads", ({ When, And, Then }) => {
    When("a visitor opens the home page", () => {});

    And("the visitor clicks the theme toggle", () => {
      render(React.createElement(ThemeToggle));
      fireEvent.click(screen.getByRole("button", { name: "Switch to light theme" }));
      expect(localStorage.getItem("theme")).toBe("light");
    });

    And("the visitor reloads the page", () => {
      // A real reload tears down the entire document and re-runs the app
      // from scratch — simulate that by clearing the DOM class the toggle
      // set. Only the localStorage-backed choice should survive.
      document.documentElement.classList.remove("light-theme");
    });

    // @covers specs/apps/wahidyankf/behavior/wahidyankf-www/gherkin/app-shell/theme.feature:Theme choice persists across reloads
    Then('the html element still has the "light-theme" class', () => {
      // A fresh ThemeToggle mount (as if the reloaded page just rendered)
      // must bring the class back from the persisted localStorage value.
      render(React.createElement(ThemeToggle));
      expect(document.documentElement.classList.contains("light-theme")).toBe(true);
    });
  });
});
