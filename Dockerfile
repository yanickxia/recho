FROM rust:1.81.0 AS builder
RUN apt-get update && apt-get -y install protobuf-compiler
ADD . /app
WORKDIR /app
RUN cargo build --release

FROM debian:12
RUN apt-get update && apt-get install -y libssl3
COPY --from=builder /app/target/release/recho /app/recho
ADD config /app/config
WORKDIR /app
ENTRYPOINT ["/app/recho"]
