/**
 * Step definitions specific to the greeting and 404-fallback scenarios; the
 * shared Given/When/Then steps live in ./health.steps.ts.
 *
 * Covers: specs/apps/baseerah/behavior/baseerah-be/gherkin/hello/greeting.feature
 */
import { expect } from "@playwright/test";
import { createBdd } from "playwright-bdd";
import { getResponse } from "../utils/response-store";

const { Then } = createBdd();

// @covers specs/apps/baseerah/behavior/baseerah-be/gherkin/hello/greeting.feature:An unknown route is refused
// oxlint-disable-next-line no-empty-pattern
Then("the response body field {string} is a non-empty string", async ({}, field: string) => {
  const body = (await getResponse().json()) as Record<string, unknown>;
  const value = body[field];
  expect(typeof value).toBe("string");
  expect((value as string).length).toBeGreaterThan(0);
});
