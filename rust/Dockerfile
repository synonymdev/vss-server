# Build stage
FROM rust:1.82-slim-bullseye AS builder

# Install system dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /build

# Copy Cargo files
COPY rust/Cargo.toml rust/Cargo.lock ./
COPY rust/api/Cargo.toml ./api/
COPY rust/auth-impls/Cargo.toml ./auth-impls/
COPY rust/impls/Cargo.toml ./impls/
COPY rust/server/Cargo.toml ./server/

# Copy source code
COPY rust/api/ ./api/
COPY rust/auth-impls/ ./auth-impls/
COPY rust/impls/ ./impls/
COPY rust/server/ ./server/
COPY proto/ ./proto/

# Build the application
RUN cargo build --release --bin vss-server

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl1.1 \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false vss

# Set working directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /build/target/release/vss-server /app/vss-server

# Copy default config
COPY rust/server/vss-server-config.toml /app/vss-server-config.toml

# Change ownership
RUN chown -R vss:vss /app

# Switch to app user
USER vss

# Expose port
EXPOSE 5050

# Run the application
CMD ["./vss-server", "/app/vss-server-config.toml"] 