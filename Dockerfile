# * --- Build Stage ---
FROM rust:1.60-alpine3.15 AS builder
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/

RUN apk add --no-cache musl-dev ca-certificates cmake musl-utils libressl-dev

# Setup tools for building
RUN rustup target add x86_64-unknown-linux-musl

# ? Create dummy project for package installation caching
RUN USER=root cargo new sirin
WORKDIR /usr/src/sirin

# Build project
COPY . .

RUN RUSTFLAGS='-C target-cpu=native' cargo install --target x86_64-unknown-linux-musl --path .

# Download Meilisearch
RUN curl -L https://install.meilisearch.com | sh

# * --- Running Stage ---
FROM alpine

RUN apk add parallel

COPY --from=builder /usr/local/cargo/bin/sirin sirin
COPY --from=builder /usr/src/sirin/meilisearch meilisearch
COPY data data
COPY parallel.sh parallel.sh

EXPOSE 8080

CMD ["./parallel.sh", "\"./meilisearch\"", "\"./sirin\""]
