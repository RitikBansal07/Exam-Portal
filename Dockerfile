FROM rust:latest

WORKDIR /app

# Copy source code
COPY . .

# Install system dependencies and sqlx-cli
RUN apt-get update && \
    apt-get install -y libmariadb-dev pkg-config libssl-dev && \
    rustup component add rustfmt

# ✅ Enable offline mode
ENV SQLX_OFFLINE=true

# Build and strip
RUN cargo build --release && \
    strip target/release/exam-portal

EXPOSE 8080

CMD ["./target/release/exam-portal"]
