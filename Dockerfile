FROM rust:1.75-bookworm as builder
WORKDIR /usr/src/ord
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/ord/target/release/ord /usr/local/bin/ord
ENTRYPOINT ["ord"]