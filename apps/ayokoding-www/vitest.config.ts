import { defineConfig } from "vitest/config";
import react from "@vitejs/plugin-react";
import tsconfigPaths from "vite-tsconfig-paths";

const sharedPlugins = [react(), tsconfigPaths()];

export default defineConfig({
  plugins: sharedPlugins,
  test: {
    passWithNoTests: true,
    coverage: {
      provider: "v8",
      include: ["src/**/*.{ts,tsx}"],
      exclude: [
        // App shell presentation (chrome) — passive UI primitives + layout shell
        "src/features/app-shell/shell/*.tsx",
        // Per-feature presentation (UI surfaces — exercised via E2E + fe-step Gherkin scenarios)
        "src/features/content/shell/*.tsx",
        "src/features/search/shell/*.tsx",
        "src/features/search/shell/use-search.ts",
        "src/features/i18n/shell/*.tsx",
        "src/features/i18n/shell/use-locale.ts",
        "src/features/navigation/shell/*.tsx",
        // Re-export shim — pure type re-export, no executable code
        "src/features/navigation/core/schemas.ts",
        // Next.js app router pages — covered via E2E
        "src/app/**",
        // Cross-cutting tRPC client wiring
        "src/lib/trpc/client.ts",
        "src/lib/trpc/provider.tsx",
        "src/lib/trpc/server.ts",
        // i18n middleware re-export shim and implementation (covered via fe-e2e)
        "src/middleware.ts",
        "src/features/i18n/shell/middleware.ts",
        // Content infrastructure adapters + scripts (covered via unit suite with mocked deps)
        "src/features/content/core/parser.ts",
        "src/features/content/shell/reader.ts",
        "src/features/content/core/repository.ts",
        "src/features/content/shell/repository-fs.ts",
        "src/features/content/core/types.ts",
        "src/features/content/shell/index-generator.ts",
        "src/features/search/shell/generate-search-data.ts",
        "src/scripts/**",
        // tRPC routers — composition-only; behaviour is exercised via BE-E2E
        "src/features/*/shell/router.ts",
        "src/features/app-shell/shell/root-router.ts",
        // Test infra
        "src/test/**",
        "**/*.{test,spec}.{ts,tsx}",
      ],
      thresholds: {
        lines: 80,
        functions: 80,
        branches: 80,
        statements: 80,
      },
      reporter: ["text", "json-summary", "lcov"],
    },
    projects: [
      {
        plugins: sharedPlugins,
        test: {
          name: "unit",
          include: ["test/unit/be-steps/**/*.steps.ts", "**/*.unit.{test,spec}.{ts,tsx}"],
          exclude: ["node_modules"],
          environment: "node",
        },
      },
      {
        plugins: sharedPlugins,
        test: {
          name: "unit-fe",
          include: ["test/unit/fe-steps/**/*.steps.{ts,tsx}", "src/features/**/*.test.{ts,tsx}"],
          exclude: ["node_modules"],
          environment: "jsdom",
          setupFiles: ["./src/test/setup.ts"],
        },
      },
    ],
  },
});
