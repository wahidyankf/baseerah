/**
 * Step definitions for the beaver-nest-be health endpoint feature, plus the
 * shared Given/When/Then steps reused by the greeting and 404-fallback
 * scenarios in ../steps/greeting.steps.ts.
 *
 * Covers: specs/apps/beaver-nest/behavior/beaver-nest-be/gherkin/health/service-health.feature
 */
import { expect } from "@playwright/test";
import { createBdd } from "playwright-bdd";
import { setResponse, getResponse, clearResponse } from "../utils/response-store";

const { Given, When, Then, Before } = createBdd();

Before(() => {
  clearResponse();
});

Given("the beaver-nest-be service is running on port {int}", async () => {
  // No-op: the test suite assumes beaver-nest-be is running at baseURL.
});

Given("the service has finished starting", async () => {
  // No-op: same assumption as above.
});

When("I send a GET request to {string}", async ({ request }, path: string) => {
  setResponse(await request.get(path));
});

// @covers specs/apps/beaver-nest/behavior/beaver-nest-be/gherkin/health/service-health.feature:The service reports liveness
// oxlint-disable-next-line no-empty-pattern
Then("the response status is {int}", async ({}, expectedStatus: number) => {
  expect(getResponse().status()).toBe(expectedStatus);
});

// oxlint-disable-next-line no-empty-pattern
Then("the response body field {string} equals {string}", async ({}, field: string, value: string) => {
  const body = (await getResponse().json()) as Record<string, unknown>;
  expect(body[field]).toBe(value);
});
