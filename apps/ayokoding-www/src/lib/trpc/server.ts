import "server-only";
import { createCallerFactory, createTRPCContext } from "@/features/app-shell/shell/trpc-init";
import { appRouter } from "@/features/app-shell/shell/root-router";

const createCaller = createCallerFactory(appRouter);

export const serverCaller = createCaller(createTRPCContext());
