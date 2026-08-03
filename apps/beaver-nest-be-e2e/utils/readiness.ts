import { expect, type APIRequestContext, type APIResponse } from "@playwright/test";

export const readinessPath = "/api/v1/readiness";

type ReadyPayload = {
  status: string;
  database: string;
  schema: string;
};

export async function expectCurrentReadiness(request: APIRequestContext): Promise<void> {
  const response = await request.get(readinessPath);
  await expectReadinessResponse(response, "ready", "ready", "current");
}

export async function expectReadinessResponse(
  response: APIResponse,
  status: string,
  database?: string,
  schema?: string,
): Promise<void> {
  expect(response.status()).toBe(status === "ready" ? 200 : 503);
  expect(response.headers()["cache-control"]).toBe("no-store");
  expect(response.headers().etag).toBeUndefined();
  expect(response.headers()["last-modified"]).toBeUndefined();

  const body = (await response.json()) as ReadyPayload;
  expect(body.status).toBe(status);

  if (database !== undefined && schema !== undefined) {
    expect(body.database).toBe(database);
    expect(body.schema).toBe(schema);
  }
}

export async function expectNoStorageDiagnostics(response: APIResponse): Promise<void> {
  const body = await response.text();
  expect(body).not.toMatch(/(?:\/(?:var|tmp|home)\/|sqlite|sql text|exception|stack trace)/i);
}
