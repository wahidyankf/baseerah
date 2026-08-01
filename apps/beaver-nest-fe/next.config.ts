import "./src/env.ts";
import type { NextConfig } from "next";
import path from "node:path";

// Both local dev (this app nested 2 levels under the real monorepo root) and the Docker build
// (which preserves that same apps/beaver-nest-fe/ nesting under a synthetic /repo root — see
// Dockerfile) genuinely have a real root 2 levels up from this file, with node_modules/next
// resolvable from it. Only npm's own lockfile-based auto-detection is ambiguous (a sibling git
// worktree's lockfile is also visible in the ancestor chain in local dev) — pinning the root
// explicitly removes the ambiguity without needing to guess a different value per context.
const workspaceRoot = path.join(__dirname, "../../");

const nextConfig: NextConfig = {
  output: "standalone",
  outputFileTracingRoot: workspaceRoot,
  // Turbopack infers its own workspace root independently of outputFileTracingRoot and hits the
  // same ambiguity — pin it to the same value so both agree on a single, unambiguous root.
  turbopack: {
    root: workspaceRoot,
  },
  transpilePackages: [
    "@open-sharia-enterprise/web-ui",
    "@open-sharia-enterprise/web-ui-token",
    "@t3-oss/env-nextjs",
    "@t3-oss/env-core",
  ],
  images: {
    unoptimized: true,
  },
  poweredByHeader: false,
  async headers() {
    return [
      {
        source: "/:path*",
        headers: [
          { key: "X-Content-Type-Options", value: "nosniff" },
          { key: "Referrer-Policy", value: "strict-origin-when-cross-origin" },
          { key: "X-Frame-Options", value: "DENY" },
          { key: "Permissions-Policy", value: "camera=(), microphone=(), geolocation=()" },
          { key: "Content-Security-Policy", value: "frame-ancestors 'none'" },
        ],
      },
    ];
  },
};

export default nextConfig;
