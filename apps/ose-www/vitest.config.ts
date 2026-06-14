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
        "src/features/content/application/types.ts",
        "src/features/content/infrastructure/reader.ts",
        "src/features/content/infrastructure/repository.ts",
        "src/features/content/infrastructure/repository-fs.ts",
        "src/features/rss-feed/application/feed-builder.ts",
        "src/features/seo/application/sitemap-builder.ts",
        "src/features/seo/presentation/**",
        "src/features/health/presentation/**",
        "src/features/landing/presentation/**",
        "src/features/app-shell/presentation/**",
        "src/features/content/presentation/**",
        "src/features/search/application/service.ts",
        "src/features/search/presentation/**",
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
