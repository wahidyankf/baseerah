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
        "src/app/**",
        "src/lib/trpc/client.ts",
        "src/lib/trpc/provider.tsx",
        "src/lib/trpc/server.ts",
        "src/features/content/core/types.ts",
        "src/features/content/core/reader.ts",
        "src/features/content/core/repository.ts",
        "src/features/content/shell/repository-fs.ts",
        "src/features/rss-feed/shell/feed-builder.ts",
        "src/features/seo/shell/sitemap-builder.ts",
        "src/features/seo/shell/metadata.ts",
        "src/features/health/shell/status-page.tsx",
        "src/features/landing/shell/*.tsx",
        "src/features/app-shell/shell/*.tsx",
        "src/features/content/shell/*.tsx",
        "src/features/search/shell/service.ts",
        "src/features/search/shell/search-dialog.tsx",
        "src/features/search/shell/search-provider.tsx",
        "src/features/search/shell/use-search.ts",
        "src/scripts/**",
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
          include: ["test/unit/fe-steps/**/*.steps.{ts,tsx}"],
          exclude: ["node_modules"],
          environment: "jsdom",
          globals: true,
          setupFiles: ["./src/test/setup.ts"],
        },
      },
      {
        plugins: sharedPlugins,
        test: {
          name: "integration",
          include: ["test/integration/be-steps/**/*.steps.ts", "**/*.integration.{test,spec}.{ts,tsx}"],
          exclude: ["node_modules"],
          environment: "node",
        },
      },
    ],
  },
});
