FROM rust:1.62 as builder
WORKDIR /usr/src/rust-gomoku
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/rust-gomoku /usr/local/bin/rust-gomoku
CMD ["rust-gomoku"]
