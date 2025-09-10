FROM rust:1.75-buster as builder

WORKDIR /app

COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/rust-crud-api .

CMD ["./rust-crud-api"]

