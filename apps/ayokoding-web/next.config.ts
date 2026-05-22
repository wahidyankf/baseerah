import type { NextConfig } from "next";
import path from "node:path";
import { learnReorgRedirects } from "./src/redirects/learn-reorg";

const nextConfig: NextConfig = {
  output: "standalone",
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
