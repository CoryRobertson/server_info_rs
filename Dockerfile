FROM rust:1.61 AS builder
COPY . .
RUN cargo build --release --package server_info_server_rs --bin server_info_server_rs

FROM debian:buster-slim
COPY --from=builder /target/release/server_info_server_rs ./target/release/server_info_server_rs
EXPOSE 8111
CMD ["./target/release/server_info_server_rs"]
