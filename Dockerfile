# Multi-stage Dockerfile for AI Agent Service

# Build stage
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Create dummy main files to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn main() {}" > src/server/main.rs && \
    echo "" > src/lib.rs

# Build dependencies
RUN cargo build --release && \
    rm -rf src

# Copy source code
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd --create-home --shell /bin/bash app

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/ai-agent-server /usr/local/bin/ai-agent-server
COPY --from=builder /app/target/release/ai-agent /usr/local/bin/ai-agent

# Copy configuration files
COPY config.toml /app/config.toml
COPY examples/docker-compose.yml /app/docker-compose.yml

# Create workspace directory
RUN mkdir /workspace && chown app:app /workspace

# Switch to app user
USER app

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Set environment variables
ENV AI_AGENT_CONFIG_FILE=/app/config.toml
ENV AI_AGENT_BIND_ADDRESS=0.0.0.0:8080
ENV AI_AGENT_LOG_LEVEL=info
ENV RUST_LOG=info

# Run the application
CMD ["ai-agent-server"]