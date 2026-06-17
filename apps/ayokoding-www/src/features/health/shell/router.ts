import { publicProcedure } from "@/features/app-shell/shell/trpc-init";

export const healthProcedures = {
  health: publicProcedure.query(() => {
    return { status: "ok" as const };
  }),
};
