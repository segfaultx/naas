FROM rust:1.97-alpine AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM node:26-alpine

COPY --from=builder /app/target/release/bw-backup-cli /app

EXPOSE 3000

ENTRYPOINT [ "/app" ]
