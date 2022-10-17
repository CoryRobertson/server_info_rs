FROM rust:1.61
COPY . .
WORKDIR "/server_info_server_rs"
RUN cargo build --release
EXPOSE 8111
CMD ["./target/release/server_info_rs"]