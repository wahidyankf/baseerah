import { getHello } from "@/generated-contracts";
import { env } from "@/env";

export async function fetchGreeting(): Promise<string> {
  const { data, error } = await getHello({
    baseUrl: env.BASEERAH_FE_API_BASE_URL,
    throwOnError: false,
  });
  if (error || !data) {
    throw new Error("baseerah-be did not return a greeting");
  }
  return data.message;
}
