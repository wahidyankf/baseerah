# Production multi-stage Dockerfile for beaver-nest-be (F# / Giraffe on .NET 10).
# Build context: repo root (e.g. `docker build -f apps/beaver-nest-be/Dockerfile .`).

# Stage 1: restore + publish the release build.
FROM mcr.microsoft.com/dotnet/sdk:10.0 AS build
WORKDIR /src

# Copy the project file first for restore-layer caching.
COPY apps/beaver-nest-be/src/BeaverNestBe/BeaverNestBe.fsproj apps/beaver-nest-be/src/BeaverNestBe/

RUN dotnet restore apps/beaver-nest-be/src/BeaverNestBe/BeaverNestBe.fsproj

# Copy the remaining sources and publish.
COPY apps/beaver-nest-be/src apps/beaver-nest-be/src/

RUN dotnet publish apps/beaver-nest-be/src/BeaverNestBe/BeaverNestBe.fsproj \
  -c Release -o /app/publish --no-restore

# Stage 2: minimal ASP.NET runtime image.
FROM mcr.microsoft.com/dotnet/aspnet:10.0 AS runtime
WORKDIR /app

COPY --from=build /app/publish .

# Container listener defaults. Host binding is explicit because containers do not
# expose the loopback interface to their published port.
ENV BEAVER_NEST_BE_HTTP_LISTEN_ADDRESS=0.0.0.0
ENV BEAVER_NEST_BE_HTTP_LISTEN_PORT=19300
EXPOSE 19300

ENTRYPOINT ["dotnet", "BeaverNestBe.dll"]
