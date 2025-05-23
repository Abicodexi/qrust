FROM rust:1.86-bullseye AS builder
WORKDIR /usr/src/qr_generator
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/
RUN cargo install --path . --locked --bin qr_web

FROM debian:bullseye-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    libssl1.1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/qr_web /usr/local/bin/qr_web
EXPOSE 8080
CMD ["qr_web"]