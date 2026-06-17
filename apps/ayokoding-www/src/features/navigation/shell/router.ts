import { z } from "zod";
import { publicProcedure } from "@/features/app-shell/shell/trpc-init";
import { localeSchema } from "@/features/i18n/core/schemas";

export const navigationProcedures = {
  getTree: publicProcedure
    .input(
      z.object({
        locale: localeSchema,
        rootSlug: z.string().optional(),
      }),
    )
    .query(async ({ ctx, input }) => {
      return ctx.contentService.getTree(input.locale, input.rootSlug);
    }),
};
