# Production multi-stage Dockerfile for ose-app-be (F# / Giraffe on .NET 10).
# Build context: repo root (e.g. `docker build -f apps/ose-app-be/Dockerfile .`).
#
# NOTE: the fsproj references generated OpenAPI contract types under
# apps/ose-app-be/generated-contracts/ (gitignored). Run `nx run ose-app-be:codegen`
# before building this image so the generated sources are present in the build context.

# Stage 1: restore + publish the release build with migrations embedded.
FROM mcr.microsoft.com/dotnet/sdk:10.0 AS build
WORKDIR /src

# Copy the project file first for restore-layer caching.
COPY apps/ose-app-be/src/OseAppBe/OseAppBe.fsproj apps/ose-app-be/src/OseAppBe/
COPY apps/ose-app-be/generated-contracts apps/ose-app-be/generated-contracts/
COPY apps/ose-app-be/db apps/ose-app-be/db/

RUN dotnet restore apps/ose-app-be/src/OseAppBe/OseAppBe.fsproj

# Copy the remaining sources and publish.
COPY apps/ose-app-be/src apps/ose-app-be/src/

RUN dotnet publish apps/ose-app-be/src/OseAppBe/OseAppBe.fsproj \
  -c Release -o /app/publish --no-restore

# Stage 2: minimal ASP.NET runtime image.
FROM mcr.microsoft.com/dotnet/aspnet:10.0 AS runtime
WORKDIR /app

COPY --from=build /app/publish .

# Default port — override at runtime via OSE_APP_BE_PORT.
ENV OSE_APP_BE_PORT=8302
EXPOSE 8302

ENTRYPOINT ["dotnet", "OseAppBe.dll"]
