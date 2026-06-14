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
        "src/app/layout.tsx",
        "src/app/**/*.css",
        "src/env.ts",
        "src/test/**",
        "**/*.config.*",
        "**/.next/**",
        "**/dist/**",
        "**/coverage/**",
      ],
      thresholds: {
        lines: 80,
        functions: 80,
        branches: 75,
        statements: 80,
      },
      reporter: ["text", "json-summary", "lcov"],
    },
    projects: [
      {
        plugins: sharedPlugins,
        test: {
          name: "unit-fe",
          include: ["src/**/*.unit.test.{ts,tsx}", "test/**/*.steps.{ts,tsx}"],
          exclude: ["node_modules"],
          environment: "jsdom",
          setupFiles: ["./src/test/setup.ts"],
        },
      },
      {
        plugins: sharedPlugins,
        test: {
          name: "integration",
          include: ["src/**/*.integration.test.{ts,tsx}"],
          exclude: ["node_modules", "**/*.unit.test.tsx"],
          environment: "node",
        },
      },
    ],
  },
});
