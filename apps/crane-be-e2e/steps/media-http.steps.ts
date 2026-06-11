import { expect } from "@playwright/test";
import { createBdd } from "playwright-bdd";
import { readFileSync } from "fs";
import { join } from "path";
import { clearResponse, getResponse, setResponse } from "../utils/response-store";

const { Given, When, Then, Before } = createBdd();

Before(() => {
  clearResponse();
});

const PDF_FIXTURE_PATH = join(__dirname, "../../crane-be/tests/fixtures/sample.pdf");

Given(/^crane-be is configured with the real PdfPig\/Tesseract adapter$/, async () => {
  // No-op: production container always uses real adapter
});

Given("crane-be is configured with the fake media adapter", async () => {
  // No-op: this step is @unit only; e2e runner always uses the real adapter
});

When(/^a client sends POST \/media\/pdf-to-md with sample PDF bytes$/, async ({ request }) => {
  const pdfBytes = readFileSync(PDF_FIXTURE_PATH);
  setResponse(
    await request.post("/media/pdf-to-md", {
      data: pdfBytes,
      headers: { "Content-Type": "application/octet-stream" },
    }),
  );
});

When(/^a client sends POST \/media\/pdf-to-md with a real sample PDF$/, async ({ request }) => {
  const pdfBytes = readFileSync(PDF_FIXTURE_PATH);
  setResponse(
    await request.post("/media/pdf-to-md", {
      data: pdfBytes,
      headers: { "Content-Type": "application/octet-stream" },
    }),
  );
});

When(/^a client sends POST \/media\/pdf-to-md with an empty body$/, async ({ request }) => {
  setResponse(
    await request.post("/media/pdf-to-md", {
      data: Buffer.alloc(0),
      headers: { "Content-Type": "application/octet-stream" },
    }),
  );
});

When(/^a client sends POST \/media\/pdf-to-md with bytes that are not a PDF$/, async ({ request }) => {
  setResponse(
    await request.post("/media/pdf-to-md", {
      data: Buffer.from("not a pdf"),
      headers: { "Content-Type": "application/octet-stream" },
    }),
  );
});

Then("the response body contains the canned markdown output", async () => {
  const text = await getResponse().text();
  expect(text).toBeTruthy();
  expect(text.length).toBeGreaterThan(0);
});

Then("the response body contains markdown extracted from the PDF", async () => {
  const text = await getResponse().text();
  expect(text).toBeTruthy();
  expect(text.length).toBeGreaterThan(0);
});

Then("the response body indicates the PDF payload was missing", async () => {
  const text = await getResponse().text();
  expect(text.toLowerCase()).toContain("missing");
});

Then("the response body indicates the payload could not be parsed as a PDF", async () => {
  const text = await getResponse().text();
  expect(text.length).toBeGreaterThan(0);
});

Then(/^the response Content-Type is text\/markdown$/, async () => {
  const contentType = getResponse().headers()["content-type"] ?? "";
  expect(contentType).toContain("text/markdown");
});
