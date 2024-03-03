# Build stage
FROM rust:1.69 as builder

WORKDIR /app

COPY . .

RUN cargo build --release

# Production
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/auto-gpt .

CMD ["./auto-gpt"]