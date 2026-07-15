# syntax = docker/dockerfile:1

FROM rust:1.86-slim AS build
WORKDIR /app

# sqlx compiles queries against the committed .sqlx offline data — no database
# is needed at build time.
ENV SQLX_OFFLINE=true

COPY Cargo.toml Cargo.lock ./
COPY .sqlx ./.sqlx
COPY src ./src
COPY migrations ./migrations
RUN cargo build --release

FROM debian:bookworm-slim AS final
RUN apt-get update -qq \
    && apt-get install --no-install-recommends -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/seemycity-backend /usr/local/bin/seemycity-backend

# Containers must listen on all interfaces; Fly routes to internal_port.
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=4000
EXPOSE 4000

CMD ["seemycity-backend"]
