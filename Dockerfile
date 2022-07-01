# * --- Meilisearch from source ---
FROM getmeili/meilisearch:v0.27.0 as meilisearch 

RUN cp /bin/meilisearch /home/meilisearch

# * --- Build Stage ---
FROM rust:1.62-alpine3.16 AS builder
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/

# Setup tools for building
RUN apk add --no-cache musl-dev ca-certificates cmake musl-utils libressl-dev

# ? Create dummy project for package installation caching
RUN USER=root cargo new app
WORKDIR /usr/src/app

# Build project
COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo build --release

# * --- Build Stage ---
FROM rust:1.62-alpine3.16 AS indexer
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/

# Setup tools for building
RUN apk add --no-cache musl-dev ca-certificates cmake musl-utils libressl-dev openssl-dev zlib

# ? Create dummy project for package installation caching
RUN USER=root cargo new app
WORKDIR /usr/src/app

# Build project
COPY --from=meilisearch /home/meilisearch meilisearch

COPY ops/setup/data data
COPY ops/setup/src src
COPY ops/setup/Cargo.toml Cargo.toml
COPY ops/setup/Cargo.lock Cargo.lock

RUN cargo run --release

# * --- Running Stage ---
FROM alpine:3.16

RUN apk add --no-cache build-base openssl-dev libgcc curl

COPY --from=builder /usr/src/app/target/release/sirin sirin
COPY --from=meilisearch /home/meilisearch meilisearch
COPY --from=indexer /usr/src/app/data.ms data.ms

COPY ops/start.sh start.sh

RUN chmod 747 ./meilisearch
RUN chmod 747 ./start.sh

EXPOSE 8080

CMD ["./start.sh"]
