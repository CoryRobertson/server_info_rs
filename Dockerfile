FROM rust:1.61 AS builder
COPY . .
WORKDIR "/server_info_server_rs"
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /server_info_server_rs/target/release/server_info_rs ./target/release/server_info_rs
EXPOSE 8111
CMD ["./target/release/server_info_rs"]