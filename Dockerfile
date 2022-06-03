# * --- Compile Meilisearch from source ---
FROM getmeili/meilisearch:v0.27.0 as meilisearch 

RUN cp /bin/meilisearch /home/meilisearch

# * --- Build Stage ---
FROM rust:1.60-alpine3.15 AS builder
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/sirin

RUN apk add --no-cache openssl-dev libgcc musl-dev

# Build project
COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo install --path .

# * --- Build Stage ---
FROM rust:1.60-alpine3.15 AS indexer

# RUN apk add --no-cache cmake libgcc openssl-dev musl-dev
RUN apk add --no-cache openssl-dev libgcc musl-dev

WORKDIR /usr/src/setup

# Build project
COPY setup/src src
COPY setup/Cargo.toml Cargo.toml
COPY setup/Cargo.lock Cargo.lock
COPY --from=meilisearch /home/meilisearch meilisearch

RUN CARGO_LOG=debug cargo run

# * --- Running Stage ---
# FROM frolvlad/alpine-glibc:alpine-3.15_glibc-2.34

# RUN apk add --no-cache bash build-base openssl-dev libgcc curl

FROM alpine:3.16

RUN apk add --no-cache libgcc

COPY --from=builder /usr/local/cargo/bin/sirin sirin
COPY --from=meilisearch /home/meilisearch meilisearch
COPY --from=indexer /usr/src/setup/data.ms data.ms
COPY ops/start.sh start.sh

RUN chmod 747 ./start.sh

CMD ["./start.sh"]

EXPOSE 7700 8080
