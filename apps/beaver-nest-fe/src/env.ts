import { createEnv } from "@t3-oss/env-nextjs";
import { z } from "zod";

export const env = createEnv({
  server: {
    BEAVER_NEST_FE_API_BASE_URL: z.url().default("http://localhost:19320"),
  },
  experimental__runtimeEnv: {},
});
