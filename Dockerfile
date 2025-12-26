# Build stage
FROM rust:1.83-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests and source
COPY Cargo.toml Cargo.lock* ./
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/rustbridge /app/rustbridge

# Copy default config
COPY config.yaml /app/config.yaml

# Create non-root user
RUN useradd -m -u 1000 rustbridge && \
    chown -R rustbridge:rustbridge /app

USER rustbridge

# Expose ports
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Run
ENV RUST_LOG=info
ENV RUSTBRIDGE_CONFIG=/app/config.yaml

CMD ["/app/rustbridge"]
