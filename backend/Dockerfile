# Build stage
FROM rust:1.83-slim as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY backend ./backend

# Build release binary
RUN cargo build --release --bin omni-server

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/omni-server /app/omni-server

# Create data directory
RUN mkdir -p /app/data/config.d /app/data/clients.d /app/data/servers.d

# Expose port
EXPOSE 8080

# Run the server
CMD ["/app/omni-server"]
