import { z } from "zod";
import { TRPCError } from "@trpc/server";
import { publicProcedure } from "@/features/app-shell/shell/trpc-init";
import { localeSchema } from "@/features/i18n/core/schemas";

export const contentProcedures = {
  getBySlug: publicProcedure
    .input(
      z.object({
        locale: localeSchema,
        slug: z.string(),
      }),
    )
    .query(async ({ ctx, input }) => {
      const result = await ctx.contentService.getBySlug(input.locale, input.slug);

      if (!result) {
        throw new TRPCError({ code: "NOT_FOUND", message: "Page not found" });
      }

      return result;
    }),

  listChildren: publicProcedure
    .input(
      z.object({
        locale: localeSchema,
        parentSlug: z.string(),
      }),
    )
    .query(async ({ ctx, input }) => {
      return ctx.contentService.listChildren(input.locale, input.parentSlug);
    }),
};
