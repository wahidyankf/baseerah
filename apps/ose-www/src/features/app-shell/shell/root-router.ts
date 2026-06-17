import { router } from "@/lib/trpc/init";
import { contentRouter } from "@/features/content/shell/router";
import { searchRouter } from "@/features/search/shell/router";
import { healthRouter } from "@/features/health/shell/router";

export const appRouter = router({
  content: contentRouter,
  search: searchRouter,
  health: healthRouter,
});

export type AppRouter = typeof appRouter;
