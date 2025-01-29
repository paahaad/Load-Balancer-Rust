FROM rust:1.68 as builder

WORKDIR /usr/src/pingora-lb
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/pingora-lb/target/release/pingora-lb /usr/local/bin/pingora-lb

CMD ["pingora-lb"]