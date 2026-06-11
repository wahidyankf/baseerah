# Production multi-stage Dockerfile for ose-app-be.
# Build context: repo root (e.g. `docker build -f apps/ose-app-be/Dockerfile .`).
#
# Stage 1: compile the release binary with all migration files embedded.
FROM rust:1.88-slim AS builder
WORKDIR /build

# Install build-time dependencies for reqwest TLS (OpenSSL) and pkg-config.
RUN apt-get update \
  && apt-get install -y --no-install-recommends pkg-config libssl-dev \
  && rm -rf /var/lib/apt/lists/*

COPY apps/ose-app-be/Cargo.toml apps/ose-app-be/Cargo.lock ./
COPY apps/ose-app-be/src ./src
COPY apps/ose-app-be/migrations ./migrations

RUN cargo build --release --bin ose-app-be

# Stage 2: minimal runtime image.
FROM debian:bookworm-slim AS runtime

# ca-certificates: TLS trust store for outbound HTTPS (reqwest).
# libssl3: OpenSSL shared library required by the dynamically linked binary.
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates libssl3 \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/target/release/ose-app-be /usr/local/bin/ose-app-be

EXPOSE 8302
CMD ["/usr/local/bin/ose-app-be"]
