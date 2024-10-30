FROM rust:1.80-slim AS builder
RUN apt-get update && \
  apt-get install -y pkg-config make g++ libssl-dev cmake libmariadb-dev-compat zlib1g && \
  rustup target add x86_64-unknown-linux-gnu
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt install -y openssl
WORKDIR /app
COPY --from=builder /app/target/release/task-manager /app/task-manager
COPY --from=builder /app/.env /app/.env
CMD ["./task-manager"]