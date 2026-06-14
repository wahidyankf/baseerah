import { router } from "@/lib/trpc/init";
import { contentRouter } from "@/features/content/application/router";
import { searchRouter } from "@/features/search/application/router";
import { healthRouter } from "@/features/health/application/router";

export const appRouter = router({
  content: contentRouter,
  search: searchRouter,
  health: healthRouter,
});

export type AppRouter = typeof appRouter;
