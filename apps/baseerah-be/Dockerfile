# Production multi-stage Dockerfile for baseerah-be (F# / Giraffe on .NET 10).
# Build context: repo root (e.g. `docker build -f apps/baseerah-be/Dockerfile .`).

# Stage 1: restore + publish the release build.
FROM mcr.microsoft.com/dotnet/sdk:10.0 AS build
WORKDIR /src

# Copy the project file first for restore-layer caching.
COPY apps/baseerah-be/src/BaseerahBe/BaseerahBe.fsproj apps/baseerah-be/src/BaseerahBe/

RUN dotnet restore apps/baseerah-be/src/BaseerahBe/BaseerahBe.fsproj

# Copy the remaining sources and publish.
COPY apps/baseerah-be/src apps/baseerah-be/src/

RUN dotnet publish apps/baseerah-be/src/BaseerahBe/BaseerahBe.fsproj \
  -c Release -o /app/publish --no-restore

# Stage 2: minimal ASP.NET runtime image.
FROM mcr.microsoft.com/dotnet/aspnet:10.0 AS runtime
WORKDIR /app

COPY --from=build /app/publish .

# Default port — override at runtime via BASEERAH_BE_PORT.
ENV BASEERAH_BE_PORT=19320
EXPOSE 19320

ENTRYPOINT ["dotnet", "BaseerahBe.dll"]
