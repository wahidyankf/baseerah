import { expect } from "@playwright/test";
import { createBdd } from "playwright-bdd";
import { clearResponse, getResponse, setResponse } from "../utils/response-store";

const { Given, When, Then, Before } = createBdd();

Before(() => {
  clearResponse();
});

Given("the crane-be service is running on its configured port", async () => {
  // No-op: assumes crane-be is running at baseURL
});

When(/^a client sends GET to \/health$/, async ({ request }) => {
  setResponse(await request.get("/health"));
});

// oxlint-disable-next-line no-empty-pattern
Then("the response status is {int}", async ({}, expectedStatus: number) => {
  expect(getResponse().status()).toBe(expectedStatus);
});

Then("the response body indicates the service is healthy", async () => {
  const body = (await getResponse().json()) as Record<string, unknown>;
  expect(body["status"]).toBe("healthy");
});
