FROM rust:1.81-slim AS builder
RUN apt-get update && apt-get install musl-tools -y && rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/app
COPY . .
RUN cargo fetch
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM debian:bookworm-slim
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/task-manager /usr/local/bin/task-manager
COPY .env ./
EXPOSE 3000
CMD ["task-manager"]