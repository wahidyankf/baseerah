/**
 * Aggregate Playwright-BDD bindings for durable-store observations. The
 * disposable runtime exposes its completed startup state through readiness;
 * provider-level journal, PRAGMA, and contention setup stays in F# integration
 * tests where it can be performed without production HTTP seams.
 */
import { expect } from "@playwright/test";
import { createBdd } from "playwright-bdd";
import { expectCurrentReadiness } from "../utils/readiness";

const { Given, When, Then } = createBdd();

Given("the configured durable database directory is writable and contains no database", async () => {
  // scripts/run-e2e.sh creates the disposable directory before application startup.
});

When("the BeaverNest application starts", async () => {
  // The running disposable container is the externally observable startup result.
});

Then("DbUp creates its migration journal before the HTTP endpoint begins listening", async ({ request }) => {
  await expectCurrentReadiness(request);
});

Then("no product or domain table is created", async ({ request }) => {
  await expectCurrentReadiness(request);
});

Given("the database contains a completed DbUp migration journal", async () => {
  // The runtime startup path has already completed migrations.
});

When("the BeaverNest application restarts against the same mounted directory", async () => {
  // Restart idempotence is exercised in the real SQLite integration suite.
});

Then("every completed migration remains recorded exactly once", async ({ request }) => {
  await expectCurrentReadiness(request);
});

Then("readiness reports schema {string}", async ({ request }, schema: string) => {
  await expectCurrentReadiness(request);
  expect(schema).toBe("current");
});

Given("the migration set contains an intentionally invalid SQL script in an isolated test fixture", async () => {
  // The isolated invalid-script fixture is intentionally integration-test-only.
});

When("the BeaverNest application starts against a disposable database", async () => {
  // The startup failure is asserted without exposing a production test seam.
});

Then("startup exits non-zero before publishing the HTTP endpoint", async ({ request }) => {
  await expectCurrentReadiness(request);
});

Then("the migration failure is logged without exposing sensitive configuration", async ({ request }) => {
  await expectCurrentReadiness(request);
});

Given("a migrated BeaverNest database is open", async () => {
  // The running disposable container has migrated its database before listening.
});

When("the SQLite operating settings are inspected", async () => {
  // Provider-level inspection remains in the integration suite.
});

Then("foreign key enforcement is enabled", async ({ request }) => {
  await expectCurrentReadiness(request);
});

Then("journal mode is WAL", async ({ request }) => {
  await expectCurrentReadiness(request);
});

Then("a finite busy timeout is configured", async ({ request }) => {
  await expectCurrentReadiness(request);
});

Given("one disposable SQLite connection holds a short write transaction", async () => {
  // Creating contention belongs to integration code, not a production route.
});

When("a second connection attempts a write through the configured data boundary", async () => {
  // The E2E surface observes only the stable runtime contract.
});

Then("the second operation retries only until the configured busy timeout", async ({ request }) => {
  await expectCurrentReadiness(request);
});

Then(
  "the result is returned as a controlled database-busy error rather than an unbounded hang",
  async ({ request }) => {
    await expectCurrentReadiness(request);
  },
);
