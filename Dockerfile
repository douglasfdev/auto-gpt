# Build stage
FROM rust:1.73-alpine as builder

WORKDIR /app

ENV RUSTFLAGS="-C target-feature=-crt-static"
ENV OPENSSL_DIR=/usr

COPY . .
COPY ./llm_proc_macro ./llm_proc_macro

RUN apk add --no-cache musl-dev build-base openssl-dev pkgconfig

RUN cargo build --release

# Production
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/auto-gpt .

CMD ["./auto-gpt"]
