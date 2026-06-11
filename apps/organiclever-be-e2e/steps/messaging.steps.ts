import { expect } from "@playwright/test";
import { createBdd } from "playwright-bdd";
import { readFileSync } from "fs";
import { join } from "path";
import { setResponse, getResponse, clearResponse } from "../utils/response-store";

const { Given, When, Then, Before } = createBdd();

Before(() => {
  clearResponse();
});

// Uses the PDF fixture from crane-be for convenience
const PDF_FIXTURE = join(__dirname, "../../crane-be/tests/fixtures/sample.pdf");

Given("ORGANICLEVER_BE_NATS_URL points to a running NATS server with JetStream enabled", async () => {
  // No-op: compose brings up NATS; host-running backend connects at startup
});

Given("organiclever-be has a JetStream durable stream and consumer for its demo subject", async () => {
  // No-op: backend runs the JetStream demo at startup
});

Given("a running stack of organiclever-be, its NATS server, and crane-be", async () => {
  // No-op: compose stack + host backend both running
});

When("organiclever-be starts up", async () => {
  // The health endpoint confirms startup; no-op here
});

When("organiclever-be publishes a demo message to that subject", async () => {
  // Demo runs at startup; just read the status endpoint in Then steps
});

When("a client sends POST to the organiclever-be media-convert endpoint with a sample PDF", async ({ request }) => {
  const pdfBytes = readFileSync(PDF_FIXTURE);
  setResponse(
    await request.post("/api/v1/media/convert", {
      data: pdfBytes,
      headers: { "Content-Type": "application/octet-stream" },
    }),
  );
});

Then("the NATS connection is established", async ({ request }) => {
  // If the backend is healthy, NATS connected (backend fails fast on missing NATS)
  const resp = await request.get("/api/v1/health");
  expect(resp.ok()).toBeTruthy();
});

Then("the backend reports healthy after connecting", async ({ request }) => {
  const resp = await request.get("/api/v1/health");
  expect(resp.ok()).toBeTruthy();
});

Then("the durable consumer receives the message", async ({ request }) => {
  const resp = await request.get("/api/v1/system/status/messaging");
  expect(resp.ok()).toBeTruthy();
});

Then("the message is acknowledged", async ({ request }) => {
  const resp = await request.get("/api/v1/system/status/messaging");
  const body = (await resp.json()) as Record<string, string>;
  expect(body["jetstream_demo"]).not.toBe("pending");
});

Then("the messaging status surface reports the demo delivered and acked", async ({ request }) => {
  const resp = await request.get("/api/v1/system/status/messaging");
  const body = (await resp.json()) as Record<string, string>;
  expect(body["jetstream_demo"]).toBe("delivered_and_acked");
});

Then("the response status is 200", async () => {
  expect(getResponse().status()).toBe(200);
});

Then("the response body contains markdown produced by crane-be", async () => {
  const text = await getResponse().text();
  expect(text.length).toBeGreaterThan(0);
});

// ── @unit step stubs ───────────────────────────────────────────────────────
// These steps appear in @unit Gherkin scenarios whose assertions are executed
// by Rust unit tests (cargo test), not by this Playwright e2e runner.
// The stubs satisfy the spec-coverage tool; the scenarios themselves are
// excluded from the e2e run via the `tags: "not @unit"` filter in playwright.config.ts.

Given("ORGANICLEVER_BE_NATS_URL is unset", async () => {
  // @unit only — covered by Rust unit tests; no-op in e2e runner
});

When("organiclever-be reads its messaging configuration", async () => {
  // @unit only — covered by Rust unit tests; no-op in e2e runner
});

Then("startup aborts with a clear missing-variable error", async () => {
  // @unit only — covered by Rust unit tests; no-op in e2e runner
});
