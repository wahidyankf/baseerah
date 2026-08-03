# Production multi-stage Dockerfile for the combined BeaverNest runtime.
# Build context: repo root (e.g. `docker build -f apps/beaver-nest-be/Dockerfile .`).

FROM node:24.16.0-alpine3.23@sha256:2bdb65ed1dab192432bc31c95f94155ca5ad7fc1392fb7eb7526ab682fa5bf14 AS frontend-build
WORKDIR /workspace

# Keep dependency installation cacheable while retaining the local workspace
# packages required by the Vite client.
COPY package.json package-lock.json nx.json tsconfig.base.json ./
COPY apps/beaver-nest-fe/package.json apps/beaver-nest-fe/package.json
COPY libs/web-ui/package.json libs/web-ui/package.json
COPY libs/web-ui-token/package.json libs/web-ui-token/package.json
RUN npm ci --workspace=beaver-nest-fe --include-workspace-root --ignore-scripts

# Build the contract bundle and generated frontend client from source; the
# final image never relies on a host-generated client or frontend artifact.
COPY specs/apps/beaver-nest/containers/contracts/ specs/apps/beaver-nest/containers/contracts/
COPY apps/beaver-nest-fe/ apps/beaver-nest-fe/
COPY libs/web-ui/ libs/web-ui/
COPY libs/web-ui-token/ libs/web-ui-token/
RUN npm exec -- nx run beaver-nest-contracts:bundle \
  && npm exec -- nx run beaver-nest-fe:codegen \
  && npm exec -- nx run beaver-nest-fe:build

FROM mcr.microsoft.com/dotnet/sdk:10.0.302-noble@sha256:72dd743782f2ae7e5476fd64f6a460045e3998dc862218b80e6944cba79a01b0 AS backend-build
WORKDIR /src

# Copy the project file first for restore-layer caching.
COPY apps/beaver-nest-be/global.json apps/beaver-nest-be/dotnet-tools.json apps/beaver-nest-be/.editorconfig ./
COPY apps/beaver-nest-be/src/BeaverNestBe/BeaverNestBe.fsproj apps/beaver-nest-be/src/BeaverNestBe/
RUN dotnet restore apps/beaver-nest-be/src/BeaverNestBe/BeaverNestBe.fsproj

COPY apps/beaver-nest-be/src/ apps/beaver-nest-be/src/
RUN dotnet publish apps/beaver-nest-be/src/BeaverNestBe/BeaverNestBe.fsproj \
  --configuration Release \
  --output /app/publish \
  --no-restore

FROM mcr.microsoft.com/dotnet/aspnet:10.0.10-noble@sha256:f1126d438ccc359f51cc6d4701a8deae513856cf10f5fe645d29ea6403dcac6b AS runtime
WORKDIR /app

RUN groupadd --gid 10001 beaver-nest \
  && useradd --uid 10001 --gid 10001 --create-home --shell /usr/sbin/nologin beaver-nest \
  && install --directory --owner=10001 --group=10001 --mode=0700 /var/lib/beaver-nest

COPY --from=backend-build --chown=10001:10001 /app/publish/ ./
COPY --from=frontend-build --chown=10001:10001 /workspace/apps/beaver-nest-fe/dist/ ./wwwroot/
COPY --chown=10001:10001 --chmod=0755 apps/beaver-nest-be/scripts/container-entrypoint.sh ./container-entrypoint.sh

ENV DOTNET_RUNNING_IN_CONTAINER=true \
  BEAVER_NEST_BE_DATA_DIRECTORY=/var/lib/beaver-nest \
  BEAVER_NEST_BE_HTTP_LISTEN_ADDRESS=0.0.0.0 \
  BEAVER_NEST_BE_HTTP_LISTEN_PORT=19300
EXPOSE 19300
USER 10001:10001
ENTRYPOINT ["/app/container-entrypoint.sh"]
CMD ["dotnet", "BeaverNestBe.dll"]
