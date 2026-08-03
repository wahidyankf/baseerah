/**
 * Aggregate HTTP observations for liveness and readiness. Detailed database
 * fault injection remains in the F# integration suite; this BDD surface
 * verifies the externally observable contract of the disposable runtime.
 */
import { expect, test } from "@playwright/test";
import { createBdd } from "playwright-bdd";
import { getResponse } from "../utils/response-store";
import { expectNoStorageDiagnostics, expectReadinessResponse } from "../utils/readiness";

const { Given, Then } = createBdd();

Given("the BeaverNest process is accepting HTTP requests", async () => {
  // The disposable runtime is started by scripts/run-e2e.sh before Playwright runs.
});

Given("startup migrations completed and SQLite accepts queries", async () => {
  // Startup is observed through the ready response below.
});

Given("SQLite cannot complete the readiness query", async () => {
  // The container runtime has no external fault-injection endpoint. The F# HTTP/
  // SQLite integration suite owns that setup; retain this as a conditional,
  // explicit guard rather than an unconditional test skip.
  test.skip(
    process.env.BEAVER_NEST_BE_E2E_UNREADY !== "true",
    "requires an explicitly provisioned unready database runtime",
  );
});

// oxlint-disable-next-line no-empty-pattern
Then("the JSON response reports status {string}", async ({}, status: string) => {
  const body = (await getResponse().json()) as { status?: unknown };
  expect(body.status).toBe(status);
});

// oxlint-disable-next-line no-empty-pattern
Then(
  "the JSON response reports status {string}, database {string} and schema {string}",
  // oxlint-disable-next-line no-empty-pattern
  async ({}, status: string, database: string, schema: string) => {
    await expectReadinessResponse(getResponse(), status, database, schema);
  },
);

// oxlint-disable-next-line no-empty-pattern
Then("the response sends {string} without a cache validator", async ({}, cacheControl: string) => {
  const response = getResponse();
  const separator = cacheControl.indexOf(":");
  expect(separator).toBeGreaterThan(0);
  const headerName = cacheControl.slice(0, separator).toLowerCase();
  const expectedValue = cacheControl.slice(separator + 1).trim();
  expect(headerName).toBe("cache-control");
  expect(expectedValue).toBe("no-store");
  expect(response.headers()[headerName.toLowerCase()]).toBe(expectedValue);
  expect(response.headers().etag).toBeUndefined();
  expect(response.headers()["last-modified"]).toBeUndefined();
});

Then("the response reveals no database path or migration detail", async () => {
  await expectNoStorageDiagnostics(getResponse());
});

Then("the response reveals no database path, SQL text or exception detail", async () => {
  await expectNoStorageDiagnostics(getResponse());
});
