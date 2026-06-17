import { TRPCError } from "@trpc/server";
import { publicProcedure } from "@/features/app-shell/shell/trpc-init";
import { searchQuerySchema } from "@/features/search/core/schemas";

export const searchProcedures = {
  query: publicProcedure.input(searchQuerySchema).query(async ({ ctx, input }) => {
    if (input.query.trim().length === 0) {
      throw new TRPCError({ code: "BAD_REQUEST", message: "Query must not be empty" });
    }

    return ctx.contentService.search(input.locale, input.query, input.limit);
  }),
};
