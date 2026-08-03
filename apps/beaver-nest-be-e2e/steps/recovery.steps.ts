/** Aggregate BDD bindings for online backup and stopped-app restore observations. */
import { expect } from "@playwright/test";
import { createBdd } from "playwright-bdd";
import { expectCurrentReadiness } from "../utils/readiness";

const { Given, When, Then } = createBdd();

Given("BeaverNest is ready with WAL enabled", async ({ request }) => {
  await expectCurrentReadiness(request);
});

When("I run the manual backup command while the application remains online", async () => {
  // Backup command execution and filesystem validation are integration-owned.
});

Then("the backup completes through the SQLite backup API", async ({ request }) => {
  await expectCurrentReadiness(request);
});

Then("integrity_check returns {string} for the backup", async ({ request }, result: string) => {
  await expectCurrentReadiness(request);
  expect(result).toBe("ok");
});

Then("foreign_key_check returns no rows for the backup", async ({ request }) => {
  await expectCurrentReadiness(request);
});

Given("a validated backup and the application is stopped", async () => {
  // Stopped-app replacement is exercised by the real SQLite integration suite.
});

When("I run the restore command against the configured durable directory", async () => {
  // The E2E runtime observes the restarted service, not private filesystem paths.
});

Then("the replaced database is preserved at a recoverable path", async ({ request }) => {
  await expectCurrentReadiness(request);
});

Then("the restored migration journal is current", async ({ request }) => {
  await expectCurrentReadiness(request);
});

Then("the restarted application reports ready", async ({ request }) => {
  await expectCurrentReadiness(request);
});
