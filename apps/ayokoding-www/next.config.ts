import "./src/env.ts";
import type { NextConfig } from "next";
import path from "node:path";
import { learnReorgRedirects } from "./src/redirects/learn-reorg";

const nextConfig: NextConfig = {
  output: "standalone",
  transpilePackages: ["@t3-oss/env-nextjs", "@t3-oss/env-core"],
  outputFileTracingRoot: path.join(__dirname, "../../"),
  outputFileTracingIncludes: {
    "/**": ["./content/**/*", "./generated/**/*"],
  },
  serverExternalPackages: ["flexsearch"],
  images: {
    unoptimized: true,
  },
  async redirects() {
    return [...learnReorgRedirects];
  },
};

export default nextConfig;
