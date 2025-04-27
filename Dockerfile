FROM rust:1.86-bullseye AS builder
WORKDIR /usr/src/qr_generator
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/
RUN cargo install --path . --locked --bin qr_web

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/qr_web /usr/local/bin/qr_web
EXPOSE 8080
CMD ["qr_web"]