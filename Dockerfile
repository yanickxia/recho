FROM rust:1.81.0 as builder
ADD . /app
WORKDIR /app
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/recho /app/recho
ADD config /app/config
WORKDIR /app
ENTRYPOINT /app