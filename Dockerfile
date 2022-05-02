# * --- Build Stage ---
FROM rust:1.60-alpine3.15 AS builder
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/

# Setup tools for building
RUN rustup target add x86_64-unknown-linux-musl

RUN apk add --no-cache musl-dev ca-certificates cmake musl-utils libressl-dev

# ? Create dummy project for package installation caching
RUN USER=root cargo new sirin
WORKDIR /usr/src/sirin

# Build project
COPY data data
COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN RUSTFLAGS='-C target-cpu=native -C prefer-dynamic' cargo install --target x86_64-unknown-linux-musl --path .

# * --- Running Stage ---
FROM frolvlad/alpine-glibc:alpine-3.15_glibc-2.34

RUN apk add --no-cache bash build-base openssl-dev libgcc curl

COPY --from=builder /usr/local/cargo/bin/sirin sirin
COPY meilisearch meilisearch
COPY data data

COPY parallel.sh parallel.sh
COPY start.sh start.sh

RUN chmod 777 ./start.sh

EXPOSE 7700 8080

CMD ["./sirin"]
