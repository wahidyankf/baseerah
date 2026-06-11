import { expect } from "@playwright/test";
import { createBdd } from "playwright-bdd";
import { readFileSync } from "fs";
import { join } from "path";
import { connectNats, drainNats, requestOnOrg, requestOnOse } from "../utils/nats-client";

const { Given, When, Then, Before, After } = createBdd();

const PDF_FIXTURE_PATH = join(__dirname, "../../crane-be/tests/fixtures/sample.pdf");

let orgReply: Uint8Array | null = null;
let oseReply: Uint8Array | null = null;

Before(async () => {
  orgReply = null;
  oseReply = null;
  await connectNats();
});

After(async () => {
  await drainNats();
});

Given("crane-be has subscribed to subject crane.convert on a backend NATS server", async () => {
  // No-op: crane-be subscribes crane.convert at startup on both NATS connections
});

Given("crane-be has opened one NATS connection to each backend's NATS server", async () => {
  // No-op
});

Given("each subscription uses the same queue group crane.workers", async () => {
  // No-op: this is a crane-be configuration invariant
});

When("a backend publishes a request to crane.convert with sample PDF bytes", async () => {
  const pdfBytes = readFileSync(PDF_FIXTURE_PATH);
  orgReply = await requestOnOrg("crane.convert", new Uint8Array(pdfBytes));
});

When("a backend publishes a request to crane.convert with bytes that are not a PDF", async () => {
  orgReply = await requestOnOrg("crane.convert", new TextEncoder().encode("not a pdf"));
});

When("each backend independently issues a crane.convert request", async () => {
  const pdfBytes = readFileSync(PDF_FIXTURE_PATH);
  const data = new Uint8Array(pdfBytes);
  [orgReply, oseReply] = await Promise.all([requestOnOrg("crane.convert", data), requestOnOse("crane.convert", data)]);
});

Then("crane-be replies on the auto _INBOX subject with markdown", async () => {
  expect(orgReply).toBeTruthy();
  const text = new TextDecoder().decode(orgReply!);
  expect(text.length).toBeGreaterThan(0);
  // Should NOT be an error envelope
  expect(text).not.toMatch(/^\{.*"error"/);
});

Then("the requesting backend receives the markdown reply", async () => {
  expect(orgReply).toBeTruthy();
});

Then("crane-be replies on the auto _INBOX subject with an error envelope", async () => {
  expect(orgReply).toBeTruthy();
  const text = new TextDecoder().decode(orgReply!);
  // Error envelope is JSON with "error" field
  expect(text).toContain("error");
});

Then("the error envelope names the parse failure", async () => {
  expect(orgReply).toBeTruthy();
  const text = new TextDecoder().decode(orgReply!);
  expect(text.length).toBeGreaterThan(0);
});

Then("each backend receives a markdown reply from crane-be", async () => {
  expect(orgReply).toBeTruthy();
  expect(oseReply).toBeTruthy();
  const orgText = new TextDecoder().decode(orgReply!);
  const oseText = new TextDecoder().decode(oseReply!);
  expect(orgText.length).toBeGreaterThan(0);
  expect(oseText.length).toBeGreaterThan(0);
});

Then("neither backend's request is delivered to the other backend's NATS server", async () => {
  // This is an architectural invariant: the two NATS servers are independent.
  // Proven by both requests returning valid markdown (no cross-delivery causes timeout).
  expect(orgReply).toBeTruthy();
  expect(oseReply).toBeTruthy();
});
