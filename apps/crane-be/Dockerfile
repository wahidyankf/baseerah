# Stage 1: Build
FROM mcr.microsoft.com/dotnet/sdk:10.0 AS build
WORKDIR /src

# Copy project files for layer caching
COPY libs/fsharp-crane-core/fsharp-crane-core.fsproj libs/fsharp-crane-core/
COPY apps/crane-be/crane-be.fsproj apps/crane-be/

# Restore dependencies as a separate layer
RUN dotnet restore apps/crane-be/crane-be.fsproj

# Copy sources
COPY libs/fsharp-crane-core/src libs/fsharp-crane-core/src/
COPY apps/crane-be/src apps/crane-be/src/
COPY apps/crane-be/tessdata apps/crane-be/tessdata/

# Publish self-contained
RUN dotnet publish apps/crane-be/crane-be.fsproj -c Release -o /app/publish --no-restore

# Stage 2: Runtime
FROM mcr.microsoft.com/dotnet/aspnet:10.0 AS runtime

# Install Tesseract OCR libs (TesseractOCR nuget), leptonica, and libgdiplus (PdfPig)
RUN apt-get update && apt-get install -y \
    tesseract-ocr \
    libtesseract-dev \
    libleptonica-dev \
    libgdiplus \
    libfontconfig1 \
    wget \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=build /app/publish .

# Default env — override at runtime
ENV CRANE_BE_PORT=8300
ENV CRANE_BE_ORGANICLEVER_NATS_URL=nats://localhost:4222
ENV CRANE_BE_OSE_APP_NATS_URL=nats://localhost:4223

EXPOSE 8300
ENTRYPOINT ["./crane-be"]
