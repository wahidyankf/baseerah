# Build context: workspace root (../../.. from apps/baseerah-fe/)
#
# WORKDIR preserves the real apps/baseerah-fe/ nesting under a synthetic /repo root — Next's
# outputFileTracingRoot and turbopack.root (next.config.ts) are pinned to "two levels up from this
# file" for both local dev (the real monorepo root) and this Docker build, so keeping the same
# nesting here means the standalone output always lands at the same relative path in both:
# .next/standalone/apps/baseerah-fe/server.js.
FROM node:24-alpine AS build
WORKDIR /repo/apps/baseerah-fe

# Copy app package.json and strip workspace dependency before install
COPY apps/baseerah-fe/package.json ./
RUN node -e " \
  const pkg = JSON.parse(require('fs').readFileSync('package.json','utf8')); \
  delete pkg.dependencies['@open-sharia-enterprise/web-ui']; \
  delete pkg.dependencies['@open-sharia-enterprise/web-ui-token']; \
  require('fs').writeFileSync('package.json', JSON.stringify(pkg, null, 2)); \
" && npm install --ignore-scripts --legacy-peer-deps

# Install web-ui transitive dependencies
RUN npm install --save @radix-ui/react-dialog@1.1.15 @radix-ui/react-dropdown-menu@2.1.16 @radix-ui/react-scroll-area@1.2.10 @radix-ui/react-separator@1.1.8 @radix-ui/react-slot@1.2.4 @radix-ui/react-tabs@1.1.13 @radix-ui/react-tooltip@1.2.8 class-variance-authority@0.7.1 clsx@2.1.1 cmdk@1.1.1 lucide-react@0.577.0 radix-ui@1.4.3 tailwind-merge@2.6.1 --ignore-scripts --legacy-peer-deps

# Copy workspace libs as resolvable node_modules packages
COPY libs/web-ui/src/ ./node_modules/@open-sharia-enterprise/web-ui/src/
COPY libs/web-ui/package.json ./node_modules/@open-sharia-enterprise/web-ui/
COPY libs/web-ui-token/src/ ./node_modules/@open-sharia-enterprise/web-ui-token/src/
COPY libs/web-ui-token/package.json ./node_modules/@open-sharia-enterprise/web-ui-token/

# Copy app source
COPY apps/baseerah-fe/src/ ./src/
COPY apps/baseerah-fe/package.json apps/baseerah-fe/tsconfig.json apps/baseerah-fe/next.config.ts apps/baseerah-fe/postcss.config.mjs ./

ENV NEXT_TELEMETRY_DISABLED=1
ARG BASEERAH_FE_API_BASE_URL=http://baseerah-be:19320
ENV BASEERAH_FE_API_BASE_URL=${BASEERAH_FE_API_BASE_URL}
RUN npx next build

FROM node:24-alpine

LABEL org.opencontainers.image.source="https://github.com/wahidyankf/baseerah"
LABEL org.opencontainers.image.description="Baseerah hello-world frontend built with Next.js"

RUN addgroup -S app && adduser -S app -G app
WORKDIR /app
COPY --from=build --chown=app:app /repo/apps/baseerah-fe/.next/standalone/apps/baseerah-fe/ ./
COPY --from=build --chown=app:app /repo/apps/baseerah-fe/.next/static ./.next/static
USER app
EXPOSE 19310
ENV PORT=19310 HOSTNAME="0.0.0.0" NODE_ENV=production NEXT_TELEMETRY_DISABLED=1
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:19310/ || exit 1

CMD ["node", "server.js"]
