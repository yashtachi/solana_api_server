# Use the official Rust image
FROM rust:1.75 as builder

# Create app directory
WORKDIR /usr/src/app

# Copy manifest files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install CA certificates for HTTPS
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false appuser

# Copy the binary from builder stage
COPY --from=builder /usr/src/app/target/release/solana_api_server /usr/local/bin/solana_api_server

# Change to app user
USER appuser

# Expose port
EXPOSE 3000

# Run the binary
CMD ["solana_api_server"]
