import { fetchRequestHandler } from "@trpc/server/adapters/fetch";
import { appRouter } from "@/features/app-shell/shell/root-router";
import { createTRPCContext } from "@/features/app-shell/shell/trpc-init";

const handler = (req: Request) =>
  fetchRequestHandler({
    endpoint: "/api/trpc",
    req,
    router: appRouter,
    createContext: createTRPCContext,
  });

export { handler as GET, handler as POST };
