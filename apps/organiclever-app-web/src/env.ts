import { createEnv } from "@t3-oss/env-nextjs";
import { z } from "zod";

export const env = createEnv({
  server: {
    ORGANICLEVER_BE_URL: z.string().optional(),
  },
  experimental__runtimeEnv: {},
});
