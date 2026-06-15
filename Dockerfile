FROM rust:1.89 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder \
    /app/target/release/certificate-service \
    .

CMD ["./certificate-service"]