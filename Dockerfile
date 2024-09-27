FROM rust:1.81.0 AS builder
ADD . /app
WORKDIR /app
RUN cargo build --release

FROM debian:12
COPY --from=builder /app/target/release/recho /app/recho
ADD config /app/config
WORKDIR /app
ENTRYPOINT ["/app/recho"]
