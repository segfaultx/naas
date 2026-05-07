FROM rust:1.94-alpine AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM node:24-alpine

COPY --from=builder /app/target/release/bw-backup-cli /app

EXPOSE 3000

ENTRYPOINT [ "/app" ]
