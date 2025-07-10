# --- Build Stage ---
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY . .

# SQLX OFFLINE
ENV SQLX_OFFLINE=true

# Install static linking dependencies
RUN apt-get update && \
    apt-get install -y musl-tools pkg-config libssl-dev libmariadb-dev && \
    cargo build --release --target x86_64-unknown-linux-musl && \
    strip target/x86_64-unknown-linux-musl/release/exam-portal

# --- Runtime Stage ---
FROM scratch

WORKDIR /app

# Copy binary and .env file
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/exam-portal .
COPY .env .

EXPOSE 8080
CMD ["./exam-portal"]

